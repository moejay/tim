use crate::parts::{ball, basket, ramp, wall};
use crate::state::*;

const GRAVITY: f32 = 900.0;

pub fn update_physics(state: &mut GameState, dt: f32) {
    if !state.ball.active {
        return;
    }

    state.ball.vel.1 += GRAVITY * dt;

    state.ball.pos.0 += state.ball.vel.0 * dt;
    state.ball.pos.1 += state.ball.vel.1 * dt;

    let r = ball::ball_radius();

    let pw = PLAYFIELD_W as f32;
    let ph = CANVAS_H as f32;

    if state.ball.pos.0 - r < 0.0 {
        state.ball.pos.0 = r;
        state.ball.vel.0 = state.ball.vel.0.abs() * 0.72;
        state.ball.vel.1 *= 0.85;
    }
    if state.ball.pos.0 + r > pw {
        state.ball.pos.0 = pw - r;
        state.ball.vel.0 = -state.ball.vel.0.abs() * 0.72;
        state.ball.vel.1 *= 0.85;
    }
    if state.ball.pos.1 - r < 0.0 {
        state.ball.pos.1 = r;
        state.ball.vel.1 = state.ball.vel.1.abs() * 0.72;
        state.ball.vel.0 *= 0.85;
    }
    if state.ball.pos.1 + r > ph {
        state.ball.pos.1 = ph - r;
        state.ball.vel.1 = -state.ball.vel.1.abs() * 0.72;
        state.ball.vel.0 *= 0.85;
    }

    let parts_snapshot: Vec<_> = state.parts.iter().cloned().collect();
    for part in &parts_snapshot {
        match &part.kind {
            PartKind::Wall { .. } => {
                wall::collide_wall(&mut state.ball, part);
            }
            PartKind::Ramp => {
                ramp::collide_ramp(&mut state.ball, part);
            }
            PartKind::Basket => {
                if basket::check_win(&state.ball, part) {
                    state.won = true;
                }
                basket::collide_basket(&mut state.ball, part);
            }
            _ => {}
        }
    }
}
