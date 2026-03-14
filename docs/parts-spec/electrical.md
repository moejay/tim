# Electrical Parts Specification

All 10 electrical part types. Every part in this category uses shared default physics: `mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity_response=Normal, is_static=true`.

**Electrical system rules (from TIM2 spec):**
- Outlet must be placed on screen first, then the electric device is connected to it.
- Each outlet has 2 plug sockets — two devices can share one outlet.
- A plugged-in device shows a black circle on the outlet.
- Power chain: Generator/Switch/Solar Panel -> Outlet -> Powered device.

---

## Generator

### Overview
- **Size**: 32x32 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: Yes

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Unpowered | Not being driven by a spinning part | Dim wheel, no glow, no sparks |
| 1 | Generating | Driven, providing power to outlet | Spinning wheel, yellow glow, electric sparks |

### Pixel Rendering
- **Housing**: Filled rectangle at `(x + 4, y + 8)`, size 24x20, color `(80, 80, 90, 255)`.
- **Center**: `(x + 16, y + 12)`.
- **Unpowered** (state 0):
  - Dim wheel: Filled circle at center, radius 10, color `(YELLOW/2)` = `(115, 100, 25, 255)`. No animation.
- **Generating** (state 1):
  - Spinning spokes: 4 lines from center outward, radius 8, spaced 90 degrees, angle = `frame * 0.15`. Color YELLOW `(230, 200, 50, 255)`.
  - Bright wheel: Filled circle at center, radius 10, color YELLOW `(230, 200, 50, 200)`.
  - Glow: Yellow glow ring at center, radius 14, color `(255, 255, 100)`.
  - Electricity sparks: Every other 2 frames (`frame % 4 < 2`), a single spark pixel at `(cx + cos(frame * 0.7) * 12, cy + sin(frame * 0.7) * 12)`, color `(255, 255, 200, 255)`.
- **Axle hub** (both states): Filled circle at center, radius 4, color `(60, 60, 70, 255)`.

### Text/Braille Rendering
- Icon: `\u26A1` (high voltage / lightning bolt)
- Color: YELLOW `(230, 200, 50)`

### Animations
- **Spoke rotation** (Generating): angle = `frame * 0.15` radians per frame.
- **Spark orbit** (Generating): angle = `frame * 0.7` radians per frame, visible only on even 2-frame windows.
- Has animation flag: Yes.

### Interactions
- Must be connected to a spinning part via belt to generate power.
- Has an outlet (2 sockets) for powering electric devices.
- Spinning parts that can drive it: gears, windmill, electric motor, mouse exercise wheel, mandrill motor.
- Part of the power chain pattern: spinning part -> belt -> generator -> outlet -> device.

---

## Electrical Outlet

### Overview
- **Size**: 16x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: Yes

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | On | Always providing power | Green power indicator always lit |

### Pixel Rendering
- **Faceplate**: Filled rectangle at `(x, y)`, size 16x24, color off-white `(230, 225, 220, 255)`.
- **Top border**: Horizontal line at `y`, color `(200, 200, 200, 255)`.
- **Socket holes**: Two filled rectangles (plug slots):
  - Left slot: `(x + 4, y + 8)`, size 3x4, color `(40, 40, 40, 255)`.
  - Right slot: `(x + 9, y + 8)`, size 3x4, color `(40, 40, 40, 255)`.
- **Power indicator**: Single blended pixel at `(x + 8, y + 2)`, color `(100, 255, 100, 200)` (green LED).

### Text/Braille Rendering
- Icon: `\u25AA` (black small square)
- Color: WHITE `(240, 240, 240)`

### Animations
- None (`has_animation = false`).

### Interactions
- Always-on power source. No input required.
- 2 sockets: two electric devices can plug in simultaneously.
- Plugged-in device shows a black circle on the outlet.
- Powers: electric fan, electric motor, toaster, can opener, electric mixer, vacuum cleaner.

---

## Switch Outlet

### Overview
- **Size**: 16x32 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: Yes
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: Yes

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Off | Switch in original position | Switch rectangle drawn at top of housing, no green LED |
| 1 | On | Toggled by falling object, providing power | Switch rectangle drawn at bottom of housing, green LED lit |

### Pixel Rendering
- **Faceplate**: Filled rectangle at `(x, y)`, size 16x32, color off-white `(230, 225, 220, 255)`.
- **Top border**: Horizontal line at `y`, color `(200, 200, 200, 255)`.
- **Socket holes**: Two filled rectangles at `(x + 4, y + h/2)` and `(x + 9, y + h/2)`, each size 3x4, color `(40, 40, 40, 255)`.
- **Switch housing**: Filled rectangle at `(x + 5, y + 4)`, size 6x10, color `(180, 180, 180, 255)`.
- **Off** (state 0): Switch toggle at `(x + 6, y + 4)`, size 4x5, color `(220, 220, 220, 255)` (switch up).
- **On** (state 1):
  - Switch toggle at `(x + 6, y + 9)`, size 4x5, color `(220, 220, 220, 255)` (switch down).
  - Green LED: blended pixel at `(x + 8, y + 2)`, color `(100, 255, 100, 200)`.

### Text/Braille Rendering
- Icon: `\u25AA` (black small square)
- Color: WHITE `(240, 240, 240)`

### Animations
- None (state-driven, not frame-animated).

### Interactions
- Toggled by a falling/moving object striking the switch.
- Switch can be oriented up or down (flippable).
- When On, provides power to 2 sockets like a standard outlet.
- Flipping changes which direction the switch must be pushed.

---

## Solar Panel

### Overview
- **Size**: 32x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: Yes

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Dark | No light source within range | Dim panel (brightness 80), no glow |
| 1 | Powered | Light source within ~80px, line-of-sight | Bright panel (brightness 220), sun reflection glow |

### Pixel Rendering
- **Panel body**: Filled rectangle at `(x, y)`, size 32x24. Color varies by state:
  - Dark: `(40, 80/3=26, 80, 255)` = dark blue.
  - Powered: `(40, 220/3=73, 220, 255)` = bright blue.
- **Grid lines (vertical)**: Every 8px, vertical line from `y` to `y + h`, color `(60, 100, 200, 180)`.
- **Grid lines (horizontal)**: Every 6px, horizontal line from `x` to `x + w`, color `(60, 100, 200, 180)`.
- **Top frame**: Horizontal line at `y`, color `(120, 120, 130, 255)`.
- **Bottom frame**: Horizontal line at `y + h - 1`, color `(120, 120, 130, 255)`.
- **Sun reflection** (Powered only): Glow at panel center `(x + w/2, y + h/2)`, radius 8, color `(255, 255, 200)`.

### Text/Braille Rendering
- Icon: `\u25A6` (square with diagonal crosshatch fill)
- Color: BLUE `(50, 100, 220)`

### Animations
- None (`has_animation = false`). Visual changes are state-driven only.

### Interactions
- Powered by light source within approximately 80px with line-of-sight.
- Valid light sources: flashlight, lava lamp (light with drawstring), candle.
- When powered, provides electricity to 2 sockets.
- Part of the power chain: light source -> solar panel -> outlet -> device.

---

## Laser-Activated Plug

### Overview
- **Size**: 16x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: Yes

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| activation_color | 0.0 | 2.0 | 1.0 | 0.0 | Color (0=R/1=G/2=B) |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Inactive | Not receiving correct laser color | Dim body (brightness 150) |
| 1 | Active | Matching laser hitting sensor | Bright body (brightness 255), spinning indicator |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 16x24.
  - Inactive: color = `(RED * 150/255, 0, 0, 200)` = dim red (for default color=0).
  - Active: color = full RED `(220, 40, 40, 200)` (for default color=0).
  - Color varies based on `activation_color` property: 0=Red, 1=Green, 2=Blue.
- **Top highlight**: Horizontal line at `y`, brighter by 30.
- **Active indicator** (state 1): Spinning blended pixel at `(x + w/2 + cos(frame * 0.15) * 4, y + h/2 + sin(frame * 0.15) * 4)`, color `(255, 255, 200, 200)`.

### Text/Braille Rendering
- Icon: `\u25C9` (fisheye)
- Color: RED `(220, 40, 40)`

### Animations
- **Active indicator** (state 1): Spinning pixel orbit at `frame * 0.15` radians per frame, radius 4px from center.

### Interactions
- Color-programmable: set to Red (0), Green (1), or Blue (2).
- Activates only when hit by a laser beam of the matching color.
- When active, provides power to 2 sockets like a standard outlet.
- Part of the laser-electrical integration: laser -> matching plug -> outlet -> device.

---

## Electric Fan

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: Yes
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
| 0 | Off | Unpowered, static | Blades drawn at fixed angles, dim color (half brightness) |
| 1 | On | Powered, operating | Fast-spinning blades, airflow lines extending in facing direction |

### Pixel Rendering
- **Center**: `(x + 12, y + 12)`.
- **Off** (state 0):
  - 5 static blades from center to radius 10, evenly spaced at `i * TAU/5`. Color `(BLUE/2)` = `(25, 50, 110, 180)`.
- **On** (state 1):
  - 5 spinning blades, angle = `frame * 0.3`, from center to radius 10. Color BLUE `(50, 100, 220, 220)`.
  - Airflow lines: 3 sets of horizontal lines in facing direction (flipped = left, default = right). Each set at `cy - 3`, `cy`, `cy + 3`. Starting position: `cx + direction * (14 + i * 6 + (frame * 0.2) % 6)`, length = `direction * 4`. Color `(200, 220, 255)` with decreasing alpha: 120, 90, 60.
  - `direction = -1` if `flipped`, else `+1`.
- **Hub** (both states): Filled circle at center, radius 3, color `(80, 80, 90, 255)`.

### Text/Braille Rendering
- Icon: `\u274A` (twelve pointed black star)
- Color: BLUE `(50, 100, 220)`

### Animations
- **Blade rotation** (On): angle = `frame * 0.3` radians per frame (fast spin).
- **Airflow lines** (On): Scrolling with `(frame * 0.2) % 6` offset, creating streaming air visual.
- Has animation flag: Yes.

### Interactions
- Requires electrical power (must be plugged into powered outlet).
- Blows continuous directional airflow approximately 120px range.
- Can push objects and spin windmills.
- Airflow direction determined by flip state.
- Can blow out candles.
- Body can serve as a ramp surface.

---

## Electric Motor

### Overview
- **Size**: 20x20 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: Yes
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
| 0 | Off | Unpowered, static | No axle animation, no vibration |
| 1 | On | Powered, operating | Spinning axle line, body vibration |

### Pixel Rendering
- **Body**: Filled rectangle at `(x, y)`, size 20x20, color GRAY `(160, 160, 160, 200)`.
- **Motor circle**: Filled circle at center `(x + 10, y + 10)`, radius 8, color `(100, 100, 110, 255)`.
- **On** (state 1):
  - Spinning axle: Line from center outward, length 7, angle = `frame * 0.2`. Color `(60, 60, 70, 255)`.
  - Vibration stripe: Filled rectangle at `(x + vib, y)`, size 2xh, where `vib = sin(frame * 0.5) * 1.0`. Color `(120, 120, 130, 120)`.
- **Hub** (both states): Filled circle at center, radius 3, color `(60, 60, 70, 255)`.

### Text/Braille Rendering
- Icon: `\u2299` (circled dot)
- Color: GRAY `(160, 160, 160)`

### Animations
- **Axle rotation** (On): angle = `frame * 0.2` radians per frame.
- **Body vibration** (On): horizontal offset = `sin(frame * 0.5) * 1.0` px.
- Has animation flag: Yes.

### Interactions
- Requires electrical power (must be plugged into powered outlet).
- Provides continuous rotation when powered.
- Drives other parts via belt connection (gears, conveyor belts, generator, etc.).
- Can be reversed (flippable) to change rotation direction.
- Body can serve as a ramp surface.

---

## Toaster

### Overview
- **Size**: 24x20 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: Yes
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| delay | 0.0 | 2.0 | 1.0 | 1.0 | Delay (0=light/1=med/2=burnt) |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Off | Unpowered | Dark slots, no glow |
| 1 | Heating | Powered, countdown to launch | Orange glowing slots with glow effect |
| 2 | Popped | Toast launched at ~600 px/s | Dark slots, toast pieces flying upward |

### Pixel Rendering
- **Body**: Filled rectangle with vertical gradient at `(x, y)`, size 24x20. Top color `(210, 210, 220)`, bottom `(170, 170, 180)`.
- **Lever**: Filled rectangle at `(x + w - 3, y + 6)`, size 3x8, color `(150, 150, 160, 255)`.
- **Off** (state 0):
  - Two dark toast slots: `(x + 4, y + 2)` size 6x3 and `(x + 14, y + 2)` size 6x3, color `(40, 40, 40, 255)`.
- **Heating** (state 1):
  - Two glowing slots: same positions, color `(255, 120, 40, 255)` (orange-red glow).
  - Glow effects: Two glow circles at `(x + 7, y + 3)` and `(x + 17, y + 3)`, radius 5, color `(255, 150, 50)`.
- **Popped** (state 2):
  - Two dark empty slots: same positions, color `(40, 40, 40, 255)`.
  - Toast pieces: Two filled rectangles (4x5 each) flying upward:
    - Toast 1: at `(x + 5, y - 8 - (frame * 2) % 16)`, color `(220, 190, 120, 255)`.
    - Toast 2: at `(x + 15, y - 6 - (frame * 2) % 16)`, color `(220, 190, 120, 255)`.

### Text/Braille Rendering
- Icon: `\u25AC` (black rectangle)
- Color: SILVER `(200, 200, 210)`

### Animations
- **Toast launch** (Popped): Toast Y offset = `-(frame * 2) % 16` px, creating upward scrolling motion.
- Glow effects are state-driven (not frame-animated).

### Interactions
- Requires electrical power.
- Launches 2 pieces of toast at approximately 600 px/s after a programmable delay.
- Delay settings: 0 = light (short), 1 = medium, 2 = burnt (long).
- Toast pieces become physics objects after launch (subject to gravity).

---

## Can Opener

### Overview
- **Size**: 20x16 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: Yes
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Off | Unpowered, static | Dim body (brightness 150), no spinning indicator |
| 1 | On | Powered, operating | Full brightness, spinning indicator pixel |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 20x16.
  - Off: color = SILVER dimmed `(200*150/255, 200*150/255, 210*150/255, 200)` = approximately `(118, 118, 124, 200)`.
  - On: color = full SILVER `(200, 200, 210, 200)`.
- **Top highlight**: Horizontal line at `y`, brighter by 30.
- **Active indicator** (On): Spinning blended pixel at `(x + w/2 + cos(frame * 0.15) * 4, y + h/2 + sin(frame * 0.15) * 4)`, color `(255, 255, 200, 200)`.

### Text/Braille Rendering
- Icon: `\u22A1` (squared dot)
- Color: SILVER `(200, 200, 210)`

### Animations
- **Active indicator** (On): Spinning pixel at `frame * 0.15` radians per frame, radius 4px.

### Interactions
- Requires electrical power (must be plugged into powered outlet).
- Opens cans when powered (mechanical opener function).

---

## Electric Mixer

### Overview
- **Size**: 20x24 px
- **Type**: Static
- **Mass**: INF
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: Yes
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Off | Unpowered, static | Dim body (brightness 150), no spinning indicator |
| 1 | On | Powered, operating | Full brightness, spinning indicator pixel |

### Pixel Rendering
- **Fallback rendering**: Filled rectangle at `(x, y)`, size 20x24.
  - Off: color = SILVER dimmed `(118, 118, 124, 200)`.
  - On: color = full SILVER `(200, 200, 210, 200)`.
- **Top highlight**: Horizontal line at `y`, brighter by 30.
- **Active indicator** (On): Spinning blended pixel at `(x + w/2 + cos(frame * 0.15) * 4, y + h/2 + sin(frame * 0.15) * 4)`, color `(255, 255, 200, 200)`.

### Text/Braille Rendering
- Icon: `\u229B` (circled asterisk)
- Color: SILVER `(200, 200, 210)`

### Animations
- **Active indicator** (On): Spinning pixel at `frame * 0.15` radians per frame, radius 4px.
- Has animation flag: Yes.

### Interactions
- Requires electrical power (must be plugged into powered outlet).
- Rotating beaters function when powered.
- Can drive belts when powered (acts as a rotational source).
- Connects to other mechanical parts via belt system.
