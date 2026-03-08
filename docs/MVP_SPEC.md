# TIM2 Terminal — MVP Specification (Dual Engine)

## Goal

A single playable puzzle that proves out the full game loop in the terminal:
place parts, run the simulation, watch the chain reaction, win.

Two fully independent rendering paths — pixel and text — each purpose-built
to look excellent in its medium. Not a feature-complete game. Every decision
optimizes for the shortest path to the loop working end-to-end in both modes.

---

## Dual Renderer Architecture

The engine ships two completely independent renderers. The active renderer is
selected at startup and never changes mid-session.

```
┌─────────────────────────────────────────────────────────┐
│               Game Logic + Physics                       │
│           (pixel coordinates, renderer-agnostic)         │
├──────────────────────┬──────────────────────────────────┤
│   Pixel Renderer     │        Text Renderer             │
│  image + viuer       │   ratatui + Unicode              │
│  640×360 RgbaImage   │   Terminal cells (responsive)    │
│  Kitty/Sixel/iTerm2  │   Any ANSI terminal              │
└──────────────────────┴──────────────────────────────────┘
```

### Mode detection

```rust
fn detect_mode() -> RenderMode {
    // CLI override: --pixel or --text
    // Auto-detect:
    //   TERM=xterm-kitty || KITTY_WINDOW_ID    → Pixel
    //   TERM_PROGRAM=WezTerm                    → Pixel
    //   TERM_PROGRAM=ghostty                    → Pixel
    //   TERM_PROGRAM=iTerm.app                  → Pixel
    //   otherwise                               → Text
}
```

CLI flags `--pixel` and `--text` override auto-detection.

### Renderer trait

```rust
trait Renderer {
    fn render_frame(&mut self, state: &GameState) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;
}

struct PixelRenderer { /* viuer config, RgbaImage buffer */ }
struct TextRenderer  { /* ratatui Terminal, layout cache  */ }
```

The game loop calls `renderer.render_frame(&state)` — no branching on mode
after initialization.

### Startup banner

Pixel mode:
```
TIM2 Terminal — Renderer: Pixel (Kitty Graphics)
```

Text mode:
```
TIM2 Terminal — Renderer: Text (ratatui)
```

---

## Terminal Compatibility

| Terminal | Renderer | Fidelity |
|---|---|---|
| **kitty** | Pixel (Kitty Graphics) | Full pixel, fastest |
| **WezTerm** | Pixel (Kitty Graphics) | Full pixel |
| **Ghostty** | Pixel (Kitty Graphics) | Full pixel |
| **foot** | Pixel (Sixel) | Full pixel |
| **xterm** (+sixel) | Pixel (Sixel) | Full pixel |
| **iTerm2** | Pixel (iTerm2 inline) | Full pixel |
| **Alacritty** | Text (ratatui) | Purpose-built cell rendering |
| **Windows Terminal** | Text (ratatui) | Purpose-built cell rendering |
| **tmux** | Text (ratatui) | Safest default; pixel possible with passthrough |
| **Any ANSI terminal** | Text (ratatui) | Works everywhere |
| **SSH sessions** | Text (ratatui) | No GPU, no display server needed |

The text renderer is **not a degraded fallback**. It is a purpose-built rendering
path designed to look good on its own terms.

---

## Stack

| Layer | Choice |
|---|---|
| Terminal I/O | `crossterm 0.28` — raw mode, alternate screen, non-blocking poll |
| Pixel renderer | `image 0.25` + `imageproc 0.25` + `viuer 0.11` |
| Text renderer | `ratatui 0.29` |
| Game loop | Raw Rust, fixed 60 fps cap |
| Physics | Manual Euler integration (Rapier2D deferred post-MVP) |
| ECS | None — plain `Vec<Part>` state struct |

---

## Coordinate System

All game state uses **pixel coordinates** in a 512×360 logical playfield.
Physics, placement, and collision all operate in this space regardless of
which renderer is active.

Each renderer maps pixel coordinates to its own output space:

| | Pixel Renderer | Text Renderer |
|---|---|---|
| **Mapping** | 1:1 — pixel coords are canvas coords | `cell_x = pixel_x / cell_w`, `cell_y = pixel_y / cell_h` |
| **Cell size** | N/A | Computed from terminal size: `cell_w = term_cols / 80`, etc. |
| **Cursor steps** | 4px normal, 16px fast | Same pixel steps; display rounds to nearest cell |
| **Sub-cell precision** | Full — smooth at any speed | No — position snaps to cell boundary for display |

The text renderer computes cell dimensions from the terminal size at startup:
```
playfield_cols = terminal_cols - bin_cols
playfield_rows = terminal_rows - hud_rows
cell_w = 512.0 / playfield_cols
cell_h = 360.0 / playfield_rows
```

Minimum terminal: 80×24. The text renderer is responsive — larger terminals
get more cells and finer-grained display.

---

## Canvas Layout

### Pixel Renderer (640×360 image)

```
+──────────────────────────────────────────────────+──────────────────+
│                                                  │                  │
│                  PLAYFIELD                        │   PARTS BIN      │
│                 512 × 360 px                     │   128 × 360 px   │
│                                                  │                  │
+──────────────────────────────────────────────────+──────────────────+
│  HUD: mode + cursor pos + key hints  (2 text rows via crossterm)   │
+────────────────────────────────────────────────────────────────────+
```

- Full 640×360 `RgbaImage` transmitted via `viuer`
- HUD rendered as terminal text below the image via `crossterm`

### Text Renderer (ratatui layout)

```
┌──────────────────────────────────────────────────┬──────────────────┐
│                                                  │                  │
│                  PLAYFIELD                        │   PARTS BIN      │
│              (responsive cells)                  │  (List widget)   │
│                                                  │                  │
├──────────────────────────────────────────────────┴──────────────────┤
│  HUD: mode + cursor pos + key hints  (Paragraph widget)            │
└────────────────────────────────────────────────────────────────────┘
```

- ratatui `Layout` with horizontal split: playfield (80%) | bin (20%)
- Vertical split: game area (terminal_rows - 3) | HUD (3 rows)
- Double-buffered diffing — only changed cells are redrawn
- Playfield border: `Borders::ALL` with `DarkGray`

---

## Placement System

Parts are placed at **free pixel coordinates** on the playfield. No grid snapping.

- Parts store position as `(x: f32, y: f32)` — top-left corner in pixel space.
- The cursor is a crosshair at `(f32, f32)` pixel coordinates.
- Cursor moves in 4px steps (normal) or 16px steps (fast).
- Parts are placed centered on the cursor position.
- **Overlap rejection:** placement and movement are rejected if the new position
  would cause an AABB overlap with any existing part.
- Physics runs in the same pixel coordinate space — no conversion needed.

---

## Vim-style Input Modes

The game has four modes. Current mode is always shown in the HUD.
Input handling is identical regardless of renderer.

```
NORMAL ──[p]────→ PLACE
NORMAL ──[e]────→ EDIT  (part under cursor)
NORMAL ──[Space]→ RUN
RUN    ──[Esc]──→ NORMAL
PLACE  ──[Esc]──→ NORMAL
PLACE  ──[Enter]→ NORMAL (places part)
EDIT   ──[Esc]──→ NORMAL
EDIT   ──[Enter]→ NORMAL
NORMAL ──[q]────→ quit
```

### NORMAL mode

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
| `f` | flip part under cursor |
| `u` | undo last action |
| `?` | toggle help overlay |
| `q` | quit |

### PLACE mode

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

Entered by pressing `e` in NORMAL mode with cursor over a non-fixed part.
An undo snapshot is taken on entry. The part is highlighted.

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
| `Esc` or `Space` | stop simulation; return to NORMAL (parts reset) |

---

## Color Palette

### Pixel Renderer Colors

| Element | Color(s) | Hex |
|---|---|---|
| Background | Deep navy | `#0A0A0E` |
| Parts bin bg | Dark charcoal | `#14141C` |
| Bin divider line | Dim blue-gray | `#2A2A3A` |
| Playfield border (top/bottom) | Subtle edge line | `#1A1A2A` |

### Text Renderer Colors

| Element | Style |
|---|---|
| Background | Terminal default |
| Playfield border | `Color::DarkGray`, `Borders::ALL` |
| Parts bin border | `Color::DarkGray`, titled `" Parts "` |
| HUD background | `Color::Reset` |
| Mode indicator | Bold, mode-specific color |

### Shared Part Colors (used by both renderers, adapted per medium)

| Part | Primary | Accent | Highlight |
|---|---|---|---|
| Ball | `#FF6B35` warm orange | `#CC3700` dark edge | `#FFFFFF` specular |
| Ramp | `#C4956A` warm wood | `#8B6914` dark grain | `#E8D5B7` light edge |
| Wall | `#5A7A8A` steel blue | `#3D5566` dark face | `#8AACBC` top bevel |
| Basket | `#D4A04A` gold wicker | `#9B7330` dark weave | `#FFD700` goal glow |
| Cannon | `#4A4A52` gunmetal | `#2C2C34` dark body | `#B08D3A` brass trim |
| Cursor (normal) | `#00E5FF` cyan | — | pulsing alpha |
| Cursor (place) | `#FFD600` yellow | — | pulsing alpha |
| Edit highlight | `#00E676` green | — | pulsing outline |
| Win overlay bg | `#00000099` semi-black | — | — |
| Win text | `#00E676` green | `#FFFFFF` white | — |

---

## MVP Parts (5 parts)

Only these five are implemented for the MVP. Enough for one interesting puzzle.

---

### 1. Ball

**Physics:** Point mass. Gravity, elastic bounce off walls and surfaces.
**Size:** 28×28 px (14 px radius)
**Gravity response:** Always falls.
**Properties:** `pos: (f32, f32)`, `vel: (f32, f32)`, `active: bool`

#### Pixel Renderer — Ball

Three-layer shaded sphere with specular highlight:

1. **Base fill:** Anti-aliased filled circle. Radial gradient from bright core
   `#FF8C42` at the center-left to dark edge `#B33A00` at the rim. Light source
   assumed at top-left.
2. **Specular highlight:** Small elliptical white spot (`#FFFFFF`, 70% opacity)
   at offset `(-4, -5)` from center, radius ~4px. Gaussian falloff.
3. **Rim light:** Thin 1px bright arc (`#FFAA66`, 40% opacity) along the
   bottom-right quadrant, simulating reflected ambient light.
4. **Drop shadow:** Filled circle 3px below the ball, same radius + 2px,
   color `#000000` at 20% opacity. Clipped to playfield bounds.

**During simulation:** Ball trail rendered as 8 previous positions with
decreasing radius (14→4 px) and decreasing opacity (40%→5%), using the ball's
primary color. Creates a smooth motion blur effect.

#### Text Renderer — Ball

- **Character:** `●` (U+25CF BLACK CIRCLE)
- **Style:** Bold, `Color::Rgb(255, 107, 53)`
- **Shadow:** `·` (U+00B7) one cell below-right, `Color::DarkGray`
- **During simulation:** Trail of `·` characters at previous positions,
  styled with decreasing color intensity:
  `Rgb(255,107,53)` → `Rgb(180,70,30)` → `Rgb(100,40,15)` → `DarkGray`
  (4 trail positions)

---

### 2. Ramp (Incline)

**Physics:** One-sided sloped surface. Ball slides along slope and deflects.
**Size:** 64×32 px
**Variants:** Slope-right `/` or slope-left `\` — toggled with `f`
**Collision:** Reflect ball velocity about the surface normal.

#### Pixel Renderer — Ramp

Filled triangle with wood-grain shading:

**Slope-right `/` — vertices:** `(0, 32)`, `(64, 32)`, `(64, 0)`
**Slope-left `\` — vertices:** `(0, 0)`, `(0, 32)`, `(64, 32)`

1. **Base fill:** Filled triangle with linear gradient along the hypotenuse.
   Light side `#DEB887` (top, catching light) to dark side `#7A5A2E` (bottom,
   in shadow). Scanline fill for each row.
2. **Hypotenuse edge:** 2px anti-aliased line in `#E8D5B7` (light highlight)
   along the slope surface — this is the surface the ball contacts.
3. **Base edge:** 1px line in `#5A3E1A` (dark shadow) along the bottom.
4. **Wood grain:** 3 thin horizontal lines (`#9B7B4A`, 30% opacity) spaced
   evenly across the triangle interior, clipped to the triangle shape.
5. **Vertical edge:** 1px line in `#8B6914` on the vertical side.

#### Text Renderer — Ramp

Slope-right `/` (occupies ~8×2 cells):
```
     ╱
────╱░
```
- Slope: `╱` (U+2571 BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO LOWER LEFT)
  styled `Color::Rgb(196, 149, 106)` Bold
- Base: `─` (U+2500) styled `Color::Rgb(139, 105, 20)`
- Fill: `░` (U+2591 LIGHT SHADE) styled `Color::Rgb(160, 120, 60)` on
  `Bg::Rgb(80, 60, 30)`
- Corner: `╱` character positioned along the diagonal

Slope-left `\` mirrors horizontally:
```
╲
░╲────
```

---

### 3. Wall (Flat Surface)

**Physics:** Static rigid surface. Ball bounces off all sides.
**Size:** 64×32 px default (horizontal). `f` swaps to 32×64 (vertical).

#### Pixel Renderer — Wall

Beveled rectangle with metallic shading:

1. **Face fill:** Solid rectangle in `#5A7A8A` (steel blue-gray).
2. **Top bevel:** 2px strip across the top in `#8AACBC` (light highlight) —
   simulates light catching the top edge.
3. **Left bevel:** 2px strip down the left in `#7A9AAA` (medium highlight).
4. **Bottom bevel:** 2px strip across the bottom in `#3D5566` (dark shadow).
5. **Right bevel:** 2px strip down the right in `#4A6676` (medium shadow).
6. **Center line:** 1px horizontal line at vertical center in `#4A6A7A`
   (subtle structural detail, 50% opacity).
7. **Corner rivets:** 4 small filled circles (2px radius) at each corner,
   color `#7090A0` — decorative industrial detail.

**Vertical wall** (after flip): Same treatment, rotated. The center line
becomes vertical.

#### Text Renderer — Wall

Horizontal (occupies ~8×2 cells):
```
▛▀▀▀▀▀▀▜
▙▄▄▄▄▄▄▟
```
- Top-left: `▛` (U+259B), Top-right: `▜` (U+259C)
- Bottom-left: `▙` (U+2599), Bottom-right: `▟` (U+259F)
- Top edge: `▀` (U+2580), Bottom edge: `▄` (U+2584)
- All styled `Color::Rgb(90, 122, 138)`, Bold
- Interior (if wall is tall enough): `█` (U+2588) in slightly darker shade

Vertical (after flip, occupies ~4×4 cells):
```
▛▀▜
█  █
█  █
▙▄▟
```

---

### 4. Basket (Goal)

**Physics:** Ball entering the basket opening triggers win condition.
**Size:** 64×64 px
**Win condition:** Ball center inside basket bounds AND `vel.y > 0` (falling in).

#### Pixel Renderer — Basket

Open-top wicker U-shape with woven texture and goal indicator:

1. **Side walls:** Two vertical rectangles, each 6px wide × 64px tall, at
   left and right edges. Filled with alternating diagonal stripes to simulate
   woven wicker: `#D4A04A` and `#9B7330`, each stripe 4px wide, 45° angle.
2. **Bottom:** Horizontal rectangle, 64px wide × 6px tall, at the bottom.
   Same woven pattern as the sides.
3. **Rim highlights:** 1px bright line (`#E8C870`) along the inner edges of
   all three sides — gives visual depth to the opening.
4. **Outer shadow:** 1px dark line (`#6B4F1A`) along the outer edges.
5. **Interior:** Slightly lighter than the playfield background `#14141C` —
   a subtle `#1A1A24` fill to show the "inside" of the basket.
6. **Goal indicator:** When a ball is within 48px of the basket center, a
   soft radial glow (`#FFD700`, 15% opacity, radius 40px) pulses around the
   basket interior. Pulse rate: `sin(frame * 0.1) * 0.5 + 0.5`.
7. **Opening arrows:** Two small downward-pointing triangles (`#FFD700`,
   40% opacity) at the top-left and top-right inner corners, each 6px tall.
   Hints to the player that the ball enters from above.

#### Text Renderer — Basket

Occupies ~8×4 cells:
```
▽      ▽
║  ••  ║
║      ║
╚══════╝
```
- Sides: `║` (U+2551) styled `Color::Rgb(212, 160, 74)` Bold
- Bottom: `═` (U+2550) styled same color
- Corner: `╚` (U+255A) and `╝` (U+255D)
- Opening arrows: `▽` (U+25BD) styled `Color::Rgb(255, 215, 0)`
- Goal dots: `••` (U+2022) styled `Color::Rgb(255, 215, 0)` Dim — pulsing
  visibility via alternating show/hide every 30 frames
- When ball is near: border color brightens to `Color::Rgb(255, 215, 0)`

**Fixed basket highlight:** Fixed baskets (puzzle fixtures) get a subtle
`Bg::Rgb(30, 25, 10)` behind the interior cells to distinguish them.

---

### 5. Cannon

**Physics:** Fires a ball at simulation start with configured angle and power.
**Size:** 96×64 px
**Properties:** `angle_deg: f32`, `power: f32`
**Firing:** On RUN start, spawns a `SimBall` at barrel tip with computed velocity.
**Flip `f`:** Mirrors cannon to face left.

#### Pixel Renderer — Cannon

Multi-component mechanical assembly:

**Body (main housing):** Rounded rectangle, 60×40 px, centered vertically.
- Fill: Linear gradient from `#4A4A52` (top, light) to `#2C2C34` (bottom, shadow).
- Border: 1px `#1A1A1E` all around.
- Panel line: 1px horizontal at vertical center, `#3A3A42`.
- Rivets: 4 small circles (2px radius) at corners, `#6A6A72`.

**Barrel:** Rotated rectangle, 40×10 px, anchored at body's center-right.
- Angle: rotated by `angle_deg` from horizontal.
- Fill: `#363636` with 1px `#5A5A5A` highlight along the top edge.
- Muzzle: 2px bright ring (`#6A6A6A`) at the tip.
- Interior: Dark bore visible as `#0A0A0A` circle (4px diameter) at the tip.

**Wheels:** Two circles at the bottom of the body, radius 10px.
- Outer rim: `#3B2F2F` filled circle with `#2A2020` 1px border.
- Hub: `#555555` filled circle, radius 3px.
- Spokes: 4 thin lines from hub to rim, `#444444`, evenly spaced at 45°.

**Brass trim:** 2px horizontal band across the body at 25% from top.
- Color: `#B08D3A` with `#D4AA4A` 1px highlight on top edge.

**Fuse:** Curved squiggle at the rear of the body.
- 3px thick line, dark red `#8B0000`, with a small spark dot at the tip
  (`#FFD700`, pulsing).

**Flipped cannon:** All horizontal coordinates mirror. Barrel points left.

#### Text Renderer — Cannon

Occupies ~12×4 cells, facing right:
```
      ╱═══▸
   ┌──────┐
   │▓▓▓▓▓▓│
 ◯ └──────┘ ◯
```
- Barrel: `═` (U+2550) and `▸` (U+25B8) styled `Color::Rgb(100, 100, 110)`
  Barrel angle shown with `╱` or `═` or `╲` depending on `angle_deg`:
  - angle < -15°: `╱═══▸` (angled up-right)
  - -15° ≤ angle ≤ 15°: `════▸` (horizontal)
  - angle > 15°: `╲═══▸` (angled down-right)
- Body: Box drawing `┌──────┐ │ │ └──────┘` styled `Color::Rgb(74, 74, 82)`
- Fill: `▓` (U+2593 DARK SHADE) styled `Color::Rgb(60, 60, 68)`
- Wheels: `◯` (U+25EF LARGE CIRCLE) styled `Color::Rgb(59, 47, 47)`
- Brass: the body's top border chars use `Color::Rgb(176, 141, 58)` Bold

Facing left (flipped):
```
◁═══╲
┌──────┐
│▓▓▓▓▓▓│
◯ └──────┘ ◯
```

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
    Ramp,                                    // 64×32 px
    Wall { width: f32, height: f32 },        // default 64×32 px
    Basket,                                  // 64×64 px
    Cannon { angle_deg: f32, power: f32 },   // 96×64 px
}

struct SimBall {
    pos:   (f32, f32),
    vel:   (f32, f32),
    active: bool,
    trail: Vec<(f32, f32)>,   // last 8 positions for motion trail
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

enum RenderMode {
    Pixel,
    Text,
}
```

---

## Physics (Manual Euler, MVP)

Run at 60 fps. `dt` clamped to `1/30` to prevent tunneling on lag spikes.

```
// Constants
GRAVITY     = 900.0    // px/s²
RESTITUTION = 0.72
FRICTION    = 0.85
MAX_DT      = 1.0/30.0

// Per frame
let dt = elapsed.min(MAX_DT);

// Gravity
ball.vel.y += GRAVITY * dt;

// Integrate
ball.pos.x += ball.vel.x * dt;
ball.pos.y += ball.vel.y * dt;

// Record trail position (every 2 frames)
if frame % 2 == 0 {
    ball.trail.push(ball.pos);
    if ball.trail.len() > 8 { ball.trail.remove(0); }
}

// Collision: for each static part, test and resolve
```

### Collision Resolution

**Ball vs Wall (axis-aligned rect):**
- AABB overlap test between ball circle and wall rect
- Find shallowest penetration axis
- Push ball out along that axis
- Reflect velocity component: `vel_n *= -RESTITUTION`
- Apply friction to tangent: `vel_t *= FRICTION`

**Ball vs Ramp:**
- Slope-right `/`: surface normal `(-0.707, -0.707)`, line from `(0, h)` to `(w, 0)`
- Slope-left `\`: surface normal `(0.707, -0.707)`, line from `(0, 0)` to `(w, h)`
- If ball center within ramp AABB and on the wrong side of the hypotenuse:
  - Push ball to surface along normal
  - Reflect velocity about normal, scale by `RESTITUTION`

**Ball vs Basket:**
- Interior zone: basket bounds inset by 8px on sides and bottom, open top
- If ball center inside interior AND `vel.y > 0`: `state.won = true`

**World bounds:**
- Ball bounces off playfield edges (x: 0..512, y: 0..360)
- Same reflect + restitution as walls

---

## Rendering Pipeline

### Pixel Renderer — Per Frame

1. Clear 640×360 `RgbaImage` to `#0A0A0E`
2. Draw playfield border: 1px `#1A1A2A` lines at x=0, x=511, y=0, y=359
3. Draw parts bin background `#14141C` (x: 512..640)
4. Draw bin divider lines `#2A2A3A` between slots
5. For each `Part`: call part-specific draw function (described above)
6. If EDIT mode: draw pulsing green outline (`#00E676`, alpha oscillates
   `sin(frame * 0.15) * 0.3 + 0.5`) around the edited part
7. If PLACE mode: draw ghost outline of selected part at cursor, 40% opacity
8. If not RUN mode: draw cursor crosshair
   - 4 line segments, 8px each, 4px gap from center
   - NORMAL: `#00E5FF`, PLACE: `#FFD600`
   - Alpha pulsing: `sin(frame * 0.12) * 0.2 + 0.8`
9. If RUN mode and ball active:
   - Draw trail (oldest first, decreasing size and opacity)
   - Draw ball at current position
10. If `won`: draw semi-transparent overlay and win text
11. Draw parts bin icons (miniature versions of each part)
12. Draw bin selection highlight (PLACE mode): `#FFD600` rect at 25% opacity
13. Transmit image via `viuer::print`
14. Write HUD text via `crossterm` (2 rows below image)

### Text Renderer — Per Frame

1. Call `terminal.draw(|frame| { ... })` via ratatui
2. Create layout:
   - Horizontal: `[Constraint::Min(60), Constraint::Length(20)]` → playfield | bin
   - Vertical: `[Constraint::Min(10), Constraint::Length(3)]` → game | HUD
3. Render playfield `Block` with `Borders::ALL`, title `" TIM2 "`, `DarkGray`
4. Map each `Part` to cell coordinates and render characters:
   - Fixed parts: styled with `Modifier::DIM` to distinguish from player parts
   - Player parts: normal brightness
5. If EDIT mode: edited part's characters get `Bg::Rgb(0, 40, 0)` background,
   pulsing between `Bg::Rgb(0, 30, 0)` and `Bg::Rgb(0, 50, 0)`
6. If PLACE mode: ghost of selected part drawn at cursor position with
   `Modifier::DIM` and `Color::DarkGray`
7. If not RUN mode: draw cursor
   - NORMAL: `┼` (U+253C) or `+` styled `Color::Cyan`, Bold
   - PLACE: `┼` styled `Color::Yellow`, Bold
8. If RUN mode and ball active:
   - Draw trail `·` characters
   - Draw ball `●` at current cell position
9. If `won`: render centered `Paragraph` overlay:
   ```
   ┌──────────────────────┐
   │    PUZZLE SOLVED!     │
   │                       │
   │  [Space] Try Again    │
   │  [q] Quit             │
   └──────────────────────┘
   ```
   Styled `Color::Green`, Bold, with `Clear` widget behind for backdrop
10. Render parts bin as ratatui `List` widget:
    - Each item: part icon character + name + quantity
    - Icons: `●` Ball, `╱` Ramp, `█` Wall, `╚╝` Basket, `▸` Cannon
    - Selected item (PLACE mode): `Modifier::REVERSED` background
    - Style: `Color::Rgb` matching part colors
11. Render HUD `Paragraph`:
    - Row 1: `MODE: {mode}` (bold, mode-colored) ` │ cursor: ({x},{y}) │ parts: {n}`
    - Row 2: context-sensitive key hints for current mode

### Parts Bin — Pixel Renderer Detail

Right panel 128×360. Five slots of 128×60 px each, plus 60px instruction area.

Each slot contains:
- Centered miniature icon of the part (drawn with the same rendering functions,
  scaled to fit ~40×40 px area)
- Part name text (rendered as pixel text, 8px font height)
- Quantity badge: small rounded rect in top-right with count
- Slot key hint: `[1]`–`[5]` in bottom-right corner, dim gray

```
slot 0: ● Ball       [1]
slot 1: ╱ Ramp       [2]
slot 2: █ Wall       [3]
slot 3: ╚╝ Basket    [4]
slot 4: ▸ Cannon     [5]
─────────────────────────
(bottom 60px: brief controls summary)
```

### Parts Bin — Text Renderer Detail

ratatui `List` widget with `Borders::ALL`, title `" Parts "`.

Each item is a `ListItem` with styled spans:
```
 ● Ball      ×1
 ╱ Ramp      ×1
 █ Wall      ×2
 ╚╝ Basket   ×1
 ▸ Cannon    ×1
```
- Icons colored with part colors (see palette)
- Quantity `×N` right-aligned, `Color::DarkGray`
- Zero-quantity items: entire row `Color::DarkGray`, `Modifier::DIM`
- Selected (PLACE mode): `Modifier::REVERSED`
- Highlight bar: `Bg::Rgb(40, 40, 20)` on selected item

### HUD Detail

**Pixel renderer:** 2 lines of terminal text below the image, written via
`crossterm::execute!` with styled content.

**Text renderer:** ratatui `Paragraph` widget in the bottom layout pane.

Row 1 format: `MODE: {mode} │ ({cursor_x}, {cursor_y}) │ parts: {count}`
- Mode colors: NORMAL → Cyan, PLACE → Yellow, EDIT → Green, RUN → Red

Row 2 format (context-sensitive):
- NORMAL: `[p]lace [e]dit [Space]run [x]del [f]lip [u]ndo [?]help [q]uit`
- PLACE: `[hjkl]move [JK]scroll [1-5]select [Enter]place [Esc]cancel`
- EDIT: `[hjkl]move [f]lip [x]del [Enter/Esc]done`
- RUN: `[Esc/Space] stop`

---

## Help Overlay

Toggled with `?` in NORMAL mode. Rendered as a centered panel in both modes.

### Pixel Renderer

Semi-transparent dark rectangle (`#000000CC`) centered on playfield, ~300×250 px.
White text listing all keybindings for the current mode, 10px font height.

### Text Renderer

ratatui `Paragraph` inside a `Block` with `Borders::ALL`, `Clear` widget behind.
`Color::White` text on default background. Same content as pixel version.

---

## The Hardcoded MVP Puzzle

One puzzle is hardcoded to avoid building a loader.

**Goal:** "Get the ball into the basket"

**Fixed (immovable) parts:**
- Cannon at (32, 160) facing right, angle -30°, power 600 px/s
- Basket at (384, 224)

**Player's inventory (parts bin):**
- 1× Ramp
- 2× Wall (64×32 px)

**One valid solution:**
- Place Wall at ~(224, 128) to redirect ball downward
- Place Ramp at ~(320, 192) to guide ball into basket

Three parts to work with, two needed, one is a decoy.

---

## Win / Lose

**Win:** Ball enters basket. Simulation pauses. Overlay appears:

Pixel renderer: Semi-transparent overlay centered on playfield.
```
╔═══════════════════════╗
║    PUZZLE SOLVED!      ║
║                        ║
║  [Space] Try Again     ║
║  [q] Quit              ║
╚═══════════════════════╝
```
Text renderer: Same content as a ratatui popup `Block`.

**No lose condition** in MVP. Ball comes to rest or exits the playfield.
Player presses `Esc` to stop and try again.

---

## Project Structure

```
game/
  Cargo.toml
  src/
    main.rs           -- entry point, CLI args, mode detection, game loop
    state.rs          -- GameState, Part, PartKind, SimBall, Mode, BinItem
    input.rs          -- handle_input() — crossterm poll, mode transitions
    physics.rs        -- update_physics(), all collision resolution
    render/
      mod.rs          -- Renderer trait, RenderMode enum
      pixel.rs        -- PixelRenderer: image + viuer pipeline
      pixel_gfx.rs    -- pixel drawing primitives (fill_circle, gradient, etc.)
      text.rs         -- TextRenderer: ratatui pipeline
    parts/
      mod.rs          -- shared Part helpers (AABB, dimensions, etc.)
      ball.rs         -- draw_ball_pixel(), draw_ball_text(), collide_ball()
      ramp.rs         -- draw_ramp_pixel(), draw_ramp_text(), collide_ramp()
      wall.rs         -- draw_wall_pixel(), draw_wall_text(), collide_wall()
      basket.rs       -- draw_basket_pixel(), draw_basket_text(), check_win()
      cannon.rs       -- draw_cannon_pixel(), draw_cannon_text(), fire_cannon()
    hud.rs            -- HUD content generation (shared), rendered by each renderer
    puzzle.rs         -- hardcoded MVP puzzle definition
```

### Dependencies

```toml
[dependencies]
# Shared
crossterm = "0.28"

# Pixel renderer
image     = "0.25"
imageproc = "0.25"
viuer     = "0.11"

# Text renderer
ratatui   = "0.29"
```

---

## Out of Scope for MVP

These are explicitly deferred:

- Rapier2D (manual Euler is sufficient for 5 parts)
- Rope / pulley / belt systems
- Electrical system
- Laser system
- Animals (Pokey, Mort, Edison)
- Pipe system
- Puzzle loader / saver
- Level progression
- Sound
- Undo beyond last placement (single undo is fine)
- Part resizing (Wall is fixed size for MVP)
- Workshop mode
- Head-to-head mode
- Preview arc (can add after physics is stable)
- Scenery / decorative elements
- Gravity / pressure sliders
- Braille sub-cell rendering for text mode (future enhancement)
