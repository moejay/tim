use log::info;
use crate::state::*;

pub const GRAVITY: f32 = 900.0;
pub const RESTITUTION: f32 = 0.72;
pub const FRICTION: f32 = 0.85;
pub const MAX_DT: f32 = 1.0 / 30.0;
pub const BALL_RADIUS: f32 = 14.0;

pub fn update_physics(state: &mut GameState, dt: f32) {
    let dt = dt.min(MAX_DT);

    if !state.ball.active || state.won {
        return;
    }

    // Apply gravity
    state.ball.vel.1 += GRAVITY * dt;

    // Integrate position
    state.ball.pos.0 += state.ball.vel.0 * dt;
    state.ball.pos.1 += state.ball.vel.1 * dt;

    // Record trail every 2 frames
    if state.frame % 2 == 0 {
        state.ball.trail.push(state.ball.pos);
        if state.ball.trail.len() > 8 {
            state.ball.trail.remove(0);
        }
    }

    // Collide with each part
    let parts_snapshot: Vec<Part> = state.parts.clone();
    for part in &parts_snapshot {
        match &part.kind {
            PartKind::Wall { width, height } => {
                collide_wall(
                    &mut state.ball,
                    part.x,
                    part.y,
                    *width,
                    *height,
                );
            }
            PartKind::Ramp => {
                collide_ramp(&mut state.ball, part.x, part.y, part.flipped);
            }
            PartKind::Basket => {
                collide_basket(&mut state.ball, part.x, part.y, &mut state.won);
            }
            PartKind::Ball => {
                // Treat placed balls as circular obstacles
                let bcx = part.x + BALL_RADIUS;
                let bcy = part.y + BALL_RADIUS;
                collide_circle(&mut state.ball, bcx, bcy, BALL_RADIUS);
            }
            PartKind::Cannon { .. } => {
                // Treat cannon body as AABB
                collide_wall(&mut state.ball, part.x + 18.0, part.y + 12.0, 60.0, 40.0);
            }
        }
    }

    // World bounds
    let bx = &mut state.ball.pos.0;
    let by = &mut state.ball.pos.1;
    let vx = &mut state.ball.vel.0;
    let vy = &mut state.ball.vel.1;

    if *bx - BALL_RADIUS < 0.0 {
        *bx = BALL_RADIUS;
        *vx = -(*vx) * RESTITUTION;
        *vy *= FRICTION;
    }
    if *bx + BALL_RADIUS > PLAYFIELD_W {
        *bx = PLAYFIELD_W - BALL_RADIUS;
        *vx = -(*vx) * RESTITUTION;
        *vy *= FRICTION;
    }
    if *by - BALL_RADIUS < 0.0 {
        *by = BALL_RADIUS;
        *vy = -(*vy) * RESTITUTION;
        *vx *= FRICTION;
    }
    if *by + BALL_RADIUS > PLAYFIELD_H {
        *by = PLAYFIELD_H - BALL_RADIUS;
        *vy = -(*vy) * RESTITUTION;
        *vx *= FRICTION;
    }

    // Deactivate ball if at rest on floor
    let speed = (state.ball.vel.0 * state.ball.vel.0 + state.ball.vel.1 * state.ball.vel.1).sqrt();
    if speed < 1.0 && state.ball.pos.1 > PLAYFIELD_H - BALL_RADIUS - 1.0 {
        info!("Ball deactivated (at rest): pos=({:.0},{:.0}), speed={:.2}",
            state.ball.pos.0, state.ball.pos.1, speed);
        state.ball.active = false;
    }
}

fn collide_wall(ball: &mut SimBall, rx: f32, ry: f32, rw: f32, rh: f32) {
    // AABB circle-vs-rect collision
    let closest_x = ball.pos.0.clamp(rx, rx + rw);
    let closest_y = ball.pos.1.clamp(ry, ry + rh);

    let dx = ball.pos.0 - closest_x;
    let dy = ball.pos.1 - closest_y;
    let dist_sq = dx * dx + dy * dy;

    if dist_sq < BALL_RADIUS * BALL_RADIUS && dist_sq > 0.0 {
        let dist = dist_sq.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;
        let penetration = BALL_RADIUS - dist;

        // Push out
        ball.pos.0 += nx * penetration;
        ball.pos.1 += ny * penetration;

        // Reflect velocity
        let dot = ball.vel.0 * nx + ball.vel.1 * ny;
        if dot < 0.0 {
            ball.vel.0 -= (1.0 + RESTITUTION) * dot * nx;
            ball.vel.1 -= (1.0 + RESTITUTION) * dot * ny;

            // Apply friction to tangent
            let tx = -ny;
            let ty = nx;
            let tang_dot = ball.vel.0 * tx + ball.vel.1 * ty;
            ball.vel.0 -= tang_dot * (1.0 - FRICTION) * tx;
            ball.vel.1 -= tang_dot * (1.0 - FRICTION) * ty;
        }
    }
}

fn collide_ramp(ball: &mut SimBall, rx: f32, ry: f32, flipped: bool) {
    let rw: f32 = 64.0;
    let rh: f32 = 32.0;

    // Check if ball is within ramp AABB (with some margin)
    if ball.pos.0 + BALL_RADIUS < rx || ball.pos.0 - BALL_RADIUS > rx + rw {
        return;
    }
    if ball.pos.1 + BALL_RADIUS < ry || ball.pos.1 - BALL_RADIUS > ry + rh {
        return;
    }

    // Also collide with bottom edge as a wall
    collide_wall(ball, rx, ry + rh - 2.0, rw, 2.0);

    // Line equation for hypotenuse
    // Not flipped (slope-right /): from (rx, ry+rh) to (rx+rw, ry)
    //   normal pointing up-left: (rh, rw) normalized
    // Flipped (slope-left \): from (rx, ry) to (rx+rw, ry+rh)
    //   normal pointing up-right: (rh, -rw) normalized

    let (x0, y0, x1, y1) = if !flipped {
        (rx, ry + rh, rx + rw, ry)
    } else {
        (rx, ry, rx + rw, ry + rh)
    };

    let edge_dx = x1 - x0;
    let edge_dy = y1 - y0;
    let len = (edge_dx * edge_dx + edge_dy * edge_dy).sqrt();

    // Normal (pointing outward from the filled side, which is "above" the line for /)
    let (nx, ny) = if !flipped {
        (-edge_dy / len, edge_dx / len) // Points up-left
    } else {
        (-edge_dy / len, edge_dx / len) // Points up-right
    };

    // Signed distance from ball center to line
    let to_ball_x = ball.pos.0 - x0;
    let to_ball_y = ball.pos.1 - y0;
    let signed_dist = to_ball_x * nx + to_ball_y * ny;

    if signed_dist < BALL_RADIUS && signed_dist > -BALL_RADIUS * 2.0 {
        // Check if projection is within the line segment
        let along = to_ball_x * edge_dx / len + to_ball_y * edge_dy / len;
        if along >= -BALL_RADIUS && along <= len + BALL_RADIUS {
            let penetration = BALL_RADIUS - signed_dist;
            if penetration > 0.0 {
                // Push out along normal
                ball.pos.0 += nx * penetration;
                ball.pos.1 += ny * penetration;

                // Reflect velocity about normal
                let dot = ball.vel.0 * nx + ball.vel.1 * ny;
                if dot < 0.0 {
                    ball.vel.0 -= (1.0 + RESTITUTION) * dot * nx;
                    ball.vel.1 -= (1.0 + RESTITUTION) * dot * ny;

                    // Friction on tangent
                    let tx = -ny;
                    let ty = nx;
                    let tang_dot = ball.vel.0 * tx + ball.vel.1 * ty;
                    ball.vel.0 -= tang_dot * (1.0 - FRICTION) * tx;
                    ball.vel.1 -= tang_dot * (1.0 - FRICTION) * ty;
                }
            }
        }
    }
}

fn collide_basket(ball: &mut SimBall, bx: f32, by: f32, won: &mut bool) {
    let bw: f32 = 64.0;
    let bh: f32 = 64.0;
    let inset: f32 = 8.0;

    // Interior zone (open top)
    let inner_left = bx + inset;
    let inner_right = bx + bw - inset;
    let inner_bottom = by + bh - inset;

    if ball.pos.0 > inner_left
        && ball.pos.0 < inner_right
        && ball.pos.1 > by
        && ball.pos.1 < inner_bottom
        && ball.vel.1 > 0.0
    {
        *won = true;
        info!("WIN! Ball entered basket at ({:.0},{:.0}), vel=({:.0},{:.0})",
            ball.pos.0, ball.pos.1, ball.vel.0, ball.vel.1);
        return;
    }

    // Collide with basket walls (left, right, bottom)
    collide_wall(ball, bx, by, inset, bh); // left wall
    collide_wall(ball, bx + bw - inset, by, inset, bh); // right wall
    collide_wall(ball, bx, by + bh - inset, bw, inset); // bottom
}

fn collide_circle(ball: &mut SimBall, cx: f32, cy: f32, radius: f32) {
    let dx = ball.pos.0 - cx;
    let dy = ball.pos.1 - cy;
    let dist_sq = dx * dx + dy * dy;
    let min_dist = BALL_RADIUS + radius;

    if dist_sq < min_dist * min_dist && dist_sq > 0.0 {
        let dist = dist_sq.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;
        let penetration = min_dist - dist;

        ball.pos.0 += nx * penetration;
        ball.pos.1 += ny * penetration;

        let dot = ball.vel.0 * nx + ball.vel.1 * ny;
        if dot < 0.0 {
            ball.vel.0 -= (1.0 + RESTITUTION) * dot * nx;
            ball.vel.1 -= (1.0 + RESTITUTION) * dot * ny;

            let tx = -ny;
            let ty = nx;
            let tang_dot = ball.vel.0 * tx + ball.vel.1 * ty;
            ball.vel.0 -= tang_dot * (1.0 - FRICTION) * tx;
            ball.vel.1 -= tang_dot * (1.0 - FRICTION) * ty;
        }
    }
}
