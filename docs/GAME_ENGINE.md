# Game Engine Stack

This document describes the technology stack chosen to build the terminal-based recreation of
The Incredible Machine 2 (TIM2). It covers every layer from the OS terminal up to the game loop,
explains why each piece was chosen, and records what was validated through the POC.

---

## Stack Overview

```
┌──────────────────────────────────────────────────────────┐
│                      Game Logic                          │
│          (puzzle solver, parts, chain reactions)         │
├──────────────────────────────────────────────────────────┤
│                    Physics Engine                        │
│                  Rapier2D (standalone)                   │
├──────────────────────────────────────────────────────────┤
│                   Game Loop / ECS                        │
│              Raw Rust  ·  manual fixed-dt                │
├──────────────────────────────────────────────────────────┤
│                  Pixel Rendering                         │
│          image 0.25  +  imageproc 0.25                   │
├──────────────────────────────────────────────────────────┤
│           Graphics Protocol (auto-detected)              │
│         viuer 0.11  →  Kitty · Sixel · iTerm2            │
├──────────────────────────────────────────────────────────┤
│               Terminal I/O / Cross-platform              │
│                    Crossterm 0.28                        │
├──────────────────────────────────────────────────────────┤
│               Terminal Emulator (user's)                 │
│   kitty · WezTerm · Ghostty · xterm · tmux · iTerm2     │
└──────────────────────────────────────────────────────────┘
```

---

## Why Pixel Rendering Over Unicode Characters

Two approaches were prototyped and compared side by side:

| | Unicode / ratatui | Pixel / Sixel+Kitty |
|---|---|---|
| Rendering unit | Terminal cell (~8×16 px) | Individual pixel |
| Ball | `●` character, snaps to cell grid | Anti-aliased sphere with specular highlight |
| Trail | Stepped `·` characters | Smooth fading circles, colour-shifts per speed |
| Flame | Flickering Unicode chars | Per-pixel procedural fire — red→orange→yellow→white |
| Laser | `═─╌` char animation | Multi-pass glow: soft halo + bright core + energy pulses |
| Glow / ambient | `░` block chars | True radial gradient, smooth falloff |
| Sub-cell motion | No — position rounded to cell | Yes — smooth at any speed |
| Colour depth | 256-colour or RGB per cell | Full 24-bit RGB per pixel |
| Terminal requirement | Any ANSI terminal | Kitty / Sixel / iTerm2 (auto-detected, fallback exists) |

The pixel stack was chosen. TIM2's parts — curved ramps, spinning gears, bouncing balls, ropes —
need sub-cell precision and smooth colour gradients to feel right. Unicode characters can
approximate these shapes but not accurately represent them.

---

## Layer by Layer

### 1. Terminal I/O — Crossterm 0.28

**What it does:** Cross-platform raw terminal access. Handles raw mode, alternate screen,
cursor control, and keyboard/mouse event reading.

**Usage in the game loop:**
```rust
terminal::enable_raw_mode()?;
execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

// Non-blocking input poll each frame:
while event::poll(Duration::ZERO)? {
    if let Event::Key(key) = event::read()? { ... }
}

// Move cursor to top-left before rendering each frame:
execute!(stdout(), cursor::MoveTo(0, 0))?;
```

---

### 2. Pixel Buffer — image 0.25 + imageproc 0.25

**What they do:** `image` provides the `RgbaImage` pixel buffer (a flat `Vec<u8>` of RGBA values).
`imageproc` provides drawing primitives on top of it.

**Each frame we:**
1. Allocate (or reuse) a fixed-size `RgbaImage` (e.g. 640×360)
2. Clear it to the background colour
3. Draw all game objects — physics parts, particles, UI — into the pixel buffer
4. Hand it to `viuer` to encode and transmit

**Key drawing operations (`gfx.rs` shared library):**

| Function | Purpose |
|---|---|
| `fill_circle(img, cx, cy, r, col, alpha)` | Anti-aliased filled circle — balls, bubbles |
| `radial_glow(img, cx, cy, max_r, col, strength)` | Soft radial gradient — ambient light, flame glow |
| `glowing_line(img, x0,y0, x1,y1, core, glow, …)` | Multi-pass glowing line — laser beams, rope, barrel |
| `draw_flame(img, cx, base_y, w, h, t, frame)` | Procedural per-pixel fire simulation |
| `fill_rect(img, x, y, w, h, col, alpha)` | Solid/transparent rectangles — walls, part bodies |
| `blend / over` | Additive and alpha-composite pixel blending |

**Canvas size:** 640×360 pixels. `viuer` scales this to fit the terminal automatically.
The physics simulation runs in canvas-pixel coordinates (no unit conversion needed).

---

### 3. Graphics Protocol — viuer 0.11

**What it does:** Auto-detects the best pixel graphics protocol supported by the running terminal
and encodes + transmits the pixel buffer each frame.

**Protocol priority (automatic):**

| Protocol | Terminals | Notes |
|---|---|---|
| **Kitty Graphics Protocol** | kitty, WezTerm, Ghostty | Fastest; delta updates possible |
| **Sixel** | xterm, mlterm, foot, many others | Widely supported; full-frame encode |
| **iTerm2 inline images** | iTerm2, Warp | macOS-oriented |
| **Unicode half-block fallback** | Any ANSI terminal | Lower fidelity; always works |

`viuer` handles the detection and fallback transparently — no code changes needed to support a
new terminal.

**Render call per frame:**
```rust
execute!(stdout(), cursor::MoveTo(0, 0))?;
viuer::print(&img, &Config {
    x: 0, y: 0,
    width:  Some(term_w as u32),
    height: Some((term_h - 2) as u32),  // leave rows for text HUD
    ..Default::default()
})?;
```

**Why not notcurses:** `libnotcurses` (C library) in the system package manager is v3.0.6,
but the Rust crate requires ≥ 3.0.11. Building from source adds a non-trivial CI/distribution
burden. `viuer` is pure Rust, zero C dependencies, and produces the same Sixel/Kitty output.

---

### 4. Game Loop — Raw Rust

**No Bevy.** The pixel rendering stack does not need an ECS for the POC. Bevy was evaluated
(see `cannon-poc/`) but removed for the following reasons:

| Reason | Detail |
|---|---|
| Version coupling | `bevy_ratatui 0.9` requires `bevy_ecs 0.16`; physics crates target different versions |
| Overhead | Bevy's plugin system, reflection, and scheduler add compile time and binary size with no benefit for a single-threaded game loop |
| Rendering model mismatch | Bevy's renderer targets GPU; bridging to pixel-buffer-over-Sixel required workarounds |
| Simpler is faster to iterate | A raw Rust loop with a `State` struct is easier to reason about for a puzzle game |

**Game loop structure:**
```rust
let frame_dur = Duration::from_secs_f64(1.0 / 60.0);
loop {
    let frame_start = Instant::now();

    handle_input(&mut state)?;       // non-blocking crossterm poll
    update_physics(&mut state, dt);  // fixed-dt Euler integration
    let img = render(&state);        // draw to pixel buffer
    display(img, &cfg)?;             // viuer encode + transmit

    let elapsed = frame_start.elapsed();
    if elapsed < frame_dur { std::thread::sleep(frame_dur - elapsed); }
}
```

**State:** A plain Rust struct. Each game part will be a struct field or `Vec<Part>`. No ECS
is needed until the part count or system complexity demands it — at which point `hecs` (a
lightweight archetypal ECS) is the preferred addition, not Bevy.

---

### 5. Physics

**Current (POC):** Manual Euler integration.
```
velocity += gravity * dt
position += velocity * dt
// elastic bounce on boundary collision
velocity *= restitution on impact
```
Sufficient for the cannon and fidelity demos. All physics constants are plain struct fields,
easily tuned at runtime.

**Production target:** Standalone **Rapier2D** (no Bevy integration layer needed).

```toml
rapier2d = "0.22"
```

Rapier2D runs independently of any game engine. We call `physics_pipeline.step(...)` once
per frame inside our raw Rust loop and read back the updated `RigidBodySet` positions to
know where to draw each part. This gives us:

- Rigid body dynamics (balls, planks, seesaws)
- Convex + compound colliders (ramps, gears, baskets)
- Joints (ropes as articulated chains, pulleys as constraints)
- Continuous collision detection (fast-moving balls)
- Deterministic simulation (same seed → same solution every time, required for puzzle replay)

---

## POC Projects

### `cannon-poc/` — ratatui stack (reference / comparison)

Unicode character rendering at 60fps. Used to establish the baseline and validate the
Bevy + bevy_ratatui integration before deciding to move to the pixel stack.

```bash
cd cannon-poc
cargo run --bin cannon --release   # physics demo
cargo run --bin demos  --release   # laser / pot / candle
```

### `sixel-poc/` — pixel stack (chosen)

Pixel rendering via viuer (Kitty/Sixel auto-detected). This is the stack that becomes
the production game.

```bash
cd sixel-poc
cargo run --bin cannon --release   # smooth sphere, glow trail, physics
cargo run --bin demos  --release   # laser / boiling pot / candle
# [1] Laser  [2] Pot  [3] Candle  [Tab] Next  [Q] Quit
```

---

## Dependency Versions

```toml
[dependencies]
image      = "0.25"
imageproc  = "0.25"
viuer      = "0.11"
crossterm  = "0.28"

# Physics (to be added for production):
rapier2d   = "0.22"
```

---

## Terminal Compatibility

| Terminal | Protocol | Fidelity |
|---|---|---|
| kitty | Kitty Graphics Protocol | Full pixel, fastest |
| WezTerm | Kitty Graphics Protocol | Full pixel |
| Ghostty | Kitty Graphics Protocol | Full pixel |
| xterm (256color) | Sixel | Full pixel |
| foot | Sixel | Full pixel |
| iTerm2 | iTerm2 inline | Full pixel |
| tmux (passthrough) | Sixel (with passthrough config) | Full pixel |
| Any ANSI terminal | Unicode half-block fallback | Lower fidelity |

SSH-safe: no GPU, no display server, no window manager required. Runs anywhere Rust runs.

---

## What's Not In The Stack

| Thing | Why excluded |
|---|---|
| Bevy | Version coupling, rendering model mismatch, overhead — see §4 |
| Ratatui | Character-cell resolution insufficient; kept in `cannon-poc` as reference only |
| Notcurses | C library version mismatch with system packages; `viuer` is a pure-Rust equivalent |
| GPU renderer | Terminal is the display — no GPU pipeline needed |
| Asset pipeline | Parts are drawn procedurally in code; no sprite files to load |
| bevy_ratatui_camera | Bridges GPU→terminal; unnecessary in our pixel-buffer approach |
