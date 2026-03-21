use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
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
use tim2::render::pixel_gfx;
use tim2::world::*;

// ── Game State ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Build,
    Running,
    Won,
}

/// What the player is doing in Build mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BuildFocus {
    /// Arrow keys move the cursor. Enter places from bin.
    Cursor,
    /// A player part is selected. Arrow keys move it. Enter/Esc deselects.
    MovingPart(InstanceId),
}

struct Game {
    world: World,
    puzzle: Puzzle,
    mode: Mode,
    frame: u64,

    // Build mode state
    focus: BuildFocus,
    cursor_x: f32,
    cursor_y: f32,

    // Parts bin
    bin: Vec<BinEntry>,
    selected_bin: usize,

    // Player-placed instance IDs
    player_instances: Vec<InstanceId>,

    // Snapshot for sim reset
    snapshot: Vec<InstanceSnapshot>,

    // Initial world state for full reset
    initial_snapshot: Vec<InstanceSnapshot>,

    // Status message (fades after a few frames)
    status_msg: String,
    status_frames: u64,

    // Playfield rect (set during render)
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
        let initial_snapshot = world.snapshot();
        Self {
            world,
            puzzle,
            mode: Mode::Build,
            frame: 0,
            focus: BuildFocus::Cursor,
            cursor_x: CANVAS_W as f32 / 2.0,
            cursor_y: CANVAS_H as f32 / 2.0,
            bin,
            selected_bin: 0,
            player_instances: Vec::new(),
            snapshot: Vec::new(),
            initial_snapshot,
            status_msg: String::new(),
            status_frames: 0,
            playfield_rect: Rect::default(),
        }
    }

    fn set_status(&mut self, msg: &str) {
        self.status_msg = msg.to_string();
        self.status_frames = 90; // ~3 seconds at 30fps
    }

    fn start_sim(&mut self) {
        if self.mode == Mode::Build {
            self.focus = BuildFocus::Cursor;
            self.snapshot = self.world.snapshot();
            self.mode = Mode::Running;
            self.set_status("Simulation running...");
        }
    }

    fn stop_sim(&mut self) {
        if self.mode == Mode::Running {
            self.world.restore(&self.snapshot);
            self.mode = Mode::Build;
            self.set_status("Simulation stopped. Parts restored.");
        }
    }

    fn reset_puzzle(&mut self) {
        // Remove all player-placed parts
        for id in self.player_instances.drain(..) {
            self.world.remove(id);
        }
        for entry in &mut self.bin {
            entry.placed = 0;
        }
        // Restore fixed parts to initial positions
        self.world.restore(&self.initial_snapshot);
        self.mode = Mode::Build;
        self.focus = BuildFocus::Cursor;
        self.set_status("Puzzle reset.");
    }

    fn tick(&mut self) {
        if self.mode != Mode::Running {
            return;
        }
        physics::tick(&mut self.world, 1.0 / 60.0);

        let status = evaluate(&self.world, &self.puzzle.win_conditions);
        if status == PuzzleStatus::Won {
            self.mode = Mode::Won;
            self.set_status("");
        }
    }

    /// Check if placing a part at (x,y) would overlap any existing part.
    fn would_overlap(&self, part_id: PartId, x: f32, y: f32, exclude: Option<InstanceId>) -> bool {
        let def = part_id.part_def();
        let (w, h) = def.default_size();
        let (ax1, ay1, ax2, ay2) = (x, y, x + w, y + h);

        for inst in &self.world.instances {
            if Some(inst.id) == exclude {
                continue;
            }
            let (bx1, by1, bx2, by2) = inst.bounds();
            if ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1 {
                return true;
            }
        }
        false
    }

    fn place_from_bin(&mut self) {
        if self.mode != Mode::Build {
            return;
        }
        if self.selected_bin >= self.bin.len() {
            return;
        }
        let entry = &self.bin[self.selected_bin];
        if entry.available() == 0 {
            self.set_status("No more of that part available!");
            return;
        }
        let part_id = entry.part_id;
        let def = part_id.part_def();
        let (pw, ph) = def.default_size();
        let px = self.cursor_x - pw / 2.0;
        let py = self.cursor_y - ph / 2.0;

        if self.would_overlap(part_id, px, py, None) {
            self.set_status("Can't place here — overlapping another part!");
            return;
        }

        let id = self.world.spawn(part_id, px, py);
        self.player_instances.push(id);
        self.bin[self.selected_bin].placed += 1;
        self.focus = BuildFocus::MovingPart(id);
        self.set_status(&format!("Placed {}. Arrows to move, Esc to deselect.", def.name()));
    }

    fn delete_selected(&mut self) {
        if let BuildFocus::MovingPart(id) = self.focus {
            if self.player_instances.contains(&id) {
                if let Some(inst) = self.world.get(id) {
                    let part_id = inst.part_id;
                    if let Some(entry) = self.bin.iter_mut().find(|e| e.part_id == part_id) {
                        entry.placed = entry.placed.saturating_sub(1);
                    }
                }
                self.world.remove(id);
                self.player_instances.retain(|i| *i != id);
                self.focus = BuildFocus::Cursor;
                self.set_status("Part returned to bin.");
            }
        }
    }

    fn move_step(&self) -> f32 {
        GRID_SIZE as f32 / 2.0 // 16px per press = ~40 presses to cross screen
    }
}

// ── Puzzle #1 ────────────────────────────────────────────────────

fn build_puzzle_1() -> (World, Puzzle) {
    use tim2::parts::balls::BallType;
    use tim2::parts::walls::WallType;

    let mut world = World::new();

    let set_size = |w: &mut World, width: f32, height: f32| {
        if let Some(inst) = w.instances.last_mut() {
            inst.props.width = width;
            inst.props.height = height;
        }
    };

    // Floor
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 340.0);
    set_size(&mut world, 640.0, 20.0);

    // Brick columns flanking the 8-ball
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 430.0, 100.0);
    set_size(&mut world, 16.0, 240.0);
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 520.0, 100.0);
    set_size(&mut world, 16.0, 240.0);

    // Decorative balls
    world.spawn_locked(PartId::Ball(BallType::BowlingBall), 50.0, 180.0);
    world.spawn_locked(PartId::Ball(BallType::Basketball), 100.0, 180.0);
    world.spawn_locked(PartId::Ball(BallType::SoccerBall), 145.0, 180.0);
    world.spawn_locked(PartId::Ball(BallType::Baseball), 185.0, 185.0);
    world.spawn_locked(PartId::Ball(BallType::TennisBall), 215.0, 188.0);
    world.spawn_locked(PartId::Ball(BallType::Pinball), 250.0, 185.0);
    world.spawn_locked(PartId::Ball(BallType::Basketball), 285.0, 180.0);
    world.spawn_locked(PartId::Ball(BallType::SoccerBall), 330.0, 180.0);

    // 8-ball floating between columns (ZeroGravity)
    let eightball_id = world.spawn_locked(PartId::Ball(BallType::PoolBall), 470.0, 140.0);
    if let Some(inst) = world.get_mut(eightball_id) {
        inst.props.values.insert("surface_number".to_string(), 8.0);
    }

    let mut puzzle = Puzzle::new(
        "Puzzle #1",
        "Knock the eight ball off the screen.",
    );

    puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
        instance_id: eightball_id,
        edge: WorldEdge::Any,
    });

    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::SuperBall), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::Basketball), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::SoccerBall), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::TennisBall), quantity: 1 });

    (world, puzzle)
}

// ── Rendering ───────────────────────────────────────────────────

fn render_frame(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, game: &mut Game) -> Result<()> {
    terminal.draw(|f| {
        let size = f.area();

        // Layout: [goal | playfield+bin | help]
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(size);

        let goal_area = rows[0];
        let middle = rows[1];
        let help_area = rows[2];

        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(24)])
            .split(middle);

        let playfield_area = cols[0];
        let bin_area = cols[1];

        game.playfield_rect = playfield_area;

        // ── Goal bar ──
        let (status_text, goal_color) = match game.mode {
            Mode::Build => (" BUILD ", Color::Cyan),
            Mode::Running => (" RUNNING ", Color::Yellow),
            Mode::Won => (" PUZZLE COMPLETE! ", Color::Green),
        };
        let mut goal_spans = vec![
            Span::styled(
                format!(" {} ", status_text),
                Style::default().fg(Color::Black).bg(goal_color).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(&game.puzzle.goal_text, Style::default().fg(Color::White)),
        ];
        // Show cursor position in build mode
        if game.mode == Mode::Build {
            goal_spans.push(Span::styled(
                format!("  [{:.0},{:.0}]", game.cursor_x, game.cursor_y),
                Style::default().fg(Color::DarkGray),
            ));
        }
        let goal = Paragraph::new(Line::from(goal_spans))
            .block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(goal, goal_area);

        // ── Playfield ──
        // Render directly at braille resolution (2px per col, 4px per row)
        let pf_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", game.puzzle.title));
        let pf_inner = pf_block.inner(playfield_area);
        f.render_widget(pf_block, playfield_area);

        let pw = (pf_inner.width as u32 * 2).max(2);
        let ph = (pf_inner.height as u32 * 4).max(4);
        let mut img = RgbaImage::from_pixel(pw, ph, image::Rgba(BG_COLOR));

        // Scale factors: map world coords (640x360) to pixel buffer
        let sx = pw as f32 / CANVAS_W as f32;
        let sy = ph as f32 / CANVAS_H as f32;

        // Grid (scaled)
        for gx in (0..CANVAS_W).step_by(GRID_SIZE as usize) {
            let px = (gx as f32 * sx) as i32;
            for py in 0..ph as i32 {
                pixel_gfx::blend_pixel(&mut img, px, py, GRID_COLOR);
            }
        }
        for gy in (0..CANVAS_H).step_by(GRID_SIZE as usize) {
            let py = (gy as f32 * sy) as i32;
            for px in 0..pw as i32 {
                pixel_gfx::blend_pixel(&mut img, px, py, GRID_COLOR);
            }
        }

        // Draw all parts (scaled positions + sizes)
        for inst in &game.world.instances {
            let def = inst.def();
            let mut scaled_props = inst.props.clone();
            scaled_props.width *= sx;
            scaled_props.height *= sy;
            def.draw_pixel(&mut img, inst.x * sx, inst.y * sy, &scaled_props, game.frame);

            // Highlight selected part
            if let BuildFocus::MovingPart(sel_id) = game.focus {
                if inst.id == sel_id {
                    let x1 = (inst.x * sx) as i32;
                    let y1 = (inst.y * sy) as i32;
                    let x2 = x1 + scaled_props.width as i32;
                    let y2 = y1 + scaled_props.height as i32;
                    let hl = [0, 255, 255, 220];
                    for x in x1..x2 { pixel_gfx::blend_pixel(&mut img, x, y1 - 1, hl); pixel_gfx::blend_pixel(&mut img, x, y2, hl); }
                    for y in y1..y2 { pixel_gfx::blend_pixel(&mut img, x1 - 1, y, hl); pixel_gfx::blend_pixel(&mut img, x2, y, hl); }
                }
            }
        }

        // Cursor crosshair (scaled) — drawn large enough to always be visible
        if game.mode == Mode::Build {
            let cx = (game.cursor_x * sx) as i32;
            let cy = (game.cursor_y * sy) as i32;
            let active = matches!(game.focus, BuildFocus::Cursor);
            let cc = if active { [255, 255, 0, 255] } else { [120, 120, 120, 180] };
            // Arms: at least 6 pixels long regardless of scale
            let arm = 6_i32;
            let gap = 1_i32;
            for d in gap..=arm {
                pixel_gfx::blend_pixel(&mut img, cx - d, cy, cc);
                pixel_gfx::blend_pixel(&mut img, cx + d, cy, cc);
                pixel_gfx::blend_pixel(&mut img, cx, cy - d, cc);
                pixel_gfx::blend_pixel(&mut img, cx, cy + d, cc);
                // Make it 2px thick for visibility
                pixel_gfx::blend_pixel(&mut img, cx - d, cy + 1, cc);
                pixel_gfx::blend_pixel(&mut img, cx + d, cy + 1, cc);
                pixel_gfx::blend_pixel(&mut img, cx + 1, cy - d, cc);
                pixel_gfx::blend_pixel(&mut img, cx + 1, cy + d, cc);
            }
            // Center dot
            pixel_gfx::blend_pixel(&mut img, cx, cy, cc);
            pixel_gfx::blend_pixel(&mut img, cx + 1, cy, cc);
            pixel_gfx::blend_pixel(&mut img, cx, cy + 1, cc);
            pixel_gfx::blend_pixel(&mut img, cx + 1, cy + 1, cc);
        }

        // Win overlay
        if game.mode == Mode::Won {
            for y in 0..ph as i32 {
                for x in 0..pw as i32 {
                    pixel_gfx::blend_pixel(&mut img, x, y, [0, 80, 0, 120]);
                }
            }
            let text = "PUZZLE COMPLETE!";
            let scale = (sx * 3.0).max(1.0) as u32;
            let char_w = 6 * scale as i32;
            let tx = (pw as i32 / 2) - (text.len() as i32 * char_w / 2);
            let ty = ph as i32 / 2 - (4 * scale as i32);
            pixel_gfx::draw_text(&mut img, tx, ty, text, [80, 255, 80, 255], scale);
        }

        braille::render_braille(&img, f.buffer_mut(), pf_inner);

        // ── Parts Bin ──
        let mut bin_lines: Vec<Line> = Vec::new();
        for (i, entry) in game.bin.iter().enumerate() {
            let def = entry.part_id.part_def();
            let avail = entry.available();
            let ic = def.icon_color();
            let marker = if i == game.selected_bin { ">" } else { " " };
            let style = if i == game.selected_bin {
                Style::default()
                    .fg(Color::Rgb(ic[0], ic[1], ic[2]))
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else if avail == 0 {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Rgb(ic[0], ic[1], ic[2]))
            };
            bin_lines.push(Line::from(Span::styled(
                format!("{}{} {} x{}", marker, def.icon_char(), def.name(), avail),
                style,
            )));
        }

        // Status message
        if game.status_frames > 0 {
            bin_lines.push(Line::from(""));
            let alpha = if game.status_frames > 30 { Color::Yellow } else { Color::DarkGray };
            bin_lines.push(Line::from(Span::styled(&game.status_msg, Style::default().fg(alpha))));
        }

        let bin_widget = Paragraph::new(bin_lines)
            .block(Block::default().borders(Borders::ALL).title(" Parts Bin "));
        f.render_widget(bin_widget, bin_area);

        // ── Help bar ──
        let help_text = match game.mode {
            Mode::Build => match game.focus {
                BuildFocus::Cursor =>
                    "Arrows:Move cursor | j/k:Select part | Enter:Place | Tab:Select placed | Space:Run | q:Quit",
                BuildFocus::MovingPart(_) =>
                    "Arrows:Move part | f:Flip | d:Delete | Esc:Deselect | Tab:Next part | Space:Run",
            },
            Mode::Running =>
                "Space:Stop | q:Quit",
            Mode::Won =>
                "r:Reset puzzle | q:Quit",
        };
        let help = Paragraph::new(Line::from(Span::styled(help_text, Style::default().fg(Color::DarkGray))))
            .block(Block::default().borders(Borders::TOP));
        f.render_widget(help, help_area);
    })?;
    Ok(())
}

// ── Main Loop ───────────────────────────────────────────────────

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (world, puzzle) = build_puzzle_1();
    let mut game = Game::new(world, puzzle);

    let tick_duration = Duration::from_millis(1000 / TARGET_FPS as u64);

    loop {
        let frame_start = Instant::now();

        // Physics ticks (2 per render frame for 60Hz at 30fps)
        if game.mode == Mode::Running {
            game.tick();
            game.tick();
        }
        game.frame += 1;
        if game.status_frames > 0 {
            game.status_frames -= 1;
        }

        render_frame(&mut terminal, &mut game)?;

        // Input
        let elapsed = frame_start.elapsed();
        let timeout = tick_duration.saturating_sub(elapsed);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('c') if key.code == KeyCode::Char('q') || key.modifiers.contains(KeyModifiers::CONTROL) => break,

                    // ── Run / Stop ──
                    KeyCode::Char(' ') => match game.mode {
                        Mode::Build => game.start_sim(),
                        Mode::Running => game.stop_sim(),
                        Mode::Won => {}
                    },

                    // ── Reset ──
                    KeyCode::Char('r') => game.reset_puzzle(),

                    // ── Bin navigation (always in build mode) ──
                    KeyCode::Char('j') if game.mode == Mode::Build => {
                        if !game.bin.is_empty() {
                            game.selected_bin = (game.selected_bin + 1) % game.bin.len();
                        }
                    }
                    KeyCode::Char('k') if game.mode == Mode::Build => {
                        if !game.bin.is_empty() {
                            game.selected_bin = (game.selected_bin + game.bin.len() - 1) % game.bin.len();
                        }
                    }

                    // ── Arrow keys ──
                    KeyCode::Left if game.mode == Mode::Build => {
                        let step = game.move_step();
                        match game.focus {
                            BuildFocus::Cursor => game.cursor_x = (game.cursor_x - step).max(0.0),
                            BuildFocus::MovingPart(id) => {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.x -= step;
                                }
                            }
                        }
                    }
                    KeyCode::Right if game.mode == Mode::Build => {
                        let step = game.move_step();
                        match game.focus {
                            BuildFocus::Cursor => game.cursor_x = (game.cursor_x + step).min(CANVAS_W as f32),
                            BuildFocus::MovingPart(id) => {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.x += step;
                                }
                            }
                        }
                    }
                    KeyCode::Up if game.mode == Mode::Build => {
                        let step = game.move_step();
                        match game.focus {
                            BuildFocus::Cursor => game.cursor_y = (game.cursor_y - step).max(0.0),
                            BuildFocus::MovingPart(id) => {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.y -= step;
                                }
                            }
                        }
                    }
                    KeyCode::Down if game.mode == Mode::Build => {
                        let step = game.move_step();
                        match game.focus {
                            BuildFocus::Cursor => game.cursor_y = (game.cursor_y + step).min(CANVAS_H as f32),
                            BuildFocus::MovingPart(id) => {
                                if let Some(inst) = game.world.get_mut(id) {
                                    inst.y += step;
                                }
                            }
                        }
                    }

                    // ── Place part ──
                    KeyCode::Enter if game.mode == Mode::Build => {
                        match game.focus {
                            BuildFocus::Cursor => game.place_from_bin(),
                            BuildFocus::MovingPart(_) => {
                                game.focus = BuildFocus::Cursor;
                                game.set_status("Part placed.");
                            }
                        }
                    }

                    // ── Deselect ──
                    KeyCode::Esc if game.mode == Mode::Build => {
                        game.focus = BuildFocus::Cursor;
                    }

                    // ── Tab: cycle through placed parts ──
                    KeyCode::Tab if game.mode == Mode::Build => {
                        if !game.player_instances.is_empty() {
                            let next = match game.focus {
                                BuildFocus::Cursor => 0,
                                BuildFocus::MovingPart(id) => {
                                    let pos = game.player_instances.iter().position(|i| *i == id).unwrap_or(0);
                                    (pos + 1) % game.player_instances.len()
                                }
                            };
                            let id = game.player_instances[next];
                            game.focus = BuildFocus::MovingPart(id);
                            // Move cursor to part center
                            if let Some(inst) = game.world.get(id) {
                                game.cursor_x = inst.x + inst.props.width / 2.0;
                                game.cursor_y = inst.y + inst.props.height / 2.0;
                            }
                            let name = game.world.get(id).map(|i| i.def().name()).unwrap_or("?");
                            game.set_status(&format!("Selected: {}", name));
                        }
                    }

                    // ── Flip ──
                    KeyCode::Char('f') if game.mode == Mode::Build => {
                        if let BuildFocus::MovingPart(id) = game.focus {
                            if let Some(inst) = game.world.get_mut(id) {
                                inst.props.flipped = !inst.props.flipped;
                                game.set_status("Flipped.");
                            }
                        }
                    }

                    // ── Delete ──
                    KeyCode::Char('d') | KeyCode::Delete if game.mode == Mode::Build => {
                        game.delete_selected();
                    }

                    _ => {}
                }
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
