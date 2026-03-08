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

fn draw_filled_circle(img: &mut RgbaImage, cx: i32, cy: i32, radius: i32, color: [u8; 4]) {
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                blend_pixel(img, cx + dx, cy + dy, color);
            }
        }
    }
}

pub fn draw_wall_pixel(img: &mut RgbaImage, x: f32, y: f32, w: f32, h: f32) {
    let ix = x as i32;
    let iy = y as i32;
    let iw = w as i32;
    let ih = h as i32;

    let face: [u8; 4] = [0x5A, 0x7A, 0x8A, 255];
    let top_bevel: [u8; 4] = [0x8A, 0xAC, 0xBC, 255];
    let left_bevel: [u8; 4] = [0x7A, 0x9A, 0xAA, 255];
    let bottom_bevel: [u8; 4] = [0x3D, 0x55, 0x66, 255];
    let right_bevel: [u8; 4] = [0x4A, 0x66, 0x76, 255];

    // Face fill
    for row in 0..ih {
        for col in 0..iw {
            blend_pixel(img, ix + col, iy + row, face);
        }
    }

    // Top 2px bevel
    for row in 0..2.min(ih) {
        for col in 0..iw {
            blend_pixel(img, ix + col, iy + row, top_bevel);
        }
    }

    // Bottom 2px bevel
    for row in (ih - 2).max(0)..ih {
        for col in 0..iw {
            blend_pixel(img, ix + col, iy + row, bottom_bevel);
        }
    }

    // Left 2px bevel
    for col in 0..2.min(iw) {
        for row in 0..ih {
            blend_pixel(img, ix + col, iy + row, left_bevel);
        }
    }

    // Right 2px bevel
    for col in (iw - 2).max(0)..iw {
        for row in 0..ih {
            blend_pixel(img, ix + col, iy + row, right_bevel);
        }
    }

    // Center line at vertical midpoint
    let mid_y = ih / 2;
    for col in 0..iw {
        blend_pixel(img, ix + col, iy + mid_y, [0x4A, 0x6A, 0x7A, 128]);
    }

    // Corner rivets (2px radius circles)
    let rivet_color: [u8; 4] = [0x70, 0x90, 0xA0, 255];
    let rivet_r = 2;
    let margin = 4;
    // Top-left
    draw_filled_circle(img, ix + margin, iy + margin, rivet_r, rivet_color);
    // Top-right
    draw_filled_circle(img, ix + iw - margin - 1, iy + margin, rivet_r, rivet_color);
    // Bottom-left
    draw_filled_circle(img, ix + margin, iy + ih - margin - 1, rivet_r, rivet_color);
    // Bottom-right
    draw_filled_circle(img, ix + iw - margin - 1, iy + ih - margin - 1, rivet_r, rivet_color);
}

pub fn draw_wall_text(
    buf: &mut Buffer,
    cell_x: u16,
    cell_y: u16,
    cell_w: u16,
    cell_h: u16,
    area: Rect,
) {
    let wall_style = Style::default().fg(Color::Rgb(90, 122, 138)).add_modifier(Modifier::BOLD);

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

            let ch = if row == 0 {
                if col == 0 {
                    '▛'
                } else if col == cell_w - 1 {
                    '▜'
                } else {
                    '▀'
                }
            } else if row == cell_h - 1 {
                if col == 0 {
                    '▙'
                } else if col == cell_w - 1 {
                    '▟'
                } else {
                    '▄'
                }
            } else if col == 0 || col == cell_w - 1 {
                '█'
            } else {
                ' '
            };

            buf[(px, py)].set_char(ch).set_style(wall_style);
        }
    }
}
