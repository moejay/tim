/// Visual fidelity demos: Laser Gun · Boiling Pot · Lit Candle
/// Navigate with [1] [2] [3] or [Tab]. [Q] to quit.
use std::time::Duration;

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
};
use bevy_ratatui::{event::KeyEvent, RatatuiContext, RatatuiPlugins};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Widget},
};

const FPS: f64 = 60.0;

// ─── Noise ───────────────────────────────────────────────────────────────────
// Seedable deterministic noise [0.0, 1.0)
fn h(seed: u32) -> f32 {
    let x = seed.wrapping_mul(2246822519).wrapping_add(2654435769);
    let x = x ^ (x >> 13);
    let x = x.wrapping_mul(3266489917);
    let x = x ^ (x >> 16);
    (x >> 8) as f32 / 16777215.0
}

// Signed noise [-1.0, 1.0)
fn hs(seed: u32) -> f32 { h(seed) * 2.0 - 1.0 }

// ─── Scene ───────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum Scene { Laser, Pot, Candle }

impl Scene {
    fn next(self) -> Self {
        match self { Self::Laser => Self::Pot, Self::Pot => Self::Candle, Self::Candle => Self::Laser }
    }
    fn label(self) -> &'static str {
        match self { Self::Laser => "Laser Gun", Self::Pot => "Boiling Pot", Self::Candle => "Lit Candle" }
    }
}

// ─── State ───────────────────────────────────────────────────────────────────
#[derive(Resource)]
struct State {
    scene: Scene,
    elapsed: f32,
    frame: u64,
    term_w: u16,
    term_h: u16,
    candle_lit: bool,
    wax_l: f32,  // how far down left wax drip has extended
    wax_r: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            scene: Scene::Laser,
            elapsed: 0.0,
            frame: 0,
            term_w: 80,
            term_h: 24,
            candle_lit: true,
            wax_l: 0.0,
            wax_r: 0.0,
        }
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / FPS),
        )))
        .add_plugins(RatatuiPlugins::default())
        .init_resource::<State>()
        .add_systems(Update, (
            sync_size,
            handle_input.after(sync_size),
            tick.after(handle_input),
            render.after(tick),
        ))
        .run();
}

// ─── Systems ─────────────────────────────────────────────────────────────────
fn sync_size(mut s: ResMut<State>) {
    if let Ok((w, h)) = crossterm::terminal::size() { s.term_w = w; s.term_h = h; }
}

fn handle_input(mut keys: EventReader<KeyEvent>, mut s: ResMut<State>, mut exit: EventWriter<AppExit>) {
    for ev in keys.read() {
        if ev.kind == KeyEventKind::Release { continue; }
        match ev.code {
            KeyCode::Char('q') | KeyCode::Esc => { exit.write(AppExit::Success); }
            KeyCode::Tab  => { s.scene = s.scene.next(); s.wax_l = 0.0; s.wax_r = 0.0; }
            KeyCode::Char('1') => { s.scene = Scene::Laser; }
            KeyCode::Char('2') => { s.scene = Scene::Pot; }
            KeyCode::Char('3') => { s.scene = Scene::Candle; s.wax_l = 0.0; s.wax_r = 0.0; }
            KeyCode::Char('l') | KeyCode::Char(' ') => {
                if s.scene == Scene::Candle { s.candle_lit = !s.candle_lit; }
            }
            _ => {}
        }
    }
}

fn tick(mut s: ResMut<State>, time: Res<Time>) {
    let dt = time.delta_secs();
    s.elapsed += dt;
    s.frame   += 1;
    if s.scene == Scene::Candle && s.candle_lit {
        s.wax_r = (s.wax_r + dt * 0.25).min(6.0);
        if s.elapsed > 4.0 { s.wax_l = (s.wax_l + dt * 0.18).min(5.0); }
    }
}

fn render(mut ctx: ResMut<RatatuiContext>, state: Res<State>) {
    let _ = ctx.draw(|f| {
        let area = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(2)])
            .split(area);
        f.render_widget(SceneView { state: &state }, chunks[0]);
        f.render_widget(HelpBar  { state: &state }, chunks[1]);
    });
}

// ─── Scene Router ─────────────────────────────────────────────────────────────
struct SceneView<'a> { state: &'a State }
struct HelpBar<'a>   { state: &'a State }

impl Widget for SceneView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.state;
        Block::default()
            .title(format!(" ─ {} ─ ", s.scene.label()))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Indexed(238)))
            .render(area, buf);

        let inner = Rect {
            x: area.x + 1, y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        match s.scene {
            Scene::Laser  => draw_laser(buf, inner, s),
            Scene::Pot    => draw_pot(buf, inner, s),
            Scene::Candle => draw_candle(buf, inner, s),
        }
    }
}

impl Widget for HelpBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.state;
        let extra = if s.scene == Scene::Candle { "  [SPACE/L] Toggle flame" } else { "" };
        let text = format!(
            " [1] Laser  [2] Pot  [3] Candle  [Tab] Next{}  [Q] Quit   t={:.1}s",
            extra, s.elapsed
        );
        Paragraph::new(Span::styled(text, Style::default().fg(Color::Indexed(240))))
            .render(area, buf);
    }
}

// ─── Helper: set one cell ────────────────────────────────────────────────────
fn put(buf: &mut Buffer, x: u16, y: u16, area: Rect, sym: &str, fg: Color) {
    if x >= area.x && x < area.right() && y >= area.y && y < area.bottom() {
        if let Some(c) = buf.cell_mut((x, y)) {
            c.set_symbol(sym);
            c.fg = fg;
        }
    }
}

fn put_style(buf: &mut Buffer, x: u16, y: u16, area: Rect, sym: &str, style: Style) {
    if x >= area.x && x < area.right() && y >= area.y && y < area.bottom() {
        if let Some(c) = buf.cell_mut((x, y)) {
            c.set_symbol(sym).set_style(style);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SCENE 1: LASER GUN
// ═════════════════════════════════════════════════════════════════════════════
fn draw_laser(buf: &mut Buffer, area: Rect, s: &State) {
    let t = s.elapsed;
    let f = s.frame;
    let w = area.width;
    let cx_gun: u16 = area.x + 3;

    // Three emitters at different heights
    let rows: [u16; 3] = [
        area.y + area.height / 2 - 3,
        area.y + area.height / 2,
        area.y + area.height / 2 + 3,
    ];

    for (beam_i, &gy) in rows.iter().enumerate() {
        let phase_offset = beam_i as f32 * 0.4;
        let active = ((t + phase_offset) * 1.2).sin() > -0.3;  // ~75% duty cycle

        // Gun body
        let gun_color = Color::Indexed(244);
        let glow_color = if active { Color::Cyan } else { Color::Indexed(30) };
        for dy in 0u16..3 {
            let row = gy.saturating_sub(1) + dy;
            put(buf, cx_gun,     row, area, "█", gun_color);
            put(buf, cx_gun + 1, row, area, "█", gun_color);
        }
        put(buf, cx_gun + 2, gy, area, "╠", glow_color);
        put(buf, cx_gun + 3, gy, area, "═", glow_color);

        if !active { continue; }

        // Beam: animate energy travelling along it
        // chars alternate between ─ and ═ creating a moving-wave illusion
        let beam_start = cx_gun + 4;
        let beam_end   = area.x + w - 2;

        for bx in beam_start..=beam_end {
            // travelling wave: phase based on position and time
            let dist = (bx - beam_start) as f32;
            let wave = ((dist - t * 18.0) * 0.4).sin();
            let (sym, col) = if wave > 0.4 {
                ("═", Color::White)
            } else if wave > -0.2 {
                ("─", Color::Cyan)
            } else {
                ("╌", Color::Indexed(37))
            };
            put(buf, bx, gy, area, sym, col);
        }

        // Tip glow
        let tip = area.x + w - 2;
        let tip_chars = ["✦", "✧", "*", "✦", "+"];
        let ti = ((t * 12.0 + beam_i as f32) as usize) % tip_chars.len();
        put(buf, tip, gy, area, tip_chars[ti], Color::White);

        // Impact sparks: procedural particles based on frame
        for spark_id in 0u32..8 {
            let seed = (beam_i as u32) * 1000 + spark_id * 17 + (f / 3) as u32;
            let age  = h(seed) * 0.6;          // age within life
            let life = h(seed + 1) * 0.5 + 0.1; // total life
            if age > life { continue; }
            let frac = age / life;
            let sx = tip as f32 - h(seed + 2) * 6.0 * frac;
            let sy = gy as f32 + hs(seed + 3) * 4.0 * frac;
            let sym = if frac < 0.3 { "✦" } else if frac < 0.6 { "·" } else { "∙" };
            let col = if frac < 0.4 { Color::White } else { Color::Indexed(51) };
            put(buf, sx as u16, sy as u16, area, sym, col);
        }
    }

    // Label
    let label_y = area.y + area.height - 1;
    let label = "  High-energy plasma beams · 60fps terminal render";
    if label_y < area.bottom() {
        for (i, ch) in label.chars().enumerate() {
            let x = area.x + i as u16;
            if x < area.right() {
                if let Some(c) = buf.cell_mut((x, label_y)) {
                    let s = ch.to_string();
                    c.set_symbol(s.as_str());
                    c.fg = Color::Indexed(238);
                }
            }
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SCENE 2: BOILING POT
// ═════════════════════════════════════════════════════════════════════════════
fn draw_pot(buf: &mut Buffer, area: Rect, s: &State) {
    let t = s.elapsed;
    let f = s.frame;

    // Pot centre
    let cx = (area.x + area.width / 2) as i32;
    let cy = (area.y + area.height / 2) as i32;

    // Rattle: offset pot position by ±1 based on fast oscillation
    let rx = (( t * 19.0).sin() * 0.7 + (t * 31.0).sin() * 0.3) as i32;
    let ry = (( t * 23.0).sin() * 0.45) as i32;
    let px = cx + rx;
    let py = cy + ry;

    let pot_w: i32 = 12; // half-width inside pot
    let pot_h: i32 = 5;

    let put_i = |buf: &mut Buffer, x: i32, y: i32, sym: &str, fg: Color| {
        if x >= 0 && y >= 0 { put(buf, x as u16, y as u16, area, sym, fg); }
    };

    // Water surface (animated wave)
    let water_y = py - pot_h + 2;
    let water_chars = ["≋", "≈", "~", "≈", "≋"];
    for dx in -(pot_w - 1)..pot_w {
        let phase = ((dx as f32 * 0.8 + t * 6.0).sin() * 2.0) as usize % water_chars.len();
        let col = Color::Indexed(39 + (((dx + pot_w) as u32 + f as u32 / 4) % 3) as u8);
        put_i(buf, px + dx, water_y, water_chars[phase], col);
    }

    // Pot walls
    let pot_color = Color::Indexed(130); // dark bronze
    let rim_color = Color::Indexed(136);

    // Top rim
    put_i(buf, px - pot_w, py - pot_h + 1, "╔", rim_color);
    put_i(buf, px + pot_w, py - pot_h + 1, "╗", rim_color);
    for dx in -(pot_w - 1)..pot_w {
        put_i(buf, px + dx, py - pot_h + 1, "═", rim_color);
    }

    // Side walls
    for dy in -(pot_h - 2)..pot_h {
        put_i(buf, px - pot_w, py + dy, "║", pot_color);
        put_i(buf, px + pot_w, py + dy, "║", pot_color);
    }

    // Handles
    put_i(buf, px - pot_w - 1, py, "═", pot_color);
    put_i(buf, px - pot_w - 2, py, "╗", pot_color);
    put_i(buf, px - pot_w - 2, py + 1, "╝", pot_color);
    put_i(buf, px + pot_w + 1, py, "═", pot_color);
    put_i(buf, px + pot_w + 2, py, "╔", pot_color);
    put_i(buf, px + pot_w + 2, py + 1, "╚", pot_color);

    // Bottom
    put_i(buf, px - pot_w, py + pot_h - 1, "╚", pot_color);
    put_i(buf, px + pot_w, py + pot_h - 1, "╝", pot_color);
    for dx in -(pot_w - 1)..pot_w {
        put_i(buf, px + dx, py + pot_h - 1, "═", pot_color);
    }

    // Heat glow under pot
    for dy in 0i32..2 {
        for dx in -(pot_w)..=pot_w {
            let gx = (px + dx) as u16;
            let gy = (py + pot_h + dy) as u16;
            if gx < area.right() && gy < area.bottom() {
                let wave = ((dx as f32 * 0.5 + t * 8.0 + dy as f32).sin() + 1.0) / 2.0;
                let col = if wave > 0.6 { Color::Indexed(196) }
                          else if wave > 0.3 { Color::Indexed(202) }
                          else { Color::Indexed(88) };
                put(buf, gx, gy, area, "▄", col);
            }
        }
    }

    // Steam particles (procedural: N particles, each on a staggered schedule)
    let steam_sym = ["∿", "~", "∿", "〰", "~"];
    for id in 0u32..18 {
        let period = 3.0 + h(id * 3) * 1.5;
        let t_offset = id as f32 * (period / 18.0);
        let age = (t + t_offset) % period;
        if age > 2.5 { continue; }

        let birth_x = px as f32 + hs(id * 7) * (pot_w as f32 - 1.0);
        let wobble = (age * 3.0 + h(id * 11) * 6.28).sin() * 1.5;
        let sx = birth_x + wobble;
        let sy = water_y as f32 - age * 2.2 - 2.0;

        let frac = age / 2.5;
        let col = if frac < 0.2 { Color::Indexed(253) }
                  else if frac < 0.5 { Color::Indexed(248) }
                  else if frac < 0.8 { Color::Indexed(243) }
                  else { Color::Indexed(238) };
        let si = (id as usize + (t * 4.0) as usize) % steam_sym.len();
        put(buf, sx as u16, sy as u16, area, steam_sym[si], col);
    }

    // Bubble particles (rise from water surface, pop at top)
    for id in 0u32..12 {
        let period = 1.0 + h(id * 5) * 1.2;
        let t_offset = id as f32 * (period / 12.0);
        let age = (t + t_offset) % period;
        let frac = age / period;

        let bx = (px as f32 + hs(id * 13) * (pot_w as f32 - 2.0)) as u16;
        let by = (water_y as f32 - frac * 1.5) as u16;

        let sym = if frac > 0.85 { "∘" } else if frac > 0.5 { "○" } else { "°" };
        let col = Color::Indexed(51);
        put(buf, bx, by, area, sym, col);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SCENE 3: LIT CANDLE
// ═════════════════════════════════════════════════════════════════════════════
fn draw_candle(buf: &mut Buffer, area: Rect, s: &State) {
    let t = s.elapsed;
    let f = s.frame;

    let cx = area.x + area.width / 2;
    let cy = area.y + area.height / 2;

    // ── Glow: ambient warm light around flame ──────────────────────────────
    if s.candle_lit {
        let _glow_cx = cx as f32;
        let _glow_cy = (cy as f32) - 3.5;
        let flicker = 1.0 + (t * 7.3).sin() * 0.15 + (t * 13.1).sin() * 0.08;

        for dy in -6i32..=4 {
            for dx in -8i32..=8 {
                let gx = (cx as i32 + dx) as u16;
                let gy = (cy as i32 + dy) as u16;
                if gx >= area.right() || gy >= area.bottom() || gx < area.x || gy < area.y { continue; }
                // Slightly squash Y (chars are taller than wide)
                let dist = ((dx as f32 / 2.0).powi(2) + (dy as f32 / 1.4).powi(2)).sqrt() / flicker;
                if dist < 1.5 { continue; } // flame draws over this
                let col = if dist < 2.5 { Color::Indexed(136) }
                          else if dist < 4.0 { Color::Indexed(52) }
                          else if dist < 5.5 { Color::Indexed(235) }
                          else { continue };
                if let Some(cell) = buf.cell_mut((gx, gy)) {
                    if cell.symbol() == " " {
                        cell.fg = col;
                        cell.set_symbol("░");
                    }
                }
            }
        }
    }

    // ── Candle body (tall, centered) ───────────────────────────────────────
    let body_top = cy + 1;
    let body_bot = cy + 5;
    let body_col = Color::Rgb(240, 220, 200);

    for row in body_top..=body_bot {
        put(buf, cx - 1, row, area, "█", body_col);
        put(buf, cx,     row, area, "█", body_col);
        put(buf, cx + 1, row, area, "█", body_col);
    }
    // Base plate
    for dx in -2i32..=2 {
        put(buf, (cx as i32 + dx) as u16, body_bot + 1, area, "▄", Color::Indexed(180));
    }

    // ── Wax drips ──────────────────────────────────────────────────────────
    let drip_col = Color::Rgb(230, 210, 190);
    // Right drip
    for i in 0..(s.wax_r as u16) {
        put(buf, cx + 1, body_top + i, area, "▌", drip_col);
    }
    // Left drip (starts later)
    for i in 0..(s.wax_l as u16) {
        put(buf, cx - 1, body_top + i, area, "▐", drip_col);
    }
    // Drip tip (slight bulge)
    if s.wax_r > 0.5 {
        let tip_y = body_top + s.wax_r as u16;
        put(buf, cx + 1, tip_y, area, "▖", drip_col);
    }
    if s.wax_l > 0.5 {
        let tip_y = body_top + s.wax_l as u16;
        put(buf, cx - 1, tip_y, area, "▗", drip_col);
    }

    // ── Wick ───────────────────────────────────────────────────────────────
    put(buf, cx, cy + 1, area, "╻", Color::Indexed(240));

    if !s.candle_lit {
        // Unlit: show a curly unlit wick with smoke puff
        put(buf, cx, cy,     area, "╿", Color::Indexed(240));
        put(buf, cx, cy - 1, area, "∿", Color::Indexed(243));
        put(buf, cx, cy - 2, area, "~", Color::Indexed(238));
        return;
    }

    // ── Flame ──────────────────────────────────────────────────────────────
    // Flicker parameters change slowly
    let flicker_x  = (t * 5.3).sin() * 0.6 + (t * 9.7).sin() * 0.3;
    let flicker_h  = 1.0 + (t * 4.1).sin() * 0.2 + (t * 11.3).sin() * 0.1;
    let flame_rows = [
        // (dy from wick, chars_options, color)
        (-1i32, ["▀", "▄", "▀"], Color::Rgb(255, 100, 10)),    // base
        (-2,    ["▲", "▲", "△"], Color::Rgb(255, 160, 20)),    // lower flame
        (-3,    ["△", "▲", "△"], Color::Rgb(255, 210, 40)),    // mid flame
        (-4,    ["╿", "│", "╷"], Color::Rgb(255, 240, 100)),   // upper flame
        (-5,    ["╷", "·", " "], Color::Rgb(255, 255, 200)),   // tip
    ];

    for &(dy, chars, col) in &flame_rows {
        let row = (cy as i32 + dy) as u16;
        if row < area.y || row >= area.bottom() { continue; }

        // How wide is the flame at this height?
        let height_frac = (-dy - 1) as f32 / 4.0; // 0=base, 1=tip
        let width_scale = (1.0 - height_frac * 0.8) * flicker_h;
        let width = (width_scale * 2.0 + 0.5) as i32;

        for dx in -width..=width {
            let gx = (cx as i32 + dx + flicker_x as i32) as u16;
            if gx < area.x || gx >= area.right() { continue; }
            let ci = ((f / 4 + dx.unsigned_abs() as u64 + dy.unsigned_abs() as u64) % 3) as usize;
            put_style(buf, gx, row, area, chars[ci], Style::default().fg(col).add_modifier(Modifier::BOLD));
        }
    }

    // Flame tip ember (random bright spot)
    let tip_y = (cy as i32 - 5 - flicker_h as i32) as u16;
    let tip_x = (cx as i32 + flicker_x as i32) as u16;
    let ember_chars = ["·", "∙", " ", "·"];
    let ei = (f / 2) as usize % ember_chars.len();
    put(buf, tip_x, tip_y, area, ember_chars[ei], Color::White);

    // Smoke wisps rising from flame tip (only if very tall)
    for id in 0u32..6 {
        let period = 2.5 + h(id * 7) * 1.5;
        let t_off = id as f32 * (period / 6.0);
        let age = (t + t_off) % period;
        if age > 2.0 { continue; }
        let wx = tip_x as f32 + hs(id * 11) * age * 1.5;
        let wy = tip_y as f32 - age * 1.8 - 1.0;
        let frac = age / 2.0;
        let col = if frac < 0.3 { Color::Indexed(244) }
                  else if frac < 0.6 { Color::Indexed(240) }
                  else { Color::Indexed(236) };
        put(buf, wx as u16, wy as u16, area, "∿", col);
    }

    // Status note
    let note_y = area.bottom().saturating_sub(1);
    let note = "  Wax drips over time · [SPACE/L] blow out  ";
    for (i, ch) in note.chars().enumerate() {
        let x = area.x + i as u16;
        if x >= area.right() { break; }
        if let Some(c) = buf.cell_mut((x, note_y)) {
            let sym = ch.to_string();
            c.set_symbol(sym.as_str());
            c.fg = Color::Indexed(238);
        }
    }
}
