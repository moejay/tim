# Pipes

All pipe parts belong to the `PipeType` enum and the "Pipes" category.
Every pipe is a **static** part with infinite mass, meaning it never moves during simulation.
Pipes serve as enclosed conduits that route objects (balls, cannonballs, etc.) through the playfield
along predetermined paths. Objects entering an open pipe end travel through connected pipe segments
and exit at the other open end, subject to gravity and momentum.

---

## Straight Pipe

### Overview
- **Size**: 64x24 px (default); length is editable
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.2
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: false
- **is_resizable**: false (length controlled via property)
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: true
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `length` | 32.0 | 192.0 | 32.0 | 64.0 | Length |
| `orientation` | 0.0 | 1.0 | 1.0 | 0.0 | Orient (0=H/1=V) |

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | Static pipe segment | Standard metallic appearance |

### Pixel Rendering

**Orientation 0 (Horizontal):**
- **Body**: Vertical gradient rectangle from `[180,180,190]` (top) to `[120,120,130]` (bottom), positioned at `(x, y+4)`, size `length x 16` px. Simulates a cylindrical metal tube with top highlight and bottom shadow.
- **Center highlight**: Horizontal line from `(x, y+8)` to `(x+length, y+8)` in `[210,210,220]` at alpha 120. Creates a specular reflection running along the tube center.
- **Left rim**: Filled rectangle at `(x, y+3)`, size `2x18` px, color `[140,140,150]` (full alpha). Darker metal lip at the opening.
- **Right rim**: Filled rectangle at `(x+length-2, y+3)`, size `2x18` px, color `[140,140,150]` (full alpha). Matching darker metal lip.

**Orientation 1 (Vertical):**
- **Body**: Vertical gradient rectangle at `(x+4, y)`, size `16 x length` px, colors `[180,180,190]` to `[120,120,130]`.
- **Center highlight**: Vertical line from `(x+12, y)` to `(x+12, y+length)` in `[210,210,220]` at alpha 120.
- **Top rim**: Filled rectangle at `(x+3, y)`, size `18x2` px, color `[140,140,150]`.
- **Bottom rim**: Filled rectangle at `(x+3, y+length-2)`, size `18x2` px, color `[140,140,150]`.

### Text/Braille Rendering
- **Icon character**: `\u{2550}` (BOX DRAWINGS DOUBLE HORIZONTAL)
- **Icon color**: GRAY `[160,160,160]`
- Rendered as a single character at center of the allocated area.

### Animations
None. `has_animation` returns false.

### Interactions
- Objects (balls, cannonballs, etc.) enter through either open end and travel through the pipe interior.
- Objects maintain momentum and are influenced by gravity while inside the pipe.
- The outer surface of the pipe can serve as a ramp (`can_be_ramp` = true), allowing balls to roll along the top or bottom of the pipe exterior.
- Pipe segments must be connected end-to-end to form continuous routes; a gap between pipe openings causes objects to exit into open space.
- Indestructible by dynamite.

---

## T-Connector

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.2
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: false
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: true
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `rotation` | 0.0 | 3.0 | 1.0 | 0.0 | Rotation (0-3) |

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | Static pipe segment | Standard metallic appearance |

### Pixel Rendering

- **Main body**: Filled rectangle at `(x, y+4)`, size `24x16` px, color GRAY `[160,160,160]` (full alpha). This is the horizontal through-channel of the T.
- **Branch stub** (depends on `rotation % 4`):
  - **Rotation 0**: Branch extends upward. Filled rectangle at `(x+8, y)`, size `8x8` px, same GRAY color. The T-junction opens toward the top.
  - **Rotation 1**: Branch extends to the right. Filled rectangle at `(x+16, y+8)`, size `8x8` px.
  - **Rotation 2**: Branch extends downward. Filled rectangle at `(x+8, y+16)`, size `8x8` px.
  - **Rotation 3**: Branch extends to the left. Filled rectangle at `(x, y+8)`, size `8x8` px.
- **Center highlight**: Horizontal line from `(x, y+8)` to `(x+24, y+8)` in `[200,200,210]` at alpha 100. Specular reflection along the main channel.

### Text/Braille Rendering
- **Icon character**: `\u{2566}` (BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL)
- **Icon color**: GRAY `[160,160,160]`

### Animations
None.

### Interactions
- 3-way junction piece for pipe networks. Objects entering from any of the three openings can exit through either of the other two, governed by momentum and gravity.
- When an object arrives at the junction, its trajectory determines which exit it takes. Objects with strong horizontal momentum continue straight through; objects with weaker momentum or affected by gravity may take the branch.
- Pipe flow rules at T-junctions:
  - An object moving horizontally through the main channel will pass straight through unless gravity pulls it into the branch (e.g., branch faces downward).
  - An object entering from the branch follows gravity and momentum to choose between the two main-channel exits.
- The outer surface can serve as a ramp (`can_be_ramp` = true).
- Indestructible by dynamite.

---

## Curved Pipe

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.2
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: false
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: true
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `rotation` | 0.0 | 3.0 | 1.0 | 0.0 | Rotation (0-3) |

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | Static pipe segment | Standard metallic appearance |

### Pixel Rendering

- **Arc**: A 90-degree arc drawn as a sweep of 90 angle steps, each with 8 thickness layers.
  - Center of the arc is at `(x+12, y+12)`.
  - For each angle step `t` in `0..90`:
    - `angle = (t + rotation * 90) * PI / 180` (rotation offsets the arc by 90-degree increments).
    - For each thickness layer `0..8`:
      - Radius `r = 8.0 + thickness`.
      - Pixel position: `px = x + 12 + cos(angle) * r`, `py = y + 12 + sin(angle) * r`.
      - Color: layers 0-2 use shade `180`, layers 3-7 use shade `140`. This creates a lighter inner edge and darker outer edge. RGB is `[shade, shade, shade+10]` at full alpha, blended onto the image.
  - **Rotation semantics**:
    - **Rotation 0**: Arc sweeps 0-90 degrees (right-to-down bend).
    - **Rotation 1**: Arc sweeps 90-180 degrees (down-to-left bend).
    - **Rotation 2**: Arc sweeps 180-270 degrees (left-to-up bend).
    - **Rotation 3**: Arc sweeps 270-360 degrees (up-to-right bend).

### Text/Braille Rendering
- **Icon character**: `\u{256E}` (BOX DRAWINGS LIGHT ARC DOWN AND LEFT)
- **Icon color**: GRAY `[160,160,160]`

### Animations
None.

### Interactions
- 90-degree bend piece for pipe networks. Changes the direction of an object's travel by 90 degrees.
- Objects entering from one opening are redirected to the perpendicular opening, with momentum partially preserved (reduced by friction 0.2 and the pipe's elasticity 0.1).
- Critical for creating complex pipe routes that navigate around obstacles or change elevation.
- Must be aligned with adjacent straight pipes or T-connectors so that openings match.
- The outer surface can serve as a ramp (`can_be_ramp` = true).
- Indestructible by dynamite.

---

## Accelerator Tube

### Overview
- **Size**: 32x24 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.2
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: true
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: true
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `speed_mult` | 1.5 | 3.0 | 0.5 | 2.0 | Speed Mult |
| `direction` | 0.0 | 3.0 | 1.0 | 0.0 | Direction (0-3) |

Available speed multiplier settings: 1.5x, 2.0x, 2.5x, 3.0x.
Direction values: 0=right, 1=down, 2=left, 3=up.

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | No object inside | Dim cyan tube with static arrows |
| 1 | Accelerating | Object passing through -- speed multiplied | Bright glowing tube with fast scrolling arrows and object streak |

### Pixel Rendering

**Both states share:**
- **Base brightness**: State 1 (Accelerating) uses `base_bright = 220`; State 0 (Idle) uses `base_bright = 140`.
- **Body**: Vertical gradient rectangle at `(x, y+4)`, size `32x16` px. Top color: `[0, base_bright*0.8, base_bright]`, bottom color: `[0, base_bright*0.5, base_bright*0.6]`. Creates a cyan-tinted glowing tube.

**State 0 (Idle):**
- **Inner glow**: Filled rectangle at `(x+2, y+6)`, size `28x12` px, color `[0,160,180]` at alpha 60. Dim interior.
- **Static arrows**: Two chevron arrows at x-offsets 8 and 24 (stepping by 16), drawn as two lines each:
  - Upper line: from `(x+ax-3, y+12)` to `(x+ax, y+9)`, color `[200,200,200]` at alpha 100.
  - Lower line: from `(x+ax-3, y+12)` to `(x+ax, y+15)`, color `[200,200,200]` at alpha 100.
  - These form right-pointing chevrons indicating flow direction.

**State 1 (Accelerating):**
- **Bright inner glow**: Pulsing rectangle at `(x+2, y+6)`, size `28x12` px, color `[0,240,255]` with animated alpha.
  - Alpha formula: `glow = sin(frame * 0.2) * 0.2 + 0.8`, `alpha = glow * 140`. Oscillates between approximately 84 and 140 alpha, creating a pulsing glow effect.
- **Fast scrolling arrows**: Arrow speed scales with `speed_mult`: `arrow_speed = speed_mult * 3` (as integer). Phase: `arrow_phase = (frame * arrow_speed) % 16`. Arrows placed at `arrow_phase, arrow_phase+8, arrow_phase+16, ...` across the 32px width (step 8), skipping positions <= 0 or >= 28. Same chevron shape as idle but brighter: `[255,255,255]` at alpha 220.
- **Object streak**: A filled circle representing the object being accelerated. Position: `obj_x = x + (frame * 4) % 32`, `obj_y = y + 12`. Radius 3.0, color `[255,255,200]` at alpha 150. Simulates a bright object zooming through the tube.

### Text/Braille Rendering
- **Icon character**: `\u{21D2}` (RIGHTWARDS DOUBLE ARROW)
- **Icon color**: CYAN `[0,200,220]`

### Animations

| Animation | Trigger | Formula | Description |
|---|---|---|---|
| Inner glow pulse | State 1 (Accelerating) | `alpha = (sin(frame * 0.2) * 0.2 + 0.8) * 140` | Cyan glow oscillates in brightness |
| Arrow scrolling | State 1 (Accelerating) | `phase = (frame * floor(speed_mult * 3)) % 16` | Arrows scroll faster at higher speed multipliers |
| Object streak | State 1 (Accelerating) | `obj_x = x + (frame * 4) % 32` | Bright dot races through the tube |

### Interactions
- When an object enters the accelerator tube, the part transitions from state 0 (Idle) to state 1 (Accelerating).
- The object's velocity is multiplied by the `speed_mult` value (1.5x, 2.0x, 2.5x, or 3.0x) in the configured `direction`.
- The acceleration is applied once as the object passes through; it is not cumulative over time inside the tube.
- Multiple accelerator tubes can be chained in sequence for compound speed boosts.
- The `direction` property determines which way the speed boost is applied (0=right, 1=down, 2=left, 3=up), allowing tubes to redirect objects as well as accelerate them.
- After the object exits, the tube returns to state 0 (Idle).
- The outer surface can serve as a ramp (`can_be_ramp` = true).
- Indestructible by dynamite.

---

## Pipe System Flow Rules

1. **Connectivity**: Pipe segments must have their openings aligned and adjacent (within collision tolerance) to form a connected route. A gap between openings causes the object to exit into open space at that point.

2. **Entry**: Any gravity-affected object (ball, cannonball, etc.) that reaches a pipe opening enters the pipe system. The object's velocity at the point of entry is preserved as its initial pipe velocity.

3. **Traversal**: Inside pipes, objects follow the pipe geometry:
   - In straight pipes, objects travel along the pipe axis.
   - At curved pipes, objects change direction by 90 degrees.
   - At T-connectors, objects choose an exit based on their momentum vector and gravity.
   - At accelerator tubes, objects receive a speed multiplier boost.

4. **Gravity inside pipes**: Gravity continues to act on objects inside pipes. In vertical sections, objects accelerate downward (or decelerate upward). In horizontal sections, gravity has no direct effect on horizontal speed but may cause objects to settle to the bottom of the pipe interior.

5. **Friction inside pipes**: The pipe friction of 0.2 applies to objects sliding along pipe walls, gradually reducing their speed.

6. **Exit**: Objects exit from any open pipe end (an opening not connected to another pipe segment). Exit velocity is the object's current velocity at the opening.

7. **Branching at T-connectors**: When an object reaches a T-junction:
   - If the object has strong momentum along the main channel axis, it continues straight through.
   - If the branch direction aligns with gravity and the object has low horizontal momentum, it diverts into the branch.
   - The branch decision is deterministic based on velocity vector and gravity.

8. **Object size**: Objects must fit inside the pipe diameter (16px internal height for standard pipes, 24px external). Objects larger than the pipe interior cannot enter.

9. **Multiple objects**: Multiple objects can be inside the pipe system simultaneously. They collide with each other inside pipes using standard collision physics (elasticity, momentum transfer).

10. **No backflow prevention**: Pipes do not enforce one-way flow (except accelerator tubes which impart directional velocity). An object can travel in either direction through any pipe segment.
