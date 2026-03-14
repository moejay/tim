use image::RgbaImage;

/// Alpha-composite a single pixel onto the image.
pub fn blend_pixel(img: &mut RgbaImage, x: i32, y: i32, color: [u8; 4]) {
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

/// Fill a solid rectangle.
pub fn fill_rect(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, color: [u8; 4]) {
    for row in 0..h {
        for col in 0..w {
            blend_pixel(img, x + col, y + row, color);
        }
    }
}

/// Fill a rectangle with vertical gradient (top_color at top, bot_color at bottom).
pub fn fill_rect_gradient_v(
    img: &mut RgbaImage,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    top: [u8; 3],
    bot: [u8; 3],
) {
    if h <= 0 {
        return;
    }
    for row in 0..h {
        let t = if h > 1 {
            row as f32 / (h - 1) as f32
        } else {
            0.0
        };
        let c = lerp_color(top, bot, t);
        for col in 0..w {
            blend_pixel(img, x + col, y + row, [c[0], c[1], c[2], 255]);
        }
    }
}

/// Fill an anti-aliased circle with solid color.
pub fn fill_circle(img: &mut RgbaImage, cx: f32, cy: f32, radius: f32, color: [u8; 4]) {
    let r_ceil = radius.ceil() as i32 + 1;
    let icx = cx as i32;
    let icy = cy as i32;
    for dy in -r_ceil..=r_ceil {
        for dx in -r_ceil..=r_ceil {
            let px = icx + dx;
            let py = icy + dy;
            let dist = ((px as f32 - cx).powi(2) + (py as f32 - cy).powi(2)).sqrt();
            if dist < radius + 0.5 {
                let alpha = if dist > radius - 0.5 {
                    let aa = (radius + 0.5 - dist).clamp(0.0, 1.0);
                    (color[3] as f32 * aa) as u8
                } else {
                    color[3]
                };
                blend_pixel(img, px, py, [color[0], color[1], color[2], alpha]);
            }
        }
    }
}

/// Fill a circle with radial gradient (center_color at center, edge_color at rim).
pub fn fill_circle_gradient(
    img: &mut RgbaImage,
    cx: f32,
    cy: f32,
    radius: f32,
    center: [u8; 3],
    edge: [u8; 3],
) {
    let r_ceil = radius.ceil() as i32 + 1;
    let icx = cx as i32;
    let icy = cy as i32;
    for dy in -r_ceil..=r_ceil {
        for dx in -r_ceil..=r_ceil {
            let px = icx + dx;
            let py = icy + dy;
            let dist = ((px as f32 - cx).powi(2) + (py as f32 - cy).powi(2)).sqrt();
            if dist < radius + 0.5 {
                let t = (dist / radius).clamp(0.0, 1.0);
                let c = lerp_color(center, edge, t);
                let alpha = if dist > radius - 0.5 {
                    ((radius + 0.5 - dist).clamp(0.0, 1.0) * 255.0) as u8
                } else {
                    255
                };
                blend_pixel(img, px, py, [c[0], c[1], c[2], alpha]);
            }
        }
    }
}

/// Draw a line with Bresenham's algorithm, 1px thick.
pub fn draw_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: [u8; 4]) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut cx = x0;
    let mut cy = y0;

    loop {
        blend_pixel(img, cx, cy, color);
        if cx == x1 && cy == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            if cx == x1 {
                break;
            }
            err += dy;
            cx += sx;
        }
        if e2 <= dx {
            if cy == y1 {
                break;
            }
            err += dx;
            cy += sy;
        }
    }
}

/// Draw an anti-aliased line (Wu's algorithm).
pub fn draw_line_aa(img: &mut RgbaImage, x0: f32, y0: f32, x1: f32, y1: f32, color: [u8; 3]) {
    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    let (mut x0, mut y0, mut x1, mut y1) = if steep {
        (y0, x0, y1, x1)
    } else {
        (x0, y0, x1, y1)
    };

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let gradient = if dx.abs() < 1e-6 { 1.0 } else { dy / dx };

    // First endpoint
    let xend = x0.round();
    let yend = y0 + gradient * (xend - x0);
    let xgap = 1.0 - (x0 + 0.5).fract();
    let xpxl1 = xend as i32;
    let ypxl1 = yend.floor() as i32;

    if steep {
        blend_pixel(img, ypxl1, xpxl1, [color[0], color[1], color[2], ((1.0 - yend.fract()) * xgap * 255.0) as u8]);
        blend_pixel(img, ypxl1 + 1, xpxl1, [color[0], color[1], color[2], (yend.fract() * xgap * 255.0) as u8]);
    } else {
        blend_pixel(img, xpxl1, ypxl1, [color[0], color[1], color[2], ((1.0 - yend.fract()) * xgap * 255.0) as u8]);
        blend_pixel(img, xpxl1, ypxl1 + 1, [color[0], color[1], color[2], (yend.fract() * xgap * 255.0) as u8]);
    }

    let mut intery = yend + gradient;

    // Second endpoint
    let xend2 = x1.round();
    let yend2 = y1 + gradient * (xend2 - x1);
    let xgap2 = (x1 + 0.5).fract();
    let xpxl2 = xend2 as i32;
    let ypxl2 = yend2.floor() as i32;

    if steep {
        blend_pixel(img, ypxl2, xpxl2, [color[0], color[1], color[2], ((1.0 - yend2.fract()) * xgap2 * 255.0) as u8]);
        blend_pixel(img, ypxl2 + 1, xpxl2, [color[0], color[1], color[2], (yend2.fract() * xgap2 * 255.0) as u8]);
    } else {
        blend_pixel(img, xpxl2, ypxl2, [color[0], color[1], color[2], ((1.0 - yend2.fract()) * xgap2 * 255.0) as u8]);
        blend_pixel(img, xpxl2, ypxl2 + 1, [color[0], color[1], color[2], (yend2.fract() * xgap2 * 255.0) as u8]);
    }

    // Main loop
    for x in (xpxl1 + 1)..xpxl2 {
        let ipart = intery.floor() as i32;
        let fpart = intery.fract();
        if steep {
            blend_pixel(img, ipart, x, [color[0], color[1], color[2], ((1.0 - fpart) * 255.0) as u8]);
            blend_pixel(img, ipart + 1, x, [color[0], color[1], color[2], (fpart * 255.0) as u8]);
        } else {
            blend_pixel(img, x, ipart, [color[0], color[1], color[2], ((1.0 - fpart) * 255.0) as u8]);
            blend_pixel(img, x, ipart + 1, [color[0], color[1], color[2], (fpart * 255.0) as u8]);
        }
        intery += gradient;
    }
}

/// Fill a triangle using scanline algorithm.
pub fn fill_triangle(
    img: &mut RgbaImage,
    v0: (f32, f32),
    v1: (f32, f32),
    v2: (f32, f32),
    color: [u8; 4],
) {
    // Sort vertices by Y
    let mut verts = [v0, v1, v2];
    verts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let [top, mid, bot] = verts;

    let min_y = top.1.ceil() as i32;
    let max_y = bot.1.floor() as i32;

    for y in min_y..=max_y {
        let yf = y as f32;

        // Compute x intersections with the triangle edges
        let mut xs = Vec::new();

        // Edge: top -> bot (always spans full height)
        let dy_tb = bot.1 - top.1;
        if dy_tb.abs() > 1e-6 {
            let t = (yf - top.1) / dy_tb;
            if t >= -1e-6 && t <= 1.0 + 1e-6 {
                xs.push(top.0 + t * (bot.0 - top.0));
            }
        }

        // Edge: top -> mid
        let dy_tm = mid.1 - top.1;
        if dy_tm.abs() > 1e-6 {
            let t = (yf - top.1) / dy_tm;
            if t >= -1e-6 && t <= 1.0 + 1e-6 {
                xs.push(top.0 + t * (mid.0 - top.0));
            }
        }

        // Edge: mid -> bot
        let dy_mb = bot.1 - mid.1;
        if dy_mb.abs() > 1e-6 {
            let t = (yf - mid.1) / dy_mb;
            if t >= -1e-6 && t <= 1.0 + 1e-6 {
                xs.push(mid.0 + t * (bot.0 - mid.0));
            }
        }

        if xs.len() >= 2 {
            let left = xs.iter().cloned().fold(f32::INFINITY, f32::min);
            let right = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let x_start = left.ceil() as i32;
            let x_end = right.floor() as i32;
            for x in x_start..=x_end {
                blend_pixel(img, x, y, color);
            }
        }
    }
}

/// Fill a triangle with vertical gradient.
pub fn fill_triangle_gradient(
    img: &mut RgbaImage,
    v0: (f32, f32),
    v1: (f32, f32),
    v2: (f32, f32),
    top_color: [u8; 3],
    bot_color: [u8; 3],
) {
    let mut verts = [v0, v1, v2];
    verts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let [top, mid, bot] = verts;

    let min_y = top.1.ceil() as i32;
    let max_y = bot.1.floor() as i32;
    let total_h = bot.1 - top.1;

    for y in min_y..=max_y {
        let yf = y as f32;
        let t_grad = if total_h.abs() > 1e-6 {
            ((yf - top.1) / total_h).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let c = lerp_color(top_color, bot_color, t_grad);

        let mut xs = Vec::new();

        let dy_tb = bot.1 - top.1;
        if dy_tb.abs() > 1e-6 {
            let t = (yf - top.1) / dy_tb;
            if t >= -1e-6 && t <= 1.0 + 1e-6 {
                xs.push(top.0 + t * (bot.0 - top.0));
            }
        }

        let dy_tm = mid.1 - top.1;
        if dy_tm.abs() > 1e-6 {
            let t = (yf - top.1) / dy_tm;
            if t >= -1e-6 && t <= 1.0 + 1e-6 {
                xs.push(top.0 + t * (mid.0 - top.0));
            }
        }

        let dy_mb = bot.1 - mid.1;
        if dy_mb.abs() > 1e-6 {
            let t = (yf - mid.1) / dy_mb;
            if t >= -1e-6 && t <= 1.0 + 1e-6 {
                xs.push(mid.0 + t * (bot.0 - mid.0));
            }
        }

        if xs.len() >= 2 {
            let left = xs.iter().cloned().fold(f32::INFINITY, f32::min);
            let right = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let x_start = left.ceil() as i32;
            let x_end = right.floor() as i32;
            for x in x_start..=x_end {
                blend_pixel(img, x, y, [c[0], c[1], c[2], 255]);
            }
        }
    }
}

/// Draw crosshair cursor (4 line segments with center gap).
pub fn draw_crosshair(
    img: &mut RgbaImage,
    cx: f32,
    cy: f32,
    color: [u8; 4],
    gap: f32,
    length: f32,
) {
    let icx = cx as i32;
    let icy = cy as i32;
    let igap = gap as i32;
    let ilen = length as i32;

    // Left segment
    for x in (icx - igap - ilen)..=(icx - igap) {
        blend_pixel(img, x, icy, color);
    }
    // Right segment
    for x in (icx + igap)..=(icx + igap + ilen) {
        blend_pixel(img, x, icy, color);
    }
    // Up segment
    for y in (icy - igap - ilen)..=(icy - igap) {
        blend_pixel(img, icx, y, color);
    }
    // Down segment
    for y in (icy + igap)..=(icy + igap + ilen) {
        blend_pixel(img, icx, y, color);
    }
}

/// Lerp between two colors.
pub fn lerp_color(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        (a[0] as f32 * (1.0 - t) + b[0] as f32 * t) as u8,
        (a[1] as f32 * (1.0 - t) + b[1] as f32 * t) as u8,
        (a[2] as f32 * (1.0 - t) + b[2] as f32 * t) as u8,
    ]
}

/// Draw text string using a basic 5x7 bitmap font.
/// Only supports uppercase A-Z, 0-9, and basic punctuation.
/// Scale parameter multiplies each pixel.
pub fn draw_text(img: &mut RgbaImage, x: i32, y: i32, text: &str, color: [u8; 4], scale: u32) {
    let mut cursor_x = x;
    for ch in text.chars() {
        if let Some(glyph) = get_glyph(ch) {
            for (row, &bits) in glyph.iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) != 0 {
                        for sy in 0..scale {
                            for sx in 0..scale {
                                blend_pixel(
                                    img,
                                    cursor_x + col * scale as i32 + sx as i32,
                                    y + row as i32 * scale as i32 + sy as i32,
                                    color,
                                );
                            }
                        }
                    }
                }
            }
        }
        cursor_x += 6 * scale as i32; // 5px char + 1px spacing
    }
}

/// Returns the width in pixels that a text string would occupy at the given scale.
pub fn text_width(text: &str, scale: u32) -> i32 {
    let len = text.len() as i32;
    if len == 0 {
        return 0;
    }
    len * 6 * scale as i32 - scale as i32 // subtract trailing spacing
}

fn get_glyph(ch: char) -> Option<&'static [u8; 7]> {
    let idx = match ch.to_ascii_uppercase() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        '0' => 26,
        '1' => 27,
        '2' => 28,
        '3' => 29,
        '4' => 30,
        '5' => 31,
        '6' => 32,
        '7' => 33,
        '8' => 34,
        '9' => 35,
        ' ' => 36,
        ':' => 37,
        '.' => 38,
        ',' => 39,
        '(' => 40,
        ')' => 41,
        '[' => 42,
        ']' => 43,
        '-' => 44,
        '/' => 45,
        '|' => 46,
        '?' => 47,
        '!' => 48,
        _ => return None,
    };
    Some(&FONT_5X7[idx])
}

// 5x7 bitmap font. Each byte = one row, bits 4..0 = columns left to right.
#[rustfmt::skip]
static FONT_5X7: [[u8; 7]; 49] = [
    // A
    [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
    // B
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
    // C
    [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
    // D
    [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
    // E
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
    // F
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
    // G
    [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
    // H
    [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
    // I
    [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // J
    [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
    // K
    [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
    // L
    [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
    // M
    [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
    // N
    [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
    // O
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // P
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
    // Q
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
    // R
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
    // S
    [0b01110, 0b10001, 0b10000, 0b01110, 0b00001, 0b10001, 0b01110],
    // T
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
    // U
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // V
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
    // W
    [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001],
    // X
    [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
    // Y
    [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
    // Z
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
    // 0
    [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
    // 1
    [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // 2
    [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111],
    // 3
    [0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110],
    // 4
    [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
    // 5
    [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
    // 6
    [0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
    // 7
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
    // 8
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
    // 9
    [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
    // ' ' (space)
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    // ':'
    [0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000],
    // '.'
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100],
    // ','
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100, 0b01000],
    // '('
    [0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010],
    // ')'
    [0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000],
    // '['
    [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110],
    // ']'
    [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110],
    // '-'
    [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
    // '/'
    [0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000],
    // '|'
    [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
    // '?'
    [0b01110, 0b10001, 0b00001, 0b00110, 0b00100, 0b00000, 0b00100],
    // '!'
    [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
];
