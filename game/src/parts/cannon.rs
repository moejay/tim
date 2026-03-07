use image::{Rgba, RgbaImage};

use crate::gfx;
use crate::state::{Part, PartKind, SimBall};

const BODY_COLOR: Rgba<u8> = Rgba([80, 80, 100, 255]);
const BODY_HIGHLIGHT: Rgba<u8> = Rgba([110, 110, 130, 255]);
const BARREL_COLOR: Rgba<u8> = Rgba([60, 60, 75, 255]);

pub fn draw_cannon(img: &mut RgbaImage, part: &Part) {
    let x = part.x;
    let y = part.y;
    let (w, h) = part.size_px();

    let (angle_deg, _power) = match &part.kind {
        PartKind::Cannon { angle_deg, power } => (*angle_deg, *power),
        _ => return,
    };

    let body_x = if part.flipped { x + w * 0.3 } else { x };
    gfx::fill_rect(
        img,
        body_x as i32,
        (y + h * 0.2) as i32,
        (w * 0.7) as u32,
        (h * 0.7) as u32,
        BODY_COLOR,
    );
    gfx::fill_rect(
        img,
        body_x as i32,
        (y + h * 0.2) as i32,
        (w * 0.7) as u32,
        3,
        BODY_HIGHLIGHT,
    );

    let angle_rad = if part.flipped {
        (180.0 - angle_deg).to_radians()
    } else {
        angle_deg.to_radians()
    };
    let barrel_len = w * 0.5;
    let base_x = if part.flipped { x + w * 0.3 } else { x + w * 0.7 };
    let base_y = y + h * 0.4;
    let tip_x = base_x + barrel_len * angle_rad.cos();
    let tip_y = base_y + barrel_len * angle_rad.sin();

    gfx::draw_line(img, base_x, base_y, tip_x, tip_y, 8.0, BARREL_COLOR);

    let wheel_y = y + h * 0.85;
    let wheel_r = h * 0.12;
    gfx::fill_circle(img, x + w * 0.25, wheel_y, wheel_r, Rgba([50, 50, 60, 255]));
    gfx::fill_circle(img, x + w * 0.75, wheel_y, wheel_r, Rgba([50, 50, 60, 255]));
}

pub fn draw_cannon_icon(img: &mut RgbaImage, x: f32, y: f32) {
    gfx::fill_rect(img, x as i32, (y + 4.0) as i32, 28, 14, BODY_COLOR);
    gfx::draw_line(img, x + 28.0, y + 8.0, x + 40.0, y, 4.0, BARREL_COLOR);
}

pub fn spawn_ball(part: &Part) -> SimBall {
    let (angle_deg, power) = match &part.kind {
        PartKind::Cannon { angle_deg, power } => (*angle_deg, *power),
        _ => return SimBall::default(),
    };

    let x = part.x;
    let y = part.y;
    let (w, h) = part.size_px();

    let angle_rad = if part.flipped {
        (180.0 - angle_deg).to_radians()
    } else {
        angle_deg.to_radians()
    };

    let barrel_len = w * 0.55;
    let base_x = if part.flipped { x + w * 0.3 } else { x + w * 0.7 };
    let base_y = y + h * 0.4;
    let tip_x = base_x + barrel_len * angle_rad.cos();
    let tip_y = base_y + barrel_len * angle_rad.sin();

    SimBall {
        pos: (tip_x, tip_y),
        vel: (power * angle_rad.cos(), power * angle_rad.sin()),
        active: true,
    }
}
