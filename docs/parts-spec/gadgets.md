# Gadget Parts Specification

---

## Captain Z Super Phazer

### Overview
- **Size**: 32x16 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: Yes
- **Resizable**: No
- **Rope Point**: Yes
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has firing beam effect)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| shots | 1 | 5 | 1 | 3 | Shots |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Ready | Loaded with shots | Bright barrel color |
| 1 | Firing | Emitting blast at ~1800 px/s | Cyan beam + barrel glow |
| 2 | Empty | All shots expended | Dim barrel color |

### Pixel Rendering

- **Body**: Gradient-filled rect at (x+4, y+4), 20x8 px, top `[0, 180, 200]` to bottom `[0, 120, 140]` (cyan gradient).
- **Barrel**: Rect at barrel_x, (y+6), 8x4 px. Position: flipped=(x), unflipped=(x+24). Color: Ready/Firing=`[0, 220, 240, 255]`, Empty=`[0, 100, 110, 255]` (dimmed).
- **Grip**: Filled rect at (x+8, y+12), 8x4 px, color `[0, 140, 160, 255]`.
- **State 1 (Firing)**:
  - Beam: 20 pixel pairs projected from barrel. Direction depends on flip. Each at `bx = barrel_start + i * dir * 2`. Two pixels per column at y+7 and y+8. Color `[0, 255, 255]`, alpha `240 - i*10`.
  - Barrel glow: Glow at (barrel_x+4, y+8), radius 6px, color `[0, 255, 255]`.

### Text/Braille Rendering

- **Icon**: `\u25BA` (BLACK RIGHT-POINTING POINTER)
- **Color**: CYAN `[0, 200, 220]`

### Animations

- **Beam projection** (State 1): 20-pixel-long beam with fading alpha. Two pixels tall, extending in facing direction.

### Interactions

- 1-5 programmable blasts at ~1800 px/s.
- Rope attachment point for activation.
- Blasts can hit movable objects, pushing/destroying them.
- Programmable shot count (1-5).

---

## Egg Timer

### Overview
- **Size**: 16x20 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (sand flow when counting)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| delay | 1.0 | 10.0 | 0.5 | 3.0 | Delay (s) |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Ready | Timer not started | All sand in top chamber |
| 1 | Counting | Countdown in progress | Sand flowing from top to bottom, falling grain |
| 2 | Triggered | Spring arm deployed at ~400 px/s | All sand in bottom, spring arm extended with ball tip |

### Pixel Rendering

- **Upper hourglass**: Filled triangle (x+8, y+10) -> (x+2, y) -> (x+14, y), color `[220, 200, 160, 200]` (glass).
- **Lower hourglass**: Filled triangle (x+8, y+10) -> (x+2, y+20) -> (x+14, y+20), color `[220, 200, 160, 200]`.
- **Top band**: Line from (x+2, y) to (x+14, y), color `[120, 120, 130, 255]` (metal).
- **Bottom band**: Line from (x+2, y+20) to (x+14, y+20), same color.
- **State 0 (Ready)**: Sand triangle in top: (x+8, y+6) -> (x+4, y) -> (x+12, y), color `[200, 180, 120, 255]`.
- **State 1 (Counting)**:
  - Top sand shrinks: `top_r = max(8 - sand_level, 0)`. Triangle from (x+8, y+4) with radius `top_r`.
  - Bottom sand grows: Triangle accumulating from bottom. `sand_level = (frame * 0.04) % 1.0 * 8`.
  - Falling grain: Single pixel at (x+8, y+10 + frame%4), color `[200, 180, 120, 255]`.
- **State 2 (Triggered)**:
  - All sand at bottom: Triangle (x+8, y+14) -> (x+4, y+20) -> (x+12, y+20).
  - Spring arm: Line from (x+14, y+10) to (x+22, y+6), color `[120, 120, 130, 255]`.
  - Ball tip: Filled circle at (x+22, y+6), radius 2px, color `[200, 200, 210, 255]`.

### Text/Braille Rendering

- **Icon**: `\u231B` (HOURGLASS)
- **Color**: TAN `[210, 180, 140]`

### Animations

- **Sand flow** (State 1): `sand_level = (frame * 0.04) % 1.0 * 8.0`. Top sand shrinks, bottom grows. Single grain pixel falls through neck at `y+10 + frame%4`.

### Interactions

- Programmable delay: 1-10 seconds before spring arm deploys.
- Spring-loaded arm extends at ~400 px/s when timer completes.
- Arm can push/launch objects placed on or near the timer.
- Used as a delay mechanism in chain reactions.

---

## Eye Hook

### Overview
- **Size**: 8x12 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: Yes
- **Ramp**: No
- **Power**: No
- **Animated**: No

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static eye hook |

### Pixel Rendering

- Default fallback: Filled rect at (x, y), 8x12 px, color SILVER `[200, 200, 210, 200]`.

### Text/Braille Rendering

- **Icon**: `\u2310` (REVERSED NOT SIGN)
- **Color**: SILVER `[200, 200, 210]`

### Animations

None.

### Interactions

- Fixed rope attachment/anchor point.
- Rope threads through the eye to change direction.
- Immovable anchor for rope systems.

---

## Boat Cleat

### Overview
- **Size**: 12x8 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: Yes
- **Ramp**: No
- **Power**: No
- **Animated**: No

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static boat cleat |

### Pixel Rendering

- Default fallback: Filled rect at (x, y), 12x8 px, color SILVER `[200, 200, 210, 200]`.

### Text/Braille Rendering

- **Icon**: `\u2229` (INTERSECTION)
- **Color**: SILVER `[200, 200, 210]`

### Animations

None.

### Interactions

- Immovable rope anchor point.
- When rope needs a fixed anchor, tie it here.
- Replaced Metal Loop from earlier game versions.

---

## Gun (Revolver)

### Overview
- **Size**: 28x20 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: Yes
- **Resizable**: No
- **Rope Point**: Yes (trigger)
- **Ramp**: Yes (body serves as ramp surface)
- **Power**: No
- **Animated**: No (but has muzzle flash)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Loaded | Ready to fire | Normal gun appearance |
| 1 | Fired | Rope-triggered; hitscan bullet | Muzzle flash + smoke at barrel |

### Pixel Rendering

- **Body**: Filled rect at (x+4, y+4), 20x8 px, color `[120, 120, 130, 255]` (gun metal).
- **Barrel**: Rect at barrel_x, (y+5), 4x5 px. Position: flipped=(x), unflipped=(x+24). Color `[100, 100, 110, 255]`.
- **Grip**: Filled rect at (x+8, y+12), 6x8 px, color `[100, 70, 40, 255]` (wood grip).
- **Cylinder**: Filled circle at (x+14, y+8), radius 4px, color `[110, 110, 120, 255]`.
- **Trigger**: Line from (x+13, y+12) to (x+12, y+15), color `[80, 80, 90, 255]`.
- **State 1 (Fired)**:
  - Muzzle flash: Glow at (flash_x, y+7), radius 6px, color `[255, 255, 150]`. Position depends on flip.
  - Smoke: 3 circles expanding from barrel. Each at `sx = flash_x + dir*(4+i*3)`, `y+6-i`. Radius 2px. Alpha `120 - i*35`. Color `[180, 180, 180]`.

### Text/Braille Rendering

- **Icon**: `\u25BA` (BLACK RIGHT-POINTING POINTER)
- **Color**: GRAY `[160, 160, 160]`

### Animations

- **Muzzle flash** (State 1): Glow + 3 smoke puffs at barrel exit. Smoke drifts in firing direction.

### Interactions

- Rope attaches to trigger. Rope must pull from behind the handle.
- Fired by rope pull (via pulleys if needed to redirect angle).
- Hitscan bullet: instant travel in aimed direction.
- Bullet hits: cats, fish bowls, balls, Mel, other movable objects.
- Single shot only.
- Gun body serves as a ramp/deflector surface for rolling objects.

---

## Anti-Gravity Pad

### Overview
- **Size**: 32x8 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (pulsing glow, rising field lines)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state -- always active | Pulsing glow with rising field lines |

### Pixel Rendering

- **Pad body**: Filled rect at (x, y), 32x8 px, color `[100, 50, 150, 255]` (purple).
- **Glow intensity**: `glow = sin(frame * 0.1) * 0.3 + 0.7`. Range: 0.4 to 1.0. `alpha = glow * 140`.
- **Field lines**: Rising particles above pad. `field_phase = (frame * 2) % 16`. Every 4px along width, particles at `y - 1 - gy` for gy in (field_phase..16 step 8). Color `[180, 100, 240]`, alpha fades with height `alpha / (1 + gy/3)`.
- **Surface glow**: Row of pixels at y-1 across full width. Color `[160, 80, 220, alpha]`.

### Text/Braille Rendering

- **Icon**: `\u2261` (IDENTICAL TO)
- **Color**: PURPLE `[140, 60, 180]`

### Animations

- **Pulsing glow**: `sin(frame * 0.1) * 0.3 + 0.7`. Continuous sinusoidal intensity oscillation.
- **Rising field lines**: `phase = (frame * 2) % 16`. Particles rise upward with 16-frame cycle. Spaced every 4px horizontally.

### Interactions

- Reverses gravity in ~16px zone above the pad surface.
- Objects entering the zone float upward instead of falling.
- Always active (no activation needed).

---

## Santa Lamp

### Overview
- **Size**: 20x28 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: No

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static lamp with ambient glow |

### Pixel Rendering

- **Shade**: Filled triangle (x+10, y) -> (x+2, y+14) -> (x+18, y+14), color `[200, 50, 40, 255]` (red shade).
- **Pole**: Line from (x+10, y+14) to (x+10, y+24), color `[80, 80, 90, 255]` (metal).
- **Base**: Filled rect at (x+6, y+24), 8x4 px, color `[80, 80, 90, 255]`.
- **Light glow**: Glow at (x+10, y+13), radius 4px, color `[255, 255, 200]`.

### Text/Braille Rendering

- **Icon**: `\u2666` (BLACK DIAMOND SUIT)
- **Color**: RED `[220, 40, 40]`

### Animations

None.

### Interactions

- Decorative part -- provides ambient light.
- Replaced Christmas Tree from earlier game versions.

---

## Laundry Basket

### Overview
- **Size**: 28x32 px
- **Type**: Dynamic
- **Mass**: 1.2
- **Elasticity**: 0.1
- **Density**: 0.8
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: Yes
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has rattling when trapping)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Open | Ready to trap | Normal wicker basket outline |
| 1 | Trapping | Animal caught inside | Shaking basket with visible movement inside |

### Pixel Rendering

- **Sides**: Two vertical lines at (x, y) to (x, y+h) and (x+w-1, y) to (x+w-1, y+h). Color BROWN `[139, 90, 43, 255]`.
- **Horizontal bands**: Lines every 6px from (x, y+by) to (x+w, y+by). Color BROWN, alpha 180.
- **State 1 (Trapping)**:
  - Trapped animal blob: Circle at (x+w/2 + shake, y+h*0.6), radius 4px, color `[200, 160, 80, 120]`.
    - `shake = sin(frame * 0.3) * 2.0`.
  - Rattling sides: Lines at (x-1, y+shake) to (x-1, y+4+shake) and (x+w, y-shake) to (x+w, y+4-shake). Color BROWN, alpha 100. Visible when `frame % 4 < 2`.

### Text/Braille Rendering

- **Icon**: `\u2554` (BOX DRAWINGS DOUBLE DOWN AND RIGHT)
- **Color**: BROWN `[139, 90, 43]`

### Animations

- **Rattling** (State 1): Interior blob oscillates `sin(frame * 0.3) * 2.0`. Side vibration lines blink on 4-frame cycle.

### Interactions

- Traps animals when dropped onto them (e.g., trapping Pokey the Cat).
- Bottomless (no floor) -- container with open bottom.
- Attachable to rope (rope point).
- Heavier than Bucket (mass 1.2 vs 0.5).
- Feels gravity (dynamic).
- Replaced Bird Cage from earlier game versions.

---

## Bucket

### Overview
- **Size**: 20x20 px
- **Type**: Dynamic
- **Mass**: 0.5
- **Elasticity**: 0.1
- **Density**: 1.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: Yes
- **Ramp**: No
- **Power**: No
- **Animated**: No

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Empty | No contents | Standard bucket shape |
| 1 | Filled | Object(s) caught -- mass increased | Same visual (mass changes internally) |

### Pixel Rendering

- **Body**: Tapered shape. For each row (0 to height):
  - `t = row / height`. Width: `w*0.6 + w*0.4*t` (wider at bottom). Centered horizontally.
  - Fill color: SILVER `[200, 200, 210, 255]`.
- **Handle**: 10-point semicircular arc above bucket. Center (x+w/2), extends w*0.35 horizontally, 6px tall. Color `[120, 120, 130, 255]`.

### Text/Braille Rendering

- **Icon**: `U`
- **Color**: SILVER `[200, 200, 210]`

### Animations

None.

### Interactions

- Catches and holds objects (balls, animals, cannonballs) that land inside.
- Mass increases when objects land inside.
- Must be attached to rope to hang.
- Rope point for pulley systems.
- Can be hit by bullets (Gun hitscan).
- Feels gravity (dynamic).
- Lighter than Pokey the Cat (0.5 vs 0.8) and Laundry Basket (0.5 vs 1.2).

---

## Leaky Bucket

### Overview
- **Size**: 20x20 px
- **Type**: Dynamic
- **Mass**: 1.5 (initial, drains to 0.3)
- **Elasticity**: 0.1
- **Density**: 1.5
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: Yes
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (drip animation)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| leak_rate | 0 | 2 | 1 | 1 | Leak (0=slow/1=med/2=fast) |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Full | Mass 1.5 -- leaking | Drip animation at bottom |
| 1 | Draining | Losing mass at configured rate | Continuous drip animation |
| 2 | Empty | Mass 0.3 -- no more drip | No drip, dry bucket |

### Pixel Rendering

- **Body**: Same tapered shape as Bucket. Color GRAY `[160, 160, 160, 255]`.
- **Handle**: Same semicircular arc as Bucket.
- **Drip** (States 0 & 1): Two pixels below bucket bottom.
  - `drip_y = (frame * 0.15) % 8.0`.
  - Upper drop at (x+w/2, y+h+drip_y), color `[100, 150, 220, 200]`.
  - Lower drop at (x+w/2, y+h+drip_y+1), color `[100, 150, 220, 140]` (fainter).

### Text/Braille Rendering

- **Icon**: `U`
- **Color**: GRAY `[160, 160, 160]`

### Animations

- **Drip cycle**: `drip_y = (frame * 0.15) % 8.0`. Water droplet falls 8px before resetting. Continuous loop while draining.

### Interactions

- Mass decreases over time: 1.5 (full) to 0.3 (empty).
- Three leak rate settings: slow (0), medium (1), fast (2).
- Weight differential drives rope/pulley dynamics.
- Essential for pulley-based puzzles where changing weight creates movement.
- Rope point for attachment to pulley systems.
- Feels gravity (dynamic).

---

## Balloon

### Overview
- **Size**: 20x28 px
- **Type**: Dynamic
- **Mass**: 0.01
- **Elasticity**: 0.3
- **Density**: 0.01
- **Friction**: 0.1
- **Gravity**: Buoyant (rises under normal/weak gravity, falls under strong gravity)
- **Destructible**: No (but poppable)
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has bobbing motion)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| design | 1 | 4 | 1 | 1 | Design (1-4) |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Inflated | Rising due to buoyancy | Full balloon with specular highlight, bobbing, string |
| 1 | Popped | Destroyed | Fragments flying outward, dangling string |

### Pixel Rendering

Design colors (identical behavior, different appearance):
- Design 1: `[220, 50, 50]` (red)
- Design 2: `[50, 50, 220]` (blue)
- Design 3: `[50, 200, 50]` (green)
- Design 4: `[220, 200, 50]` (yellow)

- **State 0 (Inflated)**:
  - Bob offset: `bob = sin(frame * 0.06) * 2.0`.
  - Body: Gradient circle at (x+10, y+10+bob), radius 9px. Gradient from balloon_color to darker variant (each channel -40).
  - Specular highlight: Small circle at (x+7, y+7+bob), radius 2.5px, color `[255, 255, 255, 80]`.
  - Knot: Pixel at (x+10, y+19+bob). Color: balloon_color minus 60 per channel.
  - String: 8 pixels hanging from knot. Each at (x+10+wobble, y+20+bob+sy). `wobble = sin(sy*0.5) * 1.0`. Color `[180, 180, 180, 200]`.
- **State 1 (Popped)**:
  - 6 fragments in circular pattern. `angle = i * TAU/6 + frame*0.05`. `dist = 4 + min(frame*0.3, 12)`. Alpha fades: `200 - frame*3`. Color: balloon_color.
  - Dangling string: 8 pixels from (x+10, y+12) downward. Color `[180, 180, 180, 140]`.

### Text/Braille Rendering

- **Icon**: `\u25CB` (WHITE CIRCLE)
- **Color**: BLUE `[50, 100, 220]`

### Animations

- **Bobbing** (State 0): `sin(frame * 0.06) * 2.0`. Gentle vertical oscillation.
- **String wobble**: `sin(sy * 0.5) * 1.0` per string segment. Static sinusoidal wave.
- **Pop fragments** (State 1): 6 pieces expand outward. Distance: `4 + min(frame*0.3, 12)`. Rotation: `frame*0.05`. Alpha fades to 0.

### Interactions

- Rises under normal/weak gravity (buoyant gravity response).
- Falls under strong gravity.
- **Popped by**: Spinning gears, scissors, tacks, dynamite explosion, candle flame, rocket trail, gun bullet, laser beam.
- Can be held by rope attached to another part.
- Near-zero weight (mass 0.01).
- Four programmable appearance designs (color variants).
- Can push Bike Pump handle when rising.

---

## Hot Air Balloon

### Overview
- **Size**: 32x40 px
- **Type**: Dynamic
- **Mass**: 0.5
- **Elasticity**: 0.1
- **Density**: 0.1
- **Friction**: 0.1
- **Gravity**: Normal (requires heat to rise)
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has flame when heating/rising)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Cold | Stationary on ground | No flame in basket |
| 1 | Heating | Heat source applied -- beginning to rise | Flame visible in basket |
| 2 | Rising | Airborne -- carrying objects | Flame visible, balloon ascending with rise offset |

### Pixel Rendering

- **Rise offset** (State 2): `rise_offset = -min(frame * 0.5, 20.0)`. Balloon position shifts upward.
- **Envelope**: Gradient circle at (x+16, by+14), radius 14px. Top `[220, 60, 40]` to bottom `[180, 40, 30]` (red gradient). `by = y + rise_offset`.
- **Stripes**: 3 vertical lines at x-offsets [-6, 0, +6] from center. From by+2 to by+28. Color `[240, 220, 60, 150]` (yellow decorative stripes).
- **Ropes**: Two lines from envelope bottom to basket. Left: (x+8, by+26) to (x+10, by+32). Right: (x+24, by+26) to (x+22, by+32). Color `[120, 80, 40, 255]` (rope brown).
- **Basket**: Filled rect at (x+10, by+32), 12x8 px, color `[140, 100, 50, 255]` (wicker).
- **Flame** (States 1 & 2): `draw_flame(x+16, by+30, 5.0, frame)` inside basket opening.

### Text/Braille Rendering

- **Icon**: `\u25EF` (LARGE CIRCLE)
- **Color**: RED `[220, 40, 40]`

### Animations

- **Rise** (State 2): `rise_offset = -min(frame * 0.5, 20.0)`. Balloon ascends at 0.5px/frame (30 px/s at 60fps), capping at 20px offset.
- **Basket flame** (States 1 & 2): `draw_flame` procedural animation inside basket.

### Interactions

- Rises when heated by flame source below (candle, rocket trail).
- Attachable via rope to carry objects.
- Can carry objects placed in basket.
- Gravity response is Normal (does not rise without heat, unlike Balloon which is inherently buoyant).
- Heavier than Balloon (mass 0.5 vs 0.01).
