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
use tim2::render::{braille, halfblock, pixel_gfx, detect_mode, RenderMode, TextSettings};
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

// ── Puzzle #2 ────────────────────────────────────────────────────

fn build_puzzle_2() -> (World, Puzzle) {
    use tim2::parts::balls::BallType;
    use tim2::parts::inclines::InclineType;
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

    // Bowling ball at top-left
    let bowling_id = world.spawn_locked(PartId::Ball(BallType::BowlingBall), 50.0, 20.0);

    // Fixed shelves
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 200.0, 120.0);
    set_size(&mut world, 120.0, 12.0);

    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 350.0, 200.0);
    set_size(&mut world, 120.0, 12.0);

    // Goal zone pillars (YellowBrickWall)
    world.spawn_locked(PartId::Wall(WallType::YellowBrickWall), 500.0, 280.0);
    set_size(&mut world, 16.0, 60.0);

    world.spawn_locked(PartId::Wall(WallType::YellowBrickWall), 600.0, 280.0);
    set_size(&mut world, 16.0, 60.0);

    let mut puzzle = Puzzle::new(
        "Puzzle #2: The Cascade",
        "Guide the bowling ball into the goal zone.",
    );

    puzzle.win_conditions.push(WinCondition::ObjectAtPosition {
        instance_id: bowling_id,
        region: (500.0, 300.0, 620.0, 350.0),
    });

    puzzle.bin_parts.push(BinPart { part_id: PartId::Incline(InclineType::BrickIncline), quantity: 2 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Wall(WallType::WoodenWall), quantity: 2 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::TennisBall), quantity: 1 });

    (world, puzzle)
}

// ── Puzzle #3 ────────────────────────────────────────────────────

fn build_puzzle_3() -> (World, Puzzle) {
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

    // Small platform under the pinball
    world.spawn_locked(PartId::Wall(WallType::CinderBlockWall), 80.0, 310.0);
    set_size(&mut world, 60.0, 12.0);

    // Pinball sitting on the platform
    let pinball_id = world.spawn_locked(PartId::Ball(BallType::Pinball), 100.0, 300.0);

    // Mid-height ledge
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 200.0, 220.0);
    set_size(&mut world, 80.0, 12.0);

    // Channel on the right side — left wall
    world.spawn_locked(PartId::Wall(WallType::CinderBlockWall), 450.0, 200.0);
    set_size(&mut world, 12.0, 140.0);

    // Channel top wall
    world.spawn_locked(PartId::Wall(WallType::CinderBlockWall), 450.0, 200.0);
    set_size(&mut world, 190.0, 12.0);

    // Floating PoolBall (zero gravity) as obstacle/tool
    world.spawn_locked(PartId::Ball(BallType::PoolBall), 300.0, 250.0);

    let mut puzzle = Puzzle::new(
        "Puzzle #3",
        "Launch the pinball off the right edge.",
    );

    puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
        instance_id: pinball_id,
        edge: WorldEdge::Right,
    });

    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::SuperBall), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::Basketball), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::BowlingBall), quantity: 1 });

    (world, puzzle)
}

// ── Puzzle #4 ────────────────────────────────────────────────────

fn build_puzzle_4() -> (World, Puzzle) {
    use tim2::parts::balls::BallType;
    use tim2::parts::walls::WallType;

    let mut world = World::new();

    let set_size = |w: &mut World, width: f32, height: f32| {
        if let Some(inst) = w.instances.last_mut() {
            inst.props.width = width;
            inst.props.height = height;
        }
    };

    // Left wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 0.0);
    set_size(&mut world, 16.0, 360.0);

    // Right wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 624.0, 0.0);
    set_size(&mut world, 16.0, 360.0);

    // Top wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 0.0);
    set_size(&mut world, 640.0, 16.0);

    // Bottom-left wall (pocket edge)
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 344.0);
    set_size(&mut world, 250.0, 16.0);

    // Bottom-right wall (pocket edge)
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 390.0, 344.0);
    set_size(&mut world, 250.0, 16.0);

    // 3-ball (target) floating at center
    let three_ball_id = world.spawn_locked(PartId::Ball(BallType::PoolBall), 320.0, 180.0);
    if let Some(inst) = world.get_mut(three_ball_id) {
        inst.props.values.insert("surface_number".to_string(), 3.0);
    }

    // Cue ball floating left of center
    let cue_ball_id = world.spawn_locked(PartId::Ball(BallType::PoolBall), 150.0, 180.0);
    if let Some(inst) = world.get_mut(cue_ball_id) {
        inst.props.values.insert("surface_number".to_string(), 0.0);
    }

    // Obstacle: 1-ball
    let one_ball_id = world.spawn_locked(PartId::Ball(BallType::PoolBall), 400.0, 120.0);
    if let Some(inst) = world.get_mut(one_ball_id) {
        inst.props.values.insert("surface_number".to_string(), 1.0);
    }

    // Obstacle: 5-ball
    let five_ball_id = world.spawn_locked(PartId::Ball(BallType::PoolBall), 450.0, 250.0);
    if let Some(inst) = world.get_mut(five_ball_id) {
        inst.props.values.insert("surface_number".to_string(), 5.0);
    }

    let mut puzzle = Puzzle::new(
        "Puzzle #4: Pool Hall",
        "Sink the 3-ball into the bottom pocket.",
    );

    puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
        instance_id: three_ball_id,
        edge: WorldEdge::Bottom,
    });

    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::BowlingBall), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::Baseball), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::TennisBall), quantity: 1 });

    (world, puzzle)
}

fn build_puzzle_5() -> (World, Puzzle) {
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

    // A tall column in the center divides the arena
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 310.0, 100.0);
    set_size(&mut world, 20.0, 240.0);

    // A basketball on the left side, at rest on the floor
    world.spawn_locked(PartId::Ball(BallType::Basketball), 150.0, 312.0);

    // Target: a PoolBall (ZeroGravity) floating on the right side of the wall
    let target = world.spawn_locked(PartId::Ball(BallType::PoolBall), 460.0, 200.0);
    if let Some(inst) = world.get_mut(target) {
        inst.props.values.insert("surface_number".to_string(), 9.0);
    }

    // A shelf on the right that partially blocks the right edge
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 500.0, 250.0);
    set_size(&mut world, 140.0, 12.0);

    let mut puzzle = Puzzle::new(
        "Puzzle #5: Over the Wall",
        "Get the 9-ball off the screen.",
    );

    puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
        instance_id: target,
        edge: WorldEdge::Any,
    });

    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::BowlingBall), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::SuperBall), quantity: 2 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::TennisBall), quantity: 1 });

    (world, puzzle)
}

fn build_puzzle_6() -> (World, Puzzle) {
    use tim2::parts::balls::BallType;
    use tim2::parts::walls::WallType;

    let mut world = World::new();

    let set_size = |w: &mut World, width: f32, height: f32| {
        if let Some(inst) = w.instances.last_mut() {
            inst.props.width = width;
            inst.props.height = height;
        }
    };

    // Left wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 0.0);
    set_size(&mut world, 16.0, 360.0);

    // Right wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 624.0, 0.0);
    set_size(&mut world, 16.0, 360.0);

    // Top wall
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 0.0);
    set_size(&mut world, 640.0, 16.0);

    // Bottom-left wall (basket edge)
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 344.0);
    set_size(&mut world, 200.0, 16.0);

    // Bottom-right wall (basket edge)
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 440.0, 344.0);
    set_size(&mut world, 200.0, 16.0);

    // Internal deflectors (wooden wall bumpers)
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 150.0, 120.0);
    set_size(&mut world, 100.0, 12.0);

    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 400.0, 120.0);
    set_size(&mut world, 100.0, 12.0);

    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 280.0, 200.0);
    set_size(&mut world, 80.0, 12.0);

    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 100.0, 280.0);
    set_size(&mut world, 120.0, 12.0);

    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 420.0, 280.0);
    set_size(&mut world, 120.0, 12.0);

    // SuperBall locked at top center (target ball)
    let super_ball_id = world.spawn_locked(PartId::Ball(BallType::SuperBall), 310.0, 30.0);

    // PoolBall obstacle floating at center
    let _pool_obstacle_id = world.spawn_locked(PartId::Ball(BallType::PoolBall), 320.0, 160.0);

    let mut puzzle = Puzzle::new(
        "Puzzle #6: Bounce House",
        "Bounce the super ball into the basket.",
    );

    puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
        instance_id: super_ball_id,
        edge: WorldEdge::Bottom,
    });

    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::BowlingBall), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::Basketball), quantity: 2 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::TennisBall), quantity: 1 });

    (world, puzzle)
}

fn build_puzzle_7() -> (World, Puzzle) {
    use tim2::parts::balls::BallType;
    use tim2::parts::walls::WallType;

    let mut world = World::new();

    let set_size = |w: &mut World, width: f32, height: f32| {
        if let Some(inst) = w.instances.last_mut() {
            inst.props.width = width;
            inst.props.height = height;
        }
    };

    // Floor with gap in center — balls can fall through
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 0.0, 340.0);
    set_size(&mut world, 200.0, 20.0);
    world.spawn_locked(PartId::Wall(WallType::BrickWall), 440.0, 340.0);
    set_size(&mut world, 200.0, 20.0);

    // Tiny platform holding the tennis ball above the gap
    world.spawn_locked(PartId::Wall(WallType::WoodenWall), 305.0, 300.0);
    set_size(&mut world, 30.0, 12.0);

    // Tennis ball on the tiny platform
    let tennis_ball_id = world.spawn_locked(PartId::Ball(BallType::TennisBall), 312.0, 290.0);

    // Decorative PoolBalls floating in the arena
    let pool_7 = world.spawn_locked(PartId::Ball(BallType::PoolBall), 150.0, 120.0);
    if let Some(inst) = world.get_mut(pool_7) {
        inst.props.values.insert("surface_number".to_string(), 7.0);
    }
    let pool_9 = world.spawn_locked(PartId::Ball(BallType::PoolBall), 480.0, 200.0);
    if let Some(inst) = world.get_mut(pool_9) {
        inst.props.values.insert("surface_number".to_string(), 9.0);
    }

    let mut puzzle = Puzzle::new(
        "Puzzle #7: The Great Escape",
        "Knock the tennis ball off the screen.",
    );

    puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
        instance_id: tennis_ball_id,
        edge: WorldEdge::Any,
    });

    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::BowlingBall), quantity: 2 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::SuperBall), quantity: 1 });
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::Cannonball), quantity: 1 });

    (world, puzzle)
}

// ── Rendering ───────────────────────────────────────────────────

fn render_frame(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, game: &mut Game, render_mode: RenderMode, text_settings: TextSettings, current_level: usize, num_levels: usize) -> Result<()> {
    let mut pixel_img: Option<image::DynamicImage> = None;
    let mut pf_inner_rect = Rect::default();

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
        let level_label = format!(" {}/{} ", current_level + 1, num_levels);
        let mut goal_spans = vec![
            Span::styled(
                &level_label,
                Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::BOLD),
            ),
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
        // Show render mode label
        if render_mode == RenderMode::Text {
            goal_spans.push(Span::styled(
                format!("  [{}]", text_settings.label()),
                Style::default().fg(Color::Magenta),
            ));
        } else {
            goal_spans.push(Span::styled(
                "  [Pixel]",
                Style::default().fg(Color::Magenta),
            ));
        }
        let goal = Paragraph::new(Line::from(goal_spans))
            .block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(goal, goal_area);

        // ── Playfield ──
        let pf_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", game.puzzle.title));
        let pf_inner = pf_block.inner(playfield_area);
        f.render_widget(pf_block, playfield_area);
        pf_inner_rect = pf_inner;

        // Image dimensions depend on render mode
        let (pw, ph, sx, sy) = match render_mode {
            RenderMode::Text => {
                let (pw, ph) = text_settings.image_size(pf_inner.width as u32, pf_inner.height as u32);
                let sx = pw as f32 / CANVAS_W as f32;
                let sy = ph as f32 / CANVAS_H as f32;
                (pw, ph, sx, sy)
            }
            RenderMode::Pixel => {
                (CANVAS_W, CANVAS_H, 1.0_f32, 1.0_f32)
            }
        };

        let mut img = RgbaImage::from_pixel(pw, ph, image::Rgba(BG_COLOR));

        // Scale factors: map world coords (640x360) to pixel buffer
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
            // Arms: scale up for pixel mode so cursor is visible at full resolution
            let arm = if render_mode == RenderMode::Pixel { 12_i32 } else { 6_i32 };
            let gap = if render_mode == RenderMode::Pixel { 2_i32 } else { 1_i32 };
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
            let scale = if render_mode == RenderMode::Pixel { 3_u32 } else { (sx * 3.0).max(1.0) as u32 };
            let char_w = 6 * scale as i32;
            let tx = (pw as i32 / 2) - (text.len() as i32 * char_w / 2);
            let ty = ph as i32 / 2 - (4 * scale as i32);
            pixel_gfx::draw_text(&mut img, tx, ty, text, [80, 255, 80, 255], scale);
        }

        match render_mode {
            RenderMode::Text => {
                if text_settings.halfblock {
                    halfblock::render_halfblock(&img, f.buffer_mut(), pf_inner);
                } else if text_settings.dual_color || text_settings.supersample {
                    braille::render_braille_enhanced(
                        &img, f.buffer_mut(), pf_inner,
                        text_settings.dual_color, text_settings.supersample,
                    );
                } else {
                    braille::render_braille(&img, f.buffer_mut(), pf_inner);
                }
            }
            RenderMode::Pixel => {
                pixel_img = Some(image::DynamicImage::ImageRgba8(img));
            }
        }

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
                    "Arrows:Move | j/k:Part | Enter:Place | Tab:Cycle | Space:Run | n/p:Level | q:Quit",
                BuildFocus::MovingPart(_) =>
                    "Arrows:Move part | f:Flip | d:Delete | Esc:Deselect | Tab:Next | Space:Run",
            },
            Mode::Running =>
                "Space:Stop | q:Quit",
            Mode::Won =>
                "n:Next level | p:Prev level | r:Reset | q:Quit",
        };
        let help = Paragraph::new(Line::from(Span::styled(help_text, Style::default().fg(Color::DarkGray))))
            .block(Block::default().borders(Borders::TOP));
        f.render_widget(help, help_area);
    })?;

    // Overlay pixel image via viuer after ratatui has drawn the chrome
    if let Some(img) = pixel_img {
        execute!(io::stdout(), cursor::MoveTo(pf_inner_rect.x, pf_inner_rect.y))?;
        let conf = viuer::Config {
            width: Some(pf_inner_rect.width as u32),
            height: Some(pf_inner_rect.height as u32),
            absolute_offset: false,
            ..Default::default()
        };
        viuer::print(&img, &conf)?;
    }

    Ok(())
}

// ── Level Registry ───────────────────────────────────────────────

type PuzzleBuilder = fn() -> (World, Puzzle);

fn all_puzzles() -> Vec<PuzzleBuilder> {
    vec![
        build_puzzle_1,
        build_puzzle_2,
        build_puzzle_3,
        build_puzzle_4,
        build_puzzle_5,
        build_puzzle_6,
        build_puzzle_7,
    ]
}

// ── Main Loop ───────────────────────────────────────────────────

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let render_mode = if args.iter().any(|a| a == "--pixel") {
        RenderMode::Pixel
    } else if args.iter().any(|a| a == "--text") {
        RenderMode::Text
    } else {
        detect_mode()
    };

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let puzzles = all_puzzles();
    let num_levels = puzzles.len();
    let mut current_level: usize = 0;

    let (world, puzzle) = (puzzles[current_level])();
    let mut game = Game::new(world, puzzle);
    let mut text_settings = TextSettings::default();

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

        render_frame(&mut terminal, &mut game, render_mode, text_settings, current_level, num_levels)?;

        // Input
        let elapsed = frame_start.elapsed();
        let timeout = tick_duration.saturating_sub(elapsed);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('c') if key.code == KeyCode::Char('q') || key.modifiers.contains(KeyModifiers::CONTROL) => break,

                    // ── Level navigation (works in Build and Won modes) ──
                    KeyCode::Char('n') if game.mode != Mode::Running => {
                        if current_level + 1 < num_levels {
                            current_level += 1;
                            let (world, puzzle) = (puzzles[current_level])();
                            game = Game::new(world, puzzle);
                            game.set_status(&format!("Level {}/{}", current_level + 1, num_levels));
                        } else {
                            game.set_status("Already on last level!");
                        }
                    }
                    KeyCode::Char('p') if game.mode != Mode::Running => {
                        if current_level > 0 {
                            current_level -= 1;
                            let (world, puzzle) = (puzzles[current_level])();
                            game = Game::new(world, puzzle);
                            game.set_status(&format!("Level {}/{}", current_level + 1, num_levels));
                        } else {
                            game.set_status("Already on first level!");
                        }
                    }

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

                    // ── Graphics toggles (work in any mode) ──
                    KeyCode::Char('1') => {
                        text_settings.halfblock = !text_settings.halfblock;
                        game.set_status(&format!("Renderer: {}", text_settings.label()));
                    }
                    KeyCode::Char('2') => {
                        text_settings.dual_color = !text_settings.dual_color;
                        game.set_status(&format!("Dual color: {}", if text_settings.dual_color { "ON" } else { "OFF" }));
                    }
                    KeyCode::Char('3') => {
                        text_settings.supersample = !text_settings.supersample;
                        game.set_status(&format!("Supersample: {}", if text_settings.supersample { "ON" } else { "OFF" }));
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
