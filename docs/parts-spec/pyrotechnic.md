# Pyrotechnic Parts Specification

---

## Magnifying Glass

### Overview
- **Size**: 16x24 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: Yes
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
| 0 | Idle | Default state | Static magnifying glass display |

### Pixel Rendering

- **Lens**: Filled circle at (x+8, y+8), radius 7px, color `[200, 220, 240, 120]` (translucent light blue).
- **Rim**: 36-point circle outline at same center, radius 7px, color `[140, 140, 150, 255]` (steel gray).
- **Handle**: Two parallel lines from (x+12, y+14) to (x+14, y+22) and (x+13, y+14) to (x+15, y+22), color `[120, 80, 40, 255]` (wooden brown).

### Text/Braille Rendering

- **Icon**: `\u25CE` (BULLSEYE)
- **Color**: WHITE `[240, 240, 240]`

### Animations

None (no animation).

### Interactions

- Untouchable during simulation -- does not interact with physics objects.
- Focuses light from a Flashlight or Lava Lamp into a concentrated beam (~100px range).
- Focused beam ignites fuses, wicks, and candles.
- Must be oriented in the same direction as the light source (both flipped or both unflipped).

---

## Flashlight

### Overview
- **Size**: 32x12 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: Yes
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (beam flicker when on)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Off | Switch not struck | Dark lens `[200, 200, 180, 255]` |
| 1 | On | Object struck switch -- ~200px beam active | Bright lens `[255, 255, 220, 255]`, beam projected, glow at lens |

### Pixel Rendering

- **Body**: Gradient-filled rect at (bx, y+2), 20x8 px, top `[200, 200, 210]` to bottom `[160, 160, 170]`. Position depends on flip: flipped=(x+8), unflipped=(x).
- **Lens**: Rect at lens_x, (y), 8x12 px. Color varies by state: Off=`[200, 200, 180, 255]`, On=`[255, 255, 220, 255]`. Position: flipped=(x), unflipped=(x+20).
- **State 1 (On)**:
  - Beam: 16 pixels projected from lens in facing direction. Each pixel column has spreading width (spread = column/3). Alpha fades from 180 to 0 linearly. Color `[255, 255, 200]`.
  - Lens glow: Glow at (lens_x+4, y+6), radius 6px, color `[255, 255, 180]`.

### Text/Braille Rendering

- **Icon**: `\u25BA` (BLACK RIGHT-POINTING POINTER)
- **Color**: YELLOW `[230, 200, 50]`

### Animations

- **Beam projection** (State 1): Continuous beam rendering with fading alpha per column. Formula: `alpha = 180 * (1 - column / 16)`. Spread: `column / 3` pixels above and below center.

### Interactions

- Activated by a moving object striking its switch (object-activated).
- Provides ~200px beam range.
- Powers Solar Panels when beam reaches them.
- Provides light for Magnifying Glass to focus into ignition beam.
- Can be oriented left or right (flippable).

---

## Lava Lamp

### Overview
- **Size**: 16x32 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No (but has drawstring for rope attachment)
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (blob movement when on)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Off | Drawstring not pulled | Dark glass body `[50, 25, 70, 150]`, no blobs |
| 1 | On | Drawstring pulled -- omnidirectional ~100px light | Glowing purple body `[100, 50, 150, 200]`, animated blobs, glow effect |

### Pixel Rendering

- **Base**: Filled rect at (x+2, y+24), 12x8 px, color `[60, 60, 70, 255]` (dark metal).
- **Cap**: Filled rect at (x+4, y+2), 8x3 px, color `[100, 100, 110, 255]` (light metal).
- **State 0 (Off)**: Glass body rect at (x+3, y+4), 10x20 px, color `[50, 25, 70, 150]` (dark purple, semi-transparent).
- **State 1 (On)**:
  - Glass body rect at (x+3, y+4), 10x20 px, color `[100, 50, 150, 200]` (bright purple).
  - Blob 1: Circle at (x+8, y+12 + sin(frame*0.05)*6), radius 3px, color `[220, 110, 220, 220]`.
  - Blob 2: Circle at (x+8, y+18 + sin(frame*0.03+2)*4), radius 2px, color `[240, 130, 240, 200]`.
  - Glow: Center (x+8, y+14), radius 8px, color `[180, 80, 180]`.

### Text/Braille Rendering

- **Icon**: `\u2565` (BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE)
- **Color**: PURPLE `[140, 60, 180]`

### Animations

- **Blob 1 oscillation**: `y_offset = sin(frame * 0.05) * 6.0` -- slow vertical bob.
- **Blob 2 oscillation**: `y_offset = sin(frame * 0.03 + 2.0) * 4.0` -- slower, phase-offset vertical bob.
- Both animations only active in State 1 (On).

### Interactions

- Drawstring-activated: rope must be attached to drawstring, routed through pulleys, and pulled by a weighted object.
- Provides omnidirectional ~100px light when on.
- Powers Solar Panels.
- Provides light for Magnifying Glass to focus.

---

## Candle

### Overview
- **Size**: 8x16 px
- **Type**: Dynamic
- **Mass**: 0.05
- **Elasticity**: 0.1
- **Density**: 0.8
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (flame flicker when lit)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Unlit | No flame | Bare wax body with dark wick |
| 1 | Lit | Burning -- provides light; ignites fuses on contact | Animated flame + glow at wick tip |
| 2 | Extinguished | Was lit, now out | Charred wick, rising smoke wisp |

### Pixel Rendering

- **Wax body**: Filled rect at (x+2, y+6), 4x10 px, color `[240, 230, 200, 255]` (cream).
- **Wick**: Line from (x+4, y+6) to (x+4, y+4), color `[60, 50, 40, 255]` (dark brown).
- **State 0 (Unlit)**: Just the candle body, no additional effects.
- **State 1 (Lit)**:
  - Flame: Animated flame at (x+4, y+4), size 6px, using `draw_flame` with frame counter.
  - Glow: Center (x+4, y+2), radius 10px, color `[255, 200, 80]`.
- **State 2 (Extinguished)**:
  - Charred wick: Line from (x+4, y+4) to (x+4, y+2), color `[40, 35, 30, 255]`.
  - Smoke wisp: Pixel at (x+4, y - 2 - (frame*0.1 % 6)), alpha 100. Second pixel offset (+1, -1) at alpha 60. Both `[140, 140, 150]`.

### Text/Braille Rendering

- **Icon**: `\u2565` (BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE)
- **Color**: YELLOW `[230, 200, 50]`

### Animations

- **Flame flicker** (State 1): `draw_flame(x+4, y+4, 6.0, frame)` -- procedural animated flame.
- **Smoke rise** (State 2): `smoke_y = y - 2 - (frame * 0.1) % 6` -- smoke pixels drift upward cyclically.

### Interactions

- Lit by: Magnifying Glass + light source, laser beam, or another flame source.
- Can be blown out by: Electric Fan or Bike Pump (Bellows).
- Ignites fuses on physical contact.
- Lights other candles on contact.
- Heats Teapots placed above (produces steam).
- Heats Hot Air Balloons (causes them to rise).
- Pops balloons on contact (when lit).
- Powers Solar Panels (provides light).
- Feels gravity -- can be placed on Conveyor Belts to move while lit.

---

## Dynamite

### Overview
- **Size**: 12x20 px
- **Type**: Dynamic
- **Mass**: 0.3
- **Elasticity**: 0.1
- **Density**: 1.5
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No (itself is the explosive)
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has explosion effect)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Inert -- waiting for ignition | Red stick with fuse visible |
| 1 | FuseLit | Fuse burning -- 1.5s to detonation | Animated flame at fuse tip |
| 2 | Exploded | Destroyed walls within ~60px radius | Expanding debris ring with central glow |

### Pixel Rendering

- **States 0 & 1 (Intact)**:
  - Stick body: Filled rect at (x+2, y+4), 8x16 px, color `[210, 40, 30, 255]` (bright red).
  - Label band: Filled rect at (x+2, y+10), 8x3 px, color `[240, 220, 180, 255]` (paper label).
  - Fuse: Line from (x+6, y+4) to (x+8, y), color `[120, 80, 40, 255]` (brown cord).
  - State 1 adds: Flame at (x+8, y), size 4px, using `draw_flame` with frame counter.
- **State 2 (Exploded)**:
  - 8 debris particles in circular pattern: `angle = i * TAU/8 + frame * 0.1`, distance expands `6 + min(phase*0.5, 20)`. Color `[200, 100, 30]`, alpha fades with phase.
  - Central glow: Center (x+6, y+12), radius 14px, color `[255, 200, 50]`.

### Text/Braille Rendering

- **Icon**: `\u256B` (BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE)
- **Color**: RED `[220, 40, 40]`

### Animations

- **Fuse burning** (State 1): `draw_flame(x+8, y, 4.0, frame)` at fuse tip.
- **Explosion debris** (State 2): 8 particles orbit outward. `phase = frame * 0.2`. Distance: `6 + min(phase*0.5, 20)`. Alpha: `200 - phase*5`. Rotation: `angle += frame * 0.1`.

### Interactions

- Ignited by: Fuse, candle, rocket trail, magnifying glass + light source, laser beam.
- **Destruction radius**: ~60px. Destroys: Brick Wall, Yellow Brick Wall, Cinder Block Wall, Wooden Wall.
- **Does NOT destroy**: Inclines/ramps, Metal Piping, Metal Piping Curves, Earth/Grass, Caution Wall, Greco-Roman Wall, Log Wall, Sand Wall, Pipe Wall.
- **Chain reaction**: Nearby dynamite within destruction radius also explodes.
- Sends nearby movable objects flying from blast force.
- Pops balloons within radius.
- Feels gravity.

---

## Dynamite Plunger

### Overview
- **Size**: 20x28 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No (but has handle for rope/weight activation)
- **Ramp**: No
- **Power**: No
- **Animated**: No

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Ready | Handle up | Handle at top position (y), plunger rod extended |
| 1 | Pressed | Plunger pressed -- immediate detonation | Handle lowered to (y+8), spark at contact point |

### Pixel Rendering

- **Box**: Filled rect at (x+2, y+14), 16x14 px, color `[120, 80, 40, 255]` (wooden box).
- **Handle**: Horizontal bar at (x+4, handle_y), 12x3 px, color `[80, 80, 90, 255]` (metal). `handle_y` = y (Ready) or y+8 (Pressed).
- **Rod**: Vertical line from (x+10, handle_y) to (x+10, y+14), color `[80, 80, 90, 255]`.
- **State 1 (Pressed)**: Glow at (x+10, y+14), radius 5px, color `[255, 255, 100]` (spark flash).

### Text/Braille Rendering

- **Icon**: `\u2564` (BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE)
- **Color**: RED `[220, 40, 40]`

### Animations

None.

### Interactions

- Activated by: Dropping an object onto the handle, or pulling handle down via rope.
- Triggers instant detonation (same destruction as dynamite, ~60px radius).
- **Does NOT chain-react** from nearby dynamite explosions. Distinct from regular dynamite.
- Immovable static part.

---

## Cannon

### Overview
- **Size**: 48x24 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: Yes
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (fuse glow when ready)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Ready | Loaded -- waiting for fuse | Subtle fuse glow (blinks every 6 frames, 3 on/3 off) |
| 1 | FuseLit | Fuse burning -- ~1.0s to fire | Animated flame + glow at fuse position |
| 2 | Fired | Cannonball launched at ~1500 px/s + recoil | Smoke cloud at barrel opening, recoil offset |

### Pixel Rendering

- **Barrel**: Gradient-filled rect at (bx, y+4), 32x12 px, top `[120, 120, 130]` to bottom `[80, 80, 90]`. Position depends on flip: flipped=(x), unflipped=(x+12).
- **Opening**: Filled rect at opening_x, (y+5), 4x10 px, color `[40, 40, 45, 255]` (dark bore). Position: flipped=(x), unflipped=(x+44).
- **Wheels**: Two circles at (x+14, y+20) and (x+34, y+20), radius 5px, color `[100, 70, 40, 255]` (wood).
- **State 0 (Ready)**: Fuse glow at fuse position, radius 3px, color `[200, 160, 40]`. Blinks: visible when `frame % 6 < 3`.
- **State 1 (FuseLit)**: Flame at fuse position, size 5px. Glow radius 6px, color `[255, 200, 50]`.
- **State 2 (Fired)**: 5 smoke puffs at barrel exit. Each puff: `sx = smoke_x + sin(frame*0.3 + i) * 6`, `sy = y+8 + cos(frame*0.2 + i*1.5) * 4`. Radius decreases `4 - i*0.5`. Alpha decreases `160 - i*30`.

### Text/Braille Rendering

- **Icon**: `\u2550` (BOX DRAWINGS DOUBLE HORIZONTAL)
- **Color**: GRAY `[160, 160, 160]`

### Animations

- **Ready fuse blink** (State 0): Glow at fuse, 3-frame-on / 3-frame-off cycle.
- **Fuse burning** (State 1): `draw_flame` at fuse position + glow.
- **Smoke dissipation** (State 2): 5 smoke circles with sinusoidal drift. Positions oscillate with frame-based sin/cos functions.

### Interactions

- Ignited by: Magnifying Glass + light source, fuse, candle, laser beam.
- Fires a cannonball at ~1500 px/s in the direction it faces.
- Cannonball follows gravity after firing (no bounce -- elasticity 0).
- Produces recoil effect on firing.
- Cannonball is a separate heavy physics object.

---

## Rocket

### Overview
- **Size**: 12x28 px
- **Type**: Dynamic
- **Mass**: 0.2
- **Elasticity**: 0.1
- **Density**: 1.2
- **Friction**: 0.3
- **Gravity**: Normal (before ignition)
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has launch effects)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Grounded -- waiting for fuse | Static rocket on launch pad |
| 1 | FuseLit | Fuse burning -- ~0.8s | Flame at nozzle base |
| 2 | Launched | Flying at ~2000 px/s; trail ignites objects | Rocket ascending with exhaust trail |

### Pixel Rendering

- **States 0 & 1 (Grounded)**:
  - Body: Filled rect at (x+2, y+8), 8x16 px, color `[200, 40, 40, 255]` (red).
  - Nose cone: Filled triangle (x+6, y) -> (x+1, y+8) -> (x+11, y+8), color `[200, 40, 40, 255]`.
  - Left fin: Triangle (x, y+24) -> (x+2, y+18) -> (x+2, y+24), color `[180, 30, 30, 255]`.
  - Right fin: Triangle (x+12, y+24) -> (x+10, y+18) -> (x+10, y+24), color `[180, 30, 30, 255]`.
  - Nozzle: Filled rect at (x+3, y+24), 6x4 px, color `[80, 80, 90, 255]` (metal).
  - State 1 adds: Flame at (x+6, y+26), size 4px.
- **State 2 (Launched)**:
  - Rocket position: `fly_y = y - min(frame * 3.0, 40.0)` (ascending).
  - Body at (x+2, fly_y+8), 8x16 px. Nose cone triangle repositioned to fly_y.
  - Exhaust trail: 8 particles below rocket. Each at `ty = fly_y + 28 + i*4`, spread radius `1 + i/2`. Color `[255, 180, 50]`, alpha `220 - i*25`. Horizontal wobble: `sin(frame*0.3 + i) * 2`.

### Text/Braille Rendering

- **Icon**: `\u25B2` (BLACK UP-POINTING TRIANGLE)
- **Color**: RED `[220, 40, 40]`

### Animations

- **Fuse flame** (State 1): `draw_flame(x+6, y+26, 4.0, frame)`.
- **Ascent** (State 2): `fly_y = y - min(frame * 3.0, 40.0)`. Rocket moves upward.
- **Exhaust trail** (State 2): 8 particles with fading alpha and sinusoidal horizontal drift.

### Interactions

- Ignited by: Fuse, candle, magnifying glass + light source, laser beam.
- ~0.8s fuse delay before launch.
- Launches at ~2000 px/s upward.
- Burning exhaust trail ignites: fuses, candles, other rockets.
- Trail pops balloons on contact.
- Feels gravity before ignition.

---

## Fireworks

### Overview
- **Size**: 10x24 px
- **Type**: Dynamic
- **Mass**: 0.2
- **Elasticity**: 0.1
- **Density**: 1.2
- **Friction**: 0.3
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has burst effect)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| color1 | 0 | 255 | 25 | 255 | Color R |
| color2 | 0 | 255 | 25 | 100 | Color G |
| color3 | 0 | 255 | 25 | 200 | Color B |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Waiting for ignition | Grounded firework shell |
| 1 | FuseLit | ~1.5s fuse | Flame at base |
| 2 | Launched | Rising | Firework ascending with spark trail |
| 3 | Exploded | Cosmetic colored burst at apogee | 12-point starburst with custom colors |

### Pixel Rendering

Colors are user-programmable via `color1` (R), `color2` (G), `color3` (B) properties.

- **States 0 & 1 (Grounded)**:
  - Body: Filled rect at (x+3, y+8), 4x16 px, color `[180, 180, 180, 255]` (gray tube).
  - Nose: Filled triangle (x+5, y) -> (x+1, y+8) -> (x+9, y+8), color `[c1r, c2g, c3b, 255]` (custom color).
  - State 1 adds: Flame at (x+5, y+22), size 3px.
- **State 2 (Launched)**:
  - `fly_y = y - min(frame * 2.0, 30.0)`.
  - Body at (x+3, fly_y), 4x8 px. Nose triangle repositioned.
  - Spark trail: 5 particles, each at `ty = fly_y + 10 + i*4`. Color `[255, 200, 80]`, alpha `180 - i*35`.
- **State 3 (Exploded)**:
  - Starburst: 12 points in circular pattern. `burst_r = min(frame * 0.8, 18.0)`. Each point at angle `i * TAU/12 + frame*0.05`. Color `[c1r, c2g, c3b]`, alpha `max(255 - burst_r*10, 0)`.
  - Central glow: Center (x+5, y+4), radius `burst_r * 0.8`, custom color.

### Text/Braille Rendering

- **Icon**: `\u25B2` (BLACK UP-POINTING TRIANGLE)
- **Color**: MAGENTA `[200, 50, 200]`

### Animations

- **Fuse flame** (State 1): `draw_flame(x+5, y+22, 3.0, frame)`.
- **Ascent** (State 2): `fly_y = y - min(frame * 2.0, 30.0)`.
- **Starburst expansion** (State 3): Radius grows `burst_r = min(frame * 0.8, 18.0)`. Points rotate `frame * 0.05`. Alpha fades to 0 as radius reaches 25.5.

### Interactions

- Ignited by: Same as other pyrotechnics (fuse, candle, magnifying glass + light, laser).
- ~1.5s fuse delay.
- Cosmetic explosion only -- no destruction radius.
- Programmable explosion colors (three-channel RGB).

---

## Remote Control Bomb

### Overview
- **Size**: 16x16 px
- **Type**: Dynamic
- **Mass**: 0.4
- **Elasticity**: 0.1
- **Density**: 1.5
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No (but attachable via rope per spec)
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (LED blink)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Inert -- waiting for trigger signal | LED blinks slowly (every 20 frames) |
| 1 | FuseLit | Trigger received -- detonation imminent | LED blinks fast (every 6 frames) |
| 2 | Exploded | Same destruction as dynamite (~60px radius) | Expanding debris ring with glow |

### Pixel Rendering

- **States 0 & 1 (Intact)**:
  - Body: Filled rect at (x+2, y+4), 12x12 px, color `[180, 40, 40, 255]` (dark red).
  - Antenna: Line from (x+8, y+4) to (x+10, y), color `[80, 80, 90, 255]` (metal). Tip pixel at (x+10, y), color `[200, 200, 200, 255]`.
  - LED blink: At (x+5, y+6), blink rate depends on state. State 0: `frame % 20 < 10` (slow). State 1: `frame % 6 < 3` (fast). Color `[255, 50, 50, 255]` with glow radius 3px `[255, 50, 50]`.
- **State 2 (Exploded)**:
  - 8 debris particles in circular pattern: `angle = i * TAU/8 + frame * 0.1`, distance `6 + min(frame*0.5, 20)`. Color `[200, 100, 30, 180]`.
  - Central glow: Center (x+8, y+10), radius 14px, color `[255, 200, 50]`.

### Text/Braille Rendering

- **Icon**: `\u2731` (HEAVY ASTERISK)
- **Color**: RED `[220, 40, 40]`

### Animations

- **LED blink** (States 0 & 1): Blink rate: Idle=20-frame cycle, FuseLit=6-frame cycle. Red LED with glow.
- **Explosion debris** (State 2): Same pattern as Dynamite -- 8 orbiting particles.

### Interactions

- Detonated by trigger signal (remote activation).
- Attachable via rope.
- Same destruction effect as Dynamite (~60px radius).
- Destroys: Brick Wall, Yellow Brick Wall, Cinder Block Wall, Wooden Wall.
- Does NOT destroy: Inclines, Metal Piping, Earth/Grass.
- Chain-reacts with nearby dynamite.
- Feels gravity.

---

## Match on Spring

### Overview
- **Size**: 12x20 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
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
| 0 | Idle | Default state | Match head on coiled spring |

### Pixel Rendering

- **Spring coil**: 5 coil segments. Each at `y = y+10 + sy*2`, with horizontal wobble `sin(sy * 1.5) * 2.0`. Two pixels wide per coil, color `[160, 160, 170, 255]` (steel).
- **Match head**: Filled circle at (x+6, y+6), radius 3px, color `[200, 50, 30, 255]` (red phosphorus).
- **Match stick**: Line from (x+6, y+6) to (x+6, y+10), color `[180, 150, 100, 255]` (wood).

### Text/Braille Rendering

- **Icon**: `\u2191` (UPWARDS ARROW)
- **Color**: RED `[220, 40, 40]`

### Animations

None.

### Interactions

- Spring-loaded ignition source.
- Match head ignites on contact/compression.
- Can ignite fuses, candles, and other pyrotechnic parts.

---

## Fuse

### Overview
- **Size**: 64x4 px (default length 64px, configurable 16-200px)
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: No (but has burn progression)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| length | 16 | 200 | 8 | 64 | Length (px) |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Unburnt | Static braided wire | Full-length brown cord with sinusoidal wobble |
| 1 | Burning | Spark traveling at ~60px/s from ignition point | Charred portion behind spark, flame + glow at burn front |
| 2 | Spent | Fully burned | Entire length is charred dark |

### Pixel Rendering

- **Wire**: For each pixel along length (0 to `length`):
  - Vertical wobble: `wobble = sin(fx * 0.3) * 1.0`.
  - Unburnt portion: Color `[120, 80, 40, 255]` (brown cord).
  - Burnt portion (State >= 1, fx < burn_pos): Color `[40, 35, 30, 255]` (charred black).
- **Burn position**: `burn_pos = (frame * 1.0) % length` (approximates ~60px/s at 60fps).
- **Spark at burn front** (State 1, burn_pos < length):
  - Glow at (x + burn_pos, y+2 + wobble), radius 4px, color `[255, 220, 80]`.
  - Flame at (x + burn_pos, y+1 + wobble), size 3px.

### Text/Braille Rendering

- **Icon**: `\u2500` (BOX DRAWINGS LIGHT HORIZONTAL)
- **Color**: BROWN `[139, 90, 43]`

### Animations

- **Burn progression** (State 1): Spark advances at ~60px/s (1 pixel per frame at 60fps). `burn_pos = (frame * 1.0) % length`. Flame and glow at the moving burn front.
- **Wobble**: Each pixel of the fuse has a static sinusoidal vertical offset: `sin(position * 0.3) * 1.0`.

### Interactions

- Burns at ~60 px/s from ignition point.
- Ignited by: Candle, rocket trail, magnifying glass + light source, laser beam, another burning fuse.
- Transfers ignition to connected parts at the other end: Dynamite, Cannon, Rocket, Fireworks, another Fuse.
- Configurable length from 16px to 200px (affects burn time: length / 60 seconds).
- Burns from the ignition end toward the far end -- unidirectional.
