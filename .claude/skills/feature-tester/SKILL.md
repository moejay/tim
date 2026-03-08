# Skill: feature-tester

**Purpose:** Write and run tests for implemented features. Creates unit tests, integration tests, and property-based tests appropriate to the feature. This is the fourth phase of the SDLC pipeline.

---

## Inputs

- **Design path**: Path to the technical design for reference
- **Plan path**: Path to the implementation plan (for acceptance criteria)
- **Scope** (optional): Specific modules/functions to test

---

## Process

### 1. Analyze What to Test

- Read the implementation plan's acceptance criteria — each criterion becomes at least one test
- Read the design's module interfaces — each public function gets tests
- Read the implemented code to understand edge cases and invariants
- Prioritize: physics/collision > state transitions > rendering helpers > integration

### 2. Test Strategy by Module Type

#### Physics & Collision Tests (HIGH PRIORITY)
These are the most valuable tests for a game engine.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ball_falls_under_gravity() {
        // Setup: ball at known position with zero velocity
        // Act: run one physics step
        // Assert: ball moved downward by expected amount
    }

    #[test]
    fn ball_bounces_off_wall() {
        // Setup: ball moving toward wall
        // Act: run physics until collision
        // Assert: ball reflected, energy reduced by RESTITUTION
    }
}
```

Test patterns for physics:
- **Determinism**: Same initial state → same result after N steps
- **Energy conservation**: Ball energy after bounce = initial × RESTITUTION²
- **Boundary conditions**: Ball at exact edge, ball at corner, ball at zero velocity
- **Tunneling prevention**: Fast ball doesn't pass through thin wall

#### State & Game Logic Tests (MEDIUM PRIORITY)
```rust
#[test]
fn win_condition_triggers_when_ball_enters_basket() { ... }

#[test]
fn undo_restores_previous_parts_state() { ... }

#[test]
fn parts_overlap_detection_works() { ... }
```

#### Part Dimensions & Properties (LOW PRIORITY but easy)
```rust
#[test]
fn all_parts_have_positive_dimensions() {
    // Iterate PartKind variants, check width/height > 0
}

#[test]
fn flipped_wall_swaps_dimensions() { ... }
```

#### Rendering Tests (SKIP unless design adds complex rendering logic)
- Rendering is visual — prefer manual verification
- Only test rendering helper functions (coordinate transforms, color calculations)
- Don't test actual pixel/text output

### 3. Write Tests

#### Location Convention
- **Unit tests**: `#[cfg(test)] mod tests` at the bottom of the source file
- **Integration tests**: `mvp/tests/[feature].rs` for cross-module tests
- **Test helpers**: `mvp/tests/common/mod.rs` for shared setup

#### Test Naming Convention
```rust
#[test]
fn [module]_[scenario]_[expected_outcome]() { ... }

// Examples:
fn physics_ball_at_rest_on_floor_deactivates() { ... }
fn collision_circle_vs_wall_reflects_velocity() { ... }
fn state_push_undo_stores_current_parts() { ... }
```

#### Test Helpers to Create

```rust
// mvp/tests/common/mod.rs (or inline)
fn make_test_state() -> GameState {
    // Minimal GameState for testing — empty parts, centered cursor
    GameState::new(vec![], vec![])
}

fn make_ball_at(x: f32, y: f32, vx: f32, vy: f32) -> SimBall {
    SimBall {
        pos: (x, y),
        vel: (vx, vy),
        active: true,
        trail: Vec::new(),
    }
}

fn run_physics_steps(state: &mut GameState, steps: usize, dt: f32) {
    for _ in 0..steps {
        update_physics(state, dt);
        state.frame += 1;
    }
}
```

### 4. Run Tests

```bash
cd mvp && cargo test
```

- Fix any compilation errors in tests
- Fix any failing tests (determine if the test or the code is wrong)
- Run `cargo test -- --nocapture` to see println debug output if needed

### 5. Coverage Assessment

For each acceptance criterion from the plan:
- **Covered**: Test directly validates this criterion
- **Partially covered**: Test covers related behavior but not the exact criterion
- **Not covered**: Needs a test or is only verifiable manually (visual, interactive)

---

## Test Quality Guidelines

### Do
- Test behavior, not implementation details
- Use descriptive assertion messages: `assert!(speed < 1.0, "Ball should decelerate, got speed={speed}")`
- Test edge cases: zero values, negative values, boundary positions, maximum speeds
- Keep tests independent — no shared mutable state between tests
- Use `f32` approximate comparisons: `assert!((actual - expected).abs() < EPSILON)`

### Don't
- Don't test private functions directly — test through public interfaces
- Don't mock the renderer — it's not worth the complexity for MVP
- Don't test exact pixel positions in physics (floating point accumulation) — use ranges
- Don't write tests that depend on frame count or timing
- Don't add test dependencies to Cargo.toml unless absolutely necessary

### Float Comparison Helper
```rust
fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

const PHYSICS_EPSILON: f32 = 0.1;  // Physics values are approximate
const POSITION_EPSILON: f32 = 1.0; // Position within 1 pixel is fine
```

---

## Output

After testing:
1. Report test results (pass/fail counts)
2. List acceptance criteria coverage (covered / partial / manual-only)
3. Note any bugs found during testing
4. Suggest additional tests if critical paths are uncovered

---

## Integration with Other Skills

- **feature-developer**: Provides the code to test
- **feature-planner**: Provides acceptance criteria
- **extract-pattern**: Testing patterns (setup helpers, assertion patterns) are reusable
- **surface-gap**: If tests reveal spec ambiguities or undefined behavior, surface them
