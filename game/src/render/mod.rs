pub mod braille;
pub mod pixel_gfx;
pub mod text_chars;

/// Rendering mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Pixel,
    Text,
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
