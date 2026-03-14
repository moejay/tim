# Mechanical Parts Specification

All 20 mechanical part types. Every part in this category uses `is_static: true` physics (infinite mass, immovable by collisions). Default physics for unlisted parts: `mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity_response=Normal`.

---

## Gear

### Overview
- **Size**: 40x40 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| radius | 16.0 | 32.0 | 2.0 | 20.0 | Radius |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Not spinning | Gear drawn at fixed angle (no rotation offset) |
| 1 | Spinning | Rotating continuously | Teeth rotate at `frame * 0.08` radians per frame |

### Pixel Rendering
- **Body**: Filled circle at center `(x + radius, y + radius)` with radius from property, color SILVER `(200, 200, 210, 255)`.
- **Inner ring**: Filled circle at same center, radius `r * 0.6`, color SILVER darkened by 30 per channel `(170, 170, 180, 255)`.
- **Teeth**: 8 rectangular teeth (5x5 px each) evenly spaced around circumference at distance `r` from center. When Spinning, teeth rotate by `frame * 0.08` radians. When Idle, teeth are at angle offset 0.
- **Axle hub**: Filled circle at center, radius 3.0, color `(80, 80, 90, 255)`.

### Text/Braille Rendering
- Icon: `\u2699` (gear symbol)
- Color: SILVER `(200, 200, 210)`

### Animations
- **Spinning** (state 1): Tooth angle offset = `frame * 0.08` radians. Continuous rotation, no easing.
- Triggered by: belt drive from another spinning part, or adjacent interlocking gear.

### Interactions
- Driven by belts from other rotating parts or adjacent gears (automatic interlocking).
- Pops balloons when spinning (state 1) on contact.
- Transfers rotational motion to connected parts via belt.
- Can drive: conveyor belts, jack-in-the-box, generator, other gears.

---

## Pulley

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: Yes
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static rectangle rendering |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 24x24, color GRAY `(160, 160, 160, 200)`.
- **Top highlight**: Horizontal line at top edge, color GRAY + 30 per channel `(190, 190, 190, 255)`.
- **Bottom shadow**: Horizontal line at bottom edge, color GRAY - 30 per channel `(130, 130, 130, 255)`.

### Text/Braille Rendering
- Icon: `\u25CE` (bullseye)
- Color: GRAY `(160, 160, 160)`

### Animations
- None (`has_animation = false`).

### Interactions
- Frictionless rope redirection point. Maximum 8 pulleys per rope.
- Rope threads through pulley to change direction of force.
- Essential for connecting distant parts via rope systems.

---

## Belt

### Overview
- **Size**: 64x8 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static rectangle rendering |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 64x8, color GRAY `(160, 160, 160, 200)`.
- **Top highlight**: Horizontal line, GRAY + 30 `(190, 190, 190, 255)`.
- **Bottom shadow**: Horizontal line, GRAY - 30 `(130, 130, 130, 255)`.

### Text/Braille Rendering
- Icon: `\u2500` (box drawing horizontal)
- Color: GRAY `(160, 160, 160)`

### Animations
- None.

### Interactions
- Connects two rotating parts. Maximum stretch distance: 200 px.
- Speed ratio: 1:1 between connected parts.
- Links any spinning part to any other spinning part (gears, windmill, electric motor, mouse exercise wheel, mandrill motor to conveyor belt, jack-in-the-box, generator, other gears, etc.).

---

## Teeter-Totter

### Overview
- **Size**: 80x16 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: Yes
- **Can serve as ramp**: Yes
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| length | 60.0 | 120.0 | 10.0 | 80.0 | Length |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Level | Balanced at center | Board drawn horizontally, no tilt offset |
| 1 | TiltedLeft | Left side down | Board tilted: left end -6px, right end +6px from center |
| 2 | TiltedRight | Right side down | Board tilted: left end +6px, right end -6px from center |

### Pixel Rendering
- **Pivot triangle**: Filled triangle at bottom center `(mid, y + h - 4)` to `(mid - 8, y + h)` and `(mid + 8, y + h)`, color BROWN `(139, 90, 43, 255)`.
- **Board**: 2px thick horizontal line from `(x, y + h/2 - tilt)` to `(x + length, y + h/2 + tilt)`, color BROWN. Tilt values: Level=0, TiltedLeft=-6, TiltedRight=+6. Two lines drawn (y and y+1) for thickness.

### Text/Braille Rendering
- Icon: `\u2550` (double horizontal box drawing)
- Color: BROWN `(139, 90, 43)`

### Animations
- Tilt animation based on state transition (torque = force x distance from pivot).
- Has animation flag: Yes.

### Interactions
- Seesaw mechanics: objects landing on one side catapult objects on the other end.
- Rope can attach to either end for mechanical advantage systems.
- Can serve as a ramp/deflector surface.
- Core puzzle-solving element for weight-based puzzles.

---

## Conveyor Belt

### Overview
- **Size**: 64x16 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.1
- **Density**: 100.0
- **Friction**: 1.0
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| length | 64.0 | 256.0 | 64.0 | 64.0 | Length |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Not belt-driven | Static ribs, single faded arrow, no spoke animation |
| 1 | Running | Belt-driven, moving objects | Animated ribs, scrolling arrows, spinning wheel spokes |

### Pixel Rendering
- **Belt surface**: Filled rectangle at `(x, y + 4)`, width = length property, height 8, color GRAY `(160, 160, 160, 255)`.
- **Ribs** (Idle): Vertical lines every 8px, no offset, color GRAY - 40 `(120, 120, 120, 200)`.
- **Ribs** (Running): Vertical lines every 8px, offset = `(frame * 2) % 8`, color GRAY - 40 `(120, 120, 120, 200)`.
- **Arrows** (Running): White arrows `(255, 255, 255, 200)` every 16px, scrolling with `(frame * 2) % 16` offset. Each arrow is a horizontal line with two diagonal lines forming arrowhead.
- **Arrows** (Idle): Single faded gray arrow `(180, 180, 180, 120)` at belt center.
- **End wheels**: Two filled circles (radius 5) at `(x + 6, y + 8)` and `(x + length - 6, y + 8)`, color `(100, 100, 110, 255)`.
- **Wheel spokes** (Running only): 4 spokes per wheel at angle `frame * 0.2`, blended pixels at `(center + cos(a) * 3, center + sin(a) * 3)`, color `(60, 60, 70, 255)`.

### Text/Braille Rendering
- Icon: `\u25AC` (black rectangle)
- Color: GRAY `(160, 160, 160)`

### Animations
- **Rib scrolling**: offset = `(frame * 2) % 8` pixels per frame.
- **Arrow scrolling**: offset = `(frame * 2) % 16` pixels per frame.
- **Wheel rotation**: angle = `frame * 0.2` radians per frame.
- All triggered when state = Running (driven by belt from spinning part).

### Interactions
- Must be connected to a spinning part via belt to operate.
- Direction of object movement depends on driving part's rotation direction.
- High friction (1.0) grips objects on surface.
- Can move candles while lit. Can transport any object placed on its surface.

---

## Trampoline

### Overview
- **Size**: 48x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 1.2
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Surface at rest | Fabric at resting position (y + h/2 - 4), default blue |
| 1 | Compressed | Object landing, storing energy | Fabric drops to y + h/2 + 2, springs compressed +2px, darker blue |
| 2 | Releasing | Springing back, launching object | Fabric rises to y + h/2 - 8, brighter blue, energy release lines |

### Pixel Rendering
- **Legs**: Two diagonal lines from `(x + 4, y + h)` to `(x + 8, y + h/2)` and `(x + w - 4, y + h)` to `(x + w - 8, y + h/2)`, color `(120, 120, 130, 255)`.
- **Springs**: Zigzag lines from x+8 to x+w-8 in steps of 6px, each a short diagonal from `(x + sx, y + h/2 + compress)` to `(x + sx + 2, y + h/2 - 4 + compress)`. Compress = +2 when Compressed, 0 otherwise.
- **Fabric surface**: 2px thick horizontal line from `(x + 4, surface_y)` to `(x + w - 4, surface_y)`. Surface Y positions: Idle = `y + h/2 - 4`, Compressed = `y + h/2 + 2`, Releasing = `y + h/2 - 8`.
- **Fabric colors**: Idle = `(80, 120, 220, 255)`, Compressed = `(60, 100, 200, 255)`, Releasing = `(120, 160, 255, 255)`.
- **Energy release lines** (Releasing only): 3 horizontal lines above fabric, each progressively shorter and more transparent: `(200, 220, 255, alpha)` where alpha = 180, 130, 80.

### Text/Braille Rendering
- Icon: `\u255A` (box drawing double up and right)
- Color: BLUE `(50, 100, 220)`

### Animations
- State-driven animation. Transitions: Idle -> Compressed (on impact) -> Releasing (bounce) -> Idle.
- Max launch velocity: 1500 px/s.
- Elasticity 1.2 means objects bounce higher than they fell.

### Interactions
- Bounces objects upward on contact. Angle of approach maintained.
- Falling straight down bounces straight up; angled impacts produce angled bounces.
- Objects gain height with each bounce due to elasticity > 1.0.

---

## Jack-in-the-Box

### Overview
- **Size**: 32x32 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Closed | Lid sealed, counting rotations | Flat lid on top of box |
| 1 | WindingUp | Receiving rotation input | Lid vibrates, crank handle rotates on side |
| 2 | Open | Lid sprung open, catapult active | Lid flipped, spring + figure head visible |

### Pixel Rendering
- **Box body**: Filled rectangle at `(x, y + 12)`, size `w x (h - 12)`, color magenta `(180, 50, 180, 255)`.
- **Lid line**: Horizontal line across top of box at `y + 12`, color `(200, 80, 200, 255)`.
- **Closed** (state 0): Flat lid rectangle at `(x, y + 10)`, size `w x 3`, color `(200, 80, 200, 255)`.
- **WindingUp** (state 1):
  - Vibrating lid: rectangle at `(x + vib, y + 10)`, where `vib = sin(frame * 0.4) * 2.0`.
  - Crank handle: single blended pixel at `(x + w + 2 + cos(frame * 0.15) * 4, y + 20 + sin(frame * 0.15) * 4)`, color `(120, 120, 130, 255)`.
- **Open** (state 2):
  - Flipped lid: rectangle at `(x, y + 8)`, size `w x 3`, color `(200, 80, 200, 200)`.
  - Spring: 4 zigzag blended pixels from `y + 8` upward, wobble = `sin(sy * 2.0) * 3.0`, color `(160, 160, 170, 255)`.
  - Figure head: Filled circle at `(x + w/2, y + 2)`, radius 4, color `(255, 220, 100, 255)`.
  - Eyes: Two blended pixels at `(x + w/2 - 1, y + 1)` and `(x + w/2 + 1, y + 1)`, color `(40, 40, 40, 255)`.

### Text/Braille Rendering
- Icon: `\u2554` (box drawing double down and right)
- Color: MAGENTA `(200, 50, 200)`

### Animations
- **Lid vibration** (WindingUp): `sin(frame * 0.4) * 2.0` px horizontal offset.
- **Crank rotation** (WindingUp): angle = `frame * 0.15` radians, radius 4px.
- Opens after approximately 3 full rotations of input.

### Interactions
- Must be driven by a spinning part connected via belt.
- Catapults objects placed on its lid at approximately 500 px/s when state transitions to Open.
- Direction of catapult depends on orientation.
- Shade can be attached to rope (per TIM2 spec rope attachment list).

---

## Windmill

### Overview
- **Size**: 48x48 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: Yes
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Not spinning | Blades at fixed angle 0.4 radians |
| 1 | Spinning | Rotating continuously | Blades rotate at frame * 0.08 radians |

### Pixel Rendering
- **Center**: `(x + w/2, y + h/2)`.
- **Blade length**: `w/2 - 4` px from center.
- **4 blades**: Each blade is a line from center to tip, spaced 90 degrees apart. Angle = `frame * 0.08` (Spinning) or `0.4` (Idle).
  - Blade line: center to `(cx + cos(a) * blade_len, cy + sin(a) * blade_len)`, color BROWN `(139, 90, 43, 255)`.
  - Blade width: Additional line from 30% along blade to tip, offset perpendicular by 3px, color BROWN alpha 200.
- **Hub**: Filled circle at center, radius 4, color `(80, 80, 90, 255)`.

### Text/Braille Rendering
- Icon: `\u2731` (heavy asterisk)
- Color: BROWN `(139, 90, 43)`

### Animations
- **Spinning** (state 1): Blade angle = `frame * 0.08` radians per frame, continuous.
- **Idle** (state 0): Static angle at 0.4 radians.
- Triggered by air sources: fan, bellows/bike pump, teapot steam.

### Interactions
- Spins when blown by air (electric fan, bike pump/bellows, teapot steam).
- Can be flipped to change rotation direction.
- Drives other parts via belt connection.
- Can drive: gears, conveyor belts, jack-in-the-box, generator.

---

## Mandrill Motor

### Overview
- **Size**: 48x40 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | ShadeClosed | Shade down, monkey not pedaling | Shade rectangle covers monkey, no leg animation |
| 1 | Pedaling | Shade open, monkey pedaling | Shade retracted up, animated pedaling legs |
| 2 | Stunned | Bonked on head, stars for 2s | Shade closed, 3 rotating stars above head |

### Pixel Rendering
- **Base/frame**: Filled rectangle at `(x, y + 20)`, size `w x (h - 20)`, color `(100, 70, 40, 255)`.
- **Bicycle wheels**: Two filled circles at `(x + 12, y + 34)` radius 6 and `(x + 36, y + 34)` radius 6, color `(80, 80, 90, 255)`.
- **Bicycle frame**: Two diagonal lines forming V-shape from `(x + 12, y + 28)` to `(x + 24, y + 22)` to `(x + 36, y + 28)`, color `(80, 80, 90, 255)`.
- **Monkey body**: Filled circle (head) at `(x + 24, y + 14)` radius 6, and filled rectangle (torso) at `(x + 20, y + 20)` size 8x8, color `(160, 110, 60, 255)`.
- **ShadeClosed** (state 0): Filled rectangle at `(x + 2, y + 6)`, size 20x14, color `(200, 200, 200, 200)`. Covers monkey.
- **Pedaling** (state 1):
  - Legs: Two animated lines from torso to feet. Pedal angle = `frame * 0.2`. Foot positions: `(x + 24 + cos(a) * 6, y + 32 + sin(a) * 4)`, color `(140, 90, 40, 255)`.
  - Shade open: Small rectangle at `(x + 4, y)`, size 16x3, color `(200, 200, 200, 180)` (retracted up).
- **Stunned** (state 2):
  - Stars: 3 blended pixels orbiting above head. Phase = `frame * 0.3`. Each star at angle `phase + i * TAU/3`, radius 8 horizontal / 3 vertical from `(x + 24, y + 6)`, color `(255, 255, 100, 255)`.
  - Shade closed: same as ShadeClosed state.

### Text/Braille Rendering
- Icon: `M`
- Color: BROWN `(139, 90, 43)`

### Animations
- **Pedaling legs** (state 1): pedal angle = `frame * 0.2` radians per frame.
- **Stunned stars** (state 2): orbit phase = `frame * 0.3` radians per frame, 2-second duration.

### Interactions
- Monkey pedals only when shade is open (banana visible). Shade is attached to rope and pulled open.
- If bonked on head, monkey stops pedaling for 2 seconds (Stunned state) with stars animation.
- Drives other parts via belt connection when pedaling.
- Shade is a rope attachment point (per TIM2 spec).

---

## Mouse Exercise Wheel

### Overview
- **Size**: 40x40 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Not spinning | Spokes at angle 0, mouse sitting at bottom-left of wheel |
| 1 | Spinning | Rotating continuously | Spokes rotate, mouse orbits inside wheel |

### Pixel Rendering
- **Center**: `(x + 20, y + 20)`.
- **Wire wheel**: Filled circle at center, radius 16, color GRAY `(160, 160, 160, 120)` (semi-transparent).
- **Spokes**: 8 lines from center to radius 15, evenly spaced. Angle offset = `frame * 0.1` (Spinning) or `0` (Idle). Color GRAY `(160, 160, 160, 180)`.
- **Hub**: Filled circle at center, radius 3, color `(80, 80, 90, 255)`.
- **Mouse** (Idle): Filled circle at `(cx - 6, cy + 8)`, radius 3, color `(160, 160, 160, 255)`.
- **Mouse** (Spinning): Filled circle orbiting inside wheel at `(cx + cos(angle + 1) * 10, cy + sin(angle + 1) * 10)`, radius 3, color `(160, 160, 160, 255)`.

### Text/Braille Rendering
- Icon: `\u25CE` (bullseye)
- Color: GRAY `(160, 160, 160)`

### Animations
- **Spoke rotation** (Spinning): angle = `frame * 0.1` radians per frame.
- **Mouse orbit** (Spinning): follows spoke angle + 1.0 radian offset, radius 10px from center.

### Interactions
- Activates when nearby impact startles the mouse (object hitting wheel or landing nearby).
- Mouse direction (left/right facing) determines spin direction.
- Drives other parts via belt connection when spinning.
- Mort the Mouse proximity can also activate the wheel.

---

## Trans-Roto-Matic

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static rectangle |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 24x24, color GRAY `(160, 160, 160, 200)`.
- **Top highlight**: Horizontal line, GRAY + 30 `(190, 190, 190, 255)`.
- **Bottom shadow**: Horizontal line, GRAY - 30 `(130, 130, 130, 255)`.

### Text/Braille Rendering
- Icon: `\u229E` (squared plus)
- Color: GRAY `(160, 160, 160)`

### Animations
- None.

### Interactions
- Converts translational (linear) motion into rotational motion.
- Used in mechanical chains to bridge between linear and rotary systems.
- Rope attachment point (per TIM2 spec rope attachment list).

---

## Roto-Trans-Converter

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static rectangle |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 24x24, color GRAY `(160, 160, 160, 200)`.
- **Top highlight**: Horizontal line, GRAY + 30 `(190, 190, 190, 255)`.
- **Bottom shadow**: Horizontal line, GRAY - 30 `(130, 130, 130, 255)`.

### Text/Braille Rendering
- Icon: `\u229F` (squared minus)
- Color: GRAY `(160, 160, 160)`

### Animations
- None.

### Interactions
- Converts rotational motion into translational (linear) motion.
- Complement of Trans-Roto-Matic.
- Rope attachment point (per TIM2 spec rope attachment list).

---

## Tipsy Trailer

### Overview
- **Size**: 48x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static rectangle |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 48x24, color BROWN `(139, 90, 43, 200)`.
- **Top highlight**: Horizontal line, BROWN + 30 `(169, 120, 73, 255)`.
- **Bottom shadow**: Horizontal line, BROWN - 30 `(109, 60, 13, 255)`.

### Text/Braille Rendering
- Icon: `\u2550` (double horizontal box drawing)
- Color: BROWN `(139, 90, 43)`

### Animations
- None.

### Interactions
- Tilting platform on wheels, triggered by weight placed on one side.
- Rope attachment point (per TIM2 spec rope attachment list).

---

## Scissors

### Overview
- **Size**: 24x16 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Open | Blades apart | Blades drawn at open_angle 0.3 radians |
| 1 | Closed | Blades shut -- cuts rope, pops balloons | Blades drawn at open_angle 0.0 (parallel) |

### Pixel Rendering
- **Pivot point**: `(x + 12, y + 8)`.
- **Blades**: Two lines from pivot to `(cx + 12 * cos(open_angle), cy -/+ 12 * sin(open_angle))`. Open: angle 0.3 rad. Closed: angle 0.0 rad. Color SILVER `(200, 200, 210, 255)`.
- **Pivot circle**: Filled circle at pivot, radius 2, color `(150, 150, 160, 255)`.
- **Handles**: Two lines from pivot backward and angled: `(cx, cy)` to `(cx - 8, cy -/+ 4)`, color `(180, 60, 60, 255)`.

### Text/Braille Rendering
- Icon: `\u2702` (scissors)
- Color: SILVER `(200, 200, 210)`

### Animations
- None (state-driven, not frame-animated).

### Interactions
- Opens/closes based on pressure (object pressing on handle or sharp end).
- Closed state: cuts rope on contact, pops balloons on contact.
- Can be positioned along a rope line.
- Does NOT cut steel cable (only tin snips can).

---

## Hedge Trimmers

### Overview
- **Size**: 32x12 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state (always active) | Blades continuously animate |

### Pixel Rendering
- **Pivot**: `(x + 8, y + 6)`.
- **Blades**: Two lines from pivot to `(cx + 24, cy -/+ 6 * open)`, where `open = |sin(frame * 0.15) * 0.15|`. Continuous subtle open/close. Color GREEN `(50, 160, 50, 255)`.
- **Pivot circle**: Filled circle at pivot, radius 2, color `(80, 80, 90, 255)`.
- **Handle**: Filled rectangle at `(x, y + 3)`, size 8x6, color `(100, 70, 40, 255)` (brown wood).

### Text/Braille Rendering
- Icon: `\u2704` (white scissors)
- Color: GREEN `(50, 180, 50)`

### Animations
- **Blade oscillation**: Continuous. Open angle = `|sin(frame * 0.15) * 0.15|`. Always active regardless of state.

### Interactions
- Always active; cuts rope on contact.
- Does NOT cut steel cable.

---

## Tin Snips

### Overview
- **Size**: 24x12 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state (always active) | Blades continuously animate |

### Pixel Rendering
- **Pivot**: `(x + 8, y + 6)`.
- **Blades**: Two lines from pivot to `(cx + 16, cy -/+ 6 * open)`, where `open = |sin(frame * 0.12) * 0.2|`. Continuous subtle open/close. Color SILVER `(200, 200, 210, 255)`.
- **Pivot circle**: Filled circle at pivot, radius 2, color `(150, 150, 160, 255)`.
- **Handle**: Filled rectangle at `(x, y + 3)`, size 8x6, color `(180, 180, 190, 255)` (metallic).

### Text/Braille Rendering
- Icon: `\u2704` (white scissors)
- Color: SILVER `(200, 200, 210)`

### Animations
- **Blade oscillation**: Continuous. Open angle = `|sin(frame * 0.12) * 0.2|`. Always active.

### Interactions
- The ONLY tool that cuts BOTH rope AND steel cable.
- Always active; cuts on contact.

---

## Boxing Glove

### Overview
- **Size**: 24x32 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: Yes
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Retracted | Glove at rest | Glove at y + 8, 6 spring coils, no motion blur |
| 1 | Punching | Spring extending, ~1200 px/s force | Glove extends 12px upward, 3 compressed coils, motion blur lines |

### Pixel Rendering
- **Glove Y position**: Retracted = `y + 8`, Punching = `y + 8 - 12 = y - 4`.
- **Spring arm**: Coils drawn as zigzag pixels from glove bottom to base. Retracted = 6 coils, Punching = 3 coils (compressed). Each coil: blended pixel at `(arm_x + sin(sy * 1.5) * 3, arm_y)`, color `(160, 160, 170, 255)`. Two pixels wide.
- **Glove**: Filled circle with gradient at `(x + w/2, glove_y)`, radius 10, gradient from `(230, 50, 50)` to `(180, 30, 30)` (red).
- **Motion blur** (Punching only): 3 horizontal lines below glove at increasing distances, color `(255, 200, 200)` with decreasing alpha: 75, 50, 25.

### Text/Braille Rendering
- Icon: `B`
- Color: RED `(220, 40, 40)`

### Animations
- State-driven (not continuous frame animation). Visual changes immediate on state transition.

### Interactions
- Triggered by any object touching the flat end.
- Punches at approximately 1200 px/s.
- Sends movable objects flying in the direction it faces.
- Can be flipped to aim left or right.
- Body can be used as a ramp/deflector (per TIM2 spec).

---

## Vacuum Cleaner

### Overview
- **Size**: 40x32 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: Yes
- **Requires power**: Yes
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Off | Unpowered | Static body, no suction lines or vibration |
| 1 | On | Powered, sucking objects in ~60px radius | Animated suction lines, body vibration |

### Pixel Rendering
- **Body**: Filled rectangle at `(x, y)`, size `w x h`, color BLUE `(50, 100, 220, 200)`.
- **Motor housing**: Filled rectangle at `(x + 4, y + 4)`, size 12x20, color `(60, 90, 180, 255)`.
- **Hose**: Diagonal line from `(x + 16, y + 8)` to `(x + w - 4, y + 4)`, color `(80, 80, 90, 255)`.
- **Suction lines** (On only): 4 vertical lines extending to the right of the body. Position: `x + w + 2 + ((frame + i * 3) % 12)` for i in 0..4. Height: `y + 2` to `y + h - 2`. Color `(200, 220, 255)` with decreasing alpha: `180 - ((frame + i * 3) % 12) * 12`.
- **Vibration** (On only): Additional filled rectangle at `(x + vib, y + h - 4)`, width w, height 2, where `vib = sin(frame * 0.5) * 1.0`. Color BLUE alpha 150.

### Text/Braille Rendering
- Icon: `V`
- Color: BLUE `(50, 100, 220)`

### Animations
- **Suction lines** (On): Scrolling vertical lines moving rightward, fading with distance.
- **Body vibration** (On): Horizontal offset = `sin(frame * 0.5) * 1.0` px.

### Interactions
- Requires electrical power (must be plugged into powered outlet).
- Sucks any gravity-affected object within approximately 60px radius.
- Placing a surface over the opening traps objects against it without consuming them.
- Body can serve as a ramp surface.

---

## Pinball Bumper

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 1.5
- **Density**: 100.0
- **Friction**: 0.1
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: Yes
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Glow visible in blink pattern |

### Pixel Rendering
- **Center**: `(x + 12, y + 12)`.
- **Body**: Filled circle with gradient, center to edge: `(255, 255, 200)` to `(200, 180, 50)`, radius 12.
- **Glow ring**: Drawn when `(frame % 10 < 3) || state == 0`. Glow at center, radius 16, color `(255, 255, 100)`.
- Blink pattern creates a flashing effect: glow visible for 3 out of every 10 frames, plus always when idle.

### Text/Braille Rendering
- Icon: `\u25C9` (fisheye)
- Color: YELLOW `(230, 200, 50)`

### Animations
- **Glow blink**: Glow ring flashes with period of 10 frames (visible for frames 0-2 of each cycle).
- Has animation flag: Yes.

### Interactions
- Bounces objects away at approximately 800 px/s on contact.
- Elasticity 1.5 means objects gain energy from the bounce.
- Low friction (0.1) minimizes surface drag.
- Can serve as a ramp surface.

---

## Tack

### Overview
- **Size**: 8x8 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.0
- **Density**: 100.0
- **Friction**: 0.8
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Default state | Static rendering |

### Pixel Rendering
- **Point**: Filled triangle from `(x + 4, y)` (top center) to `(x + 1, y + 6)` (bottom left) to `(x + 7, y + 6)` (bottom right), color `(200, 200, 210, 255)`.
- **Base/head**: Filled rectangle at `(x + 1, y + 6)`, size 6x2, color `(180, 180, 190, 255)`.

### Text/Braille Rendering
- Icon: `\u25B4` (small upward triangle)
- Color: SILVER `(200, 200, 210)`

### Animations
- None.

### Interactions
- Pops balloons on contact (sharp point).
- Multiple tacks placed together create a walking surface for characters.
- Can form curved ramps when arranged in patterns.
- High friction (0.8) grips walking characters.
- Zero elasticity means no bounce.
