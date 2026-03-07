/// Cannon physics demo — Sixel/Kitty pixel rendering.
/// Cannonball rendered as a smooth anti-aliased sphere with glow + trail.
mod gfx;

use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use image::{DynamicImage, ImageBuffer, Rgba};
use viuer::Config;

// ─── Canvas ───────────────────────────────────────────────────────────────────
const W: u32 = 640;
const H: u32 = 360;
const FPS: f64 = 60.0;
const BALL_R: f32 = 10.0;

// Physics: gravity in px/s²
const G_NORMAL: f32  =  900.0;
const G_MOON: f32    =  150.0;

// ─── State ────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum GravityMode { Normal, Moon, Flipped, Zero }

impl GravityMode {
    fn accel(self) -> f32 {
        match self { Self::Normal => G_NORMAL, Self::Moon => G_MOON,
                     Self::Flipped => -G_NORMAL, Self::Zero => 0.0 }
    }
    fn label(self) -> &'static str {
        match self { Self::Normal => "Normal", Self::Moon => "Moon",
                     Self::Flipped => "Flipped", Self::Zero => "Zero-G" }
    }
}

struct State {
    pos: (f32, f32),
    vel: (f32, f32),
    active: bool,
    trail: Vec<(f32, f32)>,
    bounces: u32,
    angle_deg: f32,
    power: f32,
    gravity: GravityMode,
    restitution: f32,
    elapsed: f32,
    max_speed: f32,
    frame: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            pos: (80.0, H as f32 / 2.0),
            vel: (0.0, 0.0),
            active: false,
            trail: Vec::new(),
            bounces: 0,
            angle_deg: -30.0,
            power: 800.0,   // px/s
            gravity: GravityMode::Normal,
            restitution: 0.78,
            elapsed: 0.0,
            max_speed: 0.0,
            frame: 0,
        }
    }
}

// ─── Physics ─────────────────────────────────────────────────────────────────
fn fire(s: &mut State) {
    s.active = true;
    s.trail.clear();
    s.bounces = 0;
    s.pos = (80.0, H as f32 / 2.0);
    let rad = s.angle_deg.to_radians();
    s.vel = (s.power * rad.cos(), s.power * rad.sin());
}

fn update(s: &mut State, dt: f32) {
    if !s.active { return; }
    s.elapsed += dt;
    s.frame += 1;

    let cur = s.pos;
    s.trail.push(cur);
    if s.trail.len() > 60 { s.trail.remove(0); }

    s.vel.1 += s.gravity.accel() * dt;
    s.pos.0 += s.vel.0 * dt;
    s.pos.1 += s.vel.1 * dt;

    let spd = (s.vel.0 * s.vel.0 + s.vel.1 * s.vel.1).sqrt();
    if spd > s.max_speed { s.max_speed = spd; }

    let r = s.restitution;
    let x0 = BALL_R; let x1 = W as f32 - BALL_R;
    let y0 = BALL_R; let y1 = H as f32 - BALL_R;
    if s.pos.0 < x0 { s.pos.0 = x0; s.vel.0 =  s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.0 > x1 { s.pos.0 = x1; s.vel.0 = -s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.1 < y0 { s.pos.1 = y0; s.vel.1 =  s.vel.1.abs() * r; s.bounces += 1; }
    if s.pos.1 > y1 { s.pos.1 = y1; s.vel.1 = -s.vel.1.abs() * r; s.bounces += 1; }

    if spd < 8.0 { s.vel = (0.0, 0.0); }
}

// ─── Render ──────────────────────────────────────────────────────────────────
fn render(s: &State) -> DynamicImage {
    let mut img = ImageBuffer::from_pixel(W, H, Rgba([8u8, 8, 12, 255]));

    // ── Subtle grid (depth cue) ───────────────────────────────────────────
    for x in (0..W).step_by(40) {
        for y in 0..H {
            let px = img.get_pixel_mut(x, y);
            px[0] = px[0].saturating_add(6);
            px[1] = px[1].saturating_add(6);
            px[2] = px[2].saturating_add(6);
        }
    }
    for y in (0..H).step_by(40) {
        for x in 0..W {
            let px = img.get_pixel_mut(x, y);
            px[0] = px[0].saturating_add(6);
            px[1] = px[1].saturating_add(6);
            px[2] = px[2].saturating_add(6);
        }
    }

    // ── Border walls ─────────────────────────────────────────────────────
    for x in 0..W {
        for t in 0..3u32 {
            gfx::over(img.get_pixel_mut(x, t), 60, 80, 100, 0.9);
            gfx::over(img.get_pixel_mut(x, H - 1 - t), 60, 80, 100, 0.9);
        }
    }
    for y in 0..H {
        for t in 0..3u32 {
            gfx::over(img.get_pixel_mut(t, y), 60, 80, 100, 0.9);
            gfx::over(img.get_pixel_mut(W - 1 - t, y), 60, 80, 100, 0.9);
        }
    }

    // ── Cannon body ───────────────────────────────────────────────────────
    let cy = H as f32 / 2.0;
    gfx::fill_rect(&mut img, 20, cy as i32 - 24, 44, 48, [120, 100, 40], 1.0);
    // Barrel
    let rad = s.angle_deg.to_radians();
    let bx = 20.0 + 44.0;
    let barrel_len = 50.0;
    let ex = bx + rad.cos() * barrel_len;
    let ey = cy + rad.sin() * barrel_len;
    gfx::glowing_line(&mut img, bx, cy, ex, ey, [200, 170, 60], [120, 100, 30], 5.0, 10.0, 1.0);

    // ── Preview arc ───────────────────────────────────────────────────────
    if !s.active {
        let mut px = ex; let mut py = ey;
        let r2 = s.angle_deg.to_radians();
        let mut vx = s.power * r2.cos();
        let mut vy = s.power * r2.sin();
        let g = s.gravity.accel();
        for i in 0..80u32 {
            let dt2 = 0.016;
            vy += g * dt2;
            px += vx * dt2; py += vy * dt2;
            let t = i as f32 / 80.0;
            let a = (1.0 - t) * 0.35;
            if px >= 0.0 && px < W as f32 && py >= 0.0 && py < H as f32 {
                gfx::fill_circle(&mut img, px, py, 2.0, [100, 200, 255], a);
            }
            if px < BALL_R || px > W as f32 - BALL_R || py < BALL_R || py > H as f32 - BALL_R { break; }
        }
    }

    // ── Trail ─────────────────────────────────────────────────────────────
    let trail_len = s.trail.len();
    for (i, &(tx, ty)) in s.trail.iter().enumerate() {
        let frac = i as f32 / trail_len as f32;
        let r_size = BALL_R * frac * 0.7;
        let alpha = frac * frac * 0.5;
        // Colour shifts from blue-white (recent) to dim orange (old)
        let (cr, cg, cb) = if frac > 0.6 {
            (255u8, 180u8, 60u8)
        } else if frac > 0.3 {
            (200u8, 120u8, 200u8)
        } else {
            (80u8, 80u8, 160u8)
        };
        gfx::fill_circle(&mut img, tx, ty, r_size, [cr, cg, cb], alpha);
    }

    // ── Cannonball ────────────────────────────────────────────────────────
    if s.active {
        let (bx, by) = s.pos;
        // Outer glow (wide orange)
        gfx::radial_glow(&mut img, bx, by, BALL_R * 5.0, [255, 120, 20], 0.4);
        // Mid glow (brighter)
        gfx::radial_glow(&mut img, bx, by, BALL_R * 2.5, [255, 200, 80], 0.6);
        // Ball body (dark grey → lit)
        gfx::fill_circle(&mut img, bx, by, BALL_R, [60, 60, 70], 1.0);
        // Lit hemisphere
        gfx::fill_circle(&mut img, bx - BALL_R * 0.25, by - BALL_R * 0.3, BALL_R * 0.75, [200, 200, 220], 0.85);
        // Specular highlight
        gfx::fill_circle(&mut img, bx - BALL_R * 0.35, by - BALL_R * 0.4, BALL_R * 0.3, [255, 255, 255], 1.0);
    }

    DynamicImage::ImageRgba8(img)
}

// ─── Main ────────────────────────────────────────────────────────────────────
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (term_w, term_h) = terminal::size()?;

    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    let cfg = Config {
        x: 0, y: 0,
        width: Some(term_w as u32),
        height: Some(term_h.saturating_sub(3) as u32),
        ..Default::default()
    };

    let mut state = State::default();
    let frame_dur = Duration::from_secs_f64(1.0 / FPS);
    let mut last = Instant::now();

    loop {
        // ── Input ────────────────────────────────────────────────────────
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
                        state.active = false; state.trail.clear();
                        state.bounces = 0; state.elapsed = 0.0;
                    }
                    KeyCode::Char('1') => state.gravity = GravityMode::Normal,
                    KeyCode::Char('2') => state.gravity = GravityMode::Moon,
                    KeyCode::Char('3') => state.gravity = GravityMode::Flipped,
                    KeyCode::Char('4') => state.gravity = GravityMode::Zero,
                    KeyCode::Up    => state.angle_deg = (state.angle_deg - 5.0).clamp(-85.0, 85.0),
                    KeyCode::Down  => state.angle_deg = (state.angle_deg + 5.0).clamp(-85.0, 85.0),
                    KeyCode::Right => state.power = (state.power + 50.0).min(1800.0),
                    KeyCode::Left  => state.power = (state.power - 50.0).max(100.0),
                    KeyCode::Char('+') | KeyCode::Char('=') => state.restitution = (state.restitution + 0.05).min(1.0),
                    KeyCode::Char('-') => state.restitution = (state.restitution - 0.05).max(0.0),
                    _ => {}
                }
            }
        }

        // ── Physics ──────────────────────────────────────────────────────
        let now = Instant::now();
        let dt = now.duration_since(last).as_secs_f32().min(0.05);
        last = now;
        update(&mut state, dt);

        // ── Draw ─────────────────────────────────────────────────────────
        let img = render(&state);

        execute!(stdout(), cursor::MoveTo(0, 0))?;
        viuer::print(&img, &cfg)?;

        // HUD (below image, plain text)
        let speed = (state.vel.0 * state.vel.0 + state.vel.1 * state.vel.1).sqrt();
        execute!(stdout(), cursor::MoveTo(0, term_h - 3))?;
        print!(
            " Gravity:{:<9} Power:{:>5.0}  Angle:{:>4.0}°  Bounce:{:.0}%  Hits:{:>3}  Speed:{:>5.0}px/s",
            state.gravity.label(), state.power, state.angle_deg, state.restitution * 100.0,
            state.bounces, speed
        );
        execute!(stdout(), cursor::MoveTo(0, term_h - 2))?;
        print!(" [SPACE] Fire  [R] Reset  [↑↓] Angle  [←→] Power  [1-4] Gravity  [+/-] Bounce  [Q] Quit");
        stdout().flush()?;

        // ── Framerate cap ────────────────────────────────────────────────
        let elapsed = Instant::now().duration_since(now);
        if elapsed < frame_dur {
            std::thread::sleep(frame_dur - elapsed);
        }
    }
}
