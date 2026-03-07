/// Fidelity demos: Laser Gun · Boiling Pot · Lit Candle — Sixel/Kitty pixel rendering.
/// [1] Laser  [2] Pot  [3] Candle  [Tab] next  [Q] quit
mod gfx;

use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal,
};
use image::{DynamicImage, ImageBuffer, Rgba};
use viuer::Config;

const W: u32 = 640;
const H: u32 = 380;
const FPS: f64 = 60.0;

// ─── State ────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum Scene { Laser, Pot, Candle }
impl Scene {
    fn next(self) -> Self { match self { Self::Laser => Self::Pot, Self::Pot => Self::Candle, Self::Candle => Self::Laser } }
    fn label(self) -> &'static str { match self { Self::Laser => "Laser Gun", Self::Pot => "Boiling Pot", Self::Candle => "Lit Candle" } }
}

struct State {
    scene: Scene,
    elapsed: f32,
    frame: u64,
    candle_lit: bool,
    wax_r: f32,
    wax_l: f32,
}
impl Default for State {
    fn default() -> Self {
        Self { scene: Scene::Laser, elapsed: 0.0, frame: 0, candle_lit: true, wax_r: 0.0, wax_l: 0.0 }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SCENE: LASER GUN
// ═════════════════════════════════════════════════════════════════════════════
fn render_laser(s: &State) -> DynamicImage {
    let mut img = ImageBuffer::from_pixel(W, H, Rgba([4u8, 4, 10, 255]));
    let t = s.elapsed;
    let f = s.frame;

    // Three beam emitters stacked vertically
    let cy = H as f32 / 2.0;
    let beam_ys = [cy - 80.0, cy, cy + 80.0];

    for (bi, &by) in beam_ys.iter().enumerate() {
        let phase = bi as f32 * 0.45;
        let duty = ((t + phase) * 1.4).sin();
        let active = duty > -0.25;

        // Gun body
        gfx::fill_rect(&mut img, 30, by as i32 - 22, 50, 44, [70, 70, 80], 1.0);
        gfx::fill_rect(&mut img, 30, by as i32 - 22, 52, 44, [90, 90, 100], 0.5);
        // Barrel
        let barrel_col = if active { [80, 220, 240] } else { [20, 60, 70] };
        gfx::fill_rect(&mut img, 78, by as i32 - 6, 16, 12, barrel_col, 1.0);
        // Power cell glow
        let cell_alpha = if active { 0.9 } else { 0.2 };
        gfx::radial_glow(&mut img, 55.0, by, 18.0, [40, 180, 220], cell_alpha);

        if !active { continue; }

        // ── Beam: multi-pass glow ─────────────────────────────────────
        let bx_start = 94.0;
        let bx_end   = W as f32 - 20.0;

        // Outermost wide glow
        gfx::glowing_line(&mut img, bx_start, by, bx_end, by,
            [0, 80, 120], [0, 40, 80], 0.5, 22.0, 0.15);
        // Mid glow
        gfx::glowing_line(&mut img, bx_start, by, bx_end, by,
            [20, 180, 220], [10, 100, 160], 1.5, 12.0, 0.35);
        // Core (bright white-cyan)
        gfx::glowing_line(&mut img, bx_start, by, bx_end, by,
            [200, 240, 255], [100, 200, 240], 1.0, 5.0, 0.9);

        // Travelling energy pulses along the beam
        for pi in 0u32..5 {
            let pulse_phase = (t * 1.8 + pi as f32 * 0.22 + phase) % 1.0;
            let px = bx_start + (bx_end - bx_start) * pulse_phase;
            gfx::radial_glow(&mut img, px, by, 18.0, [150, 255, 255], 0.7);
            gfx::fill_circle(&mut img, px, by, 4.0, [255, 255, 255], 1.0);
        }

        // ── Impact flare at right wall ────────────────────────────────
        let flare_x = bx_end;
        let flicker = 0.8 + (t * 17.3 + bi as f32).sin() * 0.2;
        gfx::radial_glow(&mut img, flare_x, by, 60.0 * flicker, [0, 200, 255], 0.5);
        gfx::radial_glow(&mut img, flare_x, by, 28.0 * flicker, [150, 240, 255], 0.8);
        gfx::fill_circle(&mut img, flare_x, by, 8.0 * flicker, [255, 255, 255], 1.0);

        // Impact sparks (procedural lines radiating outward)
        for si in 0u32..12 {
            let seed = (bi as u32) * 100 + si * 7 + (f / 3) as u32;
            let angle = gfx::h(seed) * std::f32::consts::TAU;
            let length = 15.0 + gfx::h(seed + 1) * 30.0;
            let alpha = gfx::h(seed + 2) * 0.8 + 0.2;
            let ex = flare_x + angle.cos() * length;
            let ey = by + angle.sin() * length;
            gfx::glowing_line(&mut img, flare_x, by, ex, ey,
                [200, 255, 255], [50, 150, 200], 0.8, 3.0, alpha);
        }
    }

    DynamicImage::ImageRgba8(img)
}

// ═════════════════════════════════════════════════════════════════════════════
// SCENE: BOILING POT
// ═════════════════════════════════════════════════════════════════════════════
fn render_pot(s: &State) -> DynamicImage {
    let mut img = ImageBuffer::from_pixel(W, H, Rgba([6u8, 5, 8, 255]));
    let t = s.elapsed;
    let f = s.frame;

    let cx = W as f32 / 2.0;
    let cy = H as f32 / 2.0 + 20.0;

    // Rattle: high-frequency jitter on two frequencies
    let rx = (t * 19.3).sin() * 3.5 + (t * 37.1).sin() * 1.5;
    let ry = (t * 23.7).sin() * 2.0;
    let px = cx + rx;
    let py = cy + ry;

    // ── Heat shimmer / glow beneath pot ──────────────────────────────────
    gfx::radial_glow(&mut img, px, py + 70.0, 130.0, [255, 60, 0], 0.45);
    gfx::radial_glow(&mut img, px, py + 70.0, 70.0,  [255, 140, 0], 0.35);

    // Animated flame jets under pot (3 burners)
    for fi in [-30.0f32, 0.0, 30.0] {
        let fx = px + fi;
        let fbase = py + 68.0;
        let fh = 28.0 + (t * 8.0 + fi).sin() * 8.0;
        gfx::draw_flame(&mut img, fx, fbase, 14.0, fh, t + fi * 0.1, f);
    }

    // ── Pot body ─────────────────────────────────────────────────────────
    let pot_w = 130.0f32;
    let pot_h = 100.0f32;

    // Body gradient (darker at bottom)
    for dy in 0..(pot_h as u32) {
        let fy = dy as f32 / pot_h;
        let dark = (fy * 0.4) as f32;
        let col = [(80.0 - dark * 30.0) as u8, (55.0 - dark * 20.0) as u8, (20.0) as u8];
        let row_w = pot_w * (0.95 + fy * 0.05); // slight taper
        for dx in (-(row_w as i32))..=(row_w as i32) {
            let bx = (px + dx as f32) as i32;
            let by2 = (py - pot_h / 2.0 + dy as f32) as i32;
            if bx >= 0 && by2 >= 0 && (bx as u32) < W && (by2 as u32) < H {
                gfx::over(img.get_pixel_mut(bx as u32, by2 as u32), col[0], col[1], col[2], 1.0);
            }
        }
    }

    // Rim highlight
    let rim_y = py - pot_h / 2.0;
    gfx::fill_rect(&mut img, (px - pot_w) as i32, rim_y as i32 - 8, pot_w as i32 * 2, 8, [110, 80, 30], 1.0);
    gfx::glowing_line(&mut img, px - pot_w, rim_y - 4.0, px + pot_w, rim_y - 4.0,
        [180, 150, 60], [100, 80, 30], 2.0, 6.0, 0.8);

    // Handles
    let handle_col = [100, 70, 25];
    gfx::fill_rect(&mut img, (px - pot_w - 28.0) as i32, py as i32 - 18, 30, 36, handle_col, 1.0);
    gfx::fill_rect(&mut img, (px + pot_w - 2.0) as i32,  py as i32 - 18, 30, 36, handle_col, 1.0);

    // ── Water surface (animated) ──────────────────────────────────────────
    let water_y = py - pot_h / 2.0 + 12.0;
    for x_off in (-(pot_w as i32) + 4)..(pot_w as i32 - 4) {
        let wx = px + x_off as f32;
        // Multi-frequency wave
        let wave = (x_off as f32 * 0.15 + t * 7.0).sin() * 3.5
                 + (x_off as f32 * 0.32 + t * 11.0).sin() * 2.0
                 + (x_off as f32 * 0.07 + t * 4.0).cos() * 2.0;
        let wy = water_y + wave;
        if wx >= 0.0 && wx < W as f32 && wy >= 0.0 && wy < H as f32 {
            // Water surface: bright cyan with shimmer
            let shimmer = (x_off as f32 * 0.2 + t * 15.0).sin() * 0.5 + 0.5;
            let wc = [(20.0 + shimmer * 20.0) as u8, (120.0 + shimmer * 60.0) as u8, (180.0 + shimmer * 40.0) as u8];
            for dy2 in 0..3i32 {
                let fy2 = (wy + dy2 as f32) as u32;
                if (wx as u32) < W && fy2 < H {
                    gfx::over(img.get_pixel_mut(wx as u32, fy2), wc[0], wc[1], wc[2], 0.9);
                }
            }
        }
    }

    // ── Bubbles ───────────────────────────────────────────────────────────
    for id in 0u32..20 {
        let period = 0.9 + gfx::h(id * 5) * 1.2;
        let t_off = id as f32 * period / 20.0;
        let age = (t + t_off) % period;
        let frac = age / period;
        let bx = px + gfx::hs(id * 13) * (pot_w - 20.0);
        let by2 = water_y - frac * 25.0;
        let br = 3.0 + gfx::h(id * 7) * 5.0;
        let alpha = if frac > 0.85 { (1.0 - frac) / 0.15 } else { 0.7 };
        // Bubble: transparent center, bright edge
        gfx::fill_circle(&mut img, bx, by2, br, [100, 200, 220], alpha * 0.3);
        gfx::fill_circle(&mut img, bx, by2, br * 0.25, [220, 240, 255], alpha * 0.9);
    }

    // ── Steam particles ───────────────────────────────────────────────────
    for id in 0u32..25 {
        let period = 2.2 + gfx::h(id * 3) * 1.8;
        let t_off = id as f32 * period / 25.0;
        let age = (t + t_off) % period;
        if age > 2.0 { continue; }
        let frac = age / 2.0;

        let sx = px + gfx::hs(id * 7) * (pot_w - 30.0)
               + (age * 2.5 + gfx::h(id * 11) * 6.28).sin() * 20.0;
        let sy = water_y - 15.0 - age * 60.0;
        let sr = 5.0 + gfx::h(id * 9) * 12.0 + frac * 18.0; // grows as it rises

        let alpha = if frac < 0.2 { frac / 0.2 * 0.4 } else { (1.0 - frac) * 0.4 };
        let gray = (200.0 + frac * 40.0) as u8;
        gfx::fill_circle(&mut img, sx, sy, sr, [gray, gray, gray + 5], alpha);
    }

    DynamicImage::ImageRgba8(img)
}

// ═════════════════════════════════════════════════════════════════════════════
// SCENE: LIT CANDLE
// ═════════════════════════════════════════════════════════════════════════════
fn render_candle(s: &State) -> DynamicImage {
    let mut img = ImageBuffer::from_pixel(W, H, Rgba([4u8, 3, 5, 255]));
    let t = s.elapsed;
    let f = s.frame;

    let cx = W as f32 / 2.0;
    let body_top  = H as f32 / 2.0 - 10.0;
    let body_bot  = H as f32 / 2.0 + 110.0;
    let body_half = 24.0f32;

    // ── Ambient glow (drawn before everything else) ───────────────────────
    if s.candle_lit {
        let flame_cy = body_top - 50.0;
        let flicker = 1.0 + (t * 5.3).sin() * 0.12 + (t * 13.7).sin() * 0.06;
        // Large warm glow
        gfx::radial_glow(&mut img, cx, flame_cy, 250.0 * flicker, [255, 140, 20], 0.18);
        gfx::radial_glow(&mut img, cx, flame_cy, 150.0 * flicker, [255, 180, 40], 0.3);
        gfx::radial_glow(&mut img, cx, flame_cy,  80.0 * flicker, [255, 220, 80], 0.45);
    }

    // ── Candle body ───────────────────────────────────────────────────────
    // Body: slight gradient (lighter at top)
    for y in (body_top as u32)..(body_bot as u32 + 1) {
        let fy = (y as f32 - body_top) / (body_bot - body_top);
        let lightness = 1.0 - fy * 0.25;
        let r = (235.0 * lightness) as u8;
        let g = (215.0 * lightness) as u8;
        let b = (195.0 * lightness) as u8;
        if y < H {
            for dx in -(body_half as i32)..=(body_half as i32) {
                let bx = cx as i32 + dx;
                if bx >= 0 && (bx as u32) < W {
                    gfx::over(img.get_pixel_mut(bx as u32, y), r, g, b, 1.0);
                }
            }
        }
    }

    // Edge shading (makes it look cylindrical)
    for y in (body_top as u32)..(body_bot as u32 + 1) {
        if y >= H { break; }
        for edge in 0u32..8 {
            let alpha = 0.6 * (1.0 - edge as f32 / 8.0);
            let lx = (cx as i32 - body_half as i32 + edge as i32) as u32;
            let rx = (cx as i32 + body_half as i32 - edge as i32) as u32;
            if lx < W { gfx::blend(img.get_pixel_mut(lx, y), 0, 0, 0, alpha); }
            if rx < W { gfx::blend(img.get_pixel_mut(rx, y), 0, 0, 0, alpha); }
        }
    }

    // Candle top rim
    gfx::fill_rect(&mut img, (cx - body_half - 3.0) as i32, body_top as i32 - 4, (body_half * 2.0 + 6.0) as i32, 6, [245, 230, 210], 1.0);

    // Base plate
    gfx::fill_rect(&mut img, (cx - body_half - 12.0) as i32, body_bot as i32, (body_half * 2.0 + 24.0) as i32, 12, [180, 160, 130], 1.0);

    // ── Wax drips ─────────────────────────────────────────────────────────
    let drip_col = [238u8, 220, 200];
    // Right drip
    let rdrip_h = s.wax_r.min(body_bot - body_top - 10.0);
    gfx::fill_rect(&mut img, (cx + body_half - 6.0) as i32, body_top as i32, 10, rdrip_h as i32, drip_col, 1.0);
    if rdrip_h > 4.0 {
        gfx::fill_circle(&mut img, cx + body_half - 1.0, body_top + rdrip_h, 7.0, drip_col, 1.0);
    }
    // Left drip
    let ldrip_h = s.wax_l.min(body_bot - body_top - 10.0);
    gfx::fill_rect(&mut img, (cx - body_half - 4.0) as i32, body_top as i32, 10, ldrip_h as i32, drip_col, 1.0);
    if ldrip_h > 4.0 {
        gfx::fill_circle(&mut img, cx - body_half + 1.0, body_top + ldrip_h, 6.0, drip_col, 1.0);
    }

    // ── Wick ─────────────────────────────────────────────────────────────
    let wick_base = body_top;
    let wick_top  = wick_base - 14.0;
    gfx::glowing_line(&mut img, cx, wick_base, cx, wick_top,
        [40, 30, 20], [20, 15, 10], 1.5, 3.0, 1.0);

    if !s.candle_lit {
        // Smoke from unlit wick
        for id in 0u32..8 {
            let period = 2.0;
            let t_off = id as f32 * 0.25;
            let age = (t + t_off) % period;
            let frac = age / period;
            let sx = cx + gfx::hs(id * 7) * age * 12.0;
            let sy = wick_top - age * 35.0;
            let sr = 4.0 + frac * 20.0;
            let alpha = (1.0 - frac) * 0.35;
            gfx::fill_circle(&mut img, sx, sy, sr, [160, 160, 160], alpha);
        }
        return DynamicImage::ImageRgba8(img);
    }

    // ── Flame ─────────────────────────────────────────────────────────────
    let flame_base = wick_top + 4.0;
    let flame_h = 70.0 + (t * 3.1).sin() * 8.0 + (t * 7.7).sin() * 4.0;
    let flame_w = 22.0 + (t * 4.3).sin() * 3.0;

    gfx::draw_flame(&mut img, cx, flame_base, flame_w, flame_h, t, f);

    // Bright wick ember
    gfx::fill_circle(&mut img, cx, wick_top, 4.0, [255, 200, 100], 1.0);
    gfx::radial_glow(&mut img, cx, wick_top, 12.0, [255, 150, 50], 0.8);

    // ── Smoke wisps from tip ──────────────────────────────────────────────
    let tip_y = flame_base - flame_h;
    for id in 0u32..10 {
        let period = 2.5 + gfx::h(id * 7) * 1.5;
        let t_off = id as f32 * period / 10.0;
        let age = (t + t_off) % period;
        if age > 2.2 { continue; }
        let frac = age / 2.2;
        let sx = cx + gfx::hs(id * 11) * age * 18.0;
        let sy = tip_y - age * 45.0;
        let sr = 3.0 + frac * 22.0;
        let alpha = if frac < 0.15 { frac / 0.15 * 0.3 } else { (1.0 - frac) * 0.3 };
        let gray = (160.0 + frac * 60.0) as u8;
        gfx::fill_circle(&mut img, sx, sy, sr, [gray, gray, gray], alpha);
    }

    DynamicImage::ImageRgba8(img)
}

// ─── Main loop ────────────────────────────────────────────────────────────────
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (term_w, term_h) = terminal::size()?;

    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    let cfg = Config {
        x: 0, y: 0,
        width: Some(term_w as u32),
        height: Some(term_h.saturating_sub(2) as u32),
        ..Default::default()
    };

    let mut state = State::default();
    let frame_dur = Duration::from_secs_f64(1.0 / FPS);

    loop {
        let frame_start = Instant::now();

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
                    KeyCode::Tab       => { state.scene = state.scene.next(); state.elapsed = 0.0; state.wax_r = 0.0; state.wax_l = 0.0; }
                    KeyCode::Char('1') => { state.scene = Scene::Laser; state.elapsed = 0.0; }
                    KeyCode::Char('2') => { state.scene = Scene::Pot;   state.elapsed = 0.0; }
                    KeyCode::Char('3') => { state.scene = Scene::Candle; state.elapsed = 0.0; state.wax_r = 0.0; state.wax_l = 0.0; }
                    KeyCode::Char('l') | KeyCode::Char(' ') => {
                        if state.scene == Scene::Candle { state.candle_lit = !state.candle_lit; }
                    }
                    _ => {}
                }
            }
        }

        // ── Tick ─────────────────────────────────────────────────────────
        let dt = frame_dur.as_secs_f32();
        state.elapsed += dt;
        state.frame   += 1;
        if state.scene == Scene::Candle && state.candle_lit {
            state.wax_r = (state.wax_r + dt * 7.0).min(110.0);
            if state.elapsed > 4.0 { state.wax_l = (state.wax_l + dt * 5.0).min(90.0); }
        }

        // ── Render ───────────────────────────────────────────────────────
        let img = match state.scene {
            Scene::Laser  => render_laser(&state),
            Scene::Pot    => render_pot(&state),
            Scene::Candle => render_candle(&state),
        };

        execute!(stdout(), cursor::MoveTo(0, 0))?;
        viuer::print(&img, &cfg)?;

        // Status line
        execute!(stdout(), cursor::MoveTo(0, term_h - 1))?;
        let extra = if state.scene == Scene::Candle { "  [SPACE/L] flame" } else { "" };
        print!(
            " {} │ [1] Laser  [2] Pot  [3] Candle  [Tab] Next{}  [Q] Quit  t={:.1}s",
            state.scene.label(), extra, state.elapsed
        );
        stdout().flush()?;

        // ── Sleep ────────────────────────────────────────────────────────
        let elapsed = frame_start.elapsed();
        if elapsed < frame_dur { std::thread::sleep(frame_dur - elapsed); }
    }
}
