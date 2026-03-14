# Inclines

All inclines are static parts with infinite mass. All are flippable (mirror slope direction). None are destructible by dynamite. None have rope attachment points, neither require nor provide power. All inclines have a single state (Intact) and support 5 stretch/size levels that control the width, height, and effective slope angle.

---

## Brick Incline

### Overview
- **Size**: 100x36 px (default, size level 2)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: Yes
- **Resizable**: No (uses stretch/size property instead)
- **Has rope attachment**: No
- **Can serve as ramp**: Yes (primary purpose)
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| size | 0 | 4 | 1 | 2 | Stretch (0=VShort..4=VLong) |

### Size Levels
| Level | Name | Width (px) | Height (px) | Approximate Angle |
|---|---|---|---|---|
| 0 | Very Short | 30 | 28 | ~70 degrees (very steep) |
| 1 | Short | 60 | 32 | ~45 degrees |
| 2 | Medium | 100 | 36 | ~30 degrees (default) |
| 3 | Long | 150 | 40 | ~18 degrees |
| 4 | Very Long | 200 | 44 | ~10 degrees (nearly level) |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Solid incline surface | Full triangle rendered |

### Pixel Rendering

**Intact state (normal orientation, not flipped):**
1. **Filled triangle with gradient**: Vertices at (x, y), (x, y+h), (x+w, y+h).
   - Gradient from base color RED_BROWN [178, 80, 50] to darker [148, 50, 20] (base minus [30, 30, 30]).
   - The triangle is a right triangle with the vertical edge on the left and the hypotenuse sloping down to the right.
2. **Anti-aliased hypotenuse line**: From (x, y) to (x+w, y+h), color [255, 255, 255] -- white highlight along the slope surface.

**Flipped orientation:**
1. **Filled triangle with gradient**: Vertices at (x+w, y), (x, y+h), (x+w, y+h).
   - Same gradient colors. The vertical edge is on the right; hypotenuse slopes down to the left.
2. **Anti-aliased hypotenuse line**: From (x+w, y) to (x, y+h), color [255, 255, 255].

### Text/Braille Rendering
- Icon character (normal): (U+25E2) -- black lower right triangle
- Icon character (flipped): (U+25E3) -- black lower left triangle
- Icon color: RED_BROWN [178, 80, 50]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Primary ramp/slope surface. Objects roll and slide down the hypotenuse.
- Standard friction (0.4) controls rolling/sliding speed.
- Indestructible by dynamite -- dynamite does NOT destroy inclines.
- Flipping mirrors the slope direction: left-to-right vs right-to-left.
- Size levels control the steepness: shorter = steeper, longer = gentler slope.

---

## Yellow Brick Incline

### Overview
- **Size**: 100x36 px (default, size level 2)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.4
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: Yes
- **Resizable**: No (uses stretch/size property instead)
- **Has rope attachment**: No
- **Can serve as ramp**: Yes (primary purpose)
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| size | 0 | 4 | 1 | 2 | Stretch (0=VShort..4=VLong) |

### Size Levels
| Level | Name | Width (px) | Height (px) | Approximate Angle |
|---|---|---|---|---|
| 0 | Very Short | 30 | 28 | ~70 degrees (very steep) |
| 1 | Short | 60 | 32 | ~45 degrees |
| 2 | Medium | 100 | 36 | ~30 degrees (default) |
| 3 | Long | 150 | 40 | ~18 degrees |
| 4 | Very Long | 200 | 44 | ~10 degrees (nearly level) |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Solid incline surface | Full triangle rendered |

### Pixel Rendering

**Intact state (normal orientation, not flipped):**
1. **Filled triangle with gradient**: Vertices at (x, y), (x, y+h), (x+w, y+h).
   - Gradient from base color YELLOW [230, 200, 50] to darker [200, 170, 20] (base minus [30, 30, 30]).
2. **Anti-aliased hypotenuse line**: From (x, y) to (x+w, y+h), color [255, 255, 255].

**Flipped orientation:**
1. **Filled triangle with gradient**: Vertices at (x+w, y), (x, y+h), (x+w, y+h).
   - Same gradient colors.
2. **Anti-aliased hypotenuse line**: From (x+w, y) to (x, y+h), color [255, 255, 255].

### Text/Braille Rendering
- Icon character (normal): (U+25E2) -- black lower right triangle
- Icon character (flipped): (U+25E3) -- black lower left triangle
- Icon color: YELLOW [230, 200, 50]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- Identical behavior to Brick Incline. Decorative yellow variant.
- Standard friction (0.4). Indestructible by dynamite.
- Flippable and stretchable (5 size levels).

---

## Granite Incline

### Overview
- **Size**: 100x36 px (default, size level 2)
- **Type**: Static
- **Mass**: Infinity
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.3
- **Gravity**: Normal (static, unaffected)
- **Destructible by dynamite**: No
- **Flippable**: Yes
- **Resizable**: No (uses stretch/size property instead)
- **Has rope attachment**: No
- **Can serve as ramp**: Yes (primary purpose)
- **Requires power**: No
- **Provides power**: No

### Editable Properties
| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| size | 0 | 4 | 1 | 2 | Stretch (0=VShort..4=VLong) |

### Size Levels
| Level | Name | Width (px) | Height (px) | Approximate Angle |
|---|---|---|---|---|
| 0 | Very Short | 30 | 28 | ~70 degrees (very steep) |
| 1 | Short | 60 | 32 | ~45 degrees |
| 2 | Medium | 100 | 36 | ~30 degrees (default) |
| 3 | Long | 150 | 40 | ~18 degrees |
| 4 | Very Long | 200 | 44 | ~10 degrees (nearly level) |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Intact | Solid incline surface | Full triangle rendered |

### Pixel Rendering

**Intact state (normal orientation, not flipped):**
1. **Filled triangle with gradient**: Vertices at (x, y), (x, y+h), (x+w, y+h).
   - Gradient from base color GRAY [160, 160, 160] to darker [130, 130, 130] (base minus [30, 30, 30]).
2. **Anti-aliased hypotenuse line**: From (x, y) to (x+w, y+h), color [255, 255, 255].

**Flipped orientation:**
1. **Filled triangle with gradient**: Vertices at (x+w, y), (x, y+h), (x+w, y+h).
   - Same gradient colors.
2. **Anti-aliased hypotenuse line**: From (x+w, y) to (x, y+h), color [255, 255, 255].

### Text/Braille Rendering
- Icon character (normal): (U+25E2) -- black lower right triangle
- Icon character (flipped): (U+25E3) -- black lower left triangle
- Icon color: GRAY [160, 160, 160]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- None.

### Interactions
- **Lower friction (0.3)** than brick/yellow brick inclines (0.4) -- objects slide faster on granite.
- Indestructible by dynamite.
- Flippable and stretchable (5 size levels).
- The smoothest ramp surface available among inclines.
