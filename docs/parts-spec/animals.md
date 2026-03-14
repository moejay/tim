# Animal Parts Specification

---

## Pokey the Cat

### Overview
- **Size**: 24x20 px
- **Type**: Dynamic
- **Mass**: 0.8
- **Elasticity**: 0.1
- **Density**: 1.0
- **Friction**: 0.7
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (tail wag, leg movement, startle effects)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Sitting still; occasional meow | Normal fur color, slow tail wag, no leg animation |
| 1 | Walking | Moving at 50-80 px/s toward target | Moderate leg animation, moderate tail wag |
| 2 | Chasing | Pursuing mouse or broken fishbowl (200px LOS) | Wide eyes, fast tail wag, fast leg animation |
| 3 | Startled | Shriek + fur standing up (1.0s) | Bright fur, puffed ears, wide eyes, flashing stars |

### Pixel Rendering

- **Fur color**: Normal=`[230, 160, 60, 255]` (orange), Startled=`[255, 200, 80, 255]` (bright orange).
- **Body**: Filled rect at (x+4, y+8), 16x10 px, fur_color.
- **Head**: Filled circle at (x+8, y+6), radius 6px, fur_color.
- **Ears**: Two triangles. Normal: width 2px, top at y. Startled: width 3px, top at y-2 (puffed up). Left ear tip at (x+4), right at (x+12). Base at y+4. Color: fur_color.
- **Eyes**: Pixel blocks at (x+6, y+5) and (x+10, y+5). Normal: 1x1 px. Chasing/Startled: 2x2 px. Color `[30, 30, 30, 255]`.
- **Tail**: Line from (x+20, y+10) to (x+22, y+6 + tail_wave). Color `[210, 140, 40, 255]`.
  - `tail_wave = sin(frame * tail_speed) * 3.0`
  - Speed: Idle=0.1, Chasing=0.25, Startled=0.4.
- **Legs**: Two lines from body bottom to feet. Animated when walking/chasing.
  - `leg_anim = sin(frame * leg_speed) * 2.0`
  - Speed: Walking=0.15, Chasing=0.3, otherwise 0.
  - Front leg: (x+6, y+18) to (x+6+leg_anim, y+20).
  - Back leg: (x+18, y+18) to (x+18-leg_anim, y+20).
  - Color `[210, 140, 40, 255]`.
- **Startled stars** (State 3): Three blinking pixels at (x-2, y-2), (x+16, y-3), (x+20, y+2). Color `[255, 255, 100, 255]`. Visible when `frame % 6 < 3`.

### Text/Braille Rendering

- **Icon**: `C`
- **Color**: ORANGE `[230, 140, 30]`

### Animations

- **Tail wag**: `sin(frame * speed) * 3.0`. Speed varies by state (0.1 / 0.25 / 0.4).
- **Leg walk cycle**: `sin(frame * speed) * 2.0`. Speed: Walking=0.15, Chasing=0.3.
- **Startle flash**: Stars blink on 6-frame cycle (3 on, 3 off). Duration: 1.0s.

### AI Behavior

- **Idle**: Sits still until a target enters line of sight.
- **Chase trigger**: Mouse or broken fishbowl detected within 200px line of sight.
- **Chase speed**: 80 px/s toward target.
- **Walking speed**: 50-80 px/s.
- **Startle trigger**: Hit by any moving object. Duration: 1.0s. Shriek audio plays. Fur brightens, ears puff up.
- **NOT eaten by alligator**: Edison Alligator ignores Pokey.
- **Heavier than bucket**: Mass 0.8 > Bucket mass 0.5.

### Interactions

- Chases Mort the Mouse when in 200px LOS.
- Attracted to broken Bill's Fish Tank (walks toward broken fishbowl).
- Can be trapped by Laundry Basket dropped on top.
- Startled (shriek + puffed fur) when hit by any object.
- NOT eaten by Edison Alligator.
- Can be used as weight on levers and pulleys.

---

## Mort the Mouse

### Overview
- **Size**: 10x8 px
- **Type**: Dynamic
- **Mass**: 0.02
- **Elasticity**: 0.1
- **Density**: 0.5
- **Friction**: 0.6
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (leg scurry, tail wag)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Sitting still | Static mouse, no leg animation |
| 1 | Fleeing | Running from cat at 70 px/s | Fast leg animation, fast tail |
| 2 | SeekingCheese | Running toward cheese (150px range, 60 px/s) | Moderate leg animation |
| 3 | Eaten | Consumed by alligator | Fading poof of gray pixels |
| 4 | Safe | Entered mouse hole | Only eyes visible peeking from hole |

### Pixel Rendering

- **States 0, 1, 2 (Active)**:
  - Body: Filled circle at (x+5, y+5), radius 4px, color `[160, 160, 160, 255]` (gray).
  - Ear: Filled circle at (x+2, y+2), radius 2px, color `[180, 150, 150, 255]` (pink-gray).
  - Eye: Pixel at (x+3, y+4), color `[30, 30, 30, 255]`.
  - Tail: Line from (x+8, y+5) to (x+10, y+3 + tail_wave). Color `[140, 140, 140, 255]`.
    - `tail_wave = sin(frame * (0.2 + leg_speed)) * 2.0`
  - Legs (animated when moving): Two pixels at (x+3 + leg, y+8) and (x+7 - leg, y+8). Color `[140, 140, 140, 255]`.
    - `leg = sin(frame * leg_speed) * 2.0`
    - Speed: Fleeing=0.35, SeekingCheese=0.2, Idle=0.
- **State 3 (Eaten)**: Single fading gray pixel at (x+5, y+4). Color `[160, 160, 160, 80]`. Visible when `frame % 8 < 4`.
- **State 4 (Safe)**: Two eye pixels at (x+4, y+6) and (x+6, y+6). Color `[200, 200, 200, 255]`.

### Text/Braille Rendering

- **Icon**: `m`
- **Color**: GRAY `[160, 160, 160]`

### Animations

- **Leg scurry**: `sin(frame * speed) * 2.0`. Speed: Fleeing=0.35, SeekingCheese=0.2.
- **Tail wag**: `sin(frame * (0.2 + leg_speed)) * 2.0`. Faster when running.
- **Eaten poof**: Pixel blinks on 8-frame cycle (4 on, 4 off).

### AI Behavior

- **Idle**: Sits still until stimulus detected.
- **Flee trigger**: Pokey the Cat detected in proximity. Flee speed: 70 px/s (slower than cat's 80 px/s chase).
- **Cheese seeking**: Cheese detected within 150px. Seek speed: 60 px/s. Runs directly toward cheese.
- **Cheese arrival**: Both Mort and cheese sit still when Mort reaches it. Cheese is NOT consumed.
- **Mouse Wheel activation**: Activates Mouse Exercise Wheel within 30px proximity.
- **Eaten**: Consumed by Edison Alligator jaw snap. Triggers alligator laugh animation.
- **Safe**: Enters Mouse Hole -- disappears, only eyes visible peeking out.
- **Tiny**: Small enough to fit through tight gaps.
- **Priority**: Flee from cat overrides cheese-seeking behavior.

### Interactions

- Flees from Pokey the Cat (cat is faster).
- Seeks Cheese within 150px range.
- Activates Mouse Exercise Wheel by proximity (~30px).
- Eaten by Edison Alligator (triggers laugh).
- Enters Mouse Hole (becomes safe).
- Can be trapped by Laundry Basket.

---

## Cheese

### Overview
- **Size**: 12x10 px
- **Type**: Dynamic
- **Mass**: 0.1
- **Elasticity**: 0.2
- **Density**: 0.8
- **Friction**: 0.5
- **Gravity**: Normal (falls under normal+, rises under weak)
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
| 0 | Idle | Default state | Static cheese wedge |

### Pixel Rendering

- **Wedge**: Filled triangle (x+6, y) -> (x, y+10) -> (x+12, y+10). Color `[240, 210, 60, 255]` (bright yellow).
- **Hole 1**: Filled circle at (x+5, y+6), radius 1.5px, color `[200, 170, 40, 255]` (darker yellow hole).
- **Hole 2**: Filled circle at (x+8, y+8), radius 1.0px, color `[200, 170, 40, 255]`.

### Text/Braille Rendering

- **Icon**: `\u25B2` (BLACK UP-POINTING TRIANGLE)
- **Color**: YELLOW `[230, 200, 50]`

### Animations

None.

### Interactions

- Attracts Mort the Mouse within 150px radius.
- Mort runs toward cheese at 60 px/s.
- NOT consumed when Mort reaches it -- both sit still.
- Feels gravity (dynamic object).

---

## Edison Alligator

### Overview
- **Size**: 64x32 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.5
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No
- **Flippable**: Yes
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (tail bounce, jaw snap, laugh)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Idle | Tail bouncing @ ~2Hz; jaw ready | Continuous tail oscillation, slight jaw gap |
| 1 | Snapping | Jaw closing on target (0.3s animation) | Wide jaw open (6px gap), then closing |
| 2 | Laughing | Laugh animation after eating (1.5s) | Belly shaking, open mouth, "ha ha" speech bubbles |

### Pixel Rendering

- **Body**: Filled rect at (x+8, y+12), 48x14 px, color `[50, 160, 50, 255]` (green).
- **Head (upper jaw)**: Filled rect at (x, y+10), 12x8 px, color `[60, 180, 60, 255]`.
- **Head (lower jaw)**: Filled rect at (x, y+18+jaw_open), 12x4 px, color `[50, 150, 50, 255]`.
  - `jaw_open`: Idle=2, Snapping=6.
- **Teeth**: Every 3px along jaws (0..12 step 3). Upper teeth at y+17, lower at y+18+jaw_open. Color `[255, 255, 255, 255]`.
- **Eye**: Filled circle at (x+8, y+10), radius 2px, color `[255, 255, 50, 255]` (yellow). Pupil pixel at (x+8, y+10), color `[30, 30, 30, 255]`.
- **Tail**: Line from (x+56, y+16) to (x+64, y+12+tail_y). Color `[40, 140, 40, 255]`.
  - `tail_y = sin(frame * 0.12) * 4.0` (continuous ~2Hz bounce).
- **Legs**: 4 legs at x-offsets [16, 28, 40, 50]. Each is a vertical line from y+26 to y+30. Color `[40, 130, 40, 255]`.
- **Scales**: Pixels every 6px along back (12..52 step 6) at y+12. Color `[40, 140, 40, 255]`.
- **State 2 (Laughing)**:
  - Belly shake: Rect at (x+20+shake, y+14), 20x2 px, color `[70, 200, 70, 200]`. `shake = sin(frame * 0.4) * 2.0`.
  - Speech bubbles: Two white pixels at (x-4, y+6) and (x-2, y+4). Blink: `frame % 10 < 5`.

### Text/Braille Rendering

- **Icon**: `A`
- **Color**: GREEN `[50, 180, 50]`

### Animations

- **Tail bounce**: Continuous. `sin(frame * 0.12) * 4.0`. Frequency: ~2Hz (0.12 rad/frame at 60fps).
- **Jaw snap** (State 1): Jaw gap widens to 6px, then closes over 0.3s.
- **Laugh shake** (State 2): Belly offset `sin(frame * 0.4) * 2.0`. Duration: 1.5s. Speech bubbles blink on 10-frame cycle.

### AI Behavior

- **Stationary**: Does not move. Fixed position.
- **Tail bounce**: Continuous during simulation at ~2Hz. Objects dropped on tail bounce at ~500 px/s.
- **Jaw snap**: Triggers when mouse or Mel passes within jaw range. Snap speed: ~600 px/s. Snaps shut, flinging/eating target.
- **Eating**: Eats Mort the Mouse and Mel Schlemming. Does NOT eat Pokey the Cat.
- **Laugh**: Plays 1.5s laugh animation after successfully eating a target. Belly shakes, "ha ha" bubbles appear.

### Interactions

- Tail bounces objects at ~500 px/s upward. Tail oscillates at ~2Hz continuously.
- Jaw snaps at ~600 px/s, eating Mort the Mouse and Mel Schlemming.
- Does NOT eat Pokey the Cat.
- Signature laugh animation (1.5s) after eating.
- Static/immovable -- cannot be pushed or moved.

---

## Mel Schlemming

### Overview
- **Size**: 16x24 px
- **Type**: Dynamic
- **Mass**: 0.5
- **Elasticity**: 0.1
- **Density**: 1.0
- **Friction**: 0.8
- **Gravity**: Normal
- **Destructible**: No (but can die)
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (walking/running cycle)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| speed | 0 | 2 | 1 | 1 | Speed (0=stop/1=walk/2=run) |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Walking | Moving at 50 px/s | Normal stride animation |
| 1 | Running | Moving at 100 px/s | Faster stride animation |
| 2 | Stationary | Standing still | No leg animation |
| 3 | EnteringHouse | Walking into Mel's House | Fading out / partially visible |
| 4 | Dead | Fatal fall/impact/eaten | Fallen figure with X eyes |

### Pixel Rendering

- **States 0, 1, 2 (Active)**:
  - Head: Filled circle at (x+8, y+5), radius 4px, color `[220, 180, 150, 255]` (skin tone).
  - Body: Filled rect at (x+4, y+9), 8x10 px, color `[60, 100, 200, 255]` (blue clothing).
  - Legs: Two lines from (x+6, y+19) and (x+10, y+19) to feet positions. Color `[60, 60, 80, 255]`.
    - `step = sin(frame * speed * 0.1) * 3.0` when moving. Front: (x+4+step, y+24). Back: (x+12-step, y+24).
  - Arms: Lines from (x+4, y+11) to (x+2, y+16) and (x+12, y+11) to (x+14, y+16). Color `[220, 180, 150, 255]`.
  - Speed multiplier: Walking=1.0, Running=2.0, Stationary=0.0.
- **State 3 (EnteringHouse)**: Fading figure. `visible = max(8 - (frame % 8), 0)`. Alpha decreases as Mel walks in. Head: alpha `visible*30`. Body: width shrinks to `min(visible, 8)`.
- **State 4 (Dead)**: Fallen horizontal figure. Head circle at (x+8, y+20), radius 4px, color `[180, 150, 120, 200]`. Body rect at (x+2, y+16), 12x4 px, blue clothing faded. X-eyes at (x+6, y+19) and (x+10, y+19), color `[60, 30, 30, 255]`.

### Text/Braille Rendering

- **Icon**: `M`
- **Color**: BLUE `[50, 100, 220]`

### Animations

- **Walk cycle**: `sin(frame * 1.0 * 0.1) * 3.0`. Alternating leg positions.
- **Run cycle**: `sin(frame * 2.0 * 0.1) * 3.0`. Same pattern, double speed.
- **Entering house**: Fade-out over 8 frames. `visible = max(8 - frame%8, 0)`.

### AI Behavior

- **Walking**: Walks in a set direction at 50 px/s until hitting a wall (turns around), reaching Mel's House (enters), or dying.
- **Running**: Same as walking but at 100 px/s.
- **Stationary**: Stands still (speed=0 setting).
- **Wall collision**: Turns around and walks the other direction.
- **House arrival**: Enters Mel's House. House transitions to Occupied state.
- **Death conditions**:
  - Fall greater than 150px vertical drop.
  - Impact speed greater than 200 px/s against a wall.
  - Eaten by Edison Alligator.
- **Multiple Mels**: Multiple instances can exist simultaneously on the same puzzle.

### Interactions

- Enters Mel's House on contact (house lights up, chimney smoke).
- Eaten by Edison Alligator (triggers alligator laugh).
- NOT eaten by Pokey the Cat.
- Can be shot by Gun (hitscan bullet).
- Programmable initial speed: stop/walk/run.

---

## Mel's House

### Overview
- **Size**: 48x48 px
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
- **Animated**: No (but has smoke when occupied)

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|----------|-----|-----|------|---------|-------|
| style | 0 | 1 | 1 | 0 | Style (0=suburban/1=cabin) |

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Empty | No occupant | Dark windows, no smoke |
| 1 | Occupied | Mel inside -- lights on, chimney smoke | Glowing yellow windows, chimney smoke puffs, Mel silhouette |

### Pixel Rendering

**Style 0 (Suburban)**:
- Walls: Filled rect at (x+4, y+20), 40x28 px, color `[200, 180, 160, 255]` (beige).
- Roof: Filled triangle (x+24, y+8) -> (x+2, y+20) -> (x+46, y+20), color `[160, 60, 40, 255]` (red-brown).
- Door: Filled rect at (x+18, y+32), 10x16 px, color `[120, 80, 40, 255]` (wood).
- Windows: Two rects at (x+8, y+24) 8x6 and (x+32, y+24) 8x6. Color: Empty=`[180, 220, 240, 255]` (sky blue), Occupied=`[255, 240, 150, 255]` (warm yellow).

**Style 1 (Cabin)**:
- Walls: Filled rect at (x+4, y+16), 40x32 px, color `[120, 70, 30, 255]` (dark wood).
- Log lines: Horizontal lines every 6px from y+16 to y+48. Color `[100, 55, 20, 255]`.
- Roof: Filled triangle (x+24, y+4) -> (x+2, y+16) -> (x+46, y+16), color `[100, 55, 20, 255]`.
- Door: Filled rect at (x+18, y+30), 10x18 px, color `[80, 45, 15, 255]`.

**State 1 (Occupied) additions**:
- Chimney: Filled rect at (x+36, chimney_top), 4x8 px, color `[140, 100, 60, 255]`. Top: suburban=y+8, cabin=y+4.
- Smoke puffs: 3 circles rising from chimney. Each: `sy = chimney_top - 4 - i*5 - (frame/4 % 4)`, `sx = chimney_x + 2 + sin(frame*0.1+i)*3`. Radius `2.5 + i*0.5`. Alpha `120 - i*30`. Color `[180, 180, 190]`.
- Mel silhouette: 3 dark pixels at window position (x+10, y+25). Color `[60, 40, 30, 180]`.

### Text/Braille Rendering

- **Icon**: `\u2302` (HOUSE)
- **Color**: BROWN `[139, 90, 43]`

### Animations

- **Chimney smoke** (State 1): 3 puffs rising. Y drift: `-(frame/4 % 4)`. X drift: `sin(frame*0.1 + i) * 3`. Radius grows with distance. Alpha fades.

### Interactions

- Mel enters when walking past (transitions to Occupied state).
- Window color changes to warm yellow when occupied.
- Chimney begins producing smoke when occupied.
- Mel visible as silhouette through window.
- Two visual styles: suburban (0) or log cabin (1).

---

## Bill's Fish Tank

### Overview
- **Size**: 20x24 px
- **Type**: Static
- **Mass**: Infinity (immovable)
- **Elasticity**: 0.2
- **Density**: 100.0
- **Friction**: 0.5
- **Gravity**: Normal
- **Destructible**: No (but breakable by impact)
- **Flippable**: No
- **Resizable**: No
- **Rope Point**: No
- **Ramp**: No
- **Power**: No
- **Animated**: Yes (fish swimming when intact)

### Editable Properties

None.

### States

| # | State | Description | Visual Changes |
|---|-------|-------------|----------------|
| 0 | Intact | Glass bowl with live fish | Round bowl, water, swimming fish |
| 1 | Broken | Shattered -- fish dies after 3s; attracts cat | Glass shards, water puddle, dead fish |

### Pixel Rendering

- **State 0 (Intact)**:
  - Bowl outer: Filled circle at (x+10, y+12), radius 9px, color `[180, 220, 240, 120]` (translucent glass).
  - Water: Filled circle at (x+10, y+14), radius 7px, color `[100, 180, 220, 100]` (blue water).
  - Fish: Filled circle at (x+8+sin(frame*0.05)*3, y+14), radius 2px, color `[255, 140, 40, 255]` (goldfish orange). Tail: two pixels at (fish_x-3, y+13) and (fish_x-3, y+15), same color.
- **State 1 (Broken)**:
  - Water puddle: Filled circle at (x+10, y+20), radius 8px, color `[80, 150, 200, 80]`.
  - Glass shards: 5 short lines at various positions. Color `[200, 220, 240, 180]`.
  - Dead fish: Filled circle at (x+10, y+18), radius 2px, color `[200, 100, 30, 200]`. Tail pixel at (x+13, y+17), same color. Fish upside down.

### Text/Braille Rendering

- **Icon**: `\u25CB` (WHITE CIRCLE)
- **Color**: CYAN `[0, 200, 220]`

### Animations

- **Fish swimming** (State 0): `fish_x = x + 8 + sin(frame * 0.05) * 3.0`. Slow horizontal oscillation inside bowl.

### Interactions

- **Break condition**: Impact at >100 px/s breaks the bowl.
- **Fish death**: Fish dies (stops flapping) 3 seconds after bowl breaks.
- **Cat attraction**: Broken bowl attracts Pokey the Cat (Pokey walks toward it).
- **Break trigger**: Any moving object hitting the bowl above 100 px/s threshold.

---

## Mouse Hole

### Overview
- **Size**: 16x16 px
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
| 0 | Idle | Default state | Dark arch opening |

### Pixel Rendering

- **Hole interior**: Semi-circular filled region. For each pixel (dx, dy) in 16x16: if `sqrt((dx-8)^2 + (dy-16)^2) < 8` and `dy < 16`, fill with `[30, 30, 35, 255]` (near-black).
- **Arch border**: 18-point semi-circular arc at center (x+8, y+16), radius 8px. Only upper half (0 to PI). Color `[80, 60, 40, 255]` (dark wood trim).

### Text/Braille Rendering

- **Icon**: `\u25E0` (UPPER HALF CIRCLE)
- **Color**: DARK_GRAY `[80, 80, 80]`

### Animations

None.

### Interactions

- Mort the Mouse enters the hole and becomes safe (State 4 on Mort).
- Acts as a destination/goal for mouse-routing puzzles.
- Static, immovable structure.

---

## Leprechaun

### Overview
- **Size**: 24x32 px
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
| 0 | Idle | Default state | Static leprechaun figure |

### Pixel Rendering

- **Body**: Filled rect at (x+6, y+12), 12x14 px, color `[40, 160, 40, 255]` (green coat).
- **Head**: Filled circle at (x+12, y+8), radius 5px, color `[220, 180, 150, 255]` (skin tone).
- **Hat top**: Filled rect at (x+6, y), 12x4 px, color `[30, 120, 30, 255]` (dark green).
- **Hat brim**: Filled rect at (x+4, y+4), 16x2 px, color `[30, 120, 30, 255]`.
- **Legs**: Two lines. Left: (x+9, y+26) to (x+7, y+32). Right: (x+15, y+26) to (x+17, y+32). Color `[30, 120, 30, 255]`.

### Text/Braille Rendering

- **Icon**: `\u2663` (BLACK CLUB SUIT)
- **Color**: GREEN `[50, 180, 50]`

### Animations

None.

### Interactions

- Decorative/interactive character.
- Replaced Irish Shamrock from earlier game versions.
- No physics interactions.
