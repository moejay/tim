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

pub fn draw_basket_pixel(img: &mut RgbaImage, x: f32, y: f32, ball_near: bool, frame: u64) {
    let ix = x as i32;
    let iy = y as i32;
    let w: i32 = 64;
    let h: i32 = 64;
    let wall_thickness: i32 = 6;
    let stripe_width: i32 = 4;

    let color_a: [u8; 3] = [0xD4, 0xA0, 0x4A];
    let color_b: [u8; 3] = [0x9B, 0x73, 0x30];

    // Interior fill
    for row in 0..h {
        for col in wall_thickness..w - wall_thickness {
            blend_pixel(img, ix + col, iy + row, [0x1A, 0x1A, 0x24, 255]);
        }
    }

    // Goal indicator glow
    if ball_near {
        let pulse = ((frame as f32 * 0.1).sin() * 0.5 + 0.5) * 0.15;
        let cx = w as f32 / 2.0;
        let cy = h as f32 / 2.0;
        let max_r = (w as f32 / 2.0).max(h as f32 / 2.0);
        for row in wall_thickness..h - wall_thickness {
            for col in wall_thickness..w - wall_thickness {
                let dx = col as f32 - cx;
                let dy = row as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                let t = 1.0 - (dist / max_r).clamp(0.0, 1.0);
                let alpha = (t * pulse * 255.0) as u8;
                blend_pixel(img, ix + col, iy + row, [0xFF, 0xD7, 0x00, alpha]);
            }
        }
    }

    // Woven pattern helper
    let woven_color = |col: i32, row: i32| -> [u8; 3] {
        let diag = (col + row) % (stripe_width * 2);
        if diag < stripe_width { color_a } else { color_b }
    };

    // Left wall
    for row in 0..h {
        for col in 0..wall_thickness {
            let c = woven_color(col, row);
            blend_pixel(img, ix + col, iy + row, [c[0], c[1], c[2], 255]);
        }
        // Inner rim highlight
        blend_pixel(img, ix + wall_thickness, iy + row, [0xE8, 0xC8, 0x70, 255]);
        // Outer shadow
        if ix > 0 {
            blend_pixel(img, ix - 1, iy + row, [0x6B, 0x4F, 0x1A, 255]);
        }
    }

    // Right wall
    for row in 0..h {
        for col in w - wall_thickness..w {
            let c = woven_color(col, row);
            blend_pixel(img, ix + col, iy + row, [c[0], c[1], c[2], 255]);
        }
        // Inner rim highlight
        blend_pixel(img, ix + w - wall_thickness - 1, iy + row, [0xE8, 0xC8, 0x70, 255]);
        // Outer shadow
        blend_pixel(img, ix + w, iy + row, [0x6B, 0x4F, 0x1A, 255]);
    }

    // Bottom bar
    for row in h - wall_thickness..h {
        for col in 0..w {
            let c = woven_color(col, row);
            blend_pixel(img, ix + col, iy + row, [c[0], c[1], c[2], 255]);
        }
    }
    // Bottom outer shadow
    for col in 0..w {
        blend_pixel(img, ix + col, iy + h, [0x6B, 0x4F, 0x1A, 255]);
    }
    // Bottom inner rim
    for col in wall_thickness..w - wall_thickness {
        blend_pixel(img, ix + col, iy + h - wall_thickness - 1, [0xE8, 0xC8, 0x70, 255]);
    }

    // Opening arrows (downward triangles at top corners)
    let arrow_size: i32 = 6;
    let arrow_color: [u8; 4] = [0xFF, 0xD7, 0x00, 102]; // 40% opacity
    // Left arrow
    for row in 0..arrow_size {
        let half = row;
        let cx_arrow = wall_thickness + arrow_size / 2;
        for dx in -half..=half {
            blend_pixel(img, ix + cx_arrow + dx, iy + row, arrow_color);
        }
    }
    // Right arrow
    for row in 0..arrow_size {
        let half = row;
        let cx_arrow = w - wall_thickness - arrow_size / 2 - 1;
        for dx in -half..=half {
            blend_pixel(img, ix + cx_arrow + dx, iy + row, arrow_color);
        }
    }
}

pub fn draw_basket_text(
    buf: &mut Buffer,
    cell_x: u16,
    cell_y: u16,
    cell_w: u16,
    cell_h: u16,
    frame: u64,
    area: Rect,
) {
    let basket_style = Style::default().fg(Color::Rgb(212, 160, 74)).add_modifier(Modifier::BOLD);
    let gold_style = Style::default().fg(Color::Rgb(255, 215, 0));

    if cell_w == 0 || cell_h == 0 {
        return;
    }

    for row in 0..cell_h {
        for col in 0..cell_w {
            let px = cell_x + col;
            let py = cell_y + row;
            if px < area.x || px >= area.x + area.width || py < area.y || py >= area.y + area.height {
                continue;
            }

            if row == 0 {
                // Top row: arrows at edges
                if col == 0 || col == cell_w - 1 {
                    buf[(px, py)].set_char('▽').set_style(basket_style);
                }
            } else if row == cell_h - 1 {
                // Bottom row
                if col == 0 {
                    buf[(px, py)].set_char('╚').set_style(basket_style);
                } else if col == cell_w - 1 {
                    buf[(px, py)].set_char('╝').set_style(basket_style);
                } else {
                    buf[(px, py)].set_char('═').set_style(basket_style);
                }
            } else {
                // Middle rows
                if col == 0 || col == cell_w - 1 {
                    buf[(px, py)].set_char('║').set_style(basket_style);
                } else {
                    // Goal dots in center
                    let show_dots = (frame / 30) % 2 == 0;
                    let mid_col = cell_w / 2;
                    if show_dots && row == cell_h / 2 && (col == mid_col || col == mid_col.wrapping_sub(1)) {
                        buf[(px, py)].set_char('•').set_style(gold_style);
                    }
                }
            }
        }
    }
}
