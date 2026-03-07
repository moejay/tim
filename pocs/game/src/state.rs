pub const CANVAS_W: u32 = 640;
pub const CANVAS_H: u32 = 360;
pub const PLAYFIELD_W: u32 = 512;
pub const CURSOR_STEP: f32 = 4.0;
pub const CURSOR_STEP_FAST: f32 = 16.0;

#[derive(Clone, Debug)]
pub struct Part {
    pub kind: PartKind,
    pub x: f32,
    pub y: f32,
    pub flipped: bool,
    pub fixed: bool,
}

impl Part {
    pub fn size_px(&self) -> (f32, f32) {
        self.kind.size_px()
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        let (w, h) = self.size_px();
        px >= self.x && px < self.x + w && py >= self.y && py < self.y + h
    }

    pub fn overlaps(&self, other: &Part) -> bool {
        let (w1, h1) = self.size_px();
        let (w2, h2) = other.size_px();
        self.x < other.x + w2
            && self.x + w1 > other.x
            && self.y < other.y + h2
            && self.y + h1 > other.y
    }
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
    pub fn size_px(&self) -> (f32, f32) {
        match self {
            PartKind::Ball => (28.0, 28.0),
            PartKind::Ramp => (64.0, 32.0),
            PartKind::Wall { width, height } => (*width, *height),
            PartKind::Basket => (64.0, 64.0),
            PartKind::Cannon { .. } => (96.0, 64.0),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PartKind::Ball => "Ball",
            PartKind::Ramp => "Ramp",
            PartKind::Wall { .. } => "Wall",
            PartKind::Basket => "Basket",
            PartKind::Cannon { .. } => "Cannon",
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimBall {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub active: bool,
}

impl Default for SimBall {
    fn default() -> Self {
        Self {
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            active: false,
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

#[derive(Clone, Debug)]
pub struct BinItem {
    pub kind: PartKind,
    pub count: u8,
}

impl GameState {
    pub fn push_undo(&mut self) {
        self.undo_stack.push(self.parts.clone());
    }

    pub fn pop_undo(&mut self) {
        if let Some(snapshot) = self.undo_stack.pop() {
            self.parts = snapshot;
        }
    }
}
