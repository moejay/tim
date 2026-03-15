use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use image::RgbaImage;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

use tim2::constants::*;
use tim2::parts::*;
use tim2::physics;
use tim2::puzzle::*;
use tim2::render::braille;
use tim2::world::*;

// ── Game State ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Build,
    Running,
    Won,
}

struct Game {
    world: World,
    puzzle: Puzzle,
    mode: Mode,
    frame: u64,

    // Parts bin
    bin: Vec<BinEntry>,
    selected_bin: usize,

    // Dragging
    dragging: Option<DragState>,

    // Player-placed instance IDs (for removal on reset)
    player_instances: Vec<InstanceId>,

    // Playfield area in terminal coords (set during render)
    playfield_rect: Rect,
}

#[derive(Debug, Clone)]
struct BinEntry {
    part_id: PartId,
    total: u32,
    placed: u32,
}

impl BinEntry {
    fn available(&self) -> u32 {
        self.total.saturating_sub(self.placed)
    }
}

#[derive(Debug, Clone)]
struct DragState {
    instance_id: InstanceId,
    /// Offset from part origin to grab point.
    offset_x: f32,
    offset_y: f32,
}

impl Game {
    fn new(world: World, puzzle: Puzzle) -> Self {
        let bin: Vec<BinEntry> = puzzle
            .bin_parts
            .iter()
            .map(|bp| BinEntry {
                part_id: bp.part_id,
                total: bp.quantity,
                placed: 0,
            })
            .collect();
        Self {
            world,
            puzzle,
            mode: Mode::Build,
            frame: 0,
            bin,
            selected_bin: 0,
            dragging: None,
            player_instances: Vec::new(),
            playfield_rect: Rect::default(),
        }
    }

    fn start_sim(&mut self) {
        if self.mode == Mode::Build {
            self.mode = Mode::Running;
            // Snapshot velocities to zero for all player parts
            for id in &self.player_instances {
                if let Some(inst) = self.world.get_mut(*id) {
                    inst.vx = 0.0;
                    inst.vy = 0.0;
                    inst.props.current_state = 0;
                }
            }
        }
    }

    fn stop_sim(&mut self) {
        self.mode = Mode::Build;
        // Reset all dynamic parts to initial state
        // For simplicity: remove player parts and re-place them at stored positions
        // (In a full impl we'd snapshot/restore. For now, just stop physics.)
        for inst in &mut self.world.instances {
            if !inst.def().physics().is_static {
                inst.vx = 0.0;
                inst.vy = 0.0;
                inst.props.current_state = 0;
            }
        }
    }

    fn reset_puzzle(&mut self) {
        // Remove all player-placed parts
        for id in self.player_instances.drain(..) {
            self.world.remove(id);
        }
        // Reset bin counts
        for entry in &mut self.bin {
            entry.placed = 0;
        }
        self.mode = Mode::Build;
        // Reset all fixed parts state
        for inst in &mut self.world.instances {
            inst.vx = 0.0;
            inst.vy = 0.0;
            inst.props.current_state = 0;
        }
    }

    fn tick(&mut self) {
        if self.mode != Mode::Running {
            return;
        }
        let dt = 1.0 / 60.0;
        physics::tick(&mut self.world, dt);

        // Check win conditions
        let status = evaluate(&self.world, &self.puzzle.win_conditions);
        if status == PuzzleStatus::Won {
            self.mode = Mode::Won;
        }
    }

    /// Convert terminal cell coords to world/canvas coords.
    fn screen_to_world(&self, sx: u16, sy: u16) -> (f32, f32) {
        let pf = self.playfield_rect;
        if pf.width == 0 || pf.height == 0 {
            return (0.0, 0.0);
        }
        // Each terminal cell maps to some canvas pixels
        // braille: 2px wide, 4px tall per cell
        let scale_x = CANVAS_W as f32 / (pf.width as f32 * 2.0);
        let scale_y = CANVAS_H as f32 / (pf.height as f32 * 4.0);
        let lx = (sx.saturating_sub(pf.x)) as f32 * 2.0 * scale_x;
        let ly = (sy.saturating_sub(pf.y)) as f32 * 4.0 * scale_y;
        (lx, ly)
    }

    /// Find instance at world coords.
    fn instance_at(&self, wx: f32, wy: f32) -> Option<InstanceId> {
        // Search in reverse (top-drawn last = on top)
        for inst in self.world.instances.iter().rev() {
            let (x1, y1, x2, y2) = inst.bounds();
            if wx >= x1 && wx <= x2 && wy >= y1 && wy <= y2 {
                return Some(inst.id);
            }
        }
        None
    }

    fn place_from_bin(&mut self, wx: f32, wy: f32) -> Option<InstanceId> {
        if self.selected_bin >= self.bin.len() {
            return None;
        }
        let entry = &self.bin[self.selected_bin];
        if entry.available() == 0 {
            return None;
        }
        let part_id = entry.part_id;
        let def = part_id.part_def();
        let (pw, ph) = def.default_size();
        let id = self.world.spawn(part_id, wx - pw / 2.0, wy - ph / 2.0);
        self.player_instances.push(id);
        self.bin[self.selected_bin].placed += 1;
        Some(id)
    }

    fn return_to_bin(&mut self, id: InstanceId) {
        // Find which bin entry this belongs to
        if let Some(inst) = self.world.get(id) {
            let part_id = inst.part_id;
            if let Some(entry) = self.bin.iter_mut().find(|e| e.part_id == part_id) {
                entry.placed = entry.placed.saturating_sub(1);
            }
        }
        self.world.remove(id);
        self.player_instances.retain(|i| *i != id);
    }
}

// ── First Puzzle ─────────────────────────────────────────────────

fn build_tutorial_puzzle() -> (World, Puzzle) {
    use tim2::parts::balls::BallType;
    use tim2::parts::walls::WallType;

    let mut world = World::new();

    // Floor
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 340.0);
    if let Some(inst) = world.instances.last_mut() {
        inst.props.width = 640.0;
        inst.props.height = 20.0;
    }

    // Left wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 0.0);
    if let Some(inst) = world.instances.last_mut() {
        inst.props.width = 10.0;
        inst.props.height = 340.0;
    }

    // Right wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 630.0, 0.0);
    if let Some(inst) = world.instances.last_mut() {
        inst.props.width = 10.0;
        inst.props.height = 340.0;
    }

    // Platform (shelf) on right side
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 400.0, 240.0);
    if let Some(inst) = world.instances.last_mut() {
        inst.props.width = 120.0;
        inst.props.height = 10.0;
    }

    // Target zone indicator — a small platform/bucket area bottom-left
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 60.0, 320.0);
    if let Some(inst) = world.instances.last_mut() {
        inst.props.width = 80.0;
        inst.props.height = 10.0;
    }
    // Left lip
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 55.0, 300.0);
    if let Some(inst) = world.instances.last_mut() {
        inst.props.width = 8.0;
        inst.props.height = 30.0;
    }
    // Right lip
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 137.0, 300.0);
    if let Some(inst) = world.instances.last_mut() {
        inst.props.width = 8.0;
        inst.props.height = 30.0;
    }

    // Pre-place a bowling ball on the shelf (this is the "fixed" ball)
    let ball_id = world.spawn_locked(PartId::Ball(BallType::BowlingBall), 440.0, 200.0);
    // Unlock it so physics affects it (locked just means player can't move it)

    // The ball should still be affected by physics, so set locked but not static
    // Actually "locked" in our world means "player can't drag it", physics still applies

    let mut puzzle = Puzzle::new(
        "First Steps",
        "Get the bowling ball into the basket (bottom-left)!",
    );

    // Win: bowling ball center within the basket region, at rest
    puzzle.win_conditions.push(WinCondition::ObjectAtPosition {
        instance_id: ball_id,
        region: (60.0, 280.0, 140.0, 340.0),
    });

    // Player gets some walls/inclines to build a path
    puzzle.bin_parts.push(BinPart {
        part_id: PartId::Wall(WallType::WoodenWall),
        quantity: 3,
    });
    puzzle.bin_parts.push(BinPart {
        part_id: PartId::Incline(tim2::parts::inclines::InclineType::BrickIncline),
        quantity: 2,
    });

    (world, puzzle)
}

// ── Rendering ───────────────────────────────────────────────────

fn render_frame(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, game: &mut Game) -> Result<()> {
    terminal.draw(|f| {
        let size = f.area();

        // Layout: [control_col | playfield | parts_bin]
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(size);

        let goal_area = main_layout[0];
        let middle = main_layout[1];
        let help_area = main_layout[2];

        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(22)])
            .split(middle);

        let playfield_area = cols[0];
        let bin_area = cols[1];

        game.playfield_rect = playfield_area;

        // ── Goal bar ──
        let status_text = match game.mode {
            Mode::Build => " [BUILD] ",
            Mode::Running => " [RUNNING] ",
            Mode::Won => " *** YOU WIN! *** ",
        };
        let goal_color = match game.mode {
            Mode::Build => Color::White,
            Mode::Running => Color::Yellow,
            Mode::Won => Color::Green,
        };
        let goal = Paragraph::new(Line::from(vec![
            Span::styled(status_text, Style::default().fg(Color::Black).bg(goal_color)),
            Span::raw(" "),
            Span::raw(&game.puzzle.goal_text),
        ]))
        .block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(goal, goal_area);

        // ── Playfield (braille) ──
        let pf_inner = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", game.puzzle.title))
            .inner(playfield_area);

        // Render world to pixel buffer then convert to braille
        let pw = (pf_inner.width as u32 * 2).max(1);
        let ph = (pf_inner.height as u32 * 4).max(1);
        let mut img = RgbaImage::from_pixel(pw, ph, image::Rgba(BG_COLOR));

        // Scale: map canvas coords to pixel buffer
        let sx = pw as f32 / CANVAS_W as f32;
        let sy = ph as f32 / CANVAS_H as f32;

        // Draw grid
        for gx in (0..CANVAS_W).step_by(GRID_SIZE as usize) {
            let px = (gx as f32 * sx) as i32;
            for py in 0..ph as i32 {
                tim2::render::pixel_gfx::blend_pixel(&mut img, px, py, GRID_COLOR);
            }
        }
        for gy in (0..CANVAS_H).step_by(GRID_SIZE as usize) {
            let py = (gy as f32 * sy) as i32;
            for px in 0..pw as i32 {
                tim2::render::pixel_gfx::blend_pixel(&mut img, px, py, GRID_COLOR);
            }
        }

        // Draw target zone highlight
        for cond in &game.puzzle.win_conditions {
            if let WinCondition::ObjectAtPosition { region, .. } = cond {
                let rx1 = (region.0 * sx) as i32;
                let ry1 = (region.1 * sy) as i32;
                let rx2 = (region.2 * sx) as i32;
                let ry2 = (region.3 * sy) as i32;
                for y in ry1..ry2 {
                    for x in rx1..rx2 {
                        tim2::render::pixel_gfx::blend_pixel(&mut img, x, y, [50, 255, 50, 30]);
                    }
                }
            }
        }

        // Draw all parts
        for inst in &game.world.instances {
            let px = inst.x * sx;
            let py = inst.y * sy;
            let def = inst.def();
            // Scale props for rendering
            let mut rprops = inst.props.clone();
            rprops.width *= sx;
            rprops.height *= sy;
            def.draw_pixel(&mut img, px, py, &rprops, game.frame);
        }

        // Render border first
        let pf_block = Block::default().borders(Borders::ALL).title(format!(" {} ", game.puzzle.title));
        let pf_inner = pf_block.inner(playfield_area);
        f.render_widget(pf_block, playfield_area);

        // Render braille directly into the buffer
        braille::render_braille(&img, f.buffer_mut(), pf_inner);

        // ── Parts Bin ──
        let mut bin_lines: Vec<Line> = Vec::new();
        for (i, entry) in game.bin.iter().enumerate() {
            let def = entry.part_id.part_def();
            let avail = entry.available();
            let ic = def.icon_color();
            let style = if i == game.selected_bin {
                Style::default()
                    .fg(Color::Rgb(ic[0], ic[1], ic[2]))
                    .add_modifier(Modifier::REVERSED)
            } else if avail == 0 {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Rgb(ic[0], ic[1], ic[2]))
            };
            let icon = def.icon_char();
            bin_lines.push(Line::from(Span::styled(
                format!(" {} {} ×{}", icon, def.name(), avail),
                style,
            )));
        }
        let bin_widget = Paragraph::new(bin_lines)
            .block(Block::default().borders(Borders::ALL).title(" Parts Bin "));
        f.render_widget(bin_widget, bin_area);

        // ── Help bar ──
        let help_text = match game.mode {
            Mode::Build => "Space:Run | j/k:Select part | Enter:Place at center | d:Delete | r:Reset | f:Flip | q:Quit",
            Mode::Running => "Space:Stop | r:Reset | q:Quit",
            Mode::Won => "Space:Reset | n:Next puzzle | q:Quit",
        };
        let help = Paragraph::new(Line::from(Span::styled(help_text, Style::default().fg(Color::DarkGray))))
            .block(Block::default().borders(Borders::TOP));
        f.render_widget(help, help_area);
    })?;
    Ok(())
}

// ── Main Loop ───────────────────────────────────────────────────

fn main() -> Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    crossterm::execute!(stdout, crossterm::event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Build puzzle
    let (world, puzzle) = build_tutorial_puzzle();
    let mut game = Game::new(world, puzzle);

    let tick_duration = Duration::from_millis(1000 / TARGET_FPS as u64);

    loop {
        let frame_start = Instant::now();

        // Physics ticks (2 per render frame for 60Hz physics at 30fps render)
        if game.mode == Mode::Running {
            game.tick();
            game.tick();
        }
        game.frame += 1;

        // Render
        render_frame(&mut terminal, &mut game)?;

        // Handle input (non-blocking)
        let elapsed = frame_start.elapsed();
        let timeout = tick_duration.saturating_sub(elapsed);
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                        KeyCode::Char(' ') => {
                            match game.mode {
                                Mode::Build => game.start_sim(),
                                Mode::Running => game.stop_sim(),
                                Mode::Won => game.reset_puzzle(),
                            }
                        }
                        KeyCode::Char('r') => game.reset_puzzle(),

                        // Parts bin navigation
                        KeyCode::Char('j') | KeyCode::Down if game.mode == Mode::Build => {
                            if !game.bin.is_empty() {
                                game.selected_bin = (game.selected_bin + 1) % game.bin.len();
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up if game.mode == Mode::Build => {
                            if !game.bin.is_empty() {
                                game.selected_bin = (game.selected_bin + game.bin.len() - 1) % game.bin.len();
                            }
                        }

                        // Place part at center of playfield
                        KeyCode::Enter if game.mode == Mode::Build => {
                            game.place_from_bin(CANVAS_W as f32 / 2.0, CANVAS_H as f32 / 2.0);
                        }

                        // Flip last placed part
                        KeyCode::Char('f') if game.mode == Mode::Build => {
                            if let Some(&id) = game.player_instances.last() {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.props.flipped = !inst.props.flipped;
                                }
                            }
                        }

                        // Delete last placed part
                        KeyCode::Char('d') | KeyCode::Delete if game.mode == Mode::Build => {
                            if let Some(&id) = game.player_instances.last() {
                                game.return_to_bin(id);
                            }
                        }

                        // Move last placed part with arrow keys
                        KeyCode::Left if game.mode == Mode::Build => {
                            if let Some(&id) = game.player_instances.last() {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.x -= 8.0;
                                }
                            }
                        }
                        KeyCode::Right if game.mode == Mode::Build => {
                            if let Some(&id) = game.player_instances.last() {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.x += 8.0;
                                }
                            }
                        }
                        KeyCode::Up if game.mode == Mode::Build && !game.player_instances.is_empty() => {
                            if let Some(&id) = game.player_instances.last() {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.y -= 8.0;
                                }
                            }
                        }
                        KeyCode::Down if game.mode == Mode::Build && !game.player_instances.is_empty() => {
                            if let Some(&id) = game.player_instances.last() {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.y += 8.0;
                                }
                            }
                        }

                        _ => {}
                    }
                }
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    match kind {
                        MouseEventKind::Down(MouseButton::Left) if game.mode == Mode::Build => {
                            let (wx, wy) = game.screen_to_world(column, row);
                            // Check if clicking on an existing player part
                            if let Some(id) = game.instance_at(wx, wy) {
                                if game.player_instances.contains(&id) {
                                    let inst = game.world.get(id).unwrap();
                                    game.dragging = Some(DragState {
                                        instance_id: id,
                                        offset_x: wx - inst.x,
                                        offset_y: wy - inst.y,
                                    });
                                }
                            } else {
                                // Place new part from bin
                                if let Some(id) = game.place_from_bin(wx, wy) {
                                    let inst = game.world.get(id).unwrap();
                                    game.dragging = Some(DragState {
                                        instance_id: id,
                                        offset_x: inst.props.width / 2.0,
                                        offset_y: inst.props.height / 2.0,
                                    });
                                }
                            }
                        }
                        MouseEventKind::Drag(MouseButton::Left) if game.mode == Mode::Build => {
                            if let Some(ref drag) = game.dragging {
                                let (wx, wy) = game.screen_to_world(column, row);
                                let id = drag.instance_id;
                                let ox = drag.offset_x;
                                let oy = drag.offset_y;
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.x = wx - ox;
                                    inst.y = wy - oy;
                                }
                            }
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            game.dragging = None;
                        }
                        MouseEventKind::Down(MouseButton::Right) if game.mode == Mode::Build => {
                            // Right-click: delete part under cursor
                            let (wx, wy) = game.screen_to_world(column, row);
                            if let Some(id) = game.instance_at(wx, wy) {
                                if game.player_instances.contains(&id) {
                                    game.return_to_bin(id);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    // Cleanup
    crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;
    execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
