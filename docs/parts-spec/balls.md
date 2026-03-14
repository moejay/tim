# Balls

All balls are dynamic (non-static) parts. None are destructible by dynamite, resizable, or flippable. None have rope attachment points, cannot serve as ramps, and neither require nor provide power. All balls share three states (Idle, Moving, AtRest) and use the same rendering pipeline with type-specific detail overlays.

---

## Bowling Ball

### Overview
- **Size**: 32x32 px (radius 16.0)
- **Type**: Dynamic
- **Mass**: 7.0
- **Elasticity**: 0.1
- **Density**: 8.0
- **Friction**: 0.4
- **Gravity**: AlwaysFalls
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
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 16.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center color [60, 60, 70] to edge color [30, 30, 35]. When AtRest, colors are halved: center [30, 30, 35], edge [15, 15, 17].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 4.0, color [255, 255, 255, 120].

**Type-specific detail:**
- Three finger holes drawn as small filled circles with color [20, 20, 25, 200]:
  - Left hole: (cx - r*0.15, cy - r*0.25), radius r*0.12 = 1.92
  - Right hole: (cx + r*0.15, cy - r*0.25), radius r*0.12 = 1.92
  - Top hole: (cx, cy - r*0.5), radius r*0.12 = 1.92

**Moving state motion blur:**
- 4 trailing circles drawn behind the ball, each offset by i*4 pixels to the left.
- Trail circle i (1..=4): position (cx - i*4, cy), radius 16.0 * (1.0 - i*0.1), alpha (120 - i*25).

**AtRest "zzz" dots:**
- 3 pixels drawn at (cx + r + 4, cy - r - 4), (cx + r + 8, cy - r - 1), (cx + r + 12, cy - r + 2), color [180, 180, 220, 200].
- Idle phase calculated as `(frame * 0.05).sin()` but currently unused (reserved for subtle bob).

### Text/Braille Rendering
- Icon character: (U+2B24) -- large filled circle (unique among balls)
- Icon color: DARK_GRAY [80, 80, 80]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages, drawn every frame.
- **AtRest state**: Idle phase `(frame as f32 * 0.05).sin()` computed but not yet applied visually.

### Interactions
- Heaviest ball (mass 7.0); can push heavy objects on collision.
- AlwaysFalls gravity: unaffected by gravity slider adjustments.
- Minimal bounce (elasticity 0.1).

---

## Cannonball

### Overview
- **Size**: 24x24 px (radius 12.0)
- **Type**: Dynamic
- **Mass**: 6.0
- **Elasticity**: 0.0
- **Density**: 10.0
- **Friction**: 0.3
- **Gravity**: AlwaysFalls
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
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 12.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center color [40, 40, 45] to edge color [20, 20, 22]. When AtRest, center [20, 20, 22], edge [10, 10, 11].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 3.0, color [255, 255, 255, 120].

**Type-specific detail:**
- None. Plain dark iron sphere.

**Moving state motion blur:**
- 4 trailing circles drawn behind the ball, each offset by i*4 pixels to the left.
- Trail circle i (1..=4): position (cx - i*4, cy), radius 12.0 * (1.0 - i*0.1), alpha (120 - i*25).

**AtRest "zzz" dots:**
- Same as Bowling Ball pattern, offset relative to radius 12.0.

### Text/Braille Rendering
- Icon character: (U+25CF) -- filled circle
- Icon color: DARK_GRAY [80, 80, 80]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.
- **AtRest state**: Idle phase computed but not applied.

### Interactions
- Dense iron ball (density 10.0) -- densest of all balls.
- AlwaysFalls gravity: unaffected by gravity slider.
- Zero bounce (elasticity 0.0) -- no rebound at all.
- Also fired from the Cannon part (Pyrotechnic category); cannonball follows gravity after firing.

---

## Basketball

### Overview
- **Size**: 28x28 px (radius 14.0)
- **Type**: Dynamic
- **Mass**: 0.6
- **Elasticity**: 0.75
- **Density**: 0.5
- **Friction**: 0.6
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
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 14.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [240, 150, 40] to edge [180, 90, 20]. When AtRest, center [120, 75, 20], edge [90, 45, 10].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 3.5, color [255, 255, 255, 120].

**Type-specific detail -- seam lines:**
- Horizontal seam: line from (cx - r*0.9, cy) to (cx + r*0.9, cy), color [60, 30, 10, 180].
- Vertical seam: line from (cx, cy - r*0.9) to (cx, cy + r*0.9), color [60, 30, 10, 180].

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels to the left.
- Trail circle i: radius 14.0 * (1.0 - i*0.1), alpha (120 - i*25).

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: ORANGE [230, 140, 30]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.

### Interactions
- Normal gravity: falls under normal/strong gravity, rises under weak gravity.
- Good bounce (elasticity 0.75); suitable for bouncing puzzles.
- Low density (0.5) means it can be affected by atmospheric pressure.

---

## Soccer Ball

### Overview
- **Size**: 26x26 px (radius 13.0)
- **Type**: Dynamic
- **Mass**: 0.45
- **Elasticity**: 0.6
- **Density**: 0.4
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
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 13.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [250, 250, 250] to edge [200, 200, 200]. When AtRest, center [125, 125, 125], edge [100, 100, 100].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 3.25, color [255, 255, 255, 120].

**Type-specific detail -- pentagon pattern:**
- 5 filled circles arranged in a pentagonal pattern, evenly spaced around the center.
- Each pentagon dot: angle = `i * TAU / 5 - PI/2` (i=0..5), position (cx + cos(angle) * r*0.5, cy + sin(angle) * r*0.5), radius r*0.15 = 1.95, color [40, 40, 40, 160].

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels.

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: WHITE [240, 240, 240]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.

### Interactions
- Normal gravity: falls under normal/strong, rises under weak.
- Medium bounce (elasticity 0.6).
- Lowest density of all balls (0.4) -- most affected by atmospheric pressure.

---

## Baseball

### Overview
- **Size**: 12x12 px (radius 6.0)
- **Type**: Dynamic
- **Mass**: 0.15
- **Elasticity**: 0.3
- **Density**: 1.5
- **Friction**: 0.4
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
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 6.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [250, 245, 240] to edge [220, 210, 200]. When AtRest, center [125, 122, 120], edge [110, 105, 100].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 1.5, color [255, 255, 255, 120].

**Type-specific detail -- red stitching:**
- 8 individual red pixels along a curved arc.
- For i in 0..8: t = i/7, angle = `t * PI - PI/2`, position (cx + cos(angle) * r*0.6, cy + sin(angle) * r*0.6), color [200, 30, 30, 200].
- Each stitch is a single blended pixel.

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels.

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: WHITE [240, 240, 240]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.

### Interactions
- Normal gravity: falls under normal/strong, rises under weak.
- Small ball (radius 6.0) -- fits through tight gaps.
- Low bounce (elasticity 0.3).

---

## Tennis Ball

### Overview
- **Size**: 10x10 px (radius 5.0)
- **Type**: Dynamic
- **Mass**: 0.06
- **Elasticity**: 0.8
- **Density**: 0.3
- **Friction**: 0.7
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
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 5.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [200, 230, 60] to edge [140, 180, 30]. When AtRest, center [100, 115, 30], edge [70, 90, 15].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 1.25, color [255, 255, 255, 120].

**Type-specific detail -- seam line:**
- Anti-aliased horizontal line from (cx - r*0.7, cy) to (cx + r*0.7, cy), color [255, 255, 255].

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels.

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: YELLOW_GREEN [180, 220, 50]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.

### Interactions
- Lightest ball (mass 0.06) and smallest standard ball (radius 5.0).
- Normal gravity: falls under normal/strong, rises under weak.
- High bounce (elasticity 0.8) and high friction (0.7).
- Very low density (0.3) -- highly affected by atmospheric pressure.

---

## Super Ball

### Overview
- **Size**: 14x14 px (radius 7.0)
- **Type**: Dynamic
- **Mass**: 0.1
- **Elasticity**: 1.1
- **Density**: 0.8
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
|---|---|---|---|---|---|
| *(none)* | | | | | |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 7.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [220, 60, 220] to edge [140, 20, 140]. When AtRest, center [110, 30, 110], edge [70, 10, 70].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 1.75, color [255, 255, 255, 120].

**Type-specific detail -- glow effect:**
- `draw_glow` called at (cx, cy) with radius r*1.3 = 9.1, color [255, 100, 255].
- This produces a soft magenta halo around the ball extending beyond its physical radius.

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels.

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: MAGENTA [200, 50, 200]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.
- The glow effect is drawn every frame (static glow, not animated).

### Interactions
- **Elasticity > 1.0 (1.1)**: Gains height with each bounce. The only ball that gains energy on collision.
- Normal gravity: falls under normal/strong, rises under weak.
- Chaotic trajectory on angled surfaces due to energy gain.

---

## Pinball

### Overview
- **Size**: 10x10 px (radius 5.0)
- **Type**: Dynamic
- **Mass**: 0.08
- **Elasticity**: 0.15
- **Density**: 8.0
- **Friction**: 0.2
- **Gravity**: AlwaysFalls
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
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 5.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [210, 210, 220] to edge [150, 150, 160]. When AtRest, center [105, 105, 110], edge [75, 75, 80].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 1.25, color [255, 255, 255, 120].

**Type-specific detail:**
- None. Plain metallic silver sphere.

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels.

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: SILVER [200, 200, 210]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.

### Interactions
- AlwaysFalls gravity: unaffected by gravity slider.
- Very low bounce (elasticity 0.15) and very low friction (0.2) -- slides easily.
- High density (8.0) despite low mass (0.08) -- dense small ball.

---

## Pool Ball

### Overview
- **Size**: 12x12 px (radius 6.0)
- **Type**: Dynamic
- **Mass**: 0.17
- **Elasticity**: 0.9
- **Density**: 2.0
- **Friction**: 0.3
- **Gravity**: ZeroGravity
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
| surface_number | 0 | 15 | 1 | 8 | Number |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 6.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [200, 40, 40] to edge [120, 20, 20]. When AtRest, center [100, 20, 20], edge [60, 10, 10].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 1.5, color [255, 255, 255, 120].

**Type-specific detail -- number circle:**
- White inner circle: filled circle at (cx, cy) with radius r*0.45 = 2.7, color [255, 255, 255, 220].
- Number text: reads `surface_number` property (default 8), renders `num % 10` as text centered on the ball at (cx - text_width/2, cy - 3), color [0, 0, 0, 255], scale 1.

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels.

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: RED [220, 40, 40]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.

### Interactions
- **ZeroGravity**: Floats in place until struck by another object. Completely unaffected by gravity.
- Moves like a pool ball on a table -- momentum-based movement, bounces off surfaces until momentum is lost.
- High elasticity (0.9) for bouncy collisions.
- Programmable surface number (0-15) displayed on the ball.

---

## Programmable Ball

### Overview
- **Size**: 16x16 px (radius 8.0)
- **Type**: Dynamic
- **Mass**: 1.0 (default, adjustable)
- **Elasticity**: 0.8 (default, adjustable)
- **Density**: 1.0 (default, adjustable)
- **Friction**: 0.5 (default, adjustable)
- **Gravity**: Custom(1.0) (default, adjustable)
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
| mass | 0.01 | 10.0 | 0.1 | 1.0 | Mass |
| elasticity | 0.0 | 2.0 | 0.05 | 0.8 | Elasticity |
| density | 0.1 | 10.0 | 0.1 | 1.0 | Density |
| friction | 0.0 | 2.0 | 0.05 | 0.5 | Friction |
| gravity_factor | 0.0 | 2.0 | 0.1 | 1.0 | Gravity Factor |

### States
| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Idle | At rest or placed | Normal rendering |
| 1 | Moving | In motion during simulation | Motion blur trail drawn behind ball |
| 2 | AtRest | Velocity < 0.5 px/s for 30 ticks | Colors dimmed to half brightness; "zzz" dots drawn |

### Pixel Rendering

**Base sphere (all states):**
1. **Drop shadow**: Filled circle at (cx+2, cy+2) with radius 8.0, color [0, 0, 0, 80].
2. **Gradient sphere**: Radial gradient from center [0, 190, 190] to edge [0, 120, 120]. When AtRest, center [0, 95, 95], edge [0, 60, 60].
3. **Specular highlight**: Filled circle at (cx - r*0.3, cy - r*0.3) with radius r*0.25 = 2.0, color [255, 255, 255, 120].

**Type-specific detail -- hexagonal dot pattern:**
- 6 blended pixels arranged in a hexagonal pattern around the center.
- For i in 0..6: angle = `i * TAU / 6`, position (cx + cos(angle) * r*0.5, cy + sin(angle) * r*0.5), color [0, 100, 100, 180].

**Moving state motion blur:**
- 4 trailing circles, each offset by i*4 pixels.

### Text/Braille Rendering
- Icon character: (U+25CF)
- Icon color: TEAL [0, 180, 180]
- Braille: Rendered via pixel-to-braille conversion

### Animations
- **Moving state**: Motion blur trail with 4 fading afterimages.

### Interactions
- Fully customizable physics via 5 sliders.
- Gravity uses `Custom(gravity_factor)` -- at 0.0 the ball floats; at 2.0 it falls at double gravity strength.
- Elasticity up to 2.0 allows extreme energy-gaining bounces (more than Super Ball at default).
- The most versatile ball for puzzle design.
