# Lasers

All laser parts belong to the `LaserType` enum and the "Lasers" category.
Every laser part is a **static** part with infinite mass. The laser system provides
beam-based interactions: emitting colored beams, reflecting them with mirrors,
combining colors with a mixer, detecting beams with sensors, and activating
color-specific power outlets.

---

## Red Laser

### Overview
- **Size**: 24x12 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: true
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `rotation` | 0.0 | 3.0 | 1.0 | 0.0 | Direction (0-3) |

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Off | Not emitting | Dim lens; no beam |
| 1 | Emitting | Beam active -- ignites/pops on contact | Bright lens with glow; visible beam extending outward |

### Pixel Rendering

**Laser color**: `[255, 40, 40]` (bright red).

**Emitter body** (both states): Filled rectangle at `(x, y)`, size `16x12` px, color `[80,80,90]` (full alpha). Dark metallic housing.

**State 0 (Off):**
- **Dim lens**: Filled rectangle at `(x+16, y+2)`, size `4x8` px, color `[255/3, 40/3, 40/3]` = `[85, 13, 13]` at alpha 200. The lens is visible but unlit.

**State 1 (Emitting):**
- **Bright lens**: Filled rectangle at `(x+16, y+2)`, size `4x8` px, color `[255, 40, 40]` (full alpha).
- **Lens glow**: Circular glow effect centered at `(x+18, y+6)`, radius 6.0, color `[255, 40, 40]`. A soft radial bloom around the lens.
- **Laser beam**: 40 pixels long, extending rightward from `x+20`. For each beam pixel `bx` in `0..40`:
  - Center pixel at `(x+20+bx, y+6)`: color `[255, 40, 40]`, alpha `= 220 - bx*4`, with per-frame flicker: `alpha -= sin(frame*0.3 + bx*0.1) * 20`. The beam fades and flickers along its length.
  - Upper fringe at `(x+20+bx, y+5)`: same color, alpha divided by 3. Soft glow above the beam center.
  - Lower fringe at `(x+20+bx, y+7)`: same color, alpha divided by 3. Soft glow below the beam center.

### Text/Braille Rendering
- **Icon character**: `\u{2500}` (BOX DRAWINGS LIGHT HORIZONTAL)
- **Icon color**: RED `[220, 40, 40]`

### Animations

| Animation | Trigger | Formula | Description |
|---|---|---|---|
| Beam flicker | State 1 (Emitting) | `flicker = sin(frame * 0.3 + bx * 0.1) * 20` | Per-pixel alpha oscillation along beam length |

### Interactions
- **Ignites fuses**: Beam contact with a fuse lights it.
- **Ignites candles**: Beam contact with a candle wick lights the candle.
- **Pops balloons**: Beam contact with a balloon pops it instantly.
- **Reflects off mirrors**: Beam hitting an Angled Mirror redirects 90 degrees based on mirror rotation.
- **Detected by sensors**: Beam hitting a Laser Detector triggers it (any color).
- **Activates plugs**: Beam hitting a Laser-Activated Plug set to color 0 (Red) activates it.
- **Mixes in prism**: Beam entering a Laser Mixer combines with other beams.
- **Blocked by opaque objects**: Beam stops at any solid part (walls, pipes, balls, etc.).
- Beam travels in a straight line in the configured `rotation` direction until hitting an object or exiting the playfield.

---

## Green Laser

### Overview
- **Size**: 24x12 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: true
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `rotation` | 0.0 | 3.0 | 1.0 | 0.0 | Direction (0-3) |

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Off | Not emitting | Dim lens; no beam |
| 1 | Emitting | Beam active -- ignites/pops on contact | Bright lens with glow; visible beam extending outward |

### Pixel Rendering

**Laser color**: `[40, 255, 40]` (bright green).

Identical structure to Red Laser but with green color values:

**Emitter body**: `[80,80,90]`, size `16x12` px.

**State 0 (Off):**
- **Dim lens**: `[40/3, 255/3, 40/3]` = `[13, 85, 13]` at alpha 200, at `(x+16, y+2)`, size `4x8`.

**State 1 (Emitting):**
- **Bright lens**: `[40, 255, 40]` at `(x+16, y+2)`, size `4x8`.
- **Lens glow**: Center `(x+18, y+6)`, radius 6.0, color `[40, 255, 40]`.
- **Laser beam**: 40px long from `x+20`, center pixel `[40, 255, 40]` with fading alpha `220 - bx*4` and flicker `sin(frame*0.3 + bx*0.1) * 20`. Upper/lower fringe at alpha/3.

### Text/Braille Rendering
- **Icon character**: `\u{2500}` (BOX DRAWINGS LIGHT HORIZONTAL)
- **Icon color**: GREEN `[50, 180, 50]`

### Animations

| Animation | Trigger | Formula | Description |
|---|---|---|---|
| Beam flicker | State 1 (Emitting) | `flicker = sin(frame * 0.3 + bx * 0.1) * 20` | Per-pixel alpha oscillation along beam length |

### Interactions
Identical to Red Laser. Additionally:
- Activates Laser-Activated Plugs set to color 1 (Green).

---

## Blue Laser

### Overview
- **Size**: 24x12 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: true
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `rotation` | 0.0 | 3.0 | 1.0 | 0.0 | Direction (0-3) |

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Off | Not emitting | Dim lens; no beam |
| 1 | Emitting | Beam active -- ignites/pops on contact | Bright lens with glow; visible beam extending outward |

### Pixel Rendering

**Laser color**: `[40, 80, 255]` (bright blue).

Identical structure to Red Laser but with blue color values:

**Emitter body**: `[80,80,90]`, size `16x12` px.

**State 0 (Off):**
- **Dim lens**: `[40/3, 80/3, 255/3]` = `[13, 26, 85]` at alpha 200, at `(x+16, y+2)`, size `4x8`.

**State 1 (Emitting):**
- **Bright lens**: `[40, 80, 255]` at `(x+16, y+2)`, size `4x8`.
- **Lens glow**: Center `(x+18, y+6)`, radius 6.0, color `[40, 80, 255]`.
- **Laser beam**: 40px long from `x+20`, center pixel `[40, 80, 255]` with fading alpha `220 - bx*4` and flicker `sin(frame*0.3 + bx*0.1) * 20`. Upper/lower fringe at alpha/3.

### Text/Braille Rendering
- **Icon character**: `\u{2500}` (BOX DRAWINGS LIGHT HORIZONTAL)
- **Icon color**: BLUE `[50, 100, 220]`

### Animations

| Animation | Trigger | Formula | Description |
|---|---|---|---|
| Beam flicker | State 1 (Emitting) | `flicker = sin(frame * 0.3 + bx * 0.1) * 20` | Per-pixel alpha oscillation along beam length |

### Interactions
Identical to Red Laser. Additionally:
- Activates Laser-Activated Plugs set to color 2 (Blue).

---

## Angled Mirror

### Overview
- **Size**: 16x16 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: false
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
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
| 0 | Idle | Passive component | Static mirror surface |

### Pixel Rendering

- **Mirror surface**: An anti-aliased diagonal line across the 16x16 area, color `[220, 220, 230]` (bright silver).
  - **Rotation 0 or 2**: Line from `(x, y+16)` to `(x+16, y)` -- bottom-left to top-right diagonal (backslash orientation when viewed as reflecting a rightward beam upward, or a downward beam leftward).
  - **Rotation 1 or 3**: Line from `(x, y)` to `(x+16, y+16)` -- top-left to bottom-right diagonal (forward-slash orientation).
- **Base/stand**: Filled rectangle at `(x+4, y+14)`, size `8x2` px, color `[120, 120, 130]`. Small support base below the mirror.
- **Center highlight**: Single blended pixel at `(x+8, y+8)`, color `[255, 255, 255]` at alpha 180. A bright specular point at the mirror center.

### Text/Braille Rendering
- **Icon character**: `\u{2571}` (BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO LOWER LEFT)
- **Icon color**: WHITE `[240, 240, 240]`

### Animations
None.

### Interactions
- Redirects any laser beam (red, green, blue, or mixed) by 90 degrees.
- **Beam reflection rules by rotation**:
  - **Rotation 0** (backslash `\` diagonal): A beam arriving from the left is redirected upward. A beam arriving from below is redirected to the right.
  - **Rotation 1** (forward-slash `/` diagonal): A beam arriving from the left is redirected downward. A beam arriving from above is redirected to the right.
  - **Rotation 2** (backslash `\` diagonal): Same geometry as rotation 0 (renders identically). Beam from right redirected downward; beam from above redirected leftward.
  - **Rotation 3** (forward-slash `/` diagonal): Same geometry as rotation 1. Beam from right redirected upward; beam from below redirected leftward.
- The mirror preserves beam color -- it does not alter the laser's wavelength/color.
- Multiple mirrors can be chained to route a beam around corners and obstacles.
- The mirror does not absorb, split, or attenuate the beam.
- Physical objects (balls, etc.) do not interact with the mirror surface in any special way beyond standard collision.

---

## Laser Mixer

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: false
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

None. The Laser Mixer has no editable properties.

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | Passive component | Static prism with input dots and center glow |

### Pixel Rendering

- **Prism body**: Two filled triangles forming a diamond/prism shape:
  - Left triangle: vertices at `(x+12, y)`, `(x, y+12)`, `(x+12, y+24)`, color `[220, 220, 240]` at alpha 200 (translucent crystalline).
  - Right triangle: vertices at `(x+12, y)`, `(x+24, y+12)`, `(x+12, y+24)`, color `[220, 220, 240]` at alpha 200.
- **Input indicators** (left side, showing the three color channels):
  - Red dot: single pixel at `(x, y+6)`, color `[255, 40, 40]`.
  - Green dot: single pixel at `(x, y+12)`, color `[40, 255, 40]`.
  - Blue dot: single pixel at `(x, y+18)`, color `[40, 80, 255]`.
- **Center glow**: Glow effect centered at `(x+12, y+12)`, radius 6.0, color `[255, 255, 255]`. White radial bloom at the prism center where beams combine.

### Text/Braille Rendering
- **Icon character**: `\u{25C7}` (WHITE DIAMOND)
- **Icon color**: WHITE `[240, 240, 240]`

### Animations
None.

### Interactions

The Laser Mixer combines colored laser beams using additive color mixing:

**Color Mixing Rules:**

| Input Beams | Output Color | RGB |
|---|---|---|
| Red only | Red | `[255, 40, 40]` |
| Green only | Green | `[40, 255, 40]` |
| Blue only | Blue | `[40, 80, 255]` |
| Red + Green | Yellow | `[255, 255, 40]` |
| Red + Blue | Magenta/Purple | `[255, 40, 255]` |
| Green + Blue | Cyan | `[40, 255, 255]` |
| Red + Green + Blue | White | `[255, 255, 255]` |

- Input beams enter from the left side of the prism (where the three input indicator dots are).
- The mixed output beam exits from the right side of the prism.
- The prism does not alter beam direction -- input from the left, output to the right.
- A single beam passing through unchanged is valid (acts as a pass-through).
- The mixed output beam can interact with all the same targets as individual beams: mirrors, detectors, plugs, fuses, candles, balloons.
- A Laser-Activated Plug responds only to the specific color it is configured for. A mixed-color beam will NOT activate a plug unless the mixed result matches the configured color (e.g., a yellow beam from R+G will not activate a Red plug or a Green plug).

---

## Laser Detector

### Overview
- **Size**: 16x16 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: true
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

None. The Laser Detector has no editable properties.

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | No beam hitting sensor | Dim housing with periodic red LED blink |
| 1 | Triggered | Beam detected -- signal active | Green LED lit with glow |

### Pixel Rendering

**Both states:**
- **Housing**: Filled rectangle at `(x+2, y+2)`, size `12x12` px, color `[80, 80, 90]`. Dark metallic sensor body.
- **Sensor well**: Filled circle centered at `(x+8, y+8)`, radius 4.0, color `[40, 40, 50]`. Dark recessed area where the sensor sits.

**State 0 (Idle):**
- **Red LED blink**: Every 40 frames, for 5 frames (`frame % 40 < 5`), a small red dot appears:
  - Filled circle at `(x+8, y+8)`, radius 1.5, color `[200, 50, 50]` at alpha 180.
  - This is a "standby" indicator -- a brief red flash every ~0.67 seconds (at 60fps).

**State 1 (Triggered):**
- **Green LED**: Filled circle at `(x+8, y+8)`, radius 2.0, color `[100, 255, 100]` (full alpha). Solid green indicator.
- **Green glow**: Glow effect at `(x+8, y+8)`, radius 6.0, color `[100, 255, 100]`. Radial bloom around the lit LED.

### Text/Braille Rendering
- **Icon character**: `\u{25C9}` (FISHEYE)
- **Icon color**: WHITE `[240, 240, 240]`

### Animations

| Animation | Trigger | Formula | Description |
|---|---|---|---|
| Red LED blink | State 0 (Idle) | `visible = frame % 40 < 5` | Brief red flash every 40 frames as standby indicator |

### Interactions
- Triggers on **any** laser beam color (red, green, blue, or any mixed color).
- When a beam hits the detector, it transitions from state 0 (Idle) to state 1 (Triggered).
- The triggered signal can be used by the puzzle's solution recognition system to verify win conditions (e.g., "activate the laser detector").
- The detector does NOT discriminate by color -- any beam activates it. For color-specific activation, use the Laser-Activated Plug instead.
- When the beam is interrupted (laser turned off, mirror moved, beam blocked), the detector returns to state 0 (Idle).
- The detector does not reflect, absorb, or pass through the beam -- the beam terminates at the detector.
- Multiple detectors can be placed to require multiple beam paths in a puzzle.

---

## Laser-Activated Plug

### Overview
- **Size**: 16x24 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: true
- **has_animation**: false
- **is_resizable**: false
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: true

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `activation_color` | 0.0 | 2.0 | 1.0 | 0.0 | Color (0=R/1=G/2=B) |

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Inactive | Wrong color or no laser | Dim lens; no power indicator |
| 1 | Active | Correct laser color -- providing power | Bright glowing lens; green power indicator dot |

### Pixel Rendering

**Activation color mapping:**
- `activation_color = 0`: lens uses `[255, 40, 40]` (Red)
- `activation_color = 1`: lens uses `[40, 255, 40]` (Green)
- `activation_color = 2`: lens uses `[40, 80, 255]` (Blue)

**Both states:**
- **Outlet body**: Filled rectangle at `(x+2, y+6)`, size `12x18` px, color `[220, 215, 210]` (off-white, like a wall outlet plate).
- **Plug holes**: Two dark rectangles representing standard outlet sockets:
  - Left socket: `(x+4, y+14)`, size `3x4` px, color `[40, 40, 40]`.
  - Right socket: `(x+9, y+14)`, size `3x4` px, color `[40, 40, 40]`.

**State 0 (Inactive):**
- **Dim lens**: Filled circle at `(x+8, y+4)`, radius 3.0, color at 1/3 brightness: `[lens_r/3, lens_g/3, lens_b/3]` at alpha 200. The color sensor is visible but dark.

**State 1 (Active):**
- **Bright lens**: Filled circle at `(x+8, y+4)`, radius 3.0, full lens color at full alpha.
- **Lens glow**: Glow effect at `(x+8, y+4)`, radius 5.0, full lens color. Radial bloom indicating activation.
- **Power indicator**: Single blended pixel at `(x+8, y+10)`, color `[100, 255, 100]` at alpha 200. Small green dot confirming power is flowing.

### Text/Braille Rendering
- **Icon character**: `\u{25C9}` (FISHEYE)
- **Icon color**: WHITE `[240, 240, 240]`

### Animations
None.

### Interactions
- **Color-specific activation**: Only activates when hit by a laser beam matching its configured `activation_color` (0=Red, 1=Green, 2=Blue).
- A red beam activates a plug set to 0. A green beam activates a plug set to 1. A blue beam activates a plug set to 2.
- **Mixed beams**: A mixed-color beam (e.g., yellow from R+G) does NOT activate any single-color plug. The beam color must exactly match the configured color.
- **Provides power**: When active (state 1), the plug functions as an electrical outlet. Up to two electrical devices can be plugged into it (standard outlet rules: two sockets per outlet).
- **Powered devices**: Any device requiring power (Electric Fan, Electric Motor, Vacuum Cleaner, Toaster, Can Opener, Electric Mixer) can be connected.
- **Power loss**: If the laser beam is interrupted (laser turned off, mirror repositioned, beam blocked by an object), the plug immediately transitions back to state 0 (Inactive) and all connected devices lose power.
- This creates dynamic power systems where the beam path must be maintained throughout the puzzle.
- The beam terminates at the plug -- it does not pass through.

---

## Laser Beam Tracing Rules

1. **Ray casting**: Each laser emitter casts a ray in its configured direction (0=right, 1=down, 2=left, 3=up) starting from the lens position. The ray is a straight line extending until it hits an object or exits the playfield.

2. **First-hit determination**: The beam travels pixel-by-pixel (or via ray-intersection math) and stops at the first solid object it contacts. The beam does not penetrate or pass through solid objects.

3. **Mirror reflection**: If the first hit is an Angled Mirror, the beam direction changes by 90 degrees (as determined by the mirror's rotation) and a new ray is cast from the mirror's center in the new direction. This process can chain through multiple mirrors.

4. **Maximum reflections**: To prevent infinite loops (e.g., two mirrors facing each other), beam tracing should be capped at a maximum number of reflections (implementation-defined, typically 16-32 bounces).

5. **Mixer combination**: If a beam enters a Laser Mixer, it is absorbed. The mixer tracks all incoming beam colors and produces a single output beam (additive mix) from its right side.

6. **Detector/Plug termination**: Beams terminate at Laser Detectors and Laser-Activated Plugs. They do not reflect or pass through these components.

7. **Destructive interactions**: When a beam contacts a balloon, fuse, or candle wick, the interaction triggers immediately (pop, ignite, light). The beam continues past the destroyed target (a popped balloon no longer blocks the beam).

8. **Dynamic obstacles**: If a physical object (ball, animal, etc.) moves into the beam path, it blocks the beam. When the object moves away, the beam resumes. This creates timing-based puzzle opportunities.

9. **Beam visibility**: The beam is rendered as a visible line in the emitter's color. The beam rendering includes a center line at full brightness and upper/lower fringe pixels at 1/3 alpha for a soft-edge glow effect.

10. **No beam splitting**: A single laser emitter produces exactly one beam. Beams cannot be split. To achieve multiple beam paths, use multiple emitters or reflect a single beam through mirrors to hit sequential targets.
