# Game Engine Stack

This document describes the technology stack chosen to build the terminal-based recreation of
The Incredible Machine 2 (TIM2). It covers every layer from the OS terminal up to the game loop,
explains why each piece was chosen, and records what was validated through the POCs.

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
│              Dual Renderer (mode-switched)               │
│  ┌────────────────────────┬───────────────────────────┐  │
│  │     Pixel Renderer     │      Text Renderer        │  │
│  │  image 0.25 + viuer    │  ratatui 0.29 + Unicode   │  │
│  │  Kitty / Sixel / iTerm │  Any ANSI terminal        │  │
│  └────────────────────────┴───────────────────────────┘  │
├──────────────────────────────────────────────────────────┤
│               Terminal I/O / Cross-platform              │
│                    Crossterm 0.28                        │
├──────────────────────────────────────────────────────────┤
│               Terminal Emulator (user's)                 │
│   kitty · WezTerm · Ghostty · xterm · tmux · iTerm2     │
└──────────────────────────────────────────────────────────┘
```

---

## Dual Renderer — Core Requirement

The engine **must** support two completely separate rendering paths, selected at startup
based on terminal capability (or overridden via CLI flag):

| | Pixel Renderer | Text Renderer |
|---|---|---|
| **When active** | Terminal supports Kitty, Sixel, or iTerm2 graphics | Any other terminal (ANSI-only) |
| **Rendering unit** | Individual pixel (640×360 `RgbaImage`) | Terminal cell (~8×16 px) via ratatui |
| **Ball** | Anti-aliased sphere with specular highlight, glow | `●` character, bold + colour |
| **Trail** | Smooth fading circles, colour-shifts per speed | Stepped `·` / `∙` characters, fading colour |
| **Flame** | Per-pixel procedural fire — red→orange→yellow→white | Flickering Unicode chars |
| **Laser** | Multi-pass glow: soft halo + bright core + energy pulses | `═─╌` char animation |
| **Glow / ambient** | True radial gradient, smooth falloff | `░` block chars around object |
| **Sub-cell motion** | Yes — smooth at any speed | No — position rounded to cell |
| **Colour depth** | Full 24-bit RGB per pixel | 256-colour or RGB per cell |
| **Dependencies** | `image` + `imageproc` + `viuer` | `ratatui` |
| **FPS target** | 60 fps | 60 fps |

### Why dual instead of fallback

Three approaches were prototyped (`tiered-poc/`, `dual-poc/`, `hybrid-poc/`):

| Approach | Verdict |
|---|---|
| **Tiered** (single viuer pipeline, adaptive quality) | Half-block fallback still looks mediocre even at reduced resolution. Limited control over text-mode appearance. |
| **Hybrid** (ratatui layout + pixel overlay) | Complex coordination between ratatui and viuer. Potential flicker at overlay boundary. Over-engineered for what we need. |
| **Dual** (two independent renderers) | Full control over both paths. Each renderer is purpose-built for its medium. Text mode looks good on its own terms rather than being a degraded pixel mode. **Chosen.** |

### Mode detection

At startup the engine detects graphics capability via environment variables:

```rust
fn detect_mode() -> RenderMode {
    // CLI override: --pixel or --text
    // Then auto-detect:
    //   TERM=xterm-kitty || KITTY_WINDOW_ID    → Pixel
    //   TERM_PROGRAM=WezTerm                    → Pixel
    //   TERM_PROGRAM=ghostty                    → Pixel
    //   TERM_PROGRAM=iTerm.app                  → Pixel
    //   otherwise                               → Text
}
```

CLI flags `--pixel` and `--text` override auto-detection for testing and user preference.

### Renderer trait

Both renderers implement a shared interface so the game loop doesn't branch on mode
after initialization:

```rust
trait Renderer {
    fn render_frame(&mut self, state: &GameState) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;
}

struct PixelRenderer { /* viuer config, image buffer */ }
struct TextRenderer  { /* ratatui terminal */ }
```

The game loop calls `renderer.render_frame(&state)` — the active renderer handles all
drawing, HUD, and terminal output internally.

### Shared physics, separate visuals

Physics and game logic are **completely decoupled** from rendering. The game state uses
normalized coordinates or canvas-pixel coordinates — each renderer maps these to its own
output space. A game running in text mode must produce identical physics outcomes to the
same game running in pixel mode.

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

### 2a. Pixel Renderer — image 0.25 + imageproc 0.25 + viuer 0.11

Active when the terminal supports Kitty, Sixel, or iTerm2 graphics.

**Each frame:**
1. Clear a fixed-size `RgbaImage` (640×360) to the background colour
2. Draw all game objects — physics parts, particles, UI — into the pixel buffer
3. Hand it to `viuer` to encode and transmit via the detected graphics protocol

**Key drawing operations (`gfx.rs`):**

| Function | Purpose |
|---|---|
| `fill_circle(img, cx, cy, r, col, alpha)` | Anti-aliased filled circle — balls, bubbles |
| `radial_glow(img, cx, cy, max_r, col, strength)` | Soft radial gradient — ambient light, flame glow |
| `glowing_line(img, x0,y0, x1,y1, core, glow, …)` | Multi-pass glowing line — laser beams, rope, barrel |
| `draw_flame(img, cx, base_y, w, h, t, frame)` | Procedural per-pixel fire simulation |
| `fill_rect(img, x, y, w, h, col, alpha)` | Solid/transparent rectangles — walls, part bodies |
| `blend / over` | Additive and alpha-composite pixel blending |

**Canvas size:** 640×360 pixels. `viuer` scales to fit the terminal automatically.

**Graphics protocol priority (auto-detected by viuer):**

| Protocol | Terminals | Notes |
|---|---|---|
| **Kitty Graphics Protocol** | kitty, WezTerm, Ghostty | Fastest; delta updates possible |
| **Sixel** | xterm, mlterm, foot, many others | Widely supported; full-frame encode |
| **iTerm2 inline images** | iTerm2, Warp | macOS-oriented |

**Why not notcurses:** `libnotcurses` (C library) in the system package manager is v3.0.6,
but the Rust crate requires >= 3.0.11. Building from source adds a non-trivial CI/distribution
burden. `viuer` is pure Rust, zero C dependencies, and produces the same Sixel/Kitty output.

---

### 2b. Text Renderer — ratatui 0.29

Active when the terminal does not support pixel graphics (or when `--text` is passed).

**Each frame:**
1. Use ratatui's `Terminal::draw()` to get a `Frame`
2. Layout the screen with `Layout` constraints (game area + HUD)
3. Render the game area as a ratatui `Widget` using Unicode drawing characters
4. Render the HUD as styled `Paragraph` widgets

**Character vocabulary:**

| Game element | Characters | Styling |
|---|---|---|
| Ball | `●` | Bold, coloured |
| Trail | `·` `∙` | Fading indexed colours |
| Glow | `░` | Dim colour around objects |
| Cannon body | `█` | RGB colour |
| Cannon barrel | `═` | RGB colour |
| Ramp / platform | `▬` | Green / blue |
| Direction arrow | `→ ↗ ↑ ↖ ← ↙ ↓ ↘` | Orange |
| Preview arc | `·` | Dim grey |
| Walls / border | ratatui `Block` with `Borders::ALL` | DarkGray |

**Why ratatui instead of raw crossterm:** ratatui provides double-buffered diffing (only
updates cells that changed), layout management, and styled text composition. These are
exactly the things that make text-mode rendering not feel janky.

---

### 3. Game Loop — Raw Rust

**No Bevy.** Neither renderer needs an ECS. Bevy was evaluated (see `cannon-poc/`) but
removed for the following reasons:

| Reason | Detail |
|---|---|
| Version coupling | `bevy_ratatui 0.9` requires `bevy_ecs 0.16`; physics crates target different versions |
| Overhead | Bevy's plugin system, reflection, and scheduler add compile time and binary size with no benefit for a single-threaded game loop |
| Rendering model mismatch | Bevy's renderer targets GPU; bridging to pixel-buffer-over-Sixel required workarounds |
| Simpler is faster to iterate | A raw Rust loop with a `State` struct is easier to reason about for a puzzle game |

**Game loop structure:**
```rust
let mode = detect_mode();
let mut renderer: Box<dyn Renderer> = match mode {
    RenderMode::Pixel => Box::new(PixelRenderer::new()?),
    RenderMode::Text  => Box::new(TextRenderer::new()?),
};

let frame_dur = Duration::from_secs_f64(1.0 / 60.0);
loop {
    let frame_start = Instant::now();

    handle_input(&mut state)?;       // non-blocking crossterm poll
    update_physics(&mut state, dt);  // fixed-dt Euler integration
    renderer.render_frame(&state)?;  // pixel or text — polymorphic

    let elapsed = frame_start.elapsed();
    if elapsed < frame_dur { std::thread::sleep(frame_dur - elapsed); }
}
```

**State:** A plain Rust struct. Each game part will be a struct field or `Vec<Part>`. No ECS
is needed until the part count or system complexity demands it — at which point `hecs` (a
lightweight archetypal ECS) is the preferred addition, not Bevy.

---

### 4. Physics

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
- Deterministic simulation (same seed -> same solution every time, required for puzzle replay)

---

## POC Projects

### `cannon-poc/` — Bevy + ratatui (reference / comparison)

Unicode character rendering at 60fps using Bevy ECS + ratatui. Used to establish the
baseline and validate the Bevy + bevy_ratatui integration before deciding to remove Bevy.

```bash
cd pocs/cannon-poc
cargo run --bin cannon --release   # physics demo
cargo run --bin demos  --release   # laser / pot / candle
```

### `sixel-poc/` — pixel stack (original)

Pixel rendering via viuer (Kitty/Sixel auto-detected). Validated the pixel rendering
pipeline before the dual renderer decision.

```bash
cd pocs/sixel-poc
cargo run --bin cannon --release   # smooth sphere, glow trail, physics
cargo run --bin demos  --release   # laser / boiling pot / candle
# [1] Laser  [2] Pot  [3] Candle  [Tab] Next  [Q] Quit
```

### `dual-poc/` — dual renderer (chosen approach)

Two independent renderers: pixel (viuer) and text (ratatui). Auto-detects terminal
capability, or override with `--pixel` / `--text`. This is the approach that becomes
the production engine.

```bash
cd pocs/dual-poc
cargo run --release                # auto-detect mode
cargo run --release -- --pixel     # force pixel mode
cargo run --release -- --text      # force text mode
```

### `tiered-poc/` — adaptive viuer quality (evaluated, not chosen)

Single viuer pipeline with adaptive canvas size and effect complexity. Rejected because
the half-block fallback still looked mediocre.

```bash
cd pocs/tiered-poc
cargo run --release
```

### `hybrid-poc/` — ratatui layout + pixel overlay (evaluated, not chosen)

Ratatui for layout/HUD everywhere, pixel overlay for game area. Rejected due to
integration complexity and flicker at the overlay boundary.

```bash
cd pocs/hybrid-poc
cargo run --release
cargo run --release -- --text
```

---

## Dependency Versions

```toml
[dependencies]
# Shared
crossterm  = "0.28"

# Pixel renderer
image      = "0.25"
imageproc  = "0.25"
viuer      = "0.11"

# Text renderer
ratatui    = "0.29"

# Physics (to be added for production):
rapier2d   = "0.22"
```

---

## Terminal Compatibility

| Terminal | Renderer | Fidelity |
|---|---|---|
| kitty | Pixel (Kitty Graphics Protocol) | Full pixel, fastest |
| WezTerm | Pixel (Kitty Graphics Protocol) | Full pixel |
| Ghostty | Pixel (Kitty Graphics Protocol) | Full pixel |
| xterm (256color) | Pixel (Sixel) | Full pixel |
| foot | Pixel (Sixel) | Full pixel |
| iTerm2 | Pixel (iTerm2 inline) | Full pixel |
| tmux (passthrough) | Pixel (Sixel, with passthrough config) | Full pixel |
| Any ANSI terminal | Text (ratatui) | Cell-based, purpose-built |
| SSH / headless | Text (ratatui) | Cell-based, works everywhere |

SSH-safe: no GPU, no display server, no window manager required. Runs anywhere Rust runs.
The text renderer ensures full playability on any terminal — it is not a degraded fallback
but a purpose-built rendering path.

---

## What's Not In The Stack

| Thing | Why excluded |
|---|---|
| Bevy | Version coupling, rendering model mismatch, overhead — see §3 |
| Notcurses | C library version mismatch with system packages; `viuer` is a pure-Rust equivalent |
| GPU renderer | Terminal is the display — no GPU pipeline needed |
| Asset pipeline | Parts are drawn procedurally in code; no sprite files to load |
| bevy_ratatui_camera | Bridges GPU->terminal; unnecessary in our approach |
| viuer half-block fallback | Replaced by the purpose-built text renderer using ratatui |
