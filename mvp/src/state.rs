/// Canvas and playfield constants
pub const CANVAS_W: u32 = 640;
pub const CANVAS_H: u32 = 360;
pub const PLAYFIELD_W: f32 = 512.0;
pub const PLAYFIELD_H: f32 = 360.0;
pub const BIN_W: f32 = 128.0;

/// Colors (pixel renderer)
pub const BG_COLOR: [u8; 4] = [10, 10, 14, 255];
pub const BIN_BG_COLOR: [u8; 4] = [20, 20, 28, 255];
pub const BIN_DIVIDER_COLOR: [u8; 4] = [42, 42, 58, 255];

#[derive(Clone, Debug)]
pub struct Part {
    pub kind: PartKind,
    pub x: f32,
    pub y: f32,
    pub flipped: bool,
    pub fixed: bool,
}

#[derive(Clone, Debug)]
pub enum PartKind {
    Ball,
    Ramp,
    Wall { width: f32, height: f32 },
    Basket,
    Cannon { angle_deg: f32, power: f32 },
}

impl PartKind {
    pub fn label(&self) -> &'static str {
        match self {
            PartKind::Ball => "Ball",
            PartKind::Ramp => "Ramp",
            PartKind::Wall { .. } => "Wall",
            PartKind::Basket => "Basket",
            PartKind::Cannon { .. } => "Cannon",
        }
    }

    pub fn icon_char(&self) -> char {
        match self {
            PartKind::Ball => '\u{25CF}',     // ●
            PartKind::Ramp => '\u{2571}',     // ╱
            PartKind::Wall { .. } => '\u{2588}', // █
            PartKind::Basket => '\u{255A}',   // ╚
            PartKind::Cannon { .. } => '\u{25B8}', // ▸
        }
    }

    pub fn width(&self, flipped: bool) -> f32 {
        match self {
            PartKind::Ball => 28.0,
            PartKind::Ramp => 64.0,
            PartKind::Wall { width, height } => {
                if flipped { *height } else { *width }
            }
            PartKind::Basket => 64.0,
            PartKind::Cannon { .. } => 96.0,
        }
    }

    pub fn height(&self, flipped: bool) -> f32 {
        match self {
            PartKind::Ball => 28.0,
            PartKind::Ramp => 32.0,
            PartKind::Wall { width, height } => {
                if flipped { *width } else { *height }
            }
            PartKind::Basket => 64.0,
            PartKind::Cannon { .. } => 64.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimBall {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub active: bool,
    pub trail: Vec<(f32, f32)>,
}

impl SimBall {
    pub fn new() -> Self {
        Self {
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            active: false,
            trail: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Normal,
    Place { bin_idx: usize },
    Edit { part_idx: usize },
    Run,
}

#[derive(Clone, Debug)]
pub struct BinItem {
    pub kind: PartKind,
    pub count: usize,
}

pub struct GameState {
    pub parts: Vec<Part>,
    pub ball: SimBall,
    pub mode: Mode,
    pub cursor: (f32, f32),
    pub undo_stack: Vec<Vec<Part>>,
    pub won: bool,
    pub frame: u64,
    pub elapsed: f32,
    pub bin_items: Vec<BinItem>,
    pub show_help: bool,
}

impl GameState {
    pub fn new(parts: Vec<Part>, bin_items: Vec<BinItem>) -> Self {
        Self {
            parts,
            ball: SimBall::new(),
            mode: Mode::Normal,
            cursor: (256.0, 180.0),
            undo_stack: Vec::new(),
            won: false,
            frame: 0,
            elapsed: 0.0,
            bin_items,
            show_help: false,
        }
    }

    pub fn push_undo(&mut self) {
        self.undo_stack.push(self.parts.clone());
    }

    pub fn pop_undo(&mut self) {
        if let Some(snapshot) = self.undo_stack.pop() {
            self.parts = snapshot;
        }
    }

    pub fn part_under_cursor(&self) -> Option<usize> {
        let (cx, cy) = self.cursor;
        for (i, part) in self.parts.iter().enumerate() {
            let w = part.kind.width(part.flipped);
            let h = part.kind.height(part.flipped);
            if cx >= part.x && cx <= part.x + w && cy >= part.y && cy <= part.y + h {
                return Some(i);
            }
        }
        None
    }

    pub fn parts_overlap_at(&self, kind: &PartKind, x: f32, y: f32, flipped: bool, exclude: Option<usize>) -> bool {
        let w = kind.width(flipped);
        let h = kind.height(flipped);
        for (i, part) in self.parts.iter().enumerate() {
            if Some(i) == exclude {
                continue;
            }
            let pw = part.kind.width(part.flipped);
            let ph = part.kind.height(part.flipped);
            if x < part.x + pw && x + w > part.x && y < part.y + ph && y + h > part.y {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderMode {
    Pixel,
    Text,
}
