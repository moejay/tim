use image::{Rgba, RgbaImage};

use crate::gfx;
use crate::parts::ball;
use crate::state::{Part, SimBall};

const WALL_COLOR: Rgba<u8> = Rgba([140, 140, 160, 255]);
const WALL_HIGHLIGHT: Rgba<u8> = Rgba([170, 170, 190, 255]);
const RESTITUTION: f32 = 0.72;
const FRICTION: f32 = 0.85;

pub fn draw_wall(img: &mut RgbaImage, part: &Part) {
    let (w, h) = part.size_px();
    let x = part.x as i32;
    let y = part.y as i32;

    gfx::fill_rect(img, x, y, w as u32, h as u32, WALL_COLOR);
    gfx::fill_rect(img, x, y, w as u32, 2, WALL_HIGHLIGHT);
    gfx::fill_rect(img, x, y, 2, h as u32, WALL_HIGHLIGHT);
}

pub fn draw_wall_icon(img: &mut RgbaImage, x: f32, y: f32) {
    gfx::fill_rect(img, x as i32, y as i32, 40, 12, WALL_COLOR);
    gfx::fill_rect(img, x as i32, y as i32, 40, 2, WALL_HIGHLIGHT);
}

pub fn collide_wall(ball: &mut SimBall, part: &Part) {
    let r = ball::ball_radius();
    let (ww, wh) = part.size_px();
    let wx = part.x;
    let wy = part.y;

    let closest_x = ball.pos.0.clamp(wx, wx + ww);
    let closest_y = ball.pos.1.clamp(wy, wy + wh);

    let dx = ball.pos.0 - closest_x;
    let dy = ball.pos.1 - closest_y;
    let dist2 = dx * dx + dy * dy;

    if dist2 < r * r {
        let dist = dist2.sqrt();
        if dist < 0.001 {
            let left = ball.pos.0 - wx;
            let right = wx + ww - ball.pos.0;
            let top = ball.pos.1 - wy;
            let bottom = wy + wh - ball.pos.1;
            let min_pen = left.min(right).min(top).min(bottom);

            if min_pen == left {
                ball.pos.0 = wx - r;
                ball.vel.0 = -ball.vel.0.abs() * RESTITUTION;
                ball.vel.1 *= FRICTION;
            } else if min_pen == right {
                ball.pos.0 = wx + ww + r;
                ball.vel.0 = ball.vel.0.abs() * RESTITUTION;
                ball.vel.1 *= FRICTION;
            } else if min_pen == top {
                ball.pos.1 = wy - r;
                ball.vel.1 = -ball.vel.1.abs() * RESTITUTION;
                ball.vel.0 *= FRICTION;
            } else {
                ball.pos.1 = wy + wh + r;
                ball.vel.1 = ball.vel.1.abs() * RESTITUTION;
                ball.vel.0 *= FRICTION;
            }
        } else {
            let nx = dx / dist;
            let ny = dy / dist;
            let penetration = r - dist;
            ball.pos.0 += nx * penetration;
            ball.pos.1 += ny * penetration;

            let dot = ball.vel.0 * nx + ball.vel.1 * ny;
            if dot < 0.0 {
                ball.vel.0 -= (1.0 + RESTITUTION) * dot * nx;
                ball.vel.1 -= (1.0 + RESTITUTION) * dot * ny;
                let tx = -ny;
                let ty = nx;
                let tang = ball.vel.0 * tx + ball.vel.1 * ty;
                ball.vel.0 -= tang * (1.0 - FRICTION) * tx;
                ball.vel.1 -= tang * (1.0 - FRICTION) * ty;
            }
        }
    }
}
