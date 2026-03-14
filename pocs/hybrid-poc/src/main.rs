/// Option 3: Hybrid — Ratatui for all layout/HUD, pixel overlay for game area.
///
/// Uses ratatui to manage the full terminal layout (borders, HUD, status bar).
/// The game area is rendered as a pixel image (via viuer) when the terminal
/// supports it, or as ratatui Unicode widgets when it doesn't.
///
/// This gives you ONE UI framework everywhere, with pixel rendering only
/// where it matters (the game canvas).

use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

// ─── Capability Detection ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum GraphicsCap {
    Pixel,    // Kitty/Sixel/iTerm2
    TextOnly, // ANSI-only terminal
}

fn detect_cap() -> GraphicsCap {
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--pixel" => return GraphicsCap::Pixel,
            "--text"  => return GraphicsCap::TextOnly,
            _ => {}
        }
    }
    if std::env::var("TERM").as_deref() == Ok("xterm-kitty")
        || std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("TERM_PROGRAM").as_deref() == Ok("WezTerm")
        || std::env::var("TERM_PROGRAM").as_deref() == Ok("ghostty")
        || std::env::var("TERM_PROGRAM").as_deref() == Ok("iTerm.app")
    {
        GraphicsCap::Pixel
    } else {
        GraphicsCap::TextOnly
    }
}

// ─── Physics (shared) ──────────────────────────────────────────────────────

const GRAVITY: f32 = 900.0;
const RESTITUTION: f32 = 0.78;

struct State {
    pos: (f32, f32),      // normalized [0..1]
    vel: (f32, f32),
    active: bool,
    trail: Vec<(f32, f32)>,
    bounces: u32,
    angle_deg: f32,
    power: f32,
    #[allow(dead_code)]
    cap: GraphicsCap,
    frame: u64,
    // Layout info (set by ratatui each frame)
    game_area: Option<Rect>,
}

impl State {
    fn new(cap: GraphicsCap) -> Self {
        Self {
            pos: (0.125, 0.5),
            vel: (0.0, 0.0),
            active: false,
            trail: Vec::new(),
            bounces: 0,
            angle_deg: -30.0,
            power: 800.0,
            cap,
            frame: 0,
            game_area: None,
        }
    }
}

fn fire(s: &mut State) {
    s.active = true;
    s.trail.clear();
    s.bounces = 0;
    s.pos = (0.125, 0.5);
    let rad = s.angle_deg.to_radians();
    s.vel = (s.power * rad.cos() / 640.0, s.power * rad.sin() / 360.0);
}

fn update(s: &mut State, dt: f32) {
    if !s.active { return; }
    s.frame += 1;

    s.trail.push(s.pos);
    if s.trail.len() > 50 { s.trail.remove(0); }

    s.vel.1 += (GRAVITY / 360.0) * dt;
    s.pos.0 += s.vel.0 * dt;
    s.pos.1 += s.vel.1 * dt;

    let margin = 0.02;
    let r = RESTITUTION;
    if s.pos.0 < margin       { s.pos.0 = margin;       s.vel.0 =  s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.0 > 1.0 - margin { s.pos.0 = 1.0 - margin; s.vel.0 = -s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.1 < margin       { s.pos.1 = margin;       s.vel.1 =  s.vel.1.abs() * r; s.bounces += 1; }
    if s.pos.1 > 1.0 - margin { s.pos.1 = 1.0 - margin; s.vel.1 = -s.vel.1.abs() * r; s.bounces += 1; }

    let spd = (s.vel.0 * s.vel.0 + s.vel.1 * s.vel.1).sqrt();
    if spd < 0.005 { s.vel = (0.0, 0.0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// PIXEL GAME AREA — rendered as image, displayed via viuer into the game rect
// ═══════════════════════════════════════════════════════════════════════════

mod pixel_area {
    use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

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

    fn glowing_line(img: &mut RgbaImage, x0: f32, y0: f32, x1: f32, y1: f32,
                    core: [u8; 3], glow: [u8; 3], core_w: f32, glow_w: f32, intensity: f32) {
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

    pub fn render(state: &super::State) -> DynamicImage {
        let w: u32 = 480;
        let h: u32 = 270;
        let mut img = ImageBuffer::from_pixel(w, h, Rgba([8u8, 8, 12, 255]));

        // Grid
        for x in (0..w).step_by(30) {
            for y in 0..h { img.get_pixel_mut(x, y)[0] = 14; img.get_pixel_mut(x, y)[1] = 14; img.get_pixel_mut(x, y)[2] = 18; }
        }
        for y in (0..h).step_by(30) {
            for x in 0..w { img.get_pixel_mut(x, y)[0] = 14; img.get_pixel_mut(x, y)[1] = 14; img.get_pixel_mut(x, y)[2] = 18; }
        }

        // Walls
        for x in 0..w { for t in 0..2u32 { over(img.get_pixel_mut(x, t), 60, 80, 100, 0.9); over(img.get_pixel_mut(x, h-1-t), 60, 80, 100, 0.9); } }
        for y in 0..h { for t in 0..2u32 { over(img.get_pixel_mut(t, y), 60, 80, 100, 0.9); over(img.get_pixel_mut(w-1-t, y), 60, 80, 100, 0.9); } }

        // Cannon
        let cy = h as f32 / 2.0;
        fill_rect(&mut img, 15, cy as i32 - 18, 33, 36, [120, 100, 40], 1.0);
        let rad = state.angle_deg.to_radians();
        let bx = 48.0;
        let ex = bx + rad.cos() * 38.0;
        let ey = cy + rad.sin() * 38.0;
        glowing_line(&mut img, bx, cy, ex, ey, [200, 170, 60], [120, 100, 30], 4.0, 8.0, 1.0);

        // Ramp
        fill_rect(&mut img, 260, 195, 90, 6, [80, 140, 80], 1.0);
        // Second ramp
        fill_rect(&mut img, 140, 130, 70, 6, [80, 100, 160], 1.0);

        // Preview arc
        if !state.active {
            let mut px = ex; let mut py = ey;
            let vx = state.power * rad.cos() * (w as f32 / 640.0);
            let mut vy = state.power * rad.sin() * (h as f32 / 360.0);
            for i in 0..60u32 {
                vy += 900.0 * (h as f32 / 360.0) * 0.016;
                px += vx * 0.016; py += vy * 0.016;
                let t = i as f32 / 60.0;
                if px >= 0.0 && px < w as f32 && py >= 0.0 && py < h as f32 {
                    fill_circle(&mut img, px, py, 1.5, [100, 200, 255], (1.0 - t) * 0.35);
                }
            }
        }

        // Trail
        let trail_len = state.trail.len();
        for (i, &(tx, ty)) in state.trail.iter().enumerate() {
            let frac = i as f32 / trail_len.max(1) as f32;
            let col = if frac > 0.6 { [255u8, 180, 60] } else if frac > 0.3 { [200, 120, 200] } else { [80, 80, 160] };
            fill_circle(&mut img, tx * w as f32, ty * h as f32, 7.0 * frac * 0.7, col, frac * frac * 0.5);
        }

        // Ball
        if state.active {
            let bx = state.pos.0 * w as f32;
            let by = state.pos.1 * h as f32;
            radial_glow(&mut img, bx, by, 35.0, [255, 120, 20], 0.4);
            radial_glow(&mut img, bx, by, 18.0, [255, 200, 80], 0.6);
            fill_circle(&mut img, bx, by, 7.0, [60, 60, 70], 1.0);
            fill_circle(&mut img, bx - 2.0, by - 2.0, 5.0, [200, 200, 220], 0.85);
            fill_circle(&mut img, bx - 2.5, by - 3.0, 2.0, [255, 255, 255], 1.0);
        }

        DynamicImage::ImageRgba8(img)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TEXT GAME AREA — ratatui Widget for text-only terminals
// ═══════════════════════════════════════════════════════════════════════════

struct TextGameArea<'a> {
    state: &'a State,
}

impl Widget for TextGameArea<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.state;

        // Cannon body
        let cx = area.x + 3;
        let cy = area.y + area.height / 2;
        for dy in 0..3u16 {
            for dx in 0..2u16 {
                let bx = cx + dx;
                let by = cy.saturating_sub(1) + dy;
                if in_rect(bx, by, area) {
                    if let Some(c) = buf.cell_mut((bx, by)) {
                        c.set_symbol("█");
                        c.fg = Color::Rgb(180, 140, 60);
                    }
                }
            }
        }
        // Barrel
        let (bx1, by1, bx2, by2, bx3, by3) = if s.angle_deg < -20.0 {
            (cx + 2, cy, cx + 3, cy - 1, cx + 4, cy - 1)
        } else if s.angle_deg > 20.0 {
            (cx + 2, cy, cx + 3, cy + 1, cx + 4, cy + 1)
        } else {
            (cx + 2, cy, cx + 3, cy, cx + 4, cy)
        };
        for &(bx, by) in &[(bx1, by1), (bx2, by2), (bx3, by3)] {
            if in_rect(bx, by, area) {
                if let Some(c) = buf.cell_mut((bx, by)) {
                    c.set_symbol("═");
                    c.fg = Color::Rgb(220, 180, 80);
                }
            }
        }

        // Ramps
        let ramp1_x = area.x + area.width * 55 / 100;
        let ramp1_y = area.y + area.height * 72 / 100;
        for dx in 0..12u16 {
            if in_rect(ramp1_x + dx, ramp1_y, area) {
                if let Some(c) = buf.cell_mut((ramp1_x + dx, ramp1_y)) {
                    c.set_symbol("▬"); c.fg = Color::Green;
                }
            }
        }
        let ramp2_x = area.x + area.width * 30 / 100;
        let ramp2_y = area.y + area.height * 48 / 100;
        for dx in 0..9u16 {
            if in_rect(ramp2_x + dx, ramp2_y, area) {
                if let Some(c) = buf.cell_mut((ramp2_x + dx, ramp2_y)) {
                    c.set_symbol("▬"); c.fg = Color::Blue;
                }
            }
        }

        // Preview arc
        if !s.active {
            let rad = s.angle_deg.to_radians();
            let mut px = (cx + 5) as f32;
            let mut py = cy as f32;
            let vx = s.power / 640.0 * area.width as f32 * rad.cos();
            let mut vy = s.power / 360.0 * area.height as f32 * rad.sin();
            for i in 0..50u32 {
                vy += (900.0 / 360.0 * area.height as f32) * 0.016;
                px += vx * 0.016; py += vy * 0.016;
                if i % 3 != 0 { continue; }
                let sx = px.round() as u16;
                let sy = py.round() as u16;
                if in_rect(sx, sy, area) {
                    if let Some(c) = buf.cell_mut((sx, sy)) { c.set_symbol("·"); c.fg = Color::Indexed(237); }
                }
            }
        }

        // Trail
        let trail_len = s.trail.len();
        for (i, &(tx, ty)) in s.trail.iter().enumerate() {
            let sx = (area.x as f32 + tx * area.width as f32).round() as u16;
            let sy = (area.y as f32 + ty * area.height as f32).round() as u16;
            if !in_rect(sx, sy, area) { continue; }
            let frac = i as f32 / trail_len.max(1) as f32;
            let (sym, color) = if frac < 0.3 { ("·", Color::Indexed(235)) }
                               else if frac < 0.6 { ("·", Color::Indexed(238)) }
                               else { ("∙", Color::Gray) };
            if let Some(c) = buf.cell_mut((sx, sy)) { c.set_symbol(sym); c.fg = color; }
        }

        // Ball
        if s.active {
            let bx = (area.x as f32 + s.pos.0 * area.width as f32).round() as u16;
            let by = (area.y as f32 + s.pos.1 * area.height as f32).round() as u16;
            // Glow
            for dy in -1i16..=1 {
                for dx in -1i16..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let gx = (bx as i16 + dx) as u16;
                    let gy = (by as i16 + dy) as u16;
                    if in_rect(gx, gy, area) {
                        if let Some(c) = buf.cell_mut((gx, gy)) {
                            if c.symbol() == " " { c.set_symbol("░"); c.fg = Color::Indexed(52); }
                        }
                    }
                }
            }
            if in_rect(bx, by, area) {
                if let Some(c) = buf.cell_mut((bx, by)) {
                    c.set_symbol("●"); c.fg = Color::Red; c.modifier = Modifier::BOLD;
                }
            }
        }
    }
}

fn in_rect(x: u16, y: u16, r: Rect) -> bool {
    x >= r.x && x < r.right() && y >= r.y && y < r.bottom()
}

// ═══════════════════════════════════════════════════════════════════════════
// RATATUI LAYOUT — used by BOTH modes for HUD/chrome
// ═══════════════════════════════════════════════════════════════════════════

/// Placeholder widget that just draws a border. In pixel mode, viuer writes
/// over this area after ratatui flushes the frame.
struct PixelPlaceholder;

impl Widget for PixelPlaceholder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Just clear the area — viuer will overwrite it
        for y in area.y..area.bottom() {
            for x in area.x..area.right() {
                if let Some(c) = buf.cell_mut((x, y)) {
                    c.set_symbol(" ");
                }
            }
        }
    }
}

// ─── Main ───────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cap = detect_cap();

    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout());
    let mut term = ratatui::Terminal::new(backend)?;

    let mut state = State::new(cap);
    let frame_dur = Duration::from_secs_f64(1.0 / 60.0);
    let mut last = Instant::now();

    loop {
        // ── Input ───────────────────────────────────────────────────────
        while event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release { continue; }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        execute!(term.backend_mut(), terminal::LeaveAlternateScreen, cursor::Show)?;
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

        // ── Ratatui layout pass ─────────────────────────────────────────
        let mut game_rect = Rect::default();

        term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),  // title bar
                    Constraint::Min(5),     // game area
                    Constraint::Length(3),  // HUD
                ])
                .split(f.area());

            // Title bar
            let mode_str = match cap {
                GraphicsCap::Pixel    => "PIXEL",
                GraphicsCap::TextOnly => "TEXT",
            };
            let title = Paragraph::new(Line::from(vec![
                Span::styled(" HYBRID POC ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!("— {} mode ", mode_str), Style::default().fg(Color::DarkGray)),
            ]));
            f.render_widget(title, chunks[0]);

            // Game area
            let game_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Indexed(236)))
                .title(" Game Area ");

            let inner = game_block.inner(chunks[1]);
            f.render_widget(game_block, chunks[1]);

            match cap {
                GraphicsCap::Pixel => {
                    // Just clear — viuer overwrites after this draw call
                    f.render_widget(PixelPlaceholder, inner);
                }
                GraphicsCap::TextOnly => {
                    f.render_widget(TextGameArea { state: &state }, inner);
                }
            }

            game_rect = inner;

            // HUD (always ratatui)
            let speed = (state.vel.0 * state.vel.0 + state.vel.1 * state.vel.1).sqrt() * 640.0;
            let hud = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("  Bounces: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:>3}", state.bounces), Style::default().fg(Color::Yellow)),
                    Span::styled("  Angle: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:>4.0}°", state.angle_deg), Style::default().fg(Color::Green)),
                    Span::styled("  Power: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:>5.0}", state.power), Style::default().fg(Color::Green)),
                    Span::styled("  Speed: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:>5.0}", speed),
                        Style::default().fg(if speed > 500.0 { Color::Red } else { Color::White }),
                    ),
                ]),
                Line::from(Span::styled(
                    "  [SPACE] Fire  [R] Reset  [Up/Down] Angle  [Left/Right] Power  [Q] Quit",
                    Style::default().fg(Color::Indexed(240)),
                )),
            ]).block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::Indexed(236))));
            f.render_widget(hud, chunks[2]);
        })?;

        // ── Pixel overlay (after ratatui frame) ─────────────────────────
        if cap == GraphicsCap::Pixel && game_rect.width > 0 && game_rect.height > 0 {
            state.game_area = Some(game_rect);
            let img = pixel_area::render(&state);
            let cfg = viuer::Config {
                x: game_rect.x as u16,
                y: game_rect.y as i16,
                width: Some(game_rect.width as u32),
                height: Some(game_rect.height as u32),
                absolute_offset: true,
                ..Default::default()
            };
            viuer::print(&img, &cfg)?;
            stdout().flush()?;
        }

        let elapsed = Instant::now().duration_since(now);
        if elapsed < frame_dur {
            std::thread::sleep(frame_dur - elapsed);
        }
    }
}
