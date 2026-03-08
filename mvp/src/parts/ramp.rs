use image::RgbaImage;
use ratatui::prelude::*;
use ratatui::buffer::Buffer;

fn blend_pixel(img: &mut RgbaImage, x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= img.width() as i32 || y >= img.height() as i32 {
        return;
    }
    let px = img.get_pixel_mut(x as u32, y as u32);
    let a = color[3] as f32 / 255.0;
    let inv = 1.0 - a;
    px[0] = (color[0] as f32 * a + px[0] as f32 * inv) as u8;
    px[1] = (color[1] as f32 * a + px[1] as f32 * inv) as u8;
    px[2] = (color[2] as f32 * a + px[2] as f32 * inv) as u8;
    px[3] = 255;
}

pub fn draw_ramp_pixel(img: &mut RgbaImage, x: f32, y: f32, flipped: bool) {
    let w: i32 = 64;
    let h: i32 = 32;
    let ix = x as i32;
    let iy = y as i32;

    let top_color: [u8; 3] = [0xDE, 0xB8, 0x87];
    let bot_color: [u8; 3] = [0x7A, 0x5A, 0x2E];

    // Scanline fill
    for row in 0..h {
        let t = row as f32 / (h - 1).max(1) as f32;
        let r = (top_color[0] as f32 * (1.0 - t) + bot_color[0] as f32 * t) as u8;
        let g = (top_color[1] as f32 * (1.0 - t) + bot_color[1] as f32 * t) as u8;
        let b = (top_color[2] as f32 * (1.0 - t) + bot_color[2] as f32 * t) as u8;

        // Determine x range for this scanline row
        // Not flipped (slope-right /): vertices (0,32), (64,32), (64,0)
        //   For row r from top: filled from x_start to 64
        //   hypotenuse goes from (64,0) to (0,32): x = 64 - (row * 64 / 32) = 64 - row*2
        // Flipped (slope-left \): vertices (0,0), (0,32), (64,32)
        //   hypotenuse goes from (0,0) to (64,32): x = row * 64 / 32 = row*2
        //   filled from 0 to x_end

        let (col_start, col_end) = if !flipped {
            // slope-right: hypotenuse from (64,0) to (0,32)
            let edge = (w as f32 * (1.0 - row as f32 / (h - 1).max(1) as f32)) as i32;
            (edge.max(0), w)
        } else {
            // slope-left: hypotenuse from (0,0) to (64,32)
            let edge = (w as f32 * row as f32 / (h - 1).max(1) as f32) as i32;
            (0, edge.min(w))
        };

        for col in col_start..col_end {
            blend_pixel(img, ix + col, iy + row, [r, g, b, 255]);
        }

        // Anti-alias the hypotenuse edge pixel
        if !flipped {
            let exact_edge = w as f32 * (1.0 - row as f32 / (h - 1).max(1) as f32);
            let aa_col = exact_edge.floor() as i32 - 1;
            if aa_col >= 0 {
                let frac = exact_edge - exact_edge.floor();
                let alpha = (frac * 255.0) as u8;
                blend_pixel(img, ix + aa_col, iy + row, [r, g, b, alpha]);
            }
        } else {
            let exact_edge = w as f32 * row as f32 / (h - 1).max(1) as f32;
            let aa_col = exact_edge.ceil() as i32;
            if aa_col < w {
                let frac = exact_edge.ceil() - exact_edge;
                let alpha = (frac * 255.0) as u8;
                blend_pixel(img, ix + aa_col, iy + row, [r, g, b, alpha]);
            }
        }
    }

    // Bottom line
    for col in 0..w {
        blend_pixel(img, ix + col, iy + h - 1, [0x5A, 0x3E, 0x1A, 255]);
    }

    // Hypotenuse highlight (2px)
    if !flipped {
        // Line from (64,0) to (0,32)
        for step in 0..w {
            let lx = w - 1 - step;
            let ly = (step as f32 * (h - 1) as f32 / (w - 1).max(1) as f32) as i32;
            blend_pixel(img, ix + lx, iy + ly, [0xE8, 0xD5, 0xB7, 255]);
            blend_pixel(img, ix + lx + 1, iy + ly, [0xE8, 0xD5, 0xB7, 200]);
        }
    } else {
        // Line from (0,0) to (64,32)
        for step in 0..w {
            let lx = step;
            let ly = (step as f32 * (h - 1) as f32 / (w - 1).max(1) as f32) as i32;
            blend_pixel(img, ix + lx, iy + ly, [0xE8, 0xD5, 0xB7, 255]);
            if lx > 0 {
                blend_pixel(img, ix + lx - 1, iy + ly, [0xE8, 0xD5, 0xB7, 200]);
            }
        }
    }
}

pub fn draw_ramp_text(
    buf: &mut Buffer,
    cell_x: u16,
    cell_y: u16,
    cell_w: u16,
    cell_h: u16,
    flipped: bool,
    area: Rect,
) {
    let wood = Style::default().fg(Color::Rgb(196, 149, 106));

    if cell_h == 0 || cell_w == 0 {
        return;
    }

    for row in 0..cell_h {
        for col in 0..cell_w {
            let px = cell_x + col;
            let py = cell_y + row;
            if px < area.x || px >= area.x + area.width || py < area.y || py >= area.y + area.height {
                continue;
            }

            // Bottom row
            if row == cell_h - 1 {
                buf[(px, py)].set_char('─').set_style(wood);
                continue;
            }

            // Diagonal position
            let diag_col = if !flipped {
                // slope-right /: diagonal goes from bottom-left to top-right
                let dc = cell_w as i32 - 1 - (row as i32 * cell_w as i32 / cell_h.max(1) as i32);
                dc
            } else {
                // slope-left \: diagonal goes from top-left to bottom-right
                let dc = row as i32 * cell_w as i32 / cell_h.max(1) as i32;
                dc
            };

            if col as i32 == diag_col {
                let ch = if !flipped { '╱' } else { '╲' };
                buf[(px, py)].set_char(ch).set_style(wood);
            } else {
                // Fill body area
                let inside = if !flipped {
                    col as i32 > diag_col && row < cell_h - 1
                } else {
                    (col as i32) < diag_col && row < cell_h - 1
                };
                if inside {
                    buf[(px, py)].set_char('░').set_style(wood);
                }
            }
        }
    }
}
