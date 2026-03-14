# TIM2 Project Memory

## Project Overview
Terminal-based recreation of The Incredible Machine 2 (1994 puzzle game).
Full implementation in `game/` directory, 13,400+ lines of Rust.

## Architecture
- **game/src/main.rs** - Entry point, game loop, terminal setup
- **game/src/math.rs** - Vec2, AABB, Color utilities
- **game/src/events.rs** - GameEvent enum, EventQueue
- **game/src/parts/mod.rs** - 93 PartKind variants, PartData struct, create_part factory
- **game/src/physics/mod.rs** - Rapier-style physics (gravity, collision, rest detection)
- **game/src/state.rs** - GameState, Puzzle, WinCondition, PartsBin, modes
- **game/src/render/mod.rs** - Renderer trait
- **game/src/render/text.rs** - ratatui text renderer
- **game/src/render/pixel.rs** - viuer pixel renderer (Kitty/Sixel/iTerm2)
- **game/src/input.rs** - crossterm input, vim-style keys
- **game/src/collision.rs** - Full interaction matrix (spec 11)
- **game/src/laser.rs** - Beam tracing, mirrors, color mixing
- **game/src/rope.rs** - Tension, pulleys, cutting
- **game/src/pipe.rs** - Object transport through pipe networks
- **game/src/puzzle.rs** - 25 built-in puzzles, goal evaluation, JSON save/load
- **game/src/audio.rs** - Stub audio engine (interface ready)
- **game/src/scoring.rs** - Player profiles, tier progression, H2H scoring
- **game/src/modes.rs** - Game mode management (PuzzlePlay, Workshop, H2H, Tour)

## Key Patterns
- GravityResponse: Normal, Immune, Buoyant, Reduced (not AlwaysFalls/ZeroGravity)
- PartData is a flat struct (no trait objects), PartKind is Copy
- physics_tick takes 4 args: parts, events, config, rest_counters (HashMap)
- GameState.physics is PhysicsConfig (not physics_config)
- PartsBin.items is the Vec<PartInventory> (not available)
- BeltConnection has part_a/part_b (not start/end)
- RopeSegment has waypoints (not points)
- LaserColor has 7 variants: Red, Green, Blue, Yellow, Purple, Cyan, White

## Build & Test
- `cd game && cargo build --release` - builds in ~20s
- `cargo test` - 120 tests, all passing
- `cargo run --release -- --text` for text mode
- `cargo run --release -- --pixel` for pixel mode
- Puzzles in `game/puzzles/` as JSON

## Specs
18 specs in `docs/specs/` covering physics, parts, rendering, puzzle system, etc.
See `docs/specs/INDEX.md` for dependency graph.
