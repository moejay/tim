# TIM2 Game Specs Index

Feature specifications for The Incredible Machine 2 rebuild. Each spec is implementation-ready for coding agents.

## Specs

| # | Feature | File | Description |
|---|---|---|---|
| 01 | Physics Engine | [01-physics-engine/v1.md](01-physics-engine/v1.md) | Gravity, collision, elasticity, friction, deterministic simulation |
| 02 | Parts Core | [02-parts-core/v1.md](02-parts-core/v1.md) | Part trait, registry, balls, walls, floors, inclines |
| 03 | Mechanical Parts | [03-parts-mechanical/v1.md](03-parts-mechanical/v1.md) | Gears, pulleys, belts, levers, conveyors, trampolines, scissors, vacuum |
| 04 | Electrical System | [04-parts-electrical/v1.md](04-parts-electrical/v1.md) | Power sources, outlets, fans, motors, toasters, wiring rules |
| 05 | Pyrotechnic Parts | [05-parts-pyrotechnic/v1.md](05-parts-pyrotechnic/v1.md) | Light sources, magnifying glass, fuses, dynamite, cannons, rockets |
| 06 | Animals & Characters | [06-parts-animals/v1.md](06-parts-animals/v1.md) | Cat, mouse, cheese, alligator, Mel, fishbowl, AI state machines |
| 07 | Gadgets & Specialty | [07-parts-gadgets/v1.md](07-parts-gadgets/v1.md) | Ray gun, timer, hooks, laundry basket, gun, anti-gravity, boxes, buckets |
| 08 | Rope & Cable | [08-rope-and-cable/v1.md](08-rope-and-cable/v1.md) | Rope physics, tension, pulleys, attachment points, cutting |
| 09 | Pipe System | [09-pipe-system/v1.md](09-pipe-system/v1.md) | Pipe routing, object transport, connections, accelerators |
| 10 | Laser System | [10-laser-system/v1.md](10-laser-system/v1.md) | Colored lasers, mirrors, mixing, beam tracing, interactions |
| 11 | Collisions & Interactions | [11-collision-and-interactions/v1.md](11-collision-and-interactions/v1.md) | Full interaction matrix, collision layers, event system, update order |
| 12 | Puzzle System | [12-puzzle-system/v1.md](12-puzzle-system/v1.md) | Puzzle format, win conditions, parts bin, goal evaluation, file format |
| 13 | Game Modes | [13-game-modes/v1.md](13-game-modes/v1.md) | Puzzle Play, Workshop, Head-to-Head, Guided Tour |
| 14 | UI & Controls | [14-ui-and-controls/v1.md](14-ui-and-controls/v1.md) | Layout, control panel, parts bin, handles, keyboard shortcuts |
| 15 | Level Editor | [15-level-editor/v1.md](15-level-editor/v1.md) | Workshop workflow, solution programming, scenery, validation |
| 16 | Audio | [16-audio/v1.md](16-audio/v1.md) | Sound effects, background music, sound engine |
| 17 | Rendering | [17-rendering/v1.md](17-rendering/v1.md) | Dual renderer (pixel + text), sprites, text characters, camera |
| 18 | Scoring & Progression | [18-scoring-and-progression/v1.md](18-scoring-and-progression/v1.md) | Player profiles, tier progression, passwords, H2H scoring |
| 19 | Game Flow | [19-game-flow/v1.md](19-game-flow/v1.md) | Complete screen flow, menus, dialogs, sign-in, workshop workflow, H2H rules |

## Dependency Graph

```
01-Physics ──────────────────────────┐
    │                                │
02-Parts-Core ───────────────────────┤
    │                                │
    ├── 03-Mechanical ──┐            │
    ├── 04-Electrical ──┤            │
    ├── 05-Pyrotechnic ─┤            │
    ├── 06-Animals ─────┤            │
    ├── 07-Gadgets ─────┤            ├── 11-Collisions
    ├── 08-Rope ────────┤            │
    ├── 09-Pipes ───────┤            │
    └── 10-Lasers ──────┘            │
                                     │
12-Puzzle-System ────────────────────┤
    │                                │
    ├── 13-Game-Modes                │
    ├── 15-Level-Editor              │
    └── 18-Scoring                   │
                                     │
14-UI ───────────────────────────────┤
16-Audio ────────────────────────────┤
17-Rendering ────────────────────────┤
19-Game-Flow (sign-in → menus → play)┘
```

## Versioning

Each spec starts at `v1.md`. As we iterate:
- `v1.md` — initial spec (current)
- `v2.md` — post-implementation revisions
- `v3.md` — refinements after playtesting
