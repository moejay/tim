# TIM2 Terminal — MVP Specification

## Goal

A single playable puzzle that proves out the full game loop in the terminal:
place parts, run the simulation, watch the chain reaction, win.

Not a feature-complete game. Every decision here optimizes for the shortest
path to that loop working end-to-end.

---

## Stack (inherits from sixel-poc, no changes)

| Layer | Choice |
|---|---|
| Canvas | 640 x 360 px `RgbaImage` |
| Display | `viuer 0.11` — auto Kitty / Sixel / half-block |
| Input | `crossterm 0.28` raw mode, non-blocking poll |
| Game loop | Raw Rust, fixed 60 fps cap |
| Physics | Manual Euler integration (Rapier2D deferred post-MVP) |
| Drawing | `gfx.rs` primitives from sixel-poc |
| ECS | None — plain `Vec<Part>` state struct |

---

## Canvas Layout

```
+------------------------------------------------------------------+------------------+
|                                                                  |                  |
|                    PLAYFIELD                                     |   PARTS BIN      |
|                   pixel canvas                                   |   (pixel panel)  |
|                   512 x 360 px                                   |   128 x 360 px   |
|                                                                  |                  |
+------------------------------------------------------------------+------------------+
|  HUD: mode indicator + keybinding hints (2 text rows via crossterm)                |
+------------------------------------------------------------------------------------+
```

- Playfield: x 0..512, y 0..360 (pixel coords)
- Parts bin: x 512..640, y 0..360 (drawn in pixels, part list + selection highlight)
- HUD: 2 terminal text rows below the image
- `viuer` renders the full 640x360 image; HUD text is written via `crossterm` afterward

---

## Grid System

Parts snap to a **32x32 pixel grid** on the playfield.

- Grid columns: 512 / 32 = **16 cols** (0–15)
- Grid rows:    360 / 32 = **11 rows** (0–10, last row is partial — use 0–10)
- A cursor occupies one grid cell. Parts may span multiple cells (stored as
  top-left cell + size in cells).
- Pixel position of grid cell (col, row): `(col * 32, row * 32)`
- Physics runs in pixel space. Grid is placement-only.

---

## Vim-style Input Modes

The game has four modes. Current mode is always shown in the HUD.

```
NORMAL --> [p]       --> PLACE
NORMAL --> [Space]   --> RUN
RUN    --> [Esc]     --> NORMAL
PLACE  --> [Esc]     --> NORMAL
PLACE  --> [Enter]   --> NORMAL (places part)
NORMAL --> [q]       --> quit
```

### NORMAL mode

Navigate the playfield cursor and manage placed parts.

| Key | Action |
|---|---|
| `h` / `l` | cursor left / right |
| `j` / `k` | cursor down / up |
| `p` | enter PLACE mode |
| `Space` | start simulation (enter RUN mode) |
| `x` | delete part under cursor |
| `f` | flip part under cursor (horizontal mirror) |
| `u` | undo last action |
| `?` | toggle help overlay |
| `q` | quit |

### PLACE mode

Select a part from the bin, move cursor to target cell, confirm.

Entering PLACE mode auto-focuses the first part in the bin. The bin highlights
the selected part.

| Key | Action |
|---|---|
| `h` / `l` | cursor left / right on playfield |
| `j` / `k` | cursor down / up on playfield |
| `J` / `K` | scroll bin selection down / up |
| `1`–`5` | jump directly to bin slot 1–5 |
| `Enter` | place selected part at cursor; return to NORMAL |
| `Esc` | cancel; return to NORMAL |

### RUN mode

Simulation is running. Cursor hidden.

| Key | Action |
|---|---|
| `Esc` or `Space` | stop simulation; return to NORMAL (parts reset to placed positions) |

---

## MVP Parts (5 parts)

Only these five are implemented for the MVP. Enough for at least one interesting
puzzle.

### 1. Ball

- Visual: anti-aliased sphere with specular highlight (from sixel-poc cannon.rs)
- Physics: point mass, gravity, elastic bounce off walls and surfaces
- Size: 1x1 grid cell (32px diameter, 16px radius)
- Gravity response: always falls
- Properties: `pos: (f32, f32)`, `vel: (f32, f32)`, `active: bool`

### 2. Ramp (Incline)

- Visual: filled triangle / diagonal line across a grid cell
- Physics: acts as a one-sided sloped surface; ball slides down and deflects
- Size: 2x1 grid cells (64x32 px)
- Variants: slope-right (/) or slope-left (\) — toggled with `f` in NORMAL mode
- Collision: reflect ball velocity about the surface normal

### 3. Wall (Flat Surface)

- Visual: solid filled rectangle
- Physics: static rigid surface; ball bounces off all sides
- Size: resizable in 1-cell increments, default 2x1 (horizontal plank)
- Flip: `f` rotates between horizontal and vertical

### 4. Basket (Goal)

- Visual: open-top U shape (three sides of a rectangle)
- Physics: ball entering the basket opening triggers win condition
- Size: 2x2 grid cells
- Win condition: ball center is inside basket bounds and vel.y > 0 (falling in)

### 5. Cannon

- Visual: body rectangle + angled barrel (from sixel-poc cannon.rs, adapted)
- Physics: fires a ball at simulation start with configured angle and power
- Size: 3x2 grid cells
- Properties: `angle_deg: f32`, `power: f32`
- Firing: on RUN start, spawns a Ball entity at barrel tip with computed velocity
- The player does not place a separate Ball when using the Cannon — it spawns one
- Flip `f`: mirrors cannon to face left instead of right

---

## State Model

```rust
struct Part {
    kind:    PartKind,
    col:     u8,        // grid column of top-left
    row:     u8,        // grid row of top-left
    flipped: bool,
}

enum PartKind {
    Ball,
    Ramp,
    Wall { w_cells: u8, h_cells: u8 },
    Basket,
    Cannon { angle_deg: f32, power: f32 },
}

struct SimBall {
    pos: (f32, f32),
    vel: (f32, f32),
    active: bool,
}

enum Mode {
    Normal,
    Place { bin_idx: usize },
    Run,
}

struct GameState {
    parts:      Vec<Part>,
    ball:       SimBall,          // live ball during Run mode
    mode:       Mode,
    cursor:     (u8, u8),         // grid (col, row)
    undo_stack: Vec<Vec<Part>>,   // snapshots for undo
    won:        bool,
    frame:      u64,
    elapsed:    f32,
}
```

---

## Physics (Manual Euler, MVP)

Run at 60 fps with `dt = elapsed since last frame`.

```
// Gravity
ball.vel.y += GRAVITY * dt;        // GRAVITY = 900.0 px/s²

// Integrate
ball.pos.x += ball.vel.x * dt;
ball.pos.y += ball.vel.y * dt;

// Collision: for each static part, test and resolve
```

### Collision Resolution

**Ball vs Wall (axis-aligned rect):**
- AABB overlap test between ball circle and wall rect
- Find shallowest penetration axis
- Push ball out along that axis
- Reflect velocity component on that axis, multiply by `RESTITUTION = 0.72`
- Apply friction to the other component: `vel_tangent *= 0.85`

**Ball vs Ramp (slope-right `/`):**
- Surface normal: `(-1, 1).normalize()` for `/`, `(1, 1).normalize()` for `\`
- If ball center is within ramp bounding box and on the wrong side of the line:
  - Push ball to surface
  - Reflect velocity about normal, multiply by `RESTITUTION`

**Ball vs Basket:**
- Check if ball center is inside basket interior bounds
- If yes and ball is moving downward (`vel.y > 0`): `state.won = true`

**World bounds:**
- Ball bounces off playfield edges (x: 0..512, y: 0..360)
- Same AABB resolution as walls

---

## Rendering

Each frame:

1. Clear `RgbaImage` (640x360) to background `[10, 10, 14]`
2. Draw playfield background (subtle 32px grid lines, dim)
3. Draw parts bin panel background `[18, 18, 24]`
4. For each `Part` in `state.parts`: call the part's draw function
5. Draw cursor (if not RUN mode): glowing cell outline at cursor position
6. Draw live ball (if RUN mode and ball active)
7. Draw win overlay if `state.won`
8. Transmit via `viuer`
9. Write HUD text rows via `crossterm`

### Cursor Rendering

A 32x32 cell outline with a pulsing glow. Color:
- NORMAL: cyan
- PLACE: yellow (indicates snapping target)

### Parts Bin Rendering

Right panel (128x360). List of 5 parts with icons drawn in pixel art.
Selected part (in PLACE mode) gets a highlight rect behind it.
Each slot is 128x60 px, stacked vertically, plus a 60px "instructions" block
at bottom for remaining 360 - 5*60 = 60 px.

Part slots:
```
slot 0: Ball     [key: 1]
slot 1: Ramp     [key: 2]
slot 2: Wall     [key: 3]
slot 3: Basket   [key: 4]
slot 4: Cannon   [key: 5]
```

### HUD Text (2 rows below image)

Row 1: `MODE: NORMAL | cursor: (col, row) | parts placed: N`
Row 2: context-sensitive key hints for current mode

---

## The Hardcoded MVP Puzzle

To avoid building a puzzle loader for the MVP, one puzzle is hardcoded:

**Goal:** "Get the ball into the basket"

**Fixed (immovable) parts:**
- Cannon at (1, 5) facing right, angle -30 deg, power 600 px/s
- Basket at (12, 7)

**Player's inventory (parts bin):**
- 1x Ramp
- 1x Wall
- 1x Wall

**Winning sequence (one valid solution):**
- Place Wall at ~(7, 4) to redirect ball downward
- Place Ramp at ~(10, 6) to guide ball into basket

This gives the player 3 parts to work with, 2 of which are needed, 1 is a decoy.

---

## Win / Lose

**Win:** Ball enters basket. Show overlay:
```
  PUZZLE SOLVED
  [Space] Try Again   [q] Quit
```
Overlay drawn in pixels (centered on playfield).

**No lose condition** in MVP — ball just comes to rest or exits screen.
Player hits `Esc` to stop and try again.

---

## Project Structure

New crate at `game/` — standalone binary, not part of sixel-poc.

```
game/
  Cargo.toml
  src/
    main.rs       -- game loop, input, mode state machine
    state.rs      -- GameState, Part, PartKind, SimBall, Mode
    physics.rs    -- update_physics(), collision resolution
    render.rs     -- render() -> DynamicImage
    parts/
      ball.rs     -- draw_ball(), collision_ball()
      ramp.rs     -- draw_ramp(), collide_ramp()
      wall.rs     -- draw_wall(), collide_wall()
      basket.rs   -- draw_basket(), check_win()
      cannon.rs   -- draw_cannon(), spawn_ball()
    gfx.rs        -- copied/symlinked from sixel-poc (fill_circle, etc.)
    hud.rs        -- write_hud() via crossterm
    puzzle.rs     -- hardcoded MVP puzzle definition
```

---

## Out of Scope for MVP

These are explicitly deferred:

- Rapier2D (manual Euler is sufficient for 5 parts)
- Rope / pulley / belt systems
- Electrical system
- Animals (Pokey, Mort)
- Puzzle loader / saver
- Level progression
- Sound
- Undo beyond last placement (single undo is fine)
- Part resizing (Wall is fixed size for MVP)
- Workshop mode
- Head-to-head mode
- Preview arc (can add after physics is stable)
