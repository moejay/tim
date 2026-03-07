use image::RgbaImage;

use crate::gfx;
use crate::state::SimBall;

const BALL_RADIUS: f32 = 14.0;
const BALL_COLOR: [u8; 3] = [220, 60, 60];

pub fn draw_ball(img: &mut RgbaImage, ball: &SimBall) {
    if !ball.active {
        return;
    }
    gfx::fill_circle_shaded(img, ball.pos.0, ball.pos.1, BALL_RADIUS, BALL_COLOR);
}

pub fn draw_ball_icon(img: &mut RgbaImage, cx: f32, cy: f32) {
    gfx::fill_circle_shaded(img, cx, cy, 10.0, BALL_COLOR);
}

pub fn ball_radius() -> f32 {
    BALL_RADIUS
}
