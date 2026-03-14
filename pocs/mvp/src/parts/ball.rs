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

fn lerp_color(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        (a[0] as f32 * (1.0 - t) + b[0] as f32 * t) as u8,
        (a[1] as f32 * (1.0 - t) + b[1] as f32 * t) as u8,
        (a[2] as f32 * (1.0 - t) + b[2] as f32 * t) as u8,
    ]
}

pub fn draw_ball_pixel(img: &mut RgbaImage, cx: f32, cy: f32) {
    let radius: f32 = 14.0;
    let center_color: [u8; 3] = [0xFF, 0x8C, 0x42];
    let edge_color: [u8; 3] = [0xB3, 0x3A, 0x00];

    let ix = cx as i32;
    let iy = cy as i32;
    let r = radius as i32 + 2;

    // Drop shadow
    let shadow_offset = 3;
    let shadow_radius = radius + 2.0;
    for dy in -r - 3..=r + 3 {
        for dx in -r - 3..=r + 3 {
            let dist = ((dx * dx + (dy - shadow_offset) * (dy - shadow_offset)) as f32).sqrt();
            if dist < shadow_radius {
                let alpha = if dist > shadow_radius - 1.0 {
                    ((shadow_radius - dist).clamp(0.0, 1.0) * 51.0) as u8
                } else {
                    51 // 20% of 255
                };
                blend_pixel(img, ix + dx, iy + dy, [0, 0, 0, alpha]);
            }
        }
    }

    // Base sphere with radial gradient
    for dy in -r..=r {
        for dx in -r..=r {
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            if dist < radius + 1.0 {
                let t = dist / radius;
                let c = lerp_color(center_color, edge_color, t);
                let alpha = if dist > radius - 1.0 {
                    ((radius - dist).clamp(0.0, 1.0) * 255.0) as u8
                } else {
                    255
                };
                blend_pixel(img, ix + dx, iy + dy, [c[0], c[1], c[2], alpha]);
            }
        }
    }

    // Specular highlight
    let spec_ox: f32 = -4.0;
    let spec_oy: f32 = -5.0;
    let spec_radius: f32 = 4.0;
    for dy in -6..=2 {
        for dx in -8..=2 {
            let sdx = dx as f32 - spec_ox;
            let sdy = dy as f32 - spec_oy;
            let dist = (sdx * sdx + sdy * sdy).sqrt();
            if dist < spec_radius {
                let falloff = (-dist * dist / (spec_radius * spec_radius) * 2.0).exp();
                let alpha = (falloff * 178.0) as u8; // 70% max
                blend_pixel(img, ix + dx, iy + dy, [255, 255, 255, alpha]);
            }
        }
    }

    // Rim light (bottom-right quadrant near edge)
    for dy in 0..=r {
        for dx in 0..=r {
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            if dist > radius * 0.8 && dist < radius {
                let intensity = ((dist - radius * 0.8) / (radius * 0.2)).clamp(0.0, 1.0);
                let alpha = (intensity * 76.0) as u8; // 30% max
                blend_pixel(img, ix + dx, iy + dy, [0xFF, 0xAA, 0x66, alpha]);
            }
        }
    }
}

pub fn draw_ball_text(buf: &mut Buffer, cell_x: u16, cell_y: u16, area: Rect) {
    if cell_x >= area.x && cell_x < area.x + area.width && cell_y >= area.y && cell_y < area.y + area.height {
        buf[(cell_x, cell_y)].set_char('●')
            .set_style(Style::default().fg(Color::Rgb(255, 107, 53)).add_modifier(Modifier::BOLD));
    }
}

pub fn draw_trail_pixel(img: &mut RgbaImage, trail: &[(f32, f32)]) {
    let len = trail.len();
    if len == 0 {
        return;
    }
    for (i, &(tx, ty)) in trail.iter().enumerate() {
        let t = i as f32 / len.max(1) as f32;
        let radius = 10.0 - 7.0 * t; // 10 down to 3
        let opacity = 100.0 - 88.0 * t; // 100 down to 12
        let r_int = radius.ceil() as i32 + 1;
        let ix = tx as i32;
        let iy = ty as i32;
        for dy in -r_int..=r_int {
            for dx in -r_int..=r_int {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist < radius {
                    let a = if dist > radius - 1.0 {
                        ((radius - dist).clamp(0.0, 1.0) * opacity) as u8
                    } else {
                        opacity as u8
                    };
                    blend_pixel(img, ix + dx, iy + dy, [0xFF, 0x6B, 0x35, a]);
                }
            }
        }
    }
}

pub fn draw_trail_text(buf: &mut Buffer, trail_cells: &[(u16, u16)], area: Rect) {
    let len = trail_cells.len();
    if len == 0 {
        return;
    }
    for (i, &(cx, cy)) in trail_cells.iter().enumerate() {
        if cx >= area.x && cx < area.x + area.width && cy >= area.y && cy < area.y + area.height {
            let t = i as f32 / len.max(1) as f32;
            let r = (255.0 - 155.0 * t) as u8;
            let g = (107.0 - 67.0 * t) as u8;
            let b = (53.0 - 38.0 * t) as u8;
            buf[(cx, cy)].set_char('·')
                .set_style(Style::default().fg(Color::Rgb(r, g, b)));
        }
    }
}
