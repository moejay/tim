//! Braille-based terminal rendering.
//!
//! Each Unicode braille character (U+2800..U+28FF) encodes a 2×4 grid of dots,
//! giving us 2× horizontal and 4× vertical sub-cell resolution compared to
//! regular characters.  We render parts into a small pixel buffer and then
//! convert to braille with per-cell foreground color.

use image::{Rgba, RgbaImage};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

/// Braille dot positions within a 2×4 cell:
///
/// ```text
///  (0,0) (1,0)     dot 0  dot 3
///  (0,1) (1,1)     dot 1  dot 4
///  (0,2) (1,2)     dot 2  dot 5
///  (0,3) (1,3)     dot 6  dot 7
/// ```
///
/// The braille codepoint is U+2800 + bitmask where:
///   bit 0 = (0,0), bit 1 = (0,1), bit 2 = (0,2), bit 3 = (1,0),
///   bit 4 = (1,1), bit 5 = (1,2), bit 6 = (0,3), bit 7 = (1,3)
const DOT_MAP: [[u8; 2]; 4] = [
    [0, 3], // row 0: left=bit0, right=bit3
    [1, 4], // row 1: left=bit1, right=bit4
    [2, 5], // row 2: left=bit2, right=bit5
    [6, 7], // row 3: left=bit6, right=bit7
];

/// Threshold for considering a pixel "lit" (alpha-weighted luminance).
const LUMINANCE_THRESHOLD: u8 = 40;

/// Convert an RGBA pixel buffer to braille characters written into a ratatui Buffer.
///
/// `img` is rendered at braille resolution: each terminal cell = 2px wide × 4px tall.
/// So an image of size (cols*2, rows*4) maps exactly to (cols, rows) terminal cells.
///
/// The function writes into `buf` at the given `area` position.
pub fn render_braille(img: &RgbaImage, buf: &mut Buffer, area: Rect) {
    let img_w = img.width() as usize;
    let img_h = img.height() as usize;

    // How many terminal cells we can fill
    let cols = (img_w / 2).min(area.width as usize);
    let rows = (img_h / 4).min(area.height as usize);

    for cy in 0..rows {
        for cx in 0..cols {
            let mut dots: u8 = 0;
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let mut lit_count: u32 = 0;

            for dy in 0..4 {
                for dx in 0..2 {
                    let px = cx * 2 + dx;
                    let py = cy * 4 + dy;
                    if px < img_w && py < img_h {
                        let Rgba([r, g, b, a]) = *img.get_pixel(px as u32, py as u32);
                        // Alpha-weighted luminance
                        let lum = ((r as u16 * 77 + g as u16 * 150 + b as u16 * 29) >> 8) as u8;
                        let effective = ((lum as u16 * a as u16) >> 8) as u8;
                        if effective > LUMINANCE_THRESHOLD {
                            dots |= 1 << DOT_MAP[dy][dx];
                            // Accumulate color of lit dots for fg color
                            let af = a as u32;
                            r_sum += r as u32 * af / 255;
                            g_sum += g as u32 * af / 255;
                            b_sum += b as u32 * af / 255;
                            lit_count += 1;
                        }
                    }
                }
            }

            let ch = char::from_u32(0x2800 + dots as u32).unwrap_or(' ');
            let fg = if lit_count > 0 {
                Color::Rgb(
                    (r_sum / lit_count).min(255) as u8,
                    (g_sum / lit_count).min(255) as u8,
                    (b_sum / lit_count).min(255) as u8,
                )
            } else {
                Color::Reset
            };

            let tx = area.x + cx as u16;
            let ty = area.y + cy as u16;
            if tx < area.right() && ty < area.bottom() {
                buf[(tx, ty)]
                    .set_char(ch)
                    .set_style(Style::default().fg(fg));
            }
        }
    }
}

/// Enhanced braille renderer with dual-color and supersample support.
///
/// - `dual_color`: set bg color per cell to the average of *unlit* pixels,
///   giving a second color channel.
/// - `supersample`: when true, expects image at 2× braille resolution
///   (cols*4, rows*8). Each braille dot averages a 2×2 block of source pixels
///   for smoother edges.
pub fn render_braille_enhanced(
    img: &RgbaImage,
    buf: &mut Buffer,
    area: Rect,
    dual_color: bool,
    supersample: bool,
) {
    let img_w = img.width() as usize;
    let img_h = img.height() as usize;

    // When supersampling, each braille dot = 2×2 source pixels
    let ss = if supersample { 2 } else { 1 };

    let cols = (img_w / (2 * ss)).min(area.width as usize);
    let rows = (img_h / (4 * ss)).min(area.height as usize);

    for cy in 0..rows {
        for cx in 0..cols {
            let mut dots: u8 = 0;
            // Lit pixel accumulators
            let mut lr_sum: u32 = 0;
            let mut lg_sum: u32 = 0;
            let mut lb_sum: u32 = 0;
            let mut lit_count: u32 = 0;
            // Unlit pixel accumulators (for dual-color bg)
            let mut ur_sum: u32 = 0;
            let mut ug_sum: u32 = 0;
            let mut ub_sum: u32 = 0;
            let mut unlit_count: u32 = 0;

            for dy in 0..4_usize {
                for dx in 0..2_usize {
                    // Sample the source region for this dot
                    let src_x = cx * 2 * ss + dx * ss;
                    let src_y = cy * 4 * ss + dy * ss;

                    let mut lum_acc: u32 = 0;
                    let mut r_acc: u32 = 0;
                    let mut g_acc: u32 = 0;
                    let mut b_acc: u32 = 0;
                    let mut samples: u32 = 0;

                    for sy in 0..ss {
                        for sx in 0..ss {
                            let px = src_x + sx;
                            let py = src_y + sy;
                            if px < img_w && py < img_h {
                                let Rgba([r, g, b, a]) = *img.get_pixel(px as u32, py as u32);
                                let lum = ((r as u16 * 77 + g as u16 * 150 + b as u16 * 29) >> 8) as u8;
                                let effective = ((lum as u16 * a as u16) >> 8) as u8;
                                lum_acc += effective as u32;
                                let af = a as u32;
                                r_acc += r as u32 * af / 255;
                                g_acc += g as u32 * af / 255;
                                b_acc += b as u32 * af / 255;
                                samples += 1;
                            }
                        }
                    }

                    if samples == 0 {
                        continue;
                    }

                    let avg_lum = lum_acc / samples;
                    let avg_r = r_acc / samples;
                    let avg_g = g_acc / samples;
                    let avg_b = b_acc / samples;

                    if avg_lum > LUMINANCE_THRESHOLD as u32 {
                        dots |= 1 << DOT_MAP[dy][dx];
                        lr_sum += avg_r;
                        lg_sum += avg_g;
                        lb_sum += avg_b;
                        lit_count += 1;
                    } else {
                        ur_sum += avg_r;
                        ug_sum += avg_g;
                        ub_sum += avg_b;
                        unlit_count += 1;
                    }
                }
            }

            let ch = char::from_u32(0x2800 + dots as u32).unwrap_or(' ');
            let fg = if lit_count > 0 {
                Color::Rgb(
                    (lr_sum / lit_count).min(255) as u8,
                    (lg_sum / lit_count).min(255) as u8,
                    (lb_sum / lit_count).min(255) as u8,
                )
            } else {
                Color::Reset
            };

            let bg = if dual_color && unlit_count > 0 {
                Color::Rgb(
                    (ur_sum / unlit_count).min(255) as u8,
                    (ug_sum / unlit_count).min(255) as u8,
                    (ub_sum / unlit_count).min(255) as u8,
                )
            } else {
                Color::Reset
            };

            let tx = area.x + cx as u16;
            let ty = area.y + cy as u16;
            if tx < area.right() && ty < area.bottom() {
                let style = if dual_color {
                    Style::default().fg(fg).bg(bg)
                } else {
                    Style::default().fg(fg)
                };
                buf[(tx, ty)].set_char(ch).set_style(style);
            }
        }
    }
}

/// Create a pixel buffer sized for braille rendering into the given terminal area.
/// Returns (image, pixel_width, pixel_height).
pub fn braille_image_for_area(area: Rect) -> RgbaImage {
    let pw = area.width as u32 * 2;
    let ph = area.height as u32 * 4;
    RgbaImage::from_pixel(pw, ph, Rgba([0, 0, 0, 0]))
}

/// Create a pixel buffer of a specific pixel size, suitable for braille conversion.
pub fn braille_image(pixel_w: u32, pixel_h: u32) -> RgbaImage {
    RgbaImage::from_pixel(pixel_w, pixel_h, Rgba([0, 0, 0, 0]))
}
