use image::RgbaImage;
use ratatui::prelude::*;
use ratatui::buffer::Buffer;
use crate::state::*;

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

pub fn draw_cannon_pixel(
    img: &mut RgbaImage,
    x: f32,
    y: f32,
    angle_deg: f32,
    flipped: bool,
    frame: u64,
) {
    let ix = x as i32;
    let iy = y as i32;
    let total_w: i32 = 96;
    let total_h: i32 = 64;

    // Body: 60x40 rect centered vertically
    let body_w: i32 = 60;
    let body_h: i32 = 40;
    let body_x = ix + (total_w - body_w) / 2;
    let body_y = iy + (total_h - body_h) / 2;

    let top_color: [u8; 3] = [0x4A, 0x4A, 0x52];
    let bot_color: [u8; 3] = [0x2C, 0x2C, 0x34];

    // Body fill with vertical gradient
    for row in 0..body_h {
        let t = row as f32 / (body_h - 1).max(1) as f32;
        let r = (top_color[0] as f32 * (1.0 - t) + bot_color[0] as f32 * t) as u8;
        let g = (top_color[1] as f32 * (1.0 - t) + bot_color[1] as f32 * t) as u8;
        let b = (top_color[2] as f32 * (1.0 - t) + bot_color[2] as f32 * t) as u8;
        for col in 0..body_w {
            blend_pixel(img, body_x + col, body_y + row, [r, g, b, 255]);
        }
    }

    // Body border
    for col in 0..body_w {
        blend_pixel(img, body_x + col, body_y, [0x1A, 0x1A, 0x1E, 255]);
        blend_pixel(img, body_x + col, body_y + body_h - 1, [0x1A, 0x1A, 0x1E, 255]);
    }
    for row in 0..body_h {
        blend_pixel(img, body_x, body_y + row, [0x1A, 0x1A, 0x1E, 255]);
        blend_pixel(img, body_x + body_w - 1, body_y + row, [0x1A, 0x1A, 0x1E, 255]);
    }

    // Panel line at midpoint
    let panel_y = body_y + body_h / 2;
    for col in 0..body_w {
        blend_pixel(img, body_x + col, panel_y, [0x3A, 0x3A, 0x42, 255]);
    }

    // Corner rivets on body
    let rivet_color: [u8; 4] = [0x6A, 0x6A, 0x72, 255];
    let margin = 4;
    draw_filled_circle(img, body_x + margin, body_y + margin, 2, rivet_color);
    draw_filled_circle(img, body_x + body_w - margin - 1, body_y + margin, 2, rivet_color);
    draw_filled_circle(img, body_x + margin, body_y + body_h - margin - 1, 2, rivet_color);
    draw_filled_circle(img, body_x + body_w - margin - 1, body_y + body_h - margin - 1, 2, rivet_color);

    // Brass trim band at 25% from top
    let trim_y = body_y + body_h / 4;
    for col in 0..body_w {
        blend_pixel(img, body_x + col, trim_y, [0xB0, 0x8D, 0x3A, 255]);
        blend_pixel(img, body_x + col, trim_y + 1, [0xD4, 0xAA, 0x4A, 255]);
    }

    // Barrel: 40x10 rect rotated by angle_deg
    let barrel_len: f32 = 40.0;
    let barrel_half_h: f32 = 5.0;
    let angle_rad = angle_deg.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    // Anchor point
    let (anchor_x, anchor_y) = if flipped {
        (body_x as f32, body_y as f32 + body_h as f32 / 2.0)
    } else {
        (body_x as f32 + body_w as f32, body_y as f32 + body_h as f32 / 2.0)
    };

    let dir = if flipped { -1.0f32 } else { 1.0 };

    // Draw barrel by sampling rotated rectangle
    for step in 0..(barrel_len as i32 + 1) {
        for perp in -(barrel_half_h as i32)..=(barrel_half_h as i32) {
            let along = step as f32;
            let bx = anchor_x + dir * (along * cos_a - perp as f32 * sin_a);
            let by = anchor_y + along * sin_a + perp as f32 * cos_a;
            let color = if perp == -(barrel_half_h as i32) {
                [0x5A, 0x5A, 0x5A, 255] // top highlight
            } else {
                [0x36, 0x36, 0x36, 255] // fill
            };
            blend_pixel(img, bx as i32, by as i32, color);
        }
    }

    // Muzzle ring at barrel tip
    let tip_x = anchor_x + dir * barrel_len * cos_a;
    let tip_y = anchor_y + barrel_len * sin_a;
    draw_filled_circle(img, tip_x as i32, tip_y as i32, 2, [0x6A, 0x6A, 0x6A, 255]);

    // Wheels: 2 circles radius 10px at bottom
    let wheel_y = iy + total_h - 10;
    let wheel_r: i32 = 10;
    let wheel_fill: [u8; 4] = [0x3B, 0x2F, 0x2F, 255];
    let wheel_border: [u8; 4] = [0x2A, 0x20, 0x20, 255];
    let hub: [u8; 4] = [0x55, 0x55, 0x55, 255];
    let spoke: [u8; 4] = [0x44, 0x44, 0x44, 255];

    let wheel_positions = [ix + total_w / 4, ix + 3 * total_w / 4];
    for &wcx in &wheel_positions {
        // Fill
        for dy in -wheel_r..=wheel_r {
            for dx in -wheel_r..=wheel_r {
                let d2 = dx * dx + dy * dy;
                if d2 <= wheel_r * wheel_r {
                    if d2 > (wheel_r - 1) * (wheel_r - 1) {
                        blend_pixel(img, wcx + dx, wheel_y + dy, wheel_border);
                    } else {
                        blend_pixel(img, wcx + dx, wheel_y + dy, wheel_fill);
                    }
                }
            }
        }
        // Hub
        draw_filled_circle(img, wcx, wheel_y, 3, hub);
        // 4 spokes (horizontal and vertical)
        for s in -wheel_r..=wheel_r {
            blend_pixel(img, wcx + s, wheel_y, spoke);
            blend_pixel(img, wcx, wheel_y + s, spoke);
        }
    }

    // Fuse spark at rear
    let fuse_x = if flipped {
        body_x + body_w - 2
    } else {
        body_x + 2
    };
    let fuse_y = body_y + body_h / 2;
    let pulse = ((frame as f32 * 0.15).sin() * 0.5 + 0.5) * 255.0;
    draw_filled_circle(img, fuse_x, fuse_y, 2, [0xFF, 0xD7, 0x00, pulse as u8]);
}

pub fn fire_cannon(part: &Part) -> SimBall {
    if let PartKind::Cannon { angle_deg, power } = &part.kind {
        let w = 96.0;
        let h = 64.0;
        let angle_rad = angle_deg.to_radians();
        let (tip_x, tip_y) = if part.flipped {
            (part.x, part.y + h / 2.0 - angle_rad.sin() * (w * 0.4))
        } else {
            (part.x + w, part.y + h / 2.0 + angle_rad.sin() * (w * 0.4))
        };
        let dir = if part.flipped { -1.0 } else { 1.0 };
        SimBall {
            pos: (tip_x, tip_y),
            vel: (dir * angle_rad.cos() * power, angle_rad.sin() * power),
            active: true,
            trail: Vec::new(),
        }
    } else {
        SimBall::new()
    }
}

pub fn draw_cannon_text(
    buf: &mut Buffer,
    cell_x: u16,
    cell_y: u16,
    cell_w: u16,
    cell_h: u16,
    angle_deg: f32,
    flipped: bool,
    area: Rect,
) {
    let body_style = Style::default().fg(Color::Rgb(74, 74, 82));
    let barrel_style = Style::default().fg(Color::Rgb(100, 100, 110));
    let wheel_style = Style::default().fg(Color::Rgb(59, 47, 47));
    let brass_style = Style::default().fg(Color::Rgb(176, 141, 58)).add_modifier(Modifier::BOLD);

    if cell_w < 3 || cell_h < 3 {
        return;
    }

    // Body occupies rows 0..cell_h-1, barrel extends from side
    for row in 0..cell_h {
        for col in 0..cell_w {
            let px = cell_x + col;
            let py = cell_y + row;
            if px < area.x || px >= area.x + area.width || py < area.y || py >= area.y + area.height {
                continue;
            }

            // Bottom row: wheels
            if row == cell_h - 1 {
                if col == 1 || col == cell_w - 2 {
                    buf[(px, py)].set_char('◯').set_style(wheel_style);
                }
                continue;
            }

            // Top row: brass trim
            if row == 0 {
                if col == 0 {
                    buf[(px, py)].set_char('┌').set_style(brass_style);
                } else if col == cell_w - 1 {
                    buf[(px, py)].set_char('┐').set_style(brass_style);
                } else {
                    buf[(px, py)].set_char('─').set_style(brass_style);
                }
                continue;
            }

            // Bottom of body
            if row == cell_h - 2 {
                if col == 0 {
                    buf[(px, py)].set_char('└').set_style(body_style);
                } else if col == cell_w - 1 {
                    buf[(px, py)].set_char('┘').set_style(body_style);
                } else {
                    buf[(px, py)].set_char('─').set_style(body_style);
                }
                continue;
            }

            // Middle rows: body with barrel
            if col == 0 {
                buf[(px, py)].set_char('│').set_style(body_style);
            } else if col == cell_w - 1 {
                buf[(px, py)].set_char('│').set_style(body_style);
            } else if row == 1 + (cell_h - 3) / 2 {
                // Barrel row (middle of body)
                if !flipped {
                    // Barrel extends right
                    if col < cell_w - 1 {
                        buf[(px, py)].set_char('═').set_style(barrel_style);
                    }
                } else {
                    // Barrel extends left
                    if col > 0 {
                        buf[(px, py)].set_char('═').set_style(barrel_style);
                    }
                }
            } else {
                buf[(px, py)].set_char('▓').set_style(body_style);
            }
        }
    }

    // Barrel tip indicator
    let barrel_row = cell_y + 1 + (cell_h - 3) / 2;
    if barrel_row >= area.y && barrel_row < area.y + area.height {
        let angle_char = if angle_deg < -10.0 {
            '╱'
        } else if angle_deg > 10.0 {
            '╲'
        } else {
            '═'
        };

        if !flipped {
            let tip_x = cell_x + cell_w - 1;
            if tip_x >= area.x && tip_x < area.x + area.width {
                buf[(tip_x, barrel_row)].set_char('▸').set_style(barrel_style);
            }
            // Angle indicator just before tip
            if cell_w > 2 {
                let ax = cell_x + cell_w - 2;
                if ax >= area.x && ax < area.x + area.width {
                    buf[(ax, barrel_row)].set_char(angle_char).set_style(barrel_style);
                }
            }
        } else {
            let tip_x = cell_x;
            if tip_x >= area.x && tip_x < area.x + area.width {
                buf[(tip_x, barrel_row)].set_char('◁').set_style(barrel_style);
            }
            if cell_w > 2 {
                let ax = cell_x + 1;
                if ax >= area.x && ax < area.x + area.width {
                    buf[(ax, barrel_row)].set_char(angle_char).set_style(barrel_style);
                }
            }
        }
    }
}
