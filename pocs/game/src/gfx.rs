use image::{Rgba, RgbaImage};

pub fn blend_pixel(img: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x < 0 || y < 0 || x >= img.width() as i32 || y >= img.height() as i32 {
        return;
    }
    let alpha = color.0[3] as f32 / 255.0;
    if alpha <= 0.0 {
        return;
    }
    let dst = img.get_pixel(x as u32, y as u32);
    let inv = 1.0 - alpha;
    let r = (color.0[0] as f32 * alpha + dst.0[0] as f32 * inv) as u8;
    let g = (color.0[1] as f32 * alpha + dst.0[1] as f32 * inv) as u8;
    let b = (color.0[2] as f32 * alpha + dst.0[2] as f32 * inv) as u8;
    let a = (color.0[3].max(dst.0[3])) as u8;
    img.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
}

pub fn fill_rect(img: &mut RgbaImage, x: i32, y: i32, w: u32, h: u32, color: Rgba<u8>) {
    for dy in 0..h as i32 {
        for dx in 0..w as i32 {
            blend_pixel(img, x + dx, y + dy, color);
        }
    }
}

pub fn draw_rect_outline(img: &mut RgbaImage, x: i32, y: i32, w: u32, h: u32, color: Rgba<u8>) {
    let w = w as i32;
    let h = h as i32;
    for dx in 0..w {
        blend_pixel(img, x + dx, y, color);
        blend_pixel(img, x + dx, y + h - 1, color);
    }
    for dy in 0..h {
        blend_pixel(img, x, y + dy, color);
        blend_pixel(img, x + w - 1, y + dy, color);
    }
}

pub fn fill_circle(img: &mut RgbaImage, cx: f32, cy: f32, radius: f32, color: Rgba<u8>) {
    let r2 = radius * radius;
    let x0 = (cx - radius - 1.0).floor() as i32;
    let x1 = (cx + radius + 1.0).ceil() as i32;
    let y0 = (cy - radius - 1.0).floor() as i32;
    let y1 = (cy + radius + 1.0).ceil() as i32;
    for py in y0..=y1 {
        for px in x0..=x1 {
            let dx = px as f32 - cx;
            let dy = py as f32 - cy;
            let dist2 = dx * dx + dy * dy;
            if dist2 <= r2 {
                let edge = (radius - dist2.sqrt()).clamp(0.0, 1.0);
                let a = (color.0[3] as f32 * edge) as u8;
                blend_pixel(img, px, py, Rgba([color.0[0], color.0[1], color.0[2], a]));
            }
        }
    }
}

pub fn fill_circle_shaded(
    img: &mut RgbaImage,
    cx: f32,
    cy: f32,
    radius: f32,
    base_color: [u8; 3],
) {
    let r2 = radius * radius;
    let x0 = (cx - radius - 1.0).floor() as i32;
    let x1 = (cx + radius + 1.0).ceil() as i32;
    let y0 = (cy - radius - 1.0).floor() as i32;
    let y1 = (cy + radius + 1.0).ceil() as i32;
    let light_x = cx - radius * 0.3;
    let light_y = cy - radius * 0.3;
    for py in y0..=y1 {
        for px in x0..=x1 {
            let dx = px as f32 - cx;
            let dy = py as f32 - cy;
            let dist2 = dx * dx + dy * dy;
            if dist2 <= r2 {
                let edge = (radius - dist2.sqrt()).clamp(0.0, 1.0);
                let ldx = px as f32 - light_x;
                let ldy = py as f32 - light_y;
                let light_dist = (ldx * ldx + ldy * ldy).sqrt() / (radius * 1.5);
                let brightness = (1.0 - light_dist * 0.6).clamp(0.4, 1.0);
                let spec = ((1.0 - light_dist).max(0.0)).powi(8) * 180.0;
                let r = ((base_color[0] as f32 * brightness + spec).min(255.0)) as u8;
                let g = ((base_color[1] as f32 * brightness + spec).min(255.0)) as u8;
                let b = ((base_color[2] as f32 * brightness + spec).min(255.0)) as u8;
                let a = (255.0 * edge) as u8;
                blend_pixel(img, px, py, Rgba([r, g, b, a]));
            }
        }
    }
}

pub fn draw_line(
    img: &mut RgbaImage,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    thickness: f32,
    color: Rgba<u8>,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.001 {
        return;
    }
    let nx = -dy / len;
    let ny = dx / len;
    let half_t = thickness / 2.0;
    let min_x = x0.min(x1) - half_t - 1.0;
    let max_x = x0.max(x1) + half_t + 1.0;
    let min_y = y0.min(y1) - half_t - 1.0;
    let max_y = y0.max(y1) + half_t + 1.0;
    for py in min_y.floor() as i32..=max_y.ceil() as i32 {
        for px in min_x.floor() as i32..=max_x.ceil() as i32 {
            let pxf = px as f32;
            let pyf = py as f32;
            let cross = (pxf - x0) * nx + (pyf - y0) * ny;
            let dist_perp = cross.abs();
            let dot = (pxf - x0) * dx / len + (pyf - y0) * dy / len;
            let dist_along = if dot < 0.0 {
                -dot
            } else if dot > len {
                dot - len
            } else {
                0.0
            };
            let dist = (dist_perp * dist_perp + dist_along * dist_along).sqrt();
            if dist <= half_t {
                let edge = (half_t - dist).clamp(0.0, 1.0);
                let a = (color.0[3] as f32 * edge) as u8;
                blend_pixel(img, px, py, Rgba([color.0[0], color.0[1], color.0[2], a]));
            }
        }
    }
}

pub fn fill_triangle(
    img: &mut RgbaImage,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: Rgba<u8>,
) {
    let min_x = x0.min(x1).min(x2).floor() as i32;
    let max_x = x0.max(x1).max(x2).ceil() as i32;
    let min_y = y0.min(y1).min(y2).floor() as i32;
    let max_y = y0.max(y1).max(y2).ceil() as i32;
    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let pxf = px as f32 + 0.5;
            let pyf = py as f32 + 0.5;
            if point_in_triangle(pxf, pyf, x0, y0, x1, y1, x2, y2) {
                blend_pixel(img, px, py, color);
            }
        }
    }
}

fn point_in_triangle(
    px: f32, py: f32,
    x0: f32, y0: f32,
    x1: f32, y1: f32,
    x2: f32, y2: f32,
) -> bool {
    let d1 = sign(px, py, x0, y0, x1, y1);
    let d2 = sign(px, py, x1, y1, x2, y2);
    let d3 = sign(px, py, x2, y2, x0, y0);
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    !(has_neg && has_pos)
}

fn sign(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (px - x2) * (y1 - y2) - (x1 - x2) * (py - y2)
}
