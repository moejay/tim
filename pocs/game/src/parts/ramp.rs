use image::{Rgba, RgbaImage};

use crate::gfx;
use crate::state::{Part, SimBall};
use crate::parts::ball;

const RAMP_COLOR: Rgba<u8> = Rgba([160, 130, 80, 255]);
const RESTITUTION: f32 = 0.72;

pub fn draw_ramp(img: &mut RgbaImage, part: &Part) {
    let x = part.x;
    let y = part.y;
    let (w, h) = part.size_px();

    if part.flipped {
        gfx::fill_triangle(img, x, y, x + w, y + h, x, y + h, RAMP_COLOR);
    } else {
        gfx::fill_triangle(img, x + w, y, x + w, y + h, x, y + h, RAMP_COLOR);
    }
}

pub fn draw_ramp_icon(img: &mut RgbaImage, x: f32, y: f32) {
    gfx::fill_triangle(img, x + 30.0, y, x + 30.0, y + 20.0, x, y + 20.0, RAMP_COLOR);
}

pub fn collide_ramp(ball: &mut SimBall, part: &Part) {
    let r = ball::ball_radius();
    let rx = part.x;
    let ry = part.y;
    let (rw, rh) = part.size_px();

    let bx = ball.pos.0;
    let by = ball.pos.1;

    if bx + r < rx || bx - r > rx + rw || by + r < ry || by - r > ry + rh {
        return;
    }

    let (nx, ny, lx0, ly0, lx1, ly1) = if part.flipped {
        let nx = 1.0_f32;
        let ny = -rw / rh;
        let len = (nx * nx + ny * ny).sqrt();
        (nx / len, ny / len, rx, ry, rx + rw, ry + rh)
    } else {
        let nx = -1.0_f32;
        let ny = -rw / rh;
        let len = (nx * nx + ny * ny).sqrt();
        (nx / len, ny / len, rx + rw, ry, rx, ry + rh)
    };

    let dx = bx - lx0;
    let dy = by - ly0;
    let dist = dx * nx + dy * ny;

    if dist < r && dist > -r * 2.0 {
        let line_dx = lx1 - lx0;
        let line_dy = ly1 - ly0;
        let line_len = (line_dx * line_dx + line_dy * line_dy).sqrt();
        let t = ((bx - lx0) * line_dx + (by - ly0) * line_dy) / (line_len * line_len);

        if t >= -0.1 && t <= 1.1 {
            let penetration = r - dist;
            ball.pos.0 += nx * penetration;
            ball.pos.1 += ny * penetration;

            let dot = ball.vel.0 * nx + ball.vel.1 * ny;
            if dot < 0.0 {
                ball.vel.0 -= 2.0 * dot * nx;
                ball.vel.1 -= 2.0 * dot * ny;
                ball.vel.0 *= RESTITUTION;
                ball.vel.1 *= RESTITUTION;
            }
        }
    }
}
