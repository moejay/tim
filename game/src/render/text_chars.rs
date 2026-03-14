/// Text-mode character and color definitions for each part type.

/// A character + RGB color pair for text rendering.
#[derive(Debug, Clone, Copy)]
pub struct TextIcon {
    pub ch: char,
    pub color: [u8; 3],
}

// Common colors
pub const RED_BROWN: [u8; 3] = [178, 80, 50];
pub const YELLOW: [u8; 3] = [230, 200, 50];
pub const GRAY: [u8; 3] = [160, 160, 160];
pub const DARK_GRAY: [u8; 3] = [80, 80, 80];
pub const CREAM: [u8; 3] = [240, 230, 200];
pub const BROWN: [u8; 3] = [139, 90, 43];
pub const DARK_BROWN: [u8; 3] = [100, 60, 30];
pub const TAN: [u8; 3] = [210, 180, 140];
pub const STEEL: [u8; 3] = [140, 150, 160];
pub const ORANGE: [u8; 3] = [230, 140, 30];
pub const WHITE: [u8; 3] = [240, 240, 240];
pub const MAGENTA: [u8; 3] = [200, 50, 200];
pub const SILVER: [u8; 3] = [200, 200, 210];
pub const TEAL: [u8; 3] = [0, 180, 180];
pub const GREEN: [u8; 3] = [50, 180, 50];
pub const BLUE: [u8; 3] = [50, 100, 220];
pub const RED: [u8; 3] = [220, 40, 40];
pub const CYAN: [u8; 3] = [0, 200, 220];
pub const PURPLE: [u8; 3] = [140, 60, 180];
pub const YELLOW_GREEN: [u8; 3] = [180, 220, 50];
