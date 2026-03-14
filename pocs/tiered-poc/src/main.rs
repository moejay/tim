/// Option 1: Tiered Rendering — single viuer pipeline, adaptive quality.
///
/// Detects terminal graphics protocol at startup:
///   - Kitty/Sixel/iTerm2 → full 640×360 canvas, anti-aliased circles, glow, trails
///   - Half-block fallback → smaller 160×90 canvas, simplified shapes, no glow
///
/// Same physics, same code path — just different canvas size and effect complexity.

use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use viuer::Config;

// ─── Quality Tiers ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tier {
    /// Kitty / Sixel / iTerm2 — full pixel fidelity
    High,
    /// Unicode half-block fallback — reduced resolution, simplified effects
    Low,
}

impl Tier {
    fn label(self) -> &'static str {
        match self {
            Self::High => "Pixel (Kitty/Sixel)",
            Self::Low  => "Half-block (fallback)",
        }
    }

    fn canvas_size(self) -> (u32, u32) {
        match self {
            Self::High => (640, 360),
            Self::Low  => (160, 90),
        }
    }

    fn ball_radius(self) -> f32 {
        match self {
            Self::High => 10.0,
            Self::Low  => 3.0,
        }
    }

    fn fps(self) -> f64 {
        match self {
            Self::High => 60.0,
            Self::Low  => 30.0,
        }
    }

    fn trail_len(self) -> usize {
        match self {
            Self::High => 60,
            Self::Low  => 15,
        }
    }
}

fn detect_tier() -> Tier {
    // Check for Kitty
    if std::env::var("TERM").as_deref() == Ok("xterm-kitty")
        || std::env::var("KITTY_WINDOW_ID").is_ok()
    {
        return Tier::High;
    }
    // Check for WezTerm
    if std::env::var("TERM_PROGRAM").as_deref() == Ok("WezTerm") {
        return Tier::High;
    }
    // Check for Ghostty
    if std::env::var("TERM_PROGRAM").as_deref() == Ok("ghostty") {
        return Tier::High;
    }
    // Check for iTerm2
    if std::env::var("TERM_PROGRAM").as_deref() == Ok("iTerm.app") {
        return Tier::High;
    }
    // Check SIXEL support via TERM (some terminals advertise it)
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("sixel") {
            return Tier::High;
        }
    }
    Tier::Low
}

// ─── Physics ────────────────────────────────────────────────────────────────

const GRAVITY: f32 = 900.0; // px/s² (at High tier scale)
const RESTITUTION: f32 = 0.78;

struct State {
    // Positions stored in normalized [0..1] coordinates, scaled to canvas on render
    pos: (f32, f32),
    vel: (f32, f32),
    active: bool,
    trail: Vec<(f32, f32)>,
    bounces: u32,
    angle_deg: f32,
    power: f32,
    tier: Tier,
    frame: u64,
}

impl State {
    fn new(tier: Tier) -> Self {
        Self {
            pos: (0.125, 0.5),
            vel: (0.0, 0.0),
            active: false,
            trail: Vec::new(),
            bounces: 0,
            angle_deg: -30.0,
            power: 800.0,
            tier,
            frame: 0,
        }
    }
}

fn fire(s: &mut State) {
    let (w, h) = s.tier.canvas_size();
    s.active = true;
    s.trail.clear();
    s.bounces = 0;
    s.pos = (80.0 / w as f32, 0.5);
    let rad = s.angle_deg.to_radians();
    // Velocity in canvas-pixels/sec (High tier scale), normalized
    s.vel = (
        s.power * rad.cos() / w as f32,
        s.power * rad.sin() / h as f32,
    );
}

fn update(s: &mut State, dt: f32) {
    if !s.active { return; }
    s.frame += 1;

    s.trail.push(s.pos);
    let max_trail = s.tier.trail_len();
    if s.trail.len() > max_trail { s.trail.remove(0); }

    let (_w, h) = s.tier.canvas_size();
    // Gravity normalized to canvas height
    s.vel.1 += (GRAVITY / h as f32) * dt;
    s.pos.0 += s.vel.0 * dt;
    s.pos.1 += s.vel.1 * dt;

    let r_norm = s.tier.ball_radius() / h as f32;
    let r = RESTITUTION;

    if s.pos.0 < r_norm         { s.pos.0 = r_norm;         s.vel.0 =  s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.0 > 1.0 - r_norm   { s.pos.0 = 1.0 - r_norm;   s.vel.0 = -s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.1 < r_norm         { s.pos.1 = r_norm;         s.vel.1 =  s.vel.1.abs() * r; s.bounces += 1; }
    if s.pos.1 > 1.0 - r_norm   { s.pos.1 = 1.0 - r_norm;   s.vel.1 = -s.vel.1.abs() * r; s.bounces += 1; }

    let spd = (s.vel.0 * s.vel.0 + s.vel.1 * s.vel.1).sqrt();
    if spd < 0.005 { s.vel = (0.0, 0.0); }
}

// ─── GFX helpers ────────────────────────────────────────────────────────────

#[inline]
fn over(px: &mut Rgba<u8>, r: u8, g: u8, b: u8, a: f32) {
    let a = a.clamp(0.0, 1.0);
    let ia = 1.0 - a;
    px[0] = (r as f32 * a + px[0] as f32 * ia) as u8;
    px[1] = (g as f32 * a + px[1] as f32 * ia) as u8;
    px[2] = (b as f32 * a + px[2] as f32 * ia) as u8;
    px[3] = 255;
}

#[inline]
fn blend(px: &mut Rgba<u8>, r: u8, g: u8, b: u8, a: f32) {
    let a = a.clamp(0.0, 1.0);
    px[0] = (px[0] as f32 + r as f32 * a).min(255.0) as u8;
    px[1] = (px[1] as f32 + g as f32 * a).min(255.0) as u8;
    px[2] = (px[2] as f32 + b as f32 * a).min(255.0) as u8;
    px[3] = 255;
}

fn fill_circle(img: &mut RgbaImage, cx: f32, cy: f32, r: f32, col: [u8; 3], alpha: f32) {
    let x0 = ((cx - r) as i32).max(0) as u32;
    let x1 = ((cx + r) as i32 + 1).min(img.width() as i32) as u32;
    let y0 = ((cy - r) as i32).max(0) as u32;
    let y1 = ((cy + r) as i32 + 1).min(img.height() as i32) as u32;
    let r2 = r * r;
    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let d2 = dx * dx + dy * dy;
            if d2 <= r2 {
                let edge = (r2 - d2).sqrt().min(1.0);
                over(img.get_pixel_mut(x, y), col[0], col[1], col[2], alpha * edge);
            }
        }
    }
}

fn radial_glow(img: &mut RgbaImage, cx: f32, cy: f32, max_r: f32, col: [u8; 3], strength: f32) {
    let x0 = ((cx - max_r) as i32).max(0) as u32;
    let x1 = ((cx + max_r) as i32 + 1).min(img.width() as i32) as u32;
    let y0 = ((cy - max_r) as i32).max(0) as u32;
    let y1 = ((cy + max_r) as i32 + 1).min(img.height() as i32) as u32;
    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist >= max_r { continue; }
            let t = 1.0 - dist / max_r;
            let a = t * t * strength;
            blend(img.get_pixel_mut(x, y), col[0], col[1], col[2], a);
        }
    }
}

fn fill_rect(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, col: [u8; 3], alpha: f32) {
    let x0 = x.max(0) as u32;
    let x1 = (x + w).min(img.width() as i32) as u32;
    let y0 = y.max(0) as u32;
    let y1 = (y + h).min(img.height() as i32) as u32;
    for py in y0..y1 {
        for px in x0..x1 {
            over(img.get_pixel_mut(px, py), col[0], col[1], col[2], alpha);
        }
    }
}

fn glowing_line(
    img: &mut RgbaImage,
    x0: f32, y0: f32, x1: f32, y1: f32,
    core: [u8; 3], glow: [u8; 3],
    core_w: f32, glow_w: f32, intensity: f32,
) {
    let len = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
    if len == 0.0 { return; }
    let steps = (len * 2.0) as u32 + 1;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let px = x0 + (x1 - x0) * t;
        let py = y0 + (y1 - y0) * t;
        radial_glow(img, px, py, glow_w, glow, intensity * 0.25);
        fill_circle(img, px, py, core_w, core, intensity);
    }
}

// Simple thick line for low-tier (no glow passes)
fn thick_line(img: &mut RgbaImage, x0: f32, y0: f32, x1: f32, y1: f32, col: [u8; 3], width: f32) {
    let len = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
    if len == 0.0 { return; }
    let steps = (len * 2.0) as u32 + 1;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let px = x0 + (x1 - x0) * t;
        let py = y0 + (y1 - y0) * t;
        fill_circle(img, px, py, width, col, 1.0);
    }
}

// ─── Render (tier-aware) ────────────────────────────────────────────────────

fn render(s: &State) -> DynamicImage {
    let (w, h) = s.tier.canvas_size();
    let mut img = ImageBuffer::from_pixel(w, h, Rgba([8u8, 8, 12, 255]));

    let ball_r = s.tier.ball_radius();

    // ── Grid ────────────────────────────────────────────────────────────
    let grid_step = match s.tier {
        Tier::High => 40u32,
        Tier::Low  => 10,
    };
    for x in (0..w).step_by(grid_step as usize) {
        for y in 0..h {
            let px = img.get_pixel_mut(x, y);
            px[0] = px[0].saturating_add(6);
            px[1] = px[1].saturating_add(6);
            px[2] = px[2].saturating_add(6);
        }
    }
    for y in (0..h).step_by(grid_step as usize) {
        for x in 0..w {
            let px = img.get_pixel_mut(x, y);
            px[0] = px[0].saturating_add(6);
            px[1] = px[1].saturating_add(6);
            px[2] = px[2].saturating_add(6);
        }
    }

    // ── Border walls ────────────────────────────────────────────────────
    let wall_t = match s.tier { Tier::High => 3u32, Tier::Low => 1 };
    for x in 0..w {
        for t in 0..wall_t {
            over(img.get_pixel_mut(x, t), 60, 80, 100, 0.9);
            over(img.get_pixel_mut(x, h - 1 - t), 60, 80, 100, 0.9);
        }
    }
    for y in 0..h {
        for t in 0..wall_t {
            over(img.get_pixel_mut(t, y), 60, 80, 100, 0.9);
            over(img.get_pixel_mut(w - 1 - t, y), 60, 80, 100, 0.9);
        }
    }

    // ── Cannon body ─────────────────────────────────────────────────────
    let cy = h as f32 / 2.0;
    let (body_x, body_w, body_h) = match s.tier {
        Tier::High => (20i32, 44i32, 48i32),
        Tier::Low  => (5i32, 11i32, 12i32),
    };
    fill_rect(&mut img, body_x, cy as i32 - body_h / 2, body_w, body_h, [120, 100, 40], 1.0);

    // Barrel
    let rad = s.angle_deg.to_radians();
    let bx = body_x as f32 + body_w as f32;
    let barrel_len = match s.tier { Tier::High => 50.0, Tier::Low => 12.0 };
    let ex = bx + rad.cos() * barrel_len;
    let ey = cy + rad.sin() * barrel_len;

    match s.tier {
        Tier::High => {
            glowing_line(&mut img, bx, cy, ex, ey,
                [200, 170, 60], [120, 100, 30], 5.0, 10.0, 1.0);
        }
        Tier::Low => {
            thick_line(&mut img, bx, cy, ex, ey, [200, 170, 60], 2.0);
        }
    }

    // ── Preview arc ─────────────────────────────────────────────────────
    if !s.active {
        let mut px = ex; let mut py = ey;
        let vx = s.power * rad.cos();
        let mut vy = s.power * rad.sin();
        let g = GRAVITY;
        let steps = match s.tier { Tier::High => 80u32, Tier::Low => 30 };
        let dot_r = match s.tier { Tier::High => 2.0, Tier::Low => 1.0 };
        for i in 0..steps {
            vy += g * 0.016;
            let scale_x = w as f32 / 640.0;
            let scale_y = h as f32 / 360.0;
            px += vx * 0.016 * scale_x;
            py += vy * 0.016 * scale_y;
            let t = i as f32 / steps as f32;
            let a = (1.0 - t) * 0.35;
            if px >= 0.0 && px < w as f32 && py >= 0.0 && py < h as f32 {
                fill_circle(&mut img, px, py, dot_r, [100, 200, 255], a);
            }
            if px < ball_r || px > w as f32 - ball_r || py < ball_r || py > h as f32 - ball_r {
                break;
            }
        }
    }

    // ── Trail ───────────────────────────────────────────────────────────
    let trail_len = s.trail.len();
    for (i, &(tx, ty)) in s.trail.iter().enumerate() {
        let frac = i as f32 / trail_len.max(1) as f32;
        let tx_px = tx * w as f32;
        let ty_px = ty * h as f32;
        let r_size = ball_r * frac * 0.7;
        let alpha = frac * frac * 0.5;
        let col = if frac > 0.6 {
            [255u8, 180, 60]
        } else if frac > 0.3 {
            [200, 120, 200]
        } else {
            [80, 80, 160]
        };
        fill_circle(&mut img, tx_px, ty_px, r_size, col, alpha);
    }

    // ── Cannonball ──────────────────────────────────────────────────────
    if s.active {
        let bx = s.pos.0 * w as f32;
        let by = s.pos.1 * h as f32;

        match s.tier {
            Tier::High => {
                // Full glow stack
                radial_glow(&mut img, bx, by, ball_r * 5.0, [255, 120, 20], 0.4);
                radial_glow(&mut img, bx, by, ball_r * 2.5, [255, 200, 80], 0.6);
                fill_circle(&mut img, bx, by, ball_r, [60, 60, 70], 1.0);
                fill_circle(&mut img, bx, by - ball_r * 0.3, ball_r * 0.75, [200, 200, 220], 0.85);
                fill_circle(&mut img, bx - ball_r * 0.35, by - ball_r * 0.4, ball_r * 0.3, [255, 255, 255], 1.0);
            }
            Tier::Low => {
                // Simple solid circle, no glow
                fill_circle(&mut img, bx, by, ball_r, [220, 80, 40], 1.0);
                fill_circle(&mut img, bx - 1.0, by - 1.0, 1.0, [255, 255, 255], 0.8);
            }
        }
    }

    // ── Ramp (static obstacle) ──────────────────────────────────────────
    let (rx, ry, rw, rh) = match s.tier {
        Tier::High => (350i32, 260i32, 120i32, 8i32),
        Tier::Low  => (88i32, 65i32, 30i32, 2i32),
    };
    fill_rect(&mut img, rx, ry, rw, rh, [80, 140, 80], 1.0);

    DynamicImage::ImageRgba8(img)
}

// ─── Main ───────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tier = detect_tier();
    let (term_w, term_h) = terminal::size()?;

    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    // Show tier info briefly
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    print!("Detected rendering tier: {}", tier.label());
    print!("\nCanvas: {}×{} @ {}fps", tier.canvas_size().0, tier.canvas_size().1, tier.fps());
    print!("\nPress any key to start...");
    stdout().flush()?;
    loop {
        if event::poll(Duration::from_millis(100))? {
            let _ = event::read()?;
            break;
        }
    }
    execute!(stdout(), terminal::Clear(ClearType::All))?;

    let cfg = Config {
        x: 0, y: 0,
        width: Some(term_w as u32),
        height: Some(term_h.saturating_sub(3) as u32),
        ..Default::default()
    };

    let mut state = State::new(tier);
    let frame_dur = Duration::from_secs_f64(1.0 / tier.fps());
    let mut last = Instant::now();

    loop {
        // ── Input ───────────────────────────────────────────────────────
        while event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release { continue; }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
                        terminal::disable_raw_mode()?;
                        return Ok(());
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => fire(&mut state),
                    KeyCode::Char('r') => {
                        state.active = false;
                        state.trail.clear();
                        state.bounces = 0;
                    }
                    KeyCode::Up   => state.angle_deg = (state.angle_deg - 5.0).clamp(-85.0, 85.0),
                    KeyCode::Down => state.angle_deg = (state.angle_deg + 5.0).clamp(-85.0, 85.0),
                    KeyCode::Right => state.power = (state.power + 50.0).min(1800.0),
                    KeyCode::Left  => state.power = (state.power - 50.0).max(100.0),
                    _ => {}
                }
            }
        }

        // ── Physics ─────────────────────────────────────────────────────
        let now = Instant::now();
        let dt = now.duration_since(last).as_secs_f32().min(0.05);
        last = now;
        update(&mut state, dt);

        // ── Draw ────────────────────────────────────────────────────────
        let img = render(&state);
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        viuer::print(&img, &cfg)?;

        // HUD
        execute!(stdout(), cursor::MoveTo(0, term_h - 3))?;
        print!(
            " Tier:{:<22} Bounces:{:>3}  Angle:{:>4.0}°  Power:{:>5.0}",
            tier.label(), state.bounces, state.angle_deg, state.power
        );
        execute!(stdout(), cursor::MoveTo(0, term_h - 2))?;
        print!(" [SPACE] Fire  [R] Reset  [Up/Down] Angle  [Left/Right] Power  [Q] Quit");
        stdout().flush()?;

        let elapsed = Instant::now().duration_since(now);
        if elapsed < frame_dur {
            std::thread::sleep(frame_dur - elapsed);
        }
    }
}
