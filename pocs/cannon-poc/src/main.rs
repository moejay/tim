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
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

// ─── Config ──────────────────────────────────────────────────────────────────

const FPS: f64 = 60.0;
const G_NORMAL: f32 = 30.0; // terminal cells / s²
const G_MOON: f32 = 5.0;
const CANNON_COL: f32 = 2.0; // distance from left border
const TRAIL_MAX: usize = 35;
const PREVIEW_STEPS: usize = 120;
const PREVIEW_DT: f32 = 0.033;

// ─── Gravity Mode ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
enum GravityMode {
    Normal,
    Moon,
    Flipped,
    ZeroG,
}

impl GravityMode {
    fn accel(self) -> f32 {
        match self {
            Self::Normal  =>  G_NORMAL,
            Self::Moon    =>  G_MOON,
            Self::Flipped => -G_NORMAL,
            Self::ZeroG   =>  0.0,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Normal  => "Normal",
            Self::Moon    => "Moon (g/6)",
            Self::Flipped => "Flipped",
            Self::ZeroG   => "Zero-G",
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Normal  => Color::Cyan,
            Self::Moon    => Color::Blue,
            Self::Flipped => Color::Magenta,
            Self::ZeroG   => Color::Green,
        }
    }
}

// ─── Game State ──────────────────────────────────────────────────────────────

#[derive(Resource)]
struct State {
    pos: Vec2,
    vel: Vec2,
    active: bool,
    trail: Vec<Vec2>,
    bounces: u32,
    angle_deg: f32,
    power: f32,
    gravity: GravityMode,
    restitution: f32,
    term_w: u16,
    term_h: u16,
    msg: String,
    total_time: f32,
    max_speed: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            active: false,
            trail: Vec::new(),
            bounces: 0,
            angle_deg: -32.0,
            power: 42.0,
            gravity: GravityMode::Normal,
            restitution: 0.78,
            term_w: 80,
            term_h: 24,
            msg: "SPACE to fire  |  Q to quit".into(),
            total_time: 0.0,
            max_speed: 0.0,
        }
    }
}

impl State {
    fn ball_start(&self) -> Vec2 {
        Vec2::new(CANNON_COL + 7.0, self.term_h as f32 / 2.0)
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        let x0 = 1.0_f32;
        let x1 = self.term_w.saturating_sub(2) as f32;
        let y0 = 1.0_f32;
        let y1 = self.term_h.saturating_sub(4) as f32;
        (x0, x1, y0, y1)
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / FPS),
            )),
        )
        .add_plugins(RatatuiPlugins::default())
        .init_resource::<State>()
        .add_systems(
            Update,
            (
                sync_size,
                handle_input.after(sync_size),
                tick_physics.after(handle_input),
                draw.after(tick_physics),
            ),
        )
        .run();
}

// ─── Systems ─────────────────────────────────────────────────────────────────

fn sync_size(mut state: ResMut<State>) {
    if let Ok((w, h)) = crossterm::terminal::size() {
        state.term_w = w;
        state.term_h = h;
    }
}

fn handle_input(
    mut keys: EventReader<KeyEvent>,
    mut state: ResMut<State>,
    mut exit: EventWriter<AppExit>,
) {
    for ev in keys.read() {
        if ev.kind == KeyEventKind::Release {
            continue;
        }
        match ev.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                exit.write(AppExit::Success);
            }
            KeyCode::Char(' ') | KeyCode::Enter => fire(&mut state),
            KeyCode::Char('r') => {
                state.active = false;
                state.trail.clear();
                state.bounces = 0;
                state.total_time = 0.0;
                state.max_speed = 0.0;
                state.msg = "Reset. SPACE to fire.".into();
            }
            // Gravity
            KeyCode::Char('1') => {
                state.gravity = GravityMode::Normal;
                state.msg = format!("Gravity: {}", state.gravity.label());
            }
            KeyCode::Char('2') => {
                state.gravity = GravityMode::Moon;
                state.msg = format!("Gravity: {}", state.gravity.label());
            }
            KeyCode::Char('3') => {
                state.gravity = GravityMode::Flipped;
                state.msg = format!("Gravity: {}!", state.gravity.label());
            }
            KeyCode::Char('4') => {
                state.gravity = GravityMode::ZeroG;
                state.msg = format!("Gravity: {}", state.gravity.label());
            }
            // Angle (Up = aim higher = more negative in Y-down coords)
            KeyCode::Up   => state.angle_deg = (state.angle_deg - 5.0).clamp(-85.0, 85.0),
            KeyCode::Down => state.angle_deg = (state.angle_deg + 5.0).clamp(-85.0, 85.0),
            // Power
            KeyCode::Right => state.power = (state.power + 3.0).min(95.0),
            KeyCode::Left  => state.power = (state.power - 3.0).max(5.0),
            // Restitution (bounciness)
            KeyCode::Char('+') | KeyCode::Char('=') => {
                state.restitution = (state.restitution + 0.05).min(1.0);
                state.msg = format!("Bounciness: {:.0}%", state.restitution * 100.0);
            }
            KeyCode::Char('-') | KeyCode::Char('_') => {
                state.restitution = (state.restitution - 0.05).max(0.0);
                state.msg = format!("Bounciness: {:.0}%", state.restitution * 100.0);
            }
            _ => {}
        }
    }
}

fn fire(state: &mut State) {
    state.active = true;
    state.trail.clear();
    state.bounces = 0;
    state.total_time = 0.0;
    state.max_speed = 0.0;
    state.msg = "Fired!".into();
    state.pos = state.ball_start();
    let rad = state.angle_deg.to_radians();
    state.vel = Vec2::new(state.power * rad.cos(), state.power * rad.sin());
}

fn tick_physics(mut state: ResMut<State>, time: Res<Time>) {
    if !state.active {
        return;
    }

    let dt = time.delta_secs();
    state.total_time += dt;

    // Record trail (copy primitives to avoid borrow conflicts)
    let cur_pos = state.pos;
    state.trail.push(cur_pos);
    if state.trail.len() > TRAIL_MAX {
        state.trail.remove(0);
    }

    // Apply gravity (Y-down: positive = downward)
    let g = state.gravity.accel();
    state.vel.y += g * dt;

    // Integrate
    let vel = state.vel;
    state.pos += vel * dt;

    // Track max speed
    let speed = state.vel.length();
    if speed > state.max_speed {
        state.max_speed = speed;
    }

    // Bounce off walls
    let (x0, x1, y0, y1) = state.bounds();
    let r = state.restitution;
    let mut bounced = false;

    if state.pos.x < x0 { state.pos.x = x0; state.vel.x =  state.vel.x.abs() * r; bounced = true; }
    if state.pos.x > x1 { state.pos.x = x1; state.vel.x = -state.vel.x.abs() * r; bounced = true; }
    if state.pos.y < y0 { state.pos.y = y0; state.vel.y =  state.vel.y.abs() * r; bounced = true; }
    if state.pos.y > y1 { state.pos.y = y1; state.vel.y = -state.vel.y.abs() * r; bounced = true; }

    if bounced {
        state.bounces += 1;
    }

    // Rest detection
    if state.vel.length_squared() < 0.25 {
        state.vel = Vec2::ZERO;
        state.msg = format!(
            "At rest. Bounces: {}  Time: {:.1}s  Peak speed: {:.0}",
            state.bounces, state.total_time, state.max_speed
        );
    }
}

fn draw(mut ctx: ResMut<RatatuiContext>, state: Res<State>) {
    let _ = ctx.draw(|f| {
        let area = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        f.render_widget(GameCanvas { state: &state }, chunks[0]);
        f.render_widget(StatusBar { state: &state }, chunks[1]);
    });
}

// ─── Widgets ─────────────────────────────────────────────────────────────────

struct GameCanvas<'a> {
    state: &'a State,
}

struct StatusBar<'a> {
    state: &'a State,
}

impl Widget for GameCanvas<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.state;

        // Draw border / walls
        Block::default()
            .title(" ⊙ CANNON POC  ─  Terminal Physics Demo ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .render(area, buf);

        // Preview arc (shown when idle)
        if !s.active {
            let preview = compute_preview(s);
            for (i, &pos) in preview.iter().enumerate() {
                if i % 2 != 0 { continue; }
                let px = pos.x.round() as u16;
                let py = pos.y.round() as u16;
                if cell_in_area(px, py, area) {
                    if let Some(cell) = buf.cell_mut((px, py)) {
                        cell.set_symbol("·");
                        cell.fg = Color::Indexed(237);
                    }
                }
            }
        }

        // Trail (fades from dark to bright)
        let trail_len = s.trail.len();
        for (i, &pos) in s.trail.iter().enumerate() {
            let tx = pos.x.round() as u16;
            let ty = pos.y.round() as u16;
            if !cell_in_area(tx, ty, area) { continue; }
            let t = i as f32 / trail_len as f32;
            let (sym, color) = if t < 0.25 {
                ("·", Color::Indexed(235))
            } else if t < 0.5 {
                ("·", Color::Indexed(238))
            } else if t < 0.75 {
                ("·", Color::DarkGray)
            } else {
                ("∙", Color::Gray)
            };
            if let Some(cell) = buf.cell_mut((tx, ty)) {
                cell.set_symbol(sym);
                cell.fg = color;
            }
        }

        // Cannon
        let cannon_x = area.x + 1 + CANNON_COL as u16;
        let cannon_y = area.y + area.height / 2;
        draw_cannon(buf, cannon_x, cannon_y, area, s.angle_deg);

        // Cannonball
        if s.active {
            let bx = s.pos.x.round() as u16;
            let by = s.pos.y.round() as u16;
            if cell_in_area(bx, by, area) {
                // Glow effect: color the surrounding cells dim
                for dy in [bx.wrapping_sub(1), bx, bx + 1] {
                    for row in [by.wrapping_sub(1), by, by + 1] {
                        if dy == bx && row == by { continue; }
                        if cell_in_area(dy, row, area) {
                            if let Some(c) = buf.cell_mut((dy, row)) {
                                if c.symbol() == " " {
                                    c.set_symbol("░");
                                    c.fg = Color::Indexed(52); // dark red glow
                                }
                            }
                        }
                    }
                }
                if let Some(cell) = buf.cell_mut((bx, by)) {
                    cell.set_symbol("●");
                    cell.fg = Color::Red;
                    cell.modifier = Modifier::BOLD;
                }
            }
        }

        // Velocity vector indicator (small arrow showing direction)
        if s.active && s.vel.length_squared() > 1.0 {
            let vx = s.pos.x + s.vel.x.signum() * 2.0;
            let vy = s.pos.y + s.vel.y.signum() * 1.0;
            let ax = vx.round() as u16;
            let ay = vy.round() as u16;
            if cell_in_area(ax, ay, area) {
                let arrow = direction_arrow(s.vel);
                if let Some(cell) = buf.cell_mut((ax, ay)) {
                    if cell.symbol() == " " || cell.symbol() == "·" || cell.symbol() == "░" {
                        cell.set_symbol(arrow);
                        cell.fg = Color::Indexed(166); // orange
                    }
                }
            }
        }
    }
}

fn draw_cannon(buf: &mut Buffer, cx: u16, cy: u16, area: Rect, angle_deg: f32) {
    let body_style = Style::default().fg(Color::Rgb(180, 140, 60));
    let barrel_style = Style::default().fg(Color::Rgb(220, 180, 80));

    // Body block (2 wide × 3 tall, centered on cy)
    for dy in 0u16..3 {
        let row = cy.saturating_sub(1) + dy;
        for col in 0u16..2 {
            let bx = cx + col;
            if cell_in_area(bx, row, area) {
                if let Some(c) = buf.cell_mut((bx, row)) {
                    c.set_symbol("█");
                    c.set_style(body_style);
                }
            }
        }
    }

    // Barrel (3 segments + tip), direction depends on angle
    let (bx_start, by_start, bx2, by2, bx3, by3, tip_x, tip_y) = if angle_deg < -20.0 {
        // Aiming up-right
        (cx + 2, cy, cx + 3, cy - 1, cx + 4, cy - 1, cx + 5, cy - 2)
    } else if angle_deg > 20.0 {
        // Aiming down-right
        (cx + 2, cy, cx + 3, cy + 1, cx + 4, cy + 1, cx + 5, cy + 2)
    } else {
        // Horizontal
        (cx + 2, cy, cx + 3, cy, cx + 4, cy, cx + 5, cy)
    };

    for &(bx, by) in &[(bx_start, by_start), (bx2, by2), (bx3, by3)] {
        if cell_in_area(bx, by, area) {
            if let Some(c) = buf.cell_mut((bx, by)) {
                c.set_symbol("═");
                c.set_style(barrel_style);
            }
        }
    }
    if cell_in_area(tip_x, tip_y, area) {
        if let Some(c) = buf.cell_mut((tip_x, tip_y)) {
            c.set_symbol("►");
            c.set_style(barrel_style);
        }
    }

    // Wheels
    let wheel_y = cy + 2;
    for wx in [cx, cx + 1] {
        if cell_in_area(wx, wheel_y, area) {
            if let Some(c) = buf.cell_mut((wx, wheel_y)) {
                c.set_symbol("◎");
                c.fg = Color::Indexed(130);
            }
        }
    }
}

fn compute_preview(s: &State) -> Vec<Vec2> {
    let (x0, x1, y0, y1) = s.bounds();
    let mut pos = s.ball_start();
    let rad = s.angle_deg.to_radians();
    let mut vel = Vec2::new(s.power * rad.cos(), s.power * rad.sin());
    let g = s.gravity.accel();
    let r = s.restitution;
    let mut out = Vec::with_capacity(PREVIEW_STEPS);

    for _ in 0..PREVIEW_STEPS {
        vel.y += g * PREVIEW_DT;
        pos += vel * PREVIEW_DT;

        // Bounce in preview too (so it shows full path)
        if pos.x < x0 { pos.x = x0; vel.x =  vel.x.abs() * r; }
        if pos.x > x1 { pos.x = x1; vel.x = -vel.x.abs() * r; }
        if pos.y < y0 { pos.y = y0; vel.y =  vel.y.abs() * r; }
        if pos.y > y1 { pos.y = y1; vel.y = -vel.y.abs() * r; }

        out.push(pos);

        if vel.length_squared() < 0.5 {
            break;
        }
    }
    out
}

fn cell_in_area(x: u16, y: u16, area: Rect) -> bool {
    x > area.x && x < area.right().saturating_sub(1)
        && y > area.y && y < area.bottom().saturating_sub(1)
}

fn direction_arrow(vel: Vec2) -> &'static str {
    let vx = vel.x;
    let vy = vel.y;
    if vx.abs() > vy.abs() * 2.0 {
        if vx > 0.0 { "→" } else { "←" }
    } else if vy.abs() > vx.abs() * 2.0 {
        if vy > 0.0 { "↓" } else { "↑" }
    } else if vx > 0.0 {
        if vy > 0.0 { "↘" } else { "↗" }
    } else {
        if vy > 0.0 { "↙" } else { "↖" }
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.state;

        let angle_sym = if s.angle_deg < -5.0 {
            "▲"
        } else if s.angle_deg > 5.0 {
            "▼"
        } else {
            "→"
        };

        let speed = s.vel.length();
        let gravity_color = s.gravity.color();

        let line1 = Line::from(vec![
            Span::styled("  Gravity: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:<12}", s.gravity.label()),
                Style::default().fg(gravity_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  Power: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:>2.0}", s.power), Style::default().fg(Color::Green)),
            Span::styled("  Angle: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}{:.0}°", angle_sym, s.angle_deg.abs()),
                Style::default().fg(Color::Green),
            ),
            Span::styled("  Bounce: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.0}%", s.restitution * 100.0),
                Style::default().fg(Color::Green),
            ),
            Span::styled("  Hits: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", s.bounces),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled("  Speed: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:>4.0}", speed),
                Style::default().fg(if speed > 30.0 { Color::Red } else { Color::White }),
            ),
            Span::styled(
                format!("   {}", s.msg),
                Style::default().fg(Color::White),
            ),
        ]);

        let line2 = Line::from(Span::styled(
            "  [SPACE] Fire  [R] Reset  [↑↓] Angle  [←→] Power  \
             [1] Normal  [2] Moon  [3] Flip  [4] Zero-G  [+/-] Bounce  [Q] Quit",
            Style::default().fg(Color::Indexed(240)),
        ));

        Paragraph::new(vec![line1, line2])
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(Color::Indexed(236))),
            )
            .render(area, buf);
    }
}
