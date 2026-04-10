pub mod braille;
pub mod halfblock;
pub mod pixel_gfx;
pub mod text_chars;

/// Rendering mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Pixel,
    Text,
}

/// Settings for text-mode rendering (braille / halfblock).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSettings {
    /// Use half-block characters instead of braille (mutually exclusive).
    pub halfblock: bool,
    /// Use background color per cell (second color channel for braille).
    pub dual_color: bool,
    /// Render source image at 2× braille resolution and downsample per dot.
    pub supersample: bool,
}

impl Default for TextSettings {
    fn default() -> Self {
        Self {
            halfblock: false,
            dual_color: false,
            supersample: false,
        }
    }
}

impl TextSettings {
    /// Pixel dimensions needed for the given terminal area.
    pub fn image_size(&self, cols: u32, rows: u32) -> (u32, u32) {
        if self.halfblock {
            // 1px per col, 2px per row
            (cols.max(1), (rows * 2).max(2))
        } else if self.supersample {
            // 2× braille resolution: 4px per col, 8px per row
            ((cols * 4).max(4), (rows * 8).max(8))
        } else {
            // Normal braille: 2px per col, 4px per row
            ((cols * 2).max(2), (rows * 4).max(4))
        }
    }

    pub fn label(&self) -> String {
        if self.halfblock {
            return "HalfBlock".to_string();
        } else {
            let mut parts = vec!["Braille"];
            if self.dual_color { parts.push("+DualColor"); }
            if self.supersample { parts.push("+SS"); }
            parts.join("")
        }
    }
}

/// Detect best rendering mode for current terminal
pub fn detect_mode() -> RenderMode {
    // Check CLI-level override env var
    if let Ok(mode) = std::env::var("TIM2_RENDER") {
        match mode.to_lowercase().as_str() {
            "pixel" => return RenderMode::Pixel,
            "text" => return RenderMode::Text,
            _ => {}
        }
    }

    // Check for known pixel-capable terminals
    if std::env::var("KITTY_WINDOW_ID").is_ok() {
        return RenderMode::Pixel;
    }
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        match term.as_str() {
            "WezTerm" | "ghostty" | "iTerm.app" => return RenderMode::Pixel,
            _ => {}
        }
    }
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("kitty") || term.contains("xterm-kitty") {
            return RenderMode::Pixel;
        }
    }

    RenderMode::Text
}
