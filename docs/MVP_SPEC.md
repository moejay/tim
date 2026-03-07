# TIM2 Terminal — MVP Specification

## Goal

A single playable puzzle that proves out the full game loop in the terminal:
place parts, run the simulation, watch the chain reaction, win.

Not a feature-complete game. Every decision here optimizes for the shortest
path to that loop working end-to-end.

---

## Terminal Requirements

The game renders a 640x360 pixel image every frame via `viuer`, which auto-detects
the best graphics protocol the terminal supports. Visual fidelity depends entirely
on which protocol is available.

### Full pixel fidelity (recommended)

| Terminal | Protocol | Notes |
|---|---|---|
| **WezTerm** | Kitty Graphics Protocol | Primary dev/test target |
| **kitty** | Kitty Graphics Protocol | Fastest; native support |
| **Ghostty** | Kitty Graphics Protocol | Full pixel |
| **foot** | Sixel | Full pixel |
| **xterm** (with `+sixel`) | Sixel | Must be built with Sixel support |
| **iTerm2** | iTerm2 inline images | macOS |

### Degraded experience (half-block fallback)

| Terminal | Issue |
|---|---|
| **Alacritty** | No pixel graphics protocol support. Falls back to Unicode half-block characters (`▀▄█`). Resolution drops to ~80x45 pseudo-pixels in an 80-column terminal — 32px grid cells become ~4 chars wide. Parts are recognizable but blocky; anti-aliased effects are lost. |
| **Windows Terminal** | Same half-block fallback. |
| **Basic/legacy terminals** | Any terminal without Kitty/Sixel/iTerm2 support. |

### Untested

| Terminal | Notes |
|---|---|
| **tmux** | Sixel passthrough works in recent versions (`set -g allow-passthrough on`) but frame rate and flicker are untested. |
| **screen** | Unlikely to work well with pixel protocols. |
| **SSH sessions** | Depends on the local terminal, not the remote. Should work if the local terminal supports Kitty/Sixel. |

### Startup protocol detection

On launch, the game prints which `viuer` protocol was selected before entering
the game loop:

```
TIM2 Terminal — Protocol: Kitty Graphics
```

If the detected protocol is half-block, an additional warning is shown:

```
WARNING: Your terminal does not support Kitty/Sixel/iTerm2 graphics.
         Rendering will use Unicode half-block fallback (lower fidelity).
         For the best experience, use WezTerm, kitty, or Ghostty.
         Press any key to continue, or 'q' to quit.
```

This lets the player make an informed choice before committing to a session.

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

## Placement System

Parts are placed at **free pixel coordinates** on the playfield. No grid snapping.

- Parts store position as `(x: f32, y: f32)` — top-left corner in pixel space.
- The cursor is a crosshair at `(f32, f32)` pixel coordinates.
- Cursor moves in 4px steps (normal) or 16px steps (Shift / fast).
- Parts are placed centered on the cursor position.
- **Overlap rejection:** parts cannot overlap. Placement and movement are rejected
  if the new position would cause an AABB overlap with any existing part.
- Physics runs in the same pixel coordinate space — no conversion needed.

---

## Vim-style Input Modes

The game has five modes. Current mode is always shown in the HUD.

```
NORMAL --> [p]       --> PLACE
NORMAL --> [e]       --> EDIT (on part under cursor)
NORMAL --> [Space]   --> RUN
RUN    --> [Esc]     --> NORMAL
PLACE  --> [Esc]     --> NORMAL
PLACE  --> [Enter]   --> NORMAL (places part)
EDIT   --> [Esc]     --> NORMAL
EDIT   --> [Enter]   --> NORMAL
NORMAL --> [q]       --> quit
```

### NORMAL mode

Navigate the playfield cursor and manage placed parts.

| Key | Action |
|---|---|
| `h` / `l` | cursor left / right (4px) |
| `j` / `k` | cursor down / up (4px) |
| `H` / `L` | cursor left / right fast (16px) |
| `J` / `K` | cursor down / up fast (16px) |
| `p` | enter PLACE mode |
| `e` | enter EDIT mode (part under cursor) |
| `Space` | start simulation (enter RUN mode) |
| `x` | delete part under cursor |
| `f` | flip part under cursor (horizontal mirror) |
| `u` | undo last action |
| `?` | toggle help overlay |
| `q` | quit |

### PLACE mode

Select a part from the bin, move cursor to target position, confirm.

Entering PLACE mode auto-focuses the first part in the bin. The bin highlights
the selected part. A ghost outline of the selected part is shown centered on
the cursor. Placement is rejected if it would overlap an existing part.

| Key | Action |
|---|---|
| `h` / `l` | cursor left / right (4px) |
| `j` / `k` | cursor down / up (4px) |
| `H` / `L` | cursor left / right fast (16px) |
| `J` / `K` | scroll bin selection down / up |
| `1`–`5` | jump directly to bin slot 1–5 |
| `Enter` | place selected part at cursor; return to NORMAL |
| `Esc` | cancel; return to NORMAL |

### EDIT mode

Move and manipulate a placed part in-place. Entered by pressing `e` in NORMAL
mode with the cursor over a non-fixed part. An undo snapshot is taken on entry.
The part is highlighted with a pulsing green outline.

Movement is rejected if it would cause overlap with another part.

| Key | Action |
|---|---|
| `h` / `l` | move part left / right (4px) |
| `j` / `k` | move part down / up (4px) |
| `H` / `L` | move part left / right fast (16px) |
| `J` / `K` | move part down / up fast (16px) |
| `f` | flip part |
| `x` | delete part (returns to bin) |
| `Esc` or `Enter` | done editing; return to NORMAL |

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

- Visual: anti-aliased sphere with specular highlight and shading
- Physics: point mass, gravity, elastic bounce off walls and surfaces
- Size: 28x28 px (14px radius)
- Gravity response: always falls
- Properties: `pos: (f32, f32)`, `vel: (f32, f32)`, `active: bool`

### 2. Ramp (Incline)

- Visual: filled triangle
- Physics: acts as a one-sided sloped surface; ball slides down and deflects
- Size: 64x32 px
- Variants: slope-right (/) or slope-left (\) — toggled with `f`
- Collision: reflect ball velocity about the surface normal

### 3. Wall (Flat Surface)

- Visual: solid filled rectangle with highlight edges
- Physics: static rigid surface; ball bounces off all sides
- Size: default 64x32 px (horizontal plank)
- Flip: `f` swaps width and height (horizontal ↔ vertical)

### 4. Basket (Goal)

- Visual: open-top U shape (three sides of a rectangle)
- Physics: ball entering the basket opening triggers win condition
- Size: 64x64 px
- Win condition: ball center is inside basket bounds and vel.y > 0 (falling in)

### 5. Cannon

- Visual: body rectangle + angled barrel + wheels
- Physics: fires a ball at simulation start with configured angle and power
- Size: 96x64 px
- Properties: `angle_deg: f32`, `power: f32`
- Firing: on RUN start, spawns a Ball entity at barrel tip with computed velocity
- The player does not place a separate Ball when using the Cannon — it spawns one
- Flip `f`: mirrors cannon to face left instead of right

---

## State Model

```rust
struct Part {
    kind:    PartKind,
    x:       f32,        // pixel x of top-left
    y:       f32,        // pixel y of top-left
    flipped: bool,
    fixed:   bool,       // immovable puzzle fixture
}

enum PartKind {
    Ball,
    Ramp,                                    // 64x32 px
    Wall { width: f32, height: f32 },        // default 64x32 px
    Basket,                                  // 64x64 px
    Cannon { angle_deg: f32, power: f32 },   // 96x64 px
}

struct SimBall {
    pos: (f32, f32),
    vel: (f32, f32),
    active: bool,
}

enum Mode {
    Normal,
    Place { bin_idx: usize },
    Edit  { part_idx: usize },
    Run,
}

struct GameState {
    parts:      Vec<Part>,
    ball:       SimBall,            // live ball during Run mode
    mode:       Mode,
    cursor:     (f32, f32),         // pixel (x, y)
    undo_stack: Vec<Vec<Part>>,     // snapshots for undo
    won:        bool,
    frame:      u64,
    elapsed:    f32,
    bin_items:  Vec<BinItem>,
    show_help:  bool,
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
2. Draw parts bin panel background `[18, 18, 24]`
3. For each `Part` in `state.parts`: call the part's draw function
4. Draw EDIT mode highlight (pulsing green outline) if applicable
5. Draw cursor crosshair (if not RUN or EDIT mode)
6. Draw live ball (if RUN mode and ball active)
7. Draw win overlay if `state.won`
8. Transmit via `viuer`
9. Write HUD text rows via `crossterm`

### Cursor Rendering

A crosshair with a gap in the center (4 short line segments). Pulsing alpha.
- NORMAL: cyan
- PLACE: yellow, with ghost outline of selected part centered on cursor

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

Row 1: `MODE: NORMAL | cursor: (x, y) | parts: N | frame: N`
Row 2: context-sensitive key hints for current mode

---

## The Hardcoded MVP Puzzle

To avoid building a puzzle loader for the MVP, one puzzle is hardcoded:

**Goal:** "Get the ball into the basket"

**Fixed (immovable) parts:**
- Cannon at (32, 160) facing right, angle -30 deg, power 600 px/s
- Basket at (384, 224)

**Player's inventory (parts bin):**
- 1x Ramp
- 2x Wall (64x32 px)

**Winning sequence (one valid solution):**
- Place Wall at ~(224, 128) to redirect ball downward
- Place Ramp at ~(320, 192) to guide ball into basket

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
