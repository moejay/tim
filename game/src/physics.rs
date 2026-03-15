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

    // Collect dynamic indices for dynamic-to-dynamic collision.
    let dynamics: Vec<usize> = world
        .instances
        .iter()
        .enumerate()
        .filter(|(_, i)| !i.def().physics().is_static)
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

            if ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1 {
                resolve_static_collision(world, i, (bx1, by1, bx2, by2));
            }
        }

        // --- Rest detection: dampen tiny velocities ---
        let inst = &mut world.instances[i];
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
            if speed > REST_THRESHOLD * 2.0 {
                inst.props.current_state = 1; // Moving
            } else if inst.props.current_state == 1 {
                inst.props.current_state = 2; // AtRest
            }
        }
    }

    // --- Dynamic-to-dynamic collision (separate pass) ---
    for di in 0..dynamics.len() {
        for dj in (di + 1)..dynamics.len() {
            let i = dynamics[di];
            let j = dynamics[dj];
            let (ax1, ay1, ax2, ay2) = world.instances[i].bounds();
            let (bx1, by1, bx2, by2) = world.instances[j].bounds();

            if ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1 {
                resolve_dynamic_collision(world, i, j);
            }
        }
    }
}

/// Resolve collision between a dynamic object and a static AABB.
fn resolve_static_collision(world: &mut World, i: usize, static_bounds: (f32, f32, f32, f32)) {
    let phys = world.instances[i].def().physics();
    let (bx1, by1, bx2, by2) = static_bounds;
    let (ax1, ay1, ax2, ay2) = world.instances[i].bounds();
    let vx = world.instances[i].vx;
    let vy = world.instances[i].vy;
    let elasticity = phys.elasticity;

    let overlap_left = ax2 - bx1;
    let overlap_right = bx2 - ax1;
    let overlap_top = ay2 - by1;
    let overlap_bottom = by2 - ay1;

    let min_overlap = overlap_left
        .min(overlap_right)
        .min(overlap_top)
        .min(overlap_bottom);

    if min_overlap == overlap_top && vy > 0.0 {
        world.instances[i].y = by1 - world.instances[i].props.height;
        if vy.abs() < REST_THRESHOLD * 20.0 {
            world.instances[i].vy = 0.0;
        } else {
            world.instances[i].vy = -vy * elasticity;
        }
        world.instances[i].vx *= 1.0 - phys.friction * 0.1;
    } else if min_overlap == overlap_bottom && vy < 0.0 {
        world.instances[i].y = by2;
        world.instances[i].vy = -vy * elasticity;
    } else if min_overlap == overlap_left && vx > 0.0 {
        world.instances[i].x = bx1 - world.instances[i].props.width;
        world.instances[i].vx = -vx * elasticity;
    } else if min_overlap == overlap_right && vx < 0.0 {
        world.instances[i].x = bx2;
        world.instances[i].vx = -vx * elasticity;
    }
}

/// Resolve collision between two dynamic objects using impulse-based response.
fn resolve_dynamic_collision(world: &mut World, i: usize, j: usize) {
    let phys_i = world.instances[i].def().physics();
    let phys_j = world.instances[j].def().physics();

    // Centers
    let (ax1, ay1, ax2, ay2) = world.instances[i].bounds();
    let (bx1, by1, bx2, by2) = world.instances[j].bounds();
    let cx_i = (ax1 + ax2) / 2.0;
    let cy_i = (ay1 + ay2) / 2.0;
    let cx_j = (bx1 + bx2) / 2.0;
    let cy_j = (by1 + by2) / 2.0;

    // Normal vector from i to j
    let nx = cx_j - cx_i;
    let ny = cy_j - cy_i;
    let dist = (nx * nx + ny * ny).sqrt();
    if dist < 0.001 {
        return;
    }
    let nx = nx / dist;
    let ny = ny / dist;

    // Relative velocity of i with respect to j
    let dvx = world.instances[i].vx - world.instances[j].vx;
    let dvy = world.instances[i].vy - world.instances[j].vy;

    // Relative velocity along normal
    let rel_vel_normal = dvx * nx + dvy * ny;

    // Only resolve if objects are approaching
    if rel_vel_normal > 0.0 {
        return;
    }

    // Elasticity: use average
    let e = (phys_i.elasticity + phys_j.elasticity) / 2.0;

    // Impulse scalar (using masses)
    let mass_i = phys_i.mass;
    let mass_j = phys_j.mass;
    let inv_mass_i = if mass_i.is_infinite() { 0.0 } else { 1.0 / mass_i };
    let inv_mass_j = if mass_j.is_infinite() { 0.0 } else { 1.0 / mass_j };

    if inv_mass_i + inv_mass_j == 0.0 {
        return;
    }

    let impulse = -(1.0 + e) * rel_vel_normal / (inv_mass_i + inv_mass_j);

    // Apply impulse
    world.instances[i].vx += impulse * inv_mass_i * nx;
    world.instances[i].vy += impulse * inv_mass_i * ny;
    world.instances[j].vx -= impulse * inv_mass_j * nx;
    world.instances[j].vy -= impulse * inv_mass_j * ny;

    // Separate overlapping objects
    let overlap_x = (ax2 - bx1).min(bx2 - ax1);
    let overlap_y = (ay2 - by1).min(by2 - ay1);
    let overlap = overlap_x.min(overlap_y);
    let total_inv_mass = inv_mass_i + inv_mass_j;
    if total_inv_mass > 0.0 {
        let sep = overlap * 0.5;
        world.instances[i].x -= nx * sep * (inv_mass_i / total_inv_mass);
        world.instances[i].y -= ny * sep * (inv_mass_i / total_inv_mass);
        world.instances[j].x += nx * sep * (inv_mass_j / total_inv_mass);
        world.instances[j].y += ny * sep * (inv_mass_j / total_inv_mass);
    }
}
