//! Half-block terminal rendering.
//!
//! Uses the upper-half-block character (▀ U+2580) to get 1×2 sub-cell resolution
//! with **full RGB color** for both halves via fg (top) and bg (bottom).
//! Lower spatial resolution than braille (2 subpixels vs 8 per cell) but
//! vastly better color fidelity.

use image::{Rgba, RgbaImage};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

/// Convert an RGBA pixel buffer to half-block characters written into a ratatui Buffer.
///
/// `img` should be sized (cols, rows*2) — each terminal cell = 1px wide × 2px tall.
/// Top pixel → fg color, bottom pixel → bg color, character = '▀'.
pub fn render_halfblock(img: &RgbaImage, buf: &mut Buffer, area: Rect) {
    let img_w = img.width() as usize;
    let img_h = img.height() as usize;

    let cols = img_w.min(area.width as usize);
    let rows = (img_h / 2).min(area.height as usize);

    for cy in 0..rows {
        for cx in 0..cols {
            let top_y = cy * 2;
            let bot_y = cy * 2 + 1;

            let top_color = pixel_to_color(img, cx, top_y);
            let bot_color = if bot_y < img_h {
                pixel_to_color(img, cx, bot_y)
            } else {
                Color::Black
            };

            let tx = area.x + cx as u16;
            let ty = area.y + cy as u16;
            if tx < area.right() && ty < area.bottom() {
                // ▀ upper half block: fg = top half color, bg = bottom half color
                buf[(tx, ty)]
                    .set_char('▀')
                    .set_style(Style::default().fg(top_color).bg(bot_color));
            }
        }
    }
}

/// Alpha-blend a pixel against black and return a ratatui Color.
fn pixel_to_color(img: &RgbaImage, x: usize, y: usize) -> Color {
    let Rgba([r, g, b, a]) = *img.get_pixel(x as u32, y as u32);
    let af = a as u16;
    Color::Rgb(
        (r as u16 * af / 255) as u8,
        (g as u16 * af / 255) as u8,
        (b as u16 * af / 255) as u8,
    )
}
