# Walls

All walls are static parts with infinite mass. None are flippable (except Curved Pipe Wall via rotation property). None have rope attachment points, cannot serve as ramps, and neither require nor provide power. Physics properties are shared: mass = infinity, elasticity = 0.3, density = 100.0, gravity_response = Normal. Friction varies by type.

---

## Brick Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: Yes
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Full brick pattern rendered |
| 1 | Destroyed | Demolished by dynamite | Rubble debris chunks + animated dust cloud |

### Pixel Rendering

**Intact state:**
1. **Base fill**: Solid rectangle at (x, y) with dimensions (w, h), color RED_BROWN [178, 80, 50, 255].
2. **Mortar lines**: Color [138, 50, 20, 255] (base color minus [40, 30, 30]).
   - Horizontal mortar lines every 8 pixels (brick_h = 8).
   - Vertical mortar lines every 16 pixels (brick_w = 16), offset by half brick width on alternating rows (standard running bond pattern).

**Destroyed state:**
1. **Rubble**: 12 scattered debris chunks at pseudo-random positions along the bottom half.
   - Each chunk: position calculated as `(i*37+13) % w` for x and `h - (i*23+7) % (h/2) - 4` for y.
   - Chunk size: width = 3 + (i%4), height = 2 + (i%3).
   - Color: half-brightness base [89, 40, 25, alpha] where alpha = 180 - i*10.
2. **Dust cloud**: Animated filled circle at center-bottom of the wall area.
   - Position: (x + w/2, y + h*0.6), radius w*0.3.
   - Color: [180, 170, 150, alpha] where alpha = `80 + sin(frame * 0.1) * 30` (oscillates between 50 and 110).

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: RED_BROWN [178, 80, 50]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Destroyed state**: Dust cloud alpha oscillates with formula `(80.0 + (frame as f32 * 0.1).sin() * 30.0)`.

### Interactions
- Destroyed by dynamite explosions.
- Chain-reaction: nearby dynamite explodes when this wall is destroyed.
- Standard friction (0.4) for ball rolling/sliding.

---

## Yellow Brick Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: Yes
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Full brick pattern rendered |
| 1 | Destroyed | Demolished by dynamite | Rubble debris chunks + animated dust cloud |

### Pixel Rendering

**Intact state:**
1. **Base fill**: Solid rectangle, color YELLOW [230, 200, 50, 255].
2. **Mortar lines**: Color [190, 170, 20, 255] (base color minus [40, 30, 30]).
   - Same running bond pattern as Brick Wall: brick_h = 8, brick_w = 16, alternating row offsets.

**Destroyed state:**
- Same rubble and dust cloud algorithm as Brick Wall, using yellow half-brightness colors [115, 100, 25, alpha].

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: YELLOW [230, 200, 50]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Destroyed state**: Same dust cloud oscillation as Brick Wall.

### Interactions
- Same as Brick Wall. Destructible by dynamite.

---

## Cinder Block Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: Yes
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Full cinder block pattern rendered |
| 1 | Destroyed | Demolished by dynamite | Rubble debris chunks + animated dust cloud |

### Pixel Rendering

**Intact state:**
1. **Base fill**: Solid rectangle, color GRAY [160, 160, 160, 255].
2. **Mortar lines**: Lighter gray [120, 120, 120, 255].
   - Horizontal mortar lines every 16 pixels (block_h = 16) -- larger blocks than brick.
   - Vertical mortar lines every 32 pixels (block_w = 32), alternating row offsets (running bond).

**Destroyed state:**
- Same rubble and dust cloud algorithm as Brick Wall, using gray half-brightness colors [80, 80, 80, alpha].

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: GRAY [160, 160, 160]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Destroyed state**: Same dust cloud oscillation as Brick Wall.

### Interactions
- Destructible by dynamite. Heavy construction variant.

---

## Greco-Roman Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Rendered with generic wall style |

### Pixel Rendering

**Intact state (generic wall fallback):**
1. **Base fill**: Solid rectangle, color CREAM [240, 230, 200, 255].
2. **Top highlight**: Horizontal line along top edge, color [255, 255, 240, 255] (base + 40 per channel, clamped to 255).
3. **Bottom shadow**: Horizontal line along bottom edge, color [200, 190, 160, 255] (base - 40 per channel).

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: CREAM [240, 230, 200]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible by dynamite. Classical-themed marble wall.

---

## Wooden Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: Yes
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Wood grain rendered |
| 1 | Destroyed | Demolished by dynamite | Rubble debris chunks + animated dust cloud |

### Pixel Rendering

**Intact state:**
1. **Wood grain base**: Vertical gradient fill from top [160, 110, 60] to bottom [120, 70, 30].
2. **Grain lines**: Horizontal lines every 4 pixels, color [119, 75, 33, 80] (base BROWN minus [20, 15, 10] with alpha 80).

**Destroyed state:**
- Same rubble and dust cloud algorithm as Brick Wall, using brown half-brightness colors.

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: BROWN [139, 90, 43]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Destroyed state**: Same dust cloud oscillation as Brick Wall.

### Interactions
- Destructible by dynamite. Higher friction (0.5) than standard walls.

---

## Log Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Log cross-section pattern |

### Pixel Rendering

**Intact state:**
1. **Base fill**: Solid rectangle, color DARK_BROWN [100, 60, 30, 255].
2. **Log cross-sections**: Grid of circular patterns every 16 pixels (log_d = 16) in both x and y.
   - Outer ring: Filled circle at each grid center, radius 6.0, color [80, 50, 25, 180].
   - Inner ring: Filled circle at each grid center, radius 3.0, color [60, 35, 15, 200].

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: DARK_BROWN [100, 60, 30]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible by dynamite. Higher friction (0.5). Rustic variant.

---

## Caution Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.9
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Yellow/black diagonal stripes |

### Pixel Rendering

**Intact state:**
1. **Yellow base**: Solid rectangle, color [230, 200, 50, 255].
2. **Diagonal black stripes**: Drawn across the wall at 45-degree angle.
   - Stripe spacing: stripe_w = 12, stripes every 24 pixels (2 * stripe_w).
   - Each stripe: 12 parallel diagonal lines, color [30, 30, 30, 200].
   - Lines sweep from (-h..w) to cover the full area diagonally.

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: YELLOW [230, 200, 50]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- **Highest friction of all walls (0.9)**: Objects slow down dramatically when rolling/sliding on this surface.
- Indestructible by dynamite (explosion-resistant).

---

## Sand Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Rendered with generic wall style |

### Pixel Rendering

**Intact state (generic wall fallback):**
1. **Base fill**: Solid rectangle, color TAN [210, 180, 140, 255].
2. **Top highlight**: Horizontal line along top edge, color [250, 220, 180, 255] (base + 40 per channel, clamped).
3. **Bottom shadow**: Horizontal line along bottom edge, color [170, 140, 100, 255] (base - 40 per channel).

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: TAN [210, 180, 140]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible by dynamite. Sandy textured, speckled surface.

---

## Pipe Wall

### Overview
- **Size**: 64x32 px (default)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.2
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: Yes
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| width | 16 | 256 | 16 | 64 | Width |
| height | 16 | 256 | 16 | 32 | Height |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Rendered with generic wall style |

### Pixel Rendering

**Intact state (generic wall fallback):**
1. **Base fill**: Solid rectangle, color STEEL [140, 150, 160, 255].
2. **Top highlight**: Horizontal line along top edge, color [180, 190, 200, 255] (base + 40 per channel).
3. **Bottom shadow**: Horizontal line along bottom edge, color [100, 110, 120, 255] (base - 40 per channel).

### Text/Braille Rendering
- Icon character: (U+2588) -- full block
- Icon color: STEEL [140, 150, 160]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- **Indestructible** by dynamite. Steel grey surface with rivet lines.
- **Low friction (0.2)**: Objects slide easily along this surface -- tied with Curved Pipe Wall for lowest wall friction.

---

## Curved Pipe Wall

### Overview
- **Size**: 32x32 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.2
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No (uses rotation property instead)
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| rotation | 0 | 3 | 1 | 0 | Rotation |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Quarter-circle metal pipe drawn |

### Pixel Rendering

**Intact state:**
- A quarter-circle arc drawn from a corner, with 4-pixel thickness.
- **Corner position** depends on rotation value:
  - rotation 0: top-left corner (x, y)
  - rotation 1: top-right corner (x + w, y)
  - rotation 2: bottom-right corner (x + w, y + h)
  - rotation 3: bottom-left corner (x, y + h)
- **Arc rendering**: 90 degree sweep (0..90 degrees), angle offset by `rotation * PI/2`.
  - For each degree, 4 concentric pixels are drawn (thickness loop 0..4).
  - Radius = min(w, h) - thickness offset.
  - Color: STEEL [140, 150, 160, 255].

### Text/Braille Rendering
- Icon character: (U+256E) -- box drawings light arc down and left
- Icon color: STEEL [140, 150, 160]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible by dynamite. Used for corners and lips in pipe routing.
- Low friction (0.2) -- objects slide easily along the curved surface.
- 4 rotations allow placement in any corner orientation.

---

## Grass Floor

### Overview
- **Size**: 64x16 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Grass top with dirt below |

### Pixel Rendering

**Intact state:**
1. **Dirt layer**: Brown rectangle at (x, y + h/3) with height (h - h/3), color [120, 80, 40, 255].
2. **Grass layer**: Green rectangle at (x, y) with height h/3, color [50, 160, 50, 255].
3. **Grass blade tips**: Individual pixels every 3 px along the top edge.
   - Height varies: `(gx * 7 % 5 + 2)` pixels above top edge (clamped to 1 pixel above).
   - Color: [30, 180, 30, 200].

### Text/Braille Rendering
- Icon character: (U+2594) -- upper one eighth block
- Icon color: GREEN [50, 180, 50]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible natural ground surface. Fixed size, not resizable.

---

## Scaffold Barrier

### Overview
- **Size**: 32x48 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Rendered with generic wall style |

### Pixel Rendering

**Intact state (generic wall fallback):**
1. **Base fill**: Solid rectangle, color GRAY [160, 160, 160, 255].
2. **Top highlight**: Horizontal line along top edge, color [200, 200, 200, 255].
3. **Bottom shadow**: Horizontal line along bottom edge, color [120, 120, 120, 255].

### Text/Braille Rendering
- Icon character: (U+256C) -- box drawings double vertical and horizontal
- Icon color: GRAY [160, 160, 160]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible. Fixed-size metal scaffold frame.

---

## Wooden Barrier

### Overview
- **Size**: 24x24 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Rendered with generic wall style |

### Pixel Rendering

**Intact state (generic wall fallback):**
1. **Base fill**: Solid rectangle, color BROWN [139, 90, 43, 255].
2. **Top highlight**: Horizontal line along top edge, color [179, 130, 83, 255].
3. **Bottom shadow**: Horizontal line along bottom edge, color [99, 50, 3, 255].

### Text/Braille Rendering
- Icon character: (U+2573) -- box drawings light diagonal cross
- Icon color: BROWN [139, 90, 43]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible. Small fixed-size wooden cross-plank obstruction.

---

## Lattice Archway

### Overview
- **Size**: 48x64 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Rendered with generic wall style |

### Pixel Rendering

**Intact state (generic wall fallback):**
1. **Base fill**: Solid rectangle, color WHITE [240, 240, 240, 255].
2. **Top highlight**: Horizontal line along top edge, color [255, 255, 255, 255].
3. **Bottom shadow**: Horizontal line along bottom edge, color [200, 200, 200, 255].

### Text/Braille Rendering
- Icon character: (U+256C) -- box drawings double vertical and horizontal
- Icon color: WHITE [240, 240, 240]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible decorative arch. Criss-cross lattice pattern. Fixed size.

---

## Marble Archway

### Overview
- **Size**: 48x64 px
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.3
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: No
- **Resizable**: No
- **Has rope attachment**: No
- **Can serve as ramp**: No
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Normal solid state | Rendered with generic wall style |

### Pixel Rendering

**Intact state (generic wall fallback):**
1. **Base fill**: Solid rectangle, color CREAM [240, 230, 200, 255].
2. **Top highlight**: Horizontal line along top edge, color [255, 255, 240, 255].
3. **Bottom shadow**: Horizontal line along bottom edge, color [200, 190, 160, 255].

### Text/Braille Rendering
- Icon character: (U+03A0) -- Greek capital letter Pi
- Icon color: CREAM [240, 230, 200]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Indestructible classical arch. Marble columns with arch top. Fixed size.
