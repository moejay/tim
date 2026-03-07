# TIM2 Terminal

The Incredible Machine 2, reimagined as a terminal game.

A physics-based puzzle game where you place parts to guide a cannonball into a basket — rendered in full pixel fidelity directly in your terminal via Kitty/Sixel graphics.

## Screenshot

*(coming soon)*

## Quick Start

```bash
cd game
cargo run --release
```

Requires a terminal with pixel graphics support. See [Terminal Requirements](#terminal-requirements).

## How to Play

Place parts on the playfield to redirect the cannonball from the cannon into the basket. Press Space to run the simulation and watch the chain reaction.

### Controls

| Key | Action |
|---|---|
| `h/j/k/l` | Move cursor (4px) |
| `H/J/K/L` | Move cursor fast (16px) |
| `p` | Place mode — select and place a part |
| `e` | Edit mode — move a placed part |
| `Space` | Run simulation |
| `f` | Flip part |
| `x` | Delete part |
| `u` | Undo |
| `?` | Help overlay |
| `q` | Quit |

### Modes

- **NORMAL** — navigate cursor, manage parts
- **PLACE** — select from parts bin, position, confirm with Enter
- **EDIT** — move/flip/delete a placed part in-place
- **RUN** — physics simulation running, press Esc/Space to stop

## Terminal Requirements

**Full pixel fidelity:** WezTerm, kitty, Ghostty, foot, xterm (Sixel), iTerm2

**Degraded (half-block fallback):** Alacritty, Windows Terminal — playable but visually rough

The game auto-detects the best protocol and warns if rendering will be degraded.

## Tech Stack

| Layer | Choice |
|---|---|
| Language | Rust |
| Canvas | 640x360 px `RgbaImage` |
| Display | `viuer` — auto Kitty / Sixel / half-block |
| Input | `crossterm` raw mode |
| Physics | Manual Euler integration |
| Drawing | Custom `gfx.rs` pixel primitives |

## Project Structure

```
game/src/
  main.rs       — game loop, input, mode state machine
  state.rs      — GameState, Part, PartKind, SimBall, Mode
  physics.rs    — Euler integration, collision resolution
  render.rs     — pixel rendering, bitmap font, UI overlays
  gfx.rs        — fill_circle, fill_rect, draw_line, blend, etc.
  hud.rs        — crossterm HUD text
  puzzle.rs     — hardcoded MVP puzzle
  parts/
    ball.rs, ramp.rs, wall.rs, basket.rs, cannon.rs
```

## Docs

- [MVP Specification](docs/MVP_SPEC.md)
- [Game Engine Stack](docs/GAME_ENGINE.md)
- [TIM2 Game Feature Spec](docs/TIM2_GAME_SPEC.md)
