# Skill: feature-developer

**Purpose:** Implement code based on a technical design document. Writes Rust code, follows existing patterns, ensures compilation, and maintains dual-renderer parity. This is the third phase of the SDLC pipeline.

---

## Inputs

- **Design path**: Path to the technical design (e.g., `docs/designs/01-physics-engine.md`)
- **Task scope** (optional): Specific task number(s) from the plan to implement (e.g., "tasks 1-3")

---

## Process

### 1. Load Context

- Read the technical design document
- Read the implementation plan for task ordering and acceptance criteria
- Read every existing file listed in the design's "File Changes Summary"
- Check `docs/decisions/` and `docs/patterns/` for relevant conventions

### 2. Implement Task by Task

Follow the task order from the plan. For each task:

#### a. Pre-implementation Check
- Re-read the target files (they may have changed since design)
- Verify the design's assumptions still hold
- Check that upstream task dependencies are complete

#### b. Write Code
- Follow the design's type signatures and module structure exactly
- Match existing code style (check indentation, naming, comment patterns in surrounding code)
- Add the minimum code needed — no speculative features

#### c. Compile Check
- Run `cargo check` in `mvp/` after each significant change
- Fix compilation errors immediately before moving on
- If a design assumption was wrong, note it and adapt (don't blindly follow a broken design)

#### d. Integration Wiring
After implementing a module, wire it into the game loop:
- Add `mod` declarations in appropriate parent modules
- Add `use` imports where needed
- Integrate into `main.rs` game loop if it needs per-frame updates
- Add to `input.rs` if it needs keyboard handling
- Add to both renderers if it has visual elements

### 3. Verify Dual Renderer Parity

For any visual changes:
- Implement PixelRenderer path (image/imageproc drawing on 640×360 canvas)
- Implement TextRenderer path (ratatui widgets on terminal cells)
- Both must convey the same game information, though visual fidelity will differ

### 4. Final Compilation & Smoke Test

- Run `cargo build --release` in `mvp/`
- Run `cargo clippy` and address warnings (not pedantic, just default)
- Verify no regressions in existing functionality by reading through changes

---

## Coding Standards (TIM2-Specific)

### File Organization
```
mvp/src/
├── main.rs          # Game loop, CLI args — add new systems to the loop here
├── state.rs         # GameState, Part, PartKind — extend enums/structs here
├── input.rs         # All input handling — add new keybinds here
├── physics.rs       # Physics update — add collision functions here
├── hud.rs           # HUD text generation
├── puzzle.rs        # Puzzle definitions
├── logging.rs       # Logging infrastructure
├── render/
│   ├── mod.rs       # Renderer trait — add new trait methods here
│   ├── pixel.rs     # PixelRenderer impl
│   ├── pixel_gfx.rs # Drawing helpers for pixel renderer
│   └── text.rs      # TextRenderer impl
└── parts/
    ├── mod.rs        # Shared part helpers (AABB, dimensions)
    └── [part].rs     # Per-part rendering + collision
```

### Patterns to Follow

1. **New PartKind variant**: Add to enum in `state.rs`, then add match arms in:
   - `PartKind::label()`, `icon_char()`, `width()`, `height()`
   - `physics.rs` collision handling
   - `parts/mod.rs` rendering dispatch
   - Create new `parts/[name].rs` for rendering + collision specifics

2. **Collision functions**: Follow the `collide_wall` / `collide_ramp` pattern:
   - Take `&mut SimBall` plus shape parameters
   - Use BALL_RADIUS constant
   - Apply RESTITUTION and FRICTION on bounce
   - Push ball out of penetration before reflecting velocity

3. **Pixel rendering**: Use `imageproc` drawing functions:
   - `draw_filled_rect_mut`, `draw_filled_circle_mut` for shapes
   - Work on the 640×360 `RgbaImage` buffer
   - Colors as `Rgba<u8>` values

4. **Text rendering**: Use ratatui:
   - Unicode box-drawing and block characters
   - Map pixel coordinates to terminal cells (divide by cell size)
   - Use `canvas.set()` or `Paragraph` widgets

### What NOT to Do

- Don't refactor existing code unless the design explicitly calls for it
- Don't add error handling beyond what exists (the codebase uses `unwrap` / `expect` liberally for MVP)
- Don't add comments explaining obvious code
- Don't create abstractions for single-use logic
- Don't change the game loop structure unless the design requires it
- Don't add dependencies to `Cargo.toml` unless the design specifies them

---

## Output

After implementation:
1. List all files created/modified with a brief description of changes
2. Report any deviations from the design (and why)
3. Report `cargo check` / `cargo clippy` results
4. Note any acceptance criteria from the plan that need manual/visual verification

---

## Integration with Other Skills

- **feature-designer**: Provides the design blueprint
- **feature-tester**: Takes the implemented code and writes tests for it
- **extract-pattern**: If you created a new reusable pattern during implementation
- **record-architectural-decision**: If you deviated from the design or made unplanned choices
- **update-skill**: If the design or plan had issues that better skill instructions would prevent
