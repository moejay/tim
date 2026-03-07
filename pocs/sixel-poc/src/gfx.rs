/// Shared pixel-level drawing primitives used by all demos.
use image::{Rgba, RgbaImage};

// ─── Noise ────────────────────────────────────────────────────────────────────
pub fn h(seed: u32) -> f32 {
    let x = seed.wrapping_mul(2246822519).wrapping_add(2654435769);
    let x = x ^ (x >> 13);
    let x = x.wrapping_mul(3266489917);
    let x = x ^ (x >> 16);
    (x >> 8) as f32 / 16_777_215.0
}
pub fn hs(seed: u32) -> f32 { h(seed) * 2.0 - 1.0 }

// ─── Blend ────────────────────────────────────────────────────────────────────
/// Additive blend a premultiplied color+alpha onto a pixel.
#[inline]
pub fn blend(px: &mut Rgba<u8>, r: u8, g: u8, b: u8, a: f32) {
    let a = a.clamp(0.0, 1.0);
    px[0] = (px[0] as f32 + r as f32 * a).min(255.0) as u8;
    px[1] = (px[1] as f32 + g as f32 * a).min(255.0) as u8;
    px[2] = (px[2] as f32 + b as f32 * a).min(255.0) as u8;
    px[3] = 255;
}

/// Alpha-composite (standard over) a color onto a pixel.
#[inline]
pub fn over(px: &mut Rgba<u8>, r: u8, g: u8, b: u8, a: f32) {
    let a = a.clamp(0.0, 1.0);
    let ia = 1.0 - a;
    px[0] = (r as f32 * a + px[0] as f32 * ia) as u8;
    px[1] = (g as f32 * a + px[1] as f32 * ia) as u8;
    px[2] = (b as f32 * a + px[2] as f32 * ia) as u8;
    px[3] = 255;
}

// ─── Filled circle ────────────────────────────────────────────────────────────
pub fn fill_circle(img: &mut RgbaImage, cx: f32, cy: f32, r: f32, col: [u8; 3], alpha: f32) {
    let x0 = ((cx - r) as i32).max(0) as u32;
    let x1 = ((cx + r) as i32 + 1).min(img.width() as i32) as u32;
    let y0 = ((cy - r) as i32).max(0) as u32;
    let y1 = ((cy + r) as i32 + 1).min(img.height() as i32) as u32;
    let r2 = r * r;
    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let d2 = dx * dx + dy * dy;
            if d2 <= r2 {
                // Anti-alias at edge
                let edge = (r2 - d2).sqrt().min(1.0);
                over(img.get_pixel_mut(x, y), col[0], col[1], col[2], alpha * edge);
            }
        }
    }
}

// ─── Radial glow ─────────────────────────────────────────────────────────────
/// Soft radial glow — additive, falls off as (1 - dist/max_r)^2.
pub fn radial_glow(img: &mut RgbaImage, cx: f32, cy: f32, max_r: f32, col: [u8; 3], strength: f32) {
    let x0 = ((cx - max_r) as i32).max(0) as u32;
    let x1 = ((cx + max_r) as i32 + 1).min(img.width() as i32) as u32;
    let y0 = ((cy - max_r) as i32).max(0) as u32;
    let y1 = ((cy + max_r) as i32 + 1).min(img.height() as i32) as u32;
    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist >= max_r { continue; }
            let t = 1.0 - dist / max_r;
            let a = t * t * strength;
            blend(img.get_pixel_mut(x, y), col[0], col[1], col[2], a);
        }
    }
}

// ─── Glowing line ─────────────────────────────────────────────────────────────
/// Draws a line with multiple glow passes: bright core + soft halo.
pub fn glowing_line(
    img: &mut RgbaImage,
    x0: f32, y0: f32, x1: f32, y1: f32,
    core: [u8; 3],
    glow: [u8; 3],
    core_w: f32,
    glow_w: f32,
    intensity: f32,
) {
    let len = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
    if len == 0.0 { return; }
    let steps = (len * 2.0) as u32 + 1;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let px = x0 + (x1 - x0) * t;
        let py = y0 + (y1 - y0) * t;
        // Glow halo
        radial_glow(img, px, py, glow_w, glow, intensity * 0.25);
        // Core
        fill_circle(img, px, py, core_w, core, intensity);
    }
}

// ─── Filled rect ──────────────────────────────────────────────────────────────
pub fn fill_rect(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, col: [u8; 3], alpha: f32) {
    let x0 = x.max(0) as u32;
    let x1 = (x + w).min(img.width() as i32) as u32;
    let y0 = y.max(0) as u32;
    let y1 = (y + h).min(img.height() as i32) as u32;
    for py in y0..y1 {
        for px in x0..x1 {
            over(img.get_pixel_mut(px, py), col[0], col[1], col[2], alpha);
        }
    }
}

// ─── Procedural flame ────────────────────────────────────────────────────────
/// Renders a procedural fire effect into the image.
/// `base_y` is the bottom of the flame, flame grows upward.
pub fn draw_flame(
    img: &mut RgbaImage,
    cx: f32, base_y: f32,
    flame_w: f32, flame_h: f32,
    t: f32,
    frame: u64,
) {
    // Wide glow beneath flame
    radial_glow(img, cx, base_y - flame_h * 0.3, flame_w * 2.5, [255, 120, 10], 0.35);

    let steps_y = (flame_h * 2.0) as i32;
    let steps_x = (flame_w * 2.0) as i32;

    for dy in 0..=steps_y {
        let fy = dy as f32 / steps_y as f32; // 0=base, 1=tip
        let py = base_y - fy * flame_h;
        if py < 0.0 || py >= img.height() as f32 { continue; }

        // How wide is the flame at this height?
        let wobble = (t * 4.7 + fy * 5.0).sin() * 0.18 + (t * 11.3 + fy * 2.0).sin() * 0.07;
        let half_w = flame_w * (1.0 - fy * 0.88) * (1.0 + wobble);

        for dx in -steps_x..=steps_x {
            let fx = dx as f32 / steps_x as f32 * flame_w; // pixels from center
            let px = cx + fx + (t * 3.1 + fy * 2.0).sin() * flame_w * 0.12;
            if px < 0.0 || px >= img.width() as f32 { continue; }

            let dist_norm = fx.abs() / half_w;
            if dist_norm > 1.0 { continue; }

            // Noise flicker
            let noise_seed = (px as u32).wrapping_mul(7) ^ (py as u32).wrapping_mul(13) ^ (frame / 2) as u32;
            let noise = h(noise_seed) * 0.25;

            let intensity = ((1.0 - dist_norm) * (1.0 - fy * 0.75) + noise * (1.0 - fy)).clamp(0.0, 1.0);
            if intensity < 0.04 { continue; }

            // Color: dark red → orange → yellow → white
            let (r, g, b, a) = if intensity < 0.25 {
                let tt = intensity / 0.25;
                (180 + (60.0 * tt) as u8, (30.0 * tt) as u8, 0u8, intensity * 1.8)
            } else if intensity < 0.55 {
                let tt = (intensity - 0.25) / 0.30;
                (240, (30.0 + 160.0 * tt) as u8, 0u8, intensity)
            } else if intensity < 0.82 {
                let tt = (intensity - 0.55) / 0.27;
                (255u8, 190 + (60.0 * tt) as u8, (100.0 * tt) as u8, intensity)
            } else {
                let tt = (intensity - 0.82) / 0.18;
                (255u8, 255u8, (100.0 + 155.0 * tt) as u8, 1.0)
            };

            over(img.get_pixel_mut(px as u32, py as u32), r, g, b, a.clamp(0.0, 1.0));
        }
    }
}

// ─── Dark background ─────────────────────────────────────────────────────────
pub fn clear_dark(img: &mut RgbaImage, r: u8, g: u8, b: u8) {
    for px in img.pixels_mut() {
        *px = Rgba([r, g, b, 255]);
    }
}
