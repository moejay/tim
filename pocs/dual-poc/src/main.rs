/// Option 2: Dual Renderer — pixel path OR ratatui text path.
///
/// Detects terminal graphics capability at startup.
///   - Pixel-capable → viuer pipeline (image crate, anti-aliased, glow)
///   - Plain terminal → ratatui text-cell renderer (Unicode chars, 256-color)
///
/// Physics are identical. Two completely separate render functions.
/// You can also force a mode with --pixel or --text CLI args.

use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal,
};

// ─── Renderer Mode ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum RenderMode {
    Pixel,
    Text,
}

impl RenderMode {
    fn label(self) -> &'static str {
        match self {
            Self::Pixel => "Pixel (viuer)",
            Self::Text  => "Text (ratatui)",
        }
    }
}

fn detect_mode() -> RenderMode {
    // CLI override
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--pixel" => return RenderMode::Pixel,
            "--text"  => return RenderMode::Text,
            _ => {}
        }
    }
    if std::env::var("TERM").as_deref() == Ok("xterm-kitty")
        || std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("TERM_PROGRAM").as_deref() == Ok("WezTerm")
        || std::env::var("TERM_PROGRAM").as_deref() == Ok("ghostty")
        || std::env::var("TERM_PROGRAM").as_deref() == Ok("iTerm.app")
    {
        RenderMode::Pixel
    } else {
        RenderMode::Text
    }
}

// ─── Shared Physics ─────────────────────────────────────────────────────────

const GRAVITY: f32 = 900.0;
const RESTITUTION: f32 = 0.78;

struct State {
    /// Normalized [0..1] position
    pos: (f32, f32),
    vel: (f32, f32),
    active: bool,
    trail: Vec<(f32, f32)>,
    bounces: u32,
    angle_deg: f32,
    power: f32,
    frame: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            pos: (0.125, 0.5),
            vel: (0.0, 0.0),
            active: false,
            trail: Vec::new(),
            bounces: 0,
            angle_deg: -30.0,
            power: 800.0,
            frame: 0,
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
    if s.trail.len() > 60 { s.trail.remove(0); }

    s.vel.1 += (GRAVITY / 360.0) * dt;
    s.pos.0 += s.vel.0 * dt;
    s.pos.1 += s.vel.1 * dt;

    let margin = 0.02;
    let r = RESTITUTION;
    if s.pos.0 < margin     { s.pos.0 = margin;     s.vel.0 =  s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.0 > 1.0 - margin { s.pos.0 = 1.0 - margin; s.vel.0 = -s.vel.0.abs() * r; s.bounces += 1; }
    if s.pos.1 < margin     { s.pos.1 = margin;     s.vel.1 =  s.vel.1.abs() * r; s.bounces += 1; }
    if s.pos.1 > 1.0 - margin { s.pos.1 = 1.0 - margin; s.vel.1 = -s.vel.1.abs() * r; s.bounces += 1; }

    let spd = (s.vel.0 * s.vel.0 + s.vel.1 * s.vel.1).sqrt();
    if spd < 0.005 { s.vel = (0.0, 0.0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// PIXEL RENDERER (viuer + image crate)
// ═══════════════════════════════════════════════════════════════════════════

mod pixel {
    use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

    const W: u32 = 640;
    const H: u32 = 360;

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
        let mut img = ImageBuffer::from_pixel(W, H, Rgba([8u8, 8, 12, 255]));

        // Grid
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

        // Walls
        for x in 0..W {
            for t in 0..3u32 {
                over(img.get_pixel_mut(x, t), 60, 80, 100, 0.9);
                over(img.get_pixel_mut(x, H - 1 - t), 60, 80, 100, 0.9);
            }
        }
        for y in 0..H {
            for t in 0..3u32 {
                over(img.get_pixel_mut(t, y), 60, 80, 100, 0.9);
                over(img.get_pixel_mut(W - 1 - t, y), 60, 80, 100, 0.9);
            }
        }

        // Cannon body
        let cy = H as f32 / 2.0;
        fill_rect(&mut img, 20, cy as i32 - 24, 44, 48, [120, 100, 40], 1.0);

        // Barrel
        let rad = state.angle_deg.to_radians();
        let bx = 64.0;
        let ex = bx + rad.cos() * 50.0;
        let ey = cy + rad.sin() * 50.0;
        glowing_line(&mut img, bx, cy, ex, ey, [200, 170, 60], [120, 100, 30], 5.0, 10.0, 1.0);

        // Ramp obstacle
        fill_rect(&mut img, 350, 260, 120, 8, [80, 140, 80], 1.0);

        // Preview arc
        if !state.active {
            let mut px = ex; let mut py = ey;
            let vx = state.power * rad.cos();
            let mut vy = state.power * rad.sin();
            for i in 0..80u32 {
                vy += 900.0 * 0.016;
                px += vx * 0.016; py += vy * 0.016;
                let t = i as f32 / 80.0;
                let a = (1.0 - t) * 0.35;
                if px >= 0.0 && px < W as f32 && py >= 0.0 && py < H as f32 {
                    fill_circle(&mut img, px, py, 2.0, [100, 200, 255], a);
                }
                if px < 10.0 || px > W as f32 - 10.0 || py < 10.0 || py > H as f32 - 10.0 { break; }
            }
        }

        // Trail
        let trail_len = state.trail.len();
        for (i, &(tx, ty)) in state.trail.iter().enumerate() {
            let frac = i as f32 / trail_len.max(1) as f32;
            let tx_px = tx * W as f32;
            let ty_px = ty * H as f32;
            let col = if frac > 0.6 { [255u8, 180, 60] }
                      else if frac > 0.3 { [200, 120, 200] }
                      else { [80, 80, 160] };
            fill_circle(&mut img, tx_px, ty_px, 10.0 * frac * 0.7, col, frac * frac * 0.5);
        }

        // Ball
        if state.active {
            let bx = state.pos.0 * W as f32;
            let by = state.pos.1 * H as f32;
            radial_glow(&mut img, bx, by, 50.0, [255, 120, 20], 0.4);
            radial_glow(&mut img, bx, by, 25.0, [255, 200, 80], 0.6);
            fill_circle(&mut img, bx, by, 10.0, [60, 60, 70], 1.0);
            fill_circle(&mut img, bx - 2.5, by - 3.0, 7.5, [200, 200, 220], 0.85);
            fill_circle(&mut img, bx - 3.5, by - 4.0, 3.0, [255, 255, 255], 1.0);
        }

        DynamicImage::ImageRgba8(img)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TEXT RENDERER (ratatui — Unicode characters in terminal cells)
// ═══════════════════════════════════════════════════════════════════════════

mod text {
    use ratatui::{
        buffer::Buffer,
        layout::Rect,
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Widget},
    };

    pub struct GameCanvas<'a> {
        pub state: &'a super::State,
    }

    impl Widget for GameCanvas<'_> {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let s = self.state;

            // Border
            Block::default()
                .title(" DUAL POC — Text Renderer ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .render(area, buf);

            let inner = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(2),
            };

            // Cannon body
            let cx = inner.x + 3;
            let cy = inner.y + inner.height / 2;
            for dy in 0..3u16 {
                for dx in 0..2u16 {
                    let bx = cx + dx;
                    let by = cy.saturating_sub(1) + dy;
                    if let Some(c) = buf.cell_mut((bx, by)) {
                        c.set_symbol("█");
                        c.fg = Color::Rgb(180, 140, 60);
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
                if in_rect(bx, by, inner) {
                    if let Some(c) = buf.cell_mut((bx, by)) {
                        c.set_symbol("═");
                        c.fg = Color::Rgb(220, 180, 80);
                    }
                }
            }

            // Ramp obstacle
            let ramp_x = inner.x + inner.width * 55 / 100;
            let ramp_y = inner.y + inner.height * 72 / 100;
            for dx in 0..15u16 {
                let rx = ramp_x + dx;
                if in_rect(rx, ramp_y, inner) {
                    if let Some(c) = buf.cell_mut((rx, ramp_y)) {
                        c.set_symbol("▬");
                        c.fg = Color::Green;
                    }
                }
            }

            // Preview arc
            if !s.active {
                let rad = s.angle_deg.to_radians();
                let start_x = (cx + 5) as f32;
                let start_y = cy as f32;
                let mut px = start_x;
                let mut py = start_y;
                let vx = s.power / 640.0 * inner.width as f32 * rad.cos();
                let mut vy = s.power / 360.0 * inner.height as f32 * rad.sin();
                for i in 0..60u32 {
                    vy += (900.0 / 360.0 * inner.height as f32) * 0.016;
                    px += vx * 0.016;
                    py += vy * 0.016;
                    if i % 3 != 0 { continue; }
                    let sx = px.round() as u16;
                    let sy = py.round() as u16;
                    if in_rect(sx, sy, inner) {
                        if let Some(c) = buf.cell_mut((sx, sy)) {
                            c.set_symbol("·");
                            c.fg = Color::Indexed(237);
                        }
                    }
                }
            }

            // Trail
            let trail_len = s.trail.len();
            for (i, &(tx, ty)) in s.trail.iter().enumerate() {
                let sx = (inner.x as f32 + tx * inner.width as f32).round() as u16;
                let sy = (inner.y as f32 + ty * inner.height as f32).round() as u16;
                if !in_rect(sx, sy, inner) { continue; }
                let frac = i as f32 / trail_len.max(1) as f32;
                let (sym, color) = if frac < 0.3 {
                    ("·", Color::Indexed(235))
                } else if frac < 0.6 {
                    ("·", Color::Indexed(238))
                } else {
                    ("∙", Color::Gray)
                };
                if let Some(c) = buf.cell_mut((sx, sy)) {
                    c.set_symbol(sym);
                    c.fg = color;
                }
            }

            // Cannonball
            if s.active {
                let bx = (inner.x as f32 + s.pos.0 * inner.width as f32).round() as u16;
                let by = (inner.y as f32 + s.pos.1 * inner.height as f32).round() as u16;

                // Glow (surrounding cells)
                for dy in -1i16..=1 {
                    for dx in -1i16..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let gx = (bx as i16 + dx) as u16;
                        let gy = (by as i16 + dy) as u16;
                        if in_rect(gx, gy, inner) {
                            if let Some(c) = buf.cell_mut((gx, gy)) {
                                if c.symbol() == " " {
                                    c.set_symbol("░");
                                    c.fg = Color::Indexed(52);
                                }
                            }
                        }
                    }
                }

                if in_rect(bx, by, inner) {
                    if let Some(c) = buf.cell_mut((bx, by)) {
                        c.set_symbol("●");
                        c.fg = Color::Red;
                        c.modifier = Modifier::BOLD;
                    }
                }

                // Direction arrow
                let spd = (s.vel.0 * s.vel.0 + s.vel.1 * s.vel.1).sqrt();
                if spd > 0.01 {
                    let ax = (bx as f32 + s.vel.0.signum() * 2.0).round() as u16;
                    let ay = (by as f32 + s.vel.1.signum()).round() as u16;
                    if in_rect(ax, ay, inner) {
                        let arrow = if s.vel.0.abs() > s.vel.1.abs() * 2.0 {
                            if s.vel.0 > 0.0 { "→" } else { "←" }
                        } else if s.vel.1.abs() > s.vel.0.abs() * 2.0 {
                            if s.vel.1 > 0.0 { "↓" } else { "↑" }
                        } else if s.vel.0 > 0.0 {
                            if s.vel.1 > 0.0 { "↘" } else { "↗" }
                        } else {
                            if s.vel.1 > 0.0 { "↙" } else { "↖" }
                        };
                        if let Some(c) = buf.cell_mut((ax, ay)) {
                            c.set_symbol(arrow);
                            c.fg = Color::Indexed(166);
                        }
                    }
                }
            }
        }
    }

    fn in_rect(x: u16, y: u16, r: Rect) -> bool {
        x >= r.x && x < r.right() && y >= r.y && y < r.bottom()
    }
}

// ─── Main ───────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = detect_mode();
    let (term_w, term_h) = terminal::size()?;

    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut state = State::default();
    let frame_dur = Duration::from_secs_f64(1.0 / 60.0);
    let mut last = Instant::now();

    // Ratatui terminal (only used in Text mode, but cheap to create)
    let backend = ratatui::backend::CrosstermBackend::new(stdout());
    let mut term = ratatui::Terminal::new(backend)?;

    loop {
        // ── Input ───────────────────────────────────────────────────────
        while event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release { continue; }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        execute!(
                            term.backend_mut(),
                            terminal::LeaveAlternateScreen,
                            cursor::Show
                        )?;
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

        // ── Render ──────────────────────────────────────────────────────
        match mode {
            RenderMode::Pixel => {
                let img = pixel::render(&state);
                let cfg = viuer::Config {
                    x: 0, y: 0,
                    width: Some(term_w as u32),
                    height: Some(term_h.saturating_sub(3) as u32),
                    ..Default::default()
                };
                execute!(term.backend_mut(), cursor::MoveTo(0, 0))?;
                viuer::print(&img, &cfg)?;

                // HUD
                execute!(term.backend_mut(), cursor::MoveTo(0, term_h - 3))?;
                print!(
                    " Mode:{:<16} Bounces:{:>3}  Angle:{:>4.0}°  Power:{:>5.0}",
                    mode.label(), state.bounces, state.angle_deg, state.power
                );
                execute!(term.backend_mut(), cursor::MoveTo(0, term_h - 2))?;
                print!(" [SPACE] Fire  [R] Reset  [Up/Down] Angle  [Left/Right] Power  [Q] Quit");
                stdout().flush()?;
            }
            RenderMode::Text => {
                term.draw(|f| {
                    use ratatui::layout::{Constraint, Direction, Layout};
                    use ratatui::style::{Color, Style};
                    use ratatui::text::{Line, Span};
                    use ratatui::widgets::Paragraph;

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(5), Constraint::Length(2)])
                        .split(f.area());

                    f.render_widget(text::GameCanvas { state: &state }, chunks[0]);

                    let hud = Paragraph::new(vec![
                        Line::from(vec![
                            Span::styled(
                                format!(
                                    "  Mode: {:<16} Bounces: {:>3}  Angle: {:>4.0}°  Power: {:>5.0}",
                                    mode.label(), state.bounces, state.angle_deg, state.power
                                ),
                                Style::default().fg(Color::Gray),
                            ),
                        ]),
                        Line::from(Span::styled(
                            "  [SPACE] Fire  [R] Reset  [Up/Down] Angle  [Left/Right] Power  [Q] Quit",
                            Style::default().fg(Color::Indexed(240)),
                        )),
                    ]);
                    f.render_widget(hud, chunks[1]);
                })?;
            }
        }

        let elapsed = Instant::now().duration_since(now);
        if elapsed < frame_dur {
            std::thread::sleep(frame_dur - elapsed);
        }
    }
}
