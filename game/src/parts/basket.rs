use image::{Rgba, RgbaImage};

use crate::gfx;
use crate::parts::ball;
use crate::state::{Part, SimBall};

const BASKET_COLOR: Rgba<u8> = Rgba([80, 200, 80, 255]);
const BASKET_INNER: Rgba<u8> = Rgba([40, 80, 40, 120]);
const WALL_THICK: f32 = 4.0;

pub fn draw_basket(img: &mut RgbaImage, part: &Part) {
    let x = part.x as i32;
    let y = part.y as i32;
    let (w, h) = part.size_px();
    let w = w as u32;
    let h = h as u32;

    gfx::fill_rect(img, x + 4, y + 4, w - 8, h - 4, BASKET_INNER);

    let t = WALL_THICK as u32;
    gfx::fill_rect(img, x, y, t, h, BASKET_COLOR);
    gfx::fill_rect(img, x, y + h as i32 - t as i32, w, t, BASKET_COLOR);
    gfx::fill_rect(img, x + w as i32 - t as i32, y, t, h, BASKET_COLOR);
}

pub fn draw_basket_icon(img: &mut RgbaImage, x: f32, y: f32) {
    let xi = x as i32;
    let yi = y as i32;
    gfx::fill_rect(img, xi, yi, 3, 24, BASKET_COLOR);
    gfx::fill_rect(img, xi, yi + 21, 24, 3, BASKET_COLOR);
    gfx::fill_rect(img, xi + 21, yi, 3, 24, BASKET_COLOR);
}

pub fn check_win(ball: &SimBall, part: &Part) -> bool {
    if !ball.active {
        return false;
    }
    let r = ball::ball_radius();
    let (bw, bh) = part.size_px();

    let margin = WALL_THICK + r;
    let inner_left = part.x + margin;
    let inner_right = part.x + bw - margin;
    let inner_top = part.y;
    let inner_bottom = part.y + bh - margin;

    ball.pos.0 > inner_left
        && ball.pos.0 < inner_right
        && ball.pos.1 > inner_top
        && ball.pos.1 < inner_bottom
        && ball.vel.1 > 0.0
}

pub fn collide_basket(ball: &mut SimBall, part: &Part) {
    let r = ball::ball_radius();
    let (bw, bh) = part.size_px();
    let t = WALL_THICK;

    collide_rect(ball, part.x, part.y, t, bh, r);
    collide_rect(ball, part.x + bw - t, part.y, t, bh, r);
    collide_rect(ball, part.x, part.y + bh - t, bw, t, r);
}

fn collide_rect(ball: &mut SimBall, rx: f32, ry: f32, rw: f32, rh: f32, r: f32) {
    let closest_x = ball.pos.0.clamp(rx, rx + rw);
    let closest_y = ball.pos.1.clamp(ry, ry + rh);
    let dx = ball.pos.0 - closest_x;
    let dy = ball.pos.1 - closest_y;
    let dist2 = dx * dx + dy * dy;

    if dist2 < r * r && dist2 > 0.001 {
        let dist = dist2.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;
        let penetration = r - dist;
        ball.pos.0 += nx * penetration;
        ball.pos.1 += ny * penetration;

        let dot = ball.vel.0 * nx + ball.vel.1 * ny;
        if dot < 0.0 {
            ball.vel.0 -= 2.0 * dot * nx * 0.72;
            ball.vel.1 -= 2.0 * dot * ny * 0.72;
        }
    }
}
