use crate::parts::GravityResponse;
use crate::world::World;

/// Base gravity in pixels/second².
const BASE_GRAVITY: f32 = 980.0;

/// Velocity below which an object is considered at rest.
const REST_THRESHOLD: f32 = 0.5;

/// Run one physics tick (fixed timestep).
pub fn tick(world: &mut World, dt: f32) {
    let gravity = world.gravity;

    // Collect static AABBs for collision (indices of static parts).
    let statics: Vec<usize> = world
        .instances
        .iter()
        .enumerate()
        .filter(|(_, i)| i.def().physics().is_static)
        .map(|(idx, _)| idx)
        .collect();

    let len = world.instances.len();
    for i in 0..len {
        let phys = world.instances[i].def().physics();
        if phys.is_static {
            continue;
        }

        // --- Apply gravity ---
        let grav_accel = match phys.gravity_response {
            GravityResponse::AlwaysFalls => BASE_GRAVITY,
            GravityResponse::Normal => BASE_GRAVITY * gravity,
            GravityResponse::Buoyant => -BASE_GRAVITY * gravity * 0.3,
            GravityResponse::ZeroGravity => 0.0,
            GravityResponse::Custom(f) => BASE_GRAVITY * gravity * f,
        };
        world.instances[i].vy += grav_accel * dt;

        // --- Integrate position ---
        let vx = world.instances[i].vx;
        let vy = world.instances[i].vy;
        world.instances[i].x += vx * dt;
        world.instances[i].y += vy * dt;

        // --- Collision with static parts (simple AABB) ---
        let (ax1, ay1, ax2, ay2) = world.instances[i].bounds();

        for &si in &statics {
            if si == i {
                continue;
            }
            let (bx1, by1, bx2, by2) = world.instances[si].bounds();

            // Check overlap
            if ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1 {
                // Resolve: find smallest penetration axis
                let overlap_left = ax2 - bx1;
                let overlap_right = bx2 - ax1;
                let overlap_top = ay2 - by1;
                let overlap_bottom = by2 - ay1;

                let min_overlap = overlap_left
                    .min(overlap_right)
                    .min(overlap_top)
                    .min(overlap_bottom);

                let elasticity = phys.elasticity;

                if min_overlap == overlap_top && vy > 0.0 {
                    // Landing on top of static
                    world.instances[i].y = by1 - world.instances[i].props.height;
                    if vy.abs() < REST_THRESHOLD * 20.0 {
                        world.instances[i].vy = 0.0;
                    } else {
                        world.instances[i].vy = -vy * elasticity;
                    }
                    // Friction slows horizontal movement
                    world.instances[i].vx *= 1.0 - phys.friction * 0.1;
                } else if min_overlap == overlap_bottom && vy < 0.0 {
                    // Hitting from below
                    world.instances[i].y = by2;
                    world.instances[i].vy = -vy * elasticity;
                } else if min_overlap == overlap_left && vx > 0.0 {
                    // Hitting from left
                    world.instances[i].x = bx1 - world.instances[i].props.width;
                    world.instances[i].vx = -vx * elasticity;
                } else if min_overlap == overlap_right && vx < 0.0 {
                    // Hitting from right
                    world.instances[i].x = bx2;
                    world.instances[i].vx = -vx * elasticity;
                }
            }
        }

        // --- World bounds (floor at canvas bottom, ceiling at top) ---
        let canvas_h = crate::constants::CANVAS_H as f32;
        let canvas_w = crate::constants::CANVAS_W as f32;
        let inst = &mut world.instances[i];

        // Mark as exited if off-screen (don't clamp — let puzzle system detect)
        // But do clamp at sides for now to keep things visible
        if inst.x < 0.0 {
            inst.x = 0.0;
            inst.vx = -inst.vx * phys.elasticity;
        }
        if inst.x + inst.props.width > canvas_w {
            inst.x = canvas_w - inst.props.width;
            inst.vx = -inst.vx * phys.elasticity;
        }
        // Floor
        if inst.y + inst.props.height > canvas_h {
            inst.y = canvas_h - inst.props.height;
            if inst.vy.abs() < REST_THRESHOLD * 20.0 {
                inst.vy = 0.0;
            } else {
                inst.vy = -inst.vy * phys.elasticity;
            }
            inst.vx *= 1.0 - phys.friction * 0.1;
        }
        // Ceiling
        if inst.y < 0.0 {
            inst.y = 0.0;
            inst.vy = -inst.vy * phys.elasticity;
        }

        // --- Rest detection: dampen tiny velocities ---
        if inst.vx.abs() < REST_THRESHOLD {
            inst.vx = 0.0;
        }
        if inst.vy.abs() < REST_THRESHOLD {
            inst.vy = 0.0;
        }

        // --- Update state (basic: moving vs at rest) ---
        let speed = (inst.vx * inst.vx + inst.vy * inst.vy).sqrt();
        let states = inst.def().states();
        if states.len() >= 3 {
            // Convention for balls: 0=Idle, 1=Moving, 2=AtRest
            if speed > REST_THRESHOLD * 2.0 {
                inst.props.current_state = 1; // Moving
            } else if inst.props.current_state == 1 {
                inst.props.current_state = 2; // AtRest (was moving, now stopped)
            }
        }
    }
}
