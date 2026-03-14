# Ropes

All rope parts belong to the `RopeType` enum and the "Ropes" category.
Ropes are **dynamic** parts with very low mass that connect two attachment points
and transmit force when taut. They are the primary mechanism for linking distant
parts in a Rube Goldberg chain.

---

## Rope

### Overview
- **Size**: 64x32 px (default bounding box; actual shape is a curved line)
- **Type**: Dynamic
- **Mass**: 0.01
- **Elasticity**: 0.0
- **Density**: 0.5
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: false
- **has_animation**: false (vibration effect is state-driven, not flagged)
- **is_resizable**: false (length controlled via property)
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `length` | 16.0 | 600.0 | 16.0 | 64.0 | Length (max 600px) |

**Maximum rope length**: 600 pixels. This is a hard limit. Ropes cannot be extended beyond this.

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Slack | Not under tension -- no force transmitted | Deep catenary sag; no vibration |
| 1 | Taut | Under tension -- transmitting force | Nearly straight line with subtle vibration |
| 2 | Cut | Severed -- connected parts lose tension | Two halves droop downward with frayed ends at cut point |

### Pixel Rendering

**Rope color**: BROWN `[139, 90, 43]`.

**State 0 (Slack):**
- Rope rendered as a catenary curve across its full length.
- `sag_factor = 0.15` (significant droop).
- For each pixel `i` in `0..length`:
  - `t = i / length` (normalized position 0.0 to 1.0).
  - `px = x + t * length` (horizontal position).
  - `sag = sin(t * PI) * length * 0.15` (parabolic sag, maximum at rope center).
  - `py = y + sag` (vertical position, sagging downward).
  - Main pixel: `[139, 90, 43]` at full alpha.
  - Shadow pixel (1px below): `[119, 70, 23]` at alpha 200. Slightly darker, creating a 2px-thick rope appearance.

**State 1 (Taut):**
- Nearly straight line with subtle vibration.
- `sag_factor = 0.02` (very slight droop -- rope is almost but not perfectly straight).
- For each pixel `i` in `0..length`:
  - `t = i / length`.
  - `px = x + t * length`.
  - `sag = sin(t * PI) * length * 0.02`.
  - `vib = sin(frame * 0.5 + i * 0.3) * 0.5` (subtle vibration: 0.5px amplitude, propagating along rope at rate 0.3 per pixel, oscillating at rate 0.5 per frame).
  - `py = y + sag + vib`.
  - Main pixel: `[139, 90, 43]` at full alpha.
  - Shadow pixel: `[119, 70, 23]` at alpha 200.

**State 2 (Cut):**
- Two dangling halves, each drooping under gravity from their respective attachment points.
- `half = length / 2.0`.
- `cut_color = [139/2, 90/2, 43/2]` = `[69, 45, 21]` (dimmed to indicate severed rope).
- **Left half** (attached to left endpoint, drooping rightward/downward):
  - For each pixel `i` in `0..half`:
    - `t = i / half`.
    - `px = x + i`.
    - `sag = t^2 * 20` (quadratic droop, increasing toward the cut end).
    - Pixel: `[69, 45, 21]` at alpha 200.
- **Right half** (attached to right endpoint, drooping leftward/downward):
  - For each pixel `i` in `0..half`:
    - `t = i / half`.
    - `px = x + half + i`.
    - `sag = (1 - t)^2 * 20` (quadratic droop, decreasing from the cut end).
    - Pixel: `[69, 45, 21]` at alpha 200.
- **Frayed ends** at the cut point (`x + half`):
  - Three pixels of fray, vertically at `(x+half-1, y+2+dy)` and `(x+half+1, y+2+dy)` for `dy` in `0..3`.
  - Color: `[139, 90, 43]` at alpha 120. Original rope color at reduced opacity, simulating splayed fibers.

### Text/Braille Rendering
- **Icon character**: `\u{2502}` (BOX DRAWINGS LIGHT VERTICAL)
- **Icon color**: BROWN `[139, 90, 43]`

### Animations

| Animation | Trigger | Formula | Description |
|---|---|---|---|
| Taut vibration | State 1 (Taut) | `vib = sin(frame * 0.5 + i * 0.3) * 0.5` | Subtle standing-wave vibration along rope length when under tension |

### Interactions

**Attachment points**: Rope can attach to any part that has `has_rope_point` = true. Per the game spec, valid attachment points include:
- Teeter-totter (either end)
- Boat, Boat cleat
- Laundry basket, Bucket, Leaky bucket
- Captain Z Super Phazer
- Balloon, Hot air balloon
- Lava lamp (drawstring)
- Mandrill motor (shade)
- Tipsy trailer
- Match on spring
- Pulley
- Trans-roto-matic, Roto-trans-converter
- Remote control bomb
- Gun (trigger)
- Dynamite plunger
- Jack-in-the-box (shade)
- Eye hook

**Force transmission**: When taut (state 1), the rope transmits pulling force between its two endpoints. It does NOT transmit pushing force -- rope is tension-only. If the distance between endpoints is less than the rope length, the rope goes slack (state 0) and no force is transmitted.

**Cutting rules**:
- **Scissors**: Cut rope when the scissors close (triggered by an object pressing on the handle or sharp end). The rope must be positioned within the scissors' cutting zone.
- **Hedge trimmers**: Cut rope on contact.
- **Tin snips**: Cut rope on contact.
- All three cutting tools transition the rope to state 2 (Cut) immediately upon contact.
- Cutting is irreversible -- a cut rope cannot be reconnected during simulation.

**Pulley threading**:
- Rope can be threaded through pulleys to change direction of force transmission.
- **Maximum pulleys per rope**: 8. A single rope can pass through up to 8 pulleys.
- Each pulley changes the rope's direction, allowing force to be redirected around corners and obstacles.
- The total rope length (including all segments between pulleys) must not exceed 600px.
- Friction at each pulley slightly reduces the transmitted force.

**Gravity**: Rope responds to gravity normally (GravityResponse::Normal). When slack, the rope sags downward under its own weight. The sag amount depends on the rope length and the sag_factor.

**Weight**: At mass 0.01, the rope's own weight is negligible. It does not significantly affect the force balance between connected parts.

---

## Steel Cable

### Overview
- **Size**: 64x32 px (default bounding box; actual shape is a curved line)
- **Type**: Dynamic
- **Mass**: 0.01
- **Elasticity**: 0.0
- **Density**: 0.5
- **Friction**: 0.3
- **Gravity Response**: Normal
- **is_static**: false
- **has_animation**: false
- **is_resizable**: false (length controlled via property)
- **is_flippable**: false
- **destructible_by_dynamite**: false
- **can_be_ramp**: false
- **has_rope_point**: false
- **requires_power**: false
- **provides_power**: false

### Editable Properties

| Property | Min | Max | Step | Default | Label |
|---|---|---|---|---|---|
| `length` | 16.0 | 600.0 | 16.0 | 64.0 | Length (max 600px) |

**Maximum cable length**: 600 pixels (same limit as rope).

### States

| # | State | Description | Visual Changes |
|---|---|---|---|
| 0 | Slack | Not under tension -- no force transmitted | Slight catenary sag; no vibration |
| 1 | Taut | Under tension -- transmitting force | Nearly straight line with subtle vibration |
| 2 | Cut | Severed -- connected parts lose tension | Two halves droop downward with frayed ends at cut point |

### Pixel Rendering

**Cable color**: SILVER `[200, 200, 210]`.

**State 0 (Slack):**
- Rendered as a catenary curve, but with less sag than rope due to cable rigidity.
- `sag_factor = 0.05` (compared to rope's 0.15 -- steel cable is stiffer).
- For each pixel `i` in `0..length`:
  - `t = i / length`.
  - `px = x + t * length`.
  - `sag = sin(t * PI) * length * 0.05`.
  - `py = y + sag`.
  - Single pixel only: `[200, 200, 210]` at full alpha. No shadow pixel (cable is thinner/sleeker than rope -- only 1px wide).

**State 1 (Taut):**
- Nearly straight line with subtle vibration (identical formula to rope).
- `sag_factor = 0.02`.
- For each pixel `i` in `0..length`:
  - `t = i / length`.
  - `px = x + t * length`.
  - `sag = sin(t * PI) * length * 0.02`.
  - `vib = sin(frame * 0.5 + i * 0.3) * 0.5`.
  - `py = y + sag + vib`.
  - Single pixel: `[200, 200, 210]` at full alpha. No shadow pixel.

**State 2 (Cut):**
- Identical structure to rope's cut state but using cable colors.
- `cut_color = [200/2, 200/2, 210/2]` = `[100, 100, 105]`.
- Left half and right half droop with same quadratic sag formulas as rope.
- Frayed ends at cut point: `[200, 200, 210]` at alpha 120.

### Text/Braille Rendering
- **Icon character**: `\u{2502}` (BOX DRAWINGS LIGHT VERTICAL)
- **Icon color**: SILVER `[200, 200, 210]`

### Animations

| Animation | Trigger | Formula | Description |
|---|---|---|---|
| Taut vibration | State 1 (Taut) | `vib = sin(frame * 0.5 + i * 0.3) * 0.5` | Subtle standing-wave vibration along cable length when under tension |

### Interactions

**Attachment points**: Same as Rope -- can attach to any part with `has_rope_point` = true. All the same attachment points are valid.

**Force transmission**: Identical to Rope. Tension-only; no pushing force. Goes slack when endpoint distance is less than cable length.

**Cutting rules -- KEY DIFFERENCE from Rope**:
- **Scissors**: CANNOT cut steel cable. Scissors have no effect on steel cable.
- **Hedge trimmers**: CANNOT cut steel cable. Hedge trimmers have no effect on steel cable.
- **Tin snips**: CAN cut steel cable. Tin snips are the ONLY tool that cuts steel cable.
- This is the primary gameplay distinction between Rope and Steel Cable. Steel cable is used in puzzles where the player must specifically use tin snips, or where the cable must be immune to scissors and hedge trimmers that might be present in the machine.

**Pulley threading**: Same as Rope. Maximum 8 pulleys per cable. Total length including all segments must not exceed 600px.

**Visual distinction from Rope**:
- Steel cable is silver (`[200,200,210]`) vs. rope's brown (`[139,90,43]`).
- Steel cable renders as a single pixel wide (no shadow pixel), vs. rope's 2-pixel thickness.
- Steel cable has less slack sag (0.05 vs 0.15), reflecting its greater rigidity.

---

## Rope and Cable System Rules

### General Mechanics

1. **Tension model**: Ropes and cables are inextensible -- they do not stretch. When the distance between two attachment points equals or exceeds the rope/cable length, the connection becomes taut (state 1) and force is transmitted. When the distance is less than the length, the connection is slack (state 0) and no force is transmitted.

2. **Force direction**: Force is always transmitted along the line connecting the two endpoints (or the rope segment between pulleys). The force is pure tension (pulling) -- ropes and cables cannot push.

3. **Mass transfer**: When a heavy object (e.g., bowling ball in a bucket) is attached to one end and a lighter object (e.g., balloon) is attached to the other, the heavier object pulls the lighter object toward it via the rope. The net force is `F = (m1 - m2) * g` where gravity contributes to the downward force of the heavier object.

4. **Rope as constraint**: A taut rope constrains the maximum distance between its two endpoints to the rope length. Objects can move freely within the rope's slack but cannot separate beyond its length.

### Pulley System

5. **Pulley mechanics**: A pulley changes the direction of force transmission without (significantly) changing its magnitude. A rope threaded through a pulley at the top of the screen can convert a downward pull on one side into an upward pull on the other side.

6. **Maximum pulleys**: 8 pulleys per rope or cable. This limits the complexity of routing but allows for elaborate multi-corner setups.

7. **Pulley friction**: Each pulley introduces a small friction loss. The transmitted force is slightly reduced at each pulley, though for gameplay purposes this reduction is minor.

8. **Segment lengths**: The total length of all rope segments (endpoint-to-pulley, pulley-to-pulley, pulley-to-endpoint) must sum to the configured rope length. The rope redistributes between segments as objects move, but the total length remains constant.

### Cutting Mechanics

9. **Cut location**: When a cutting tool contacts the rope/cable, the cut occurs at the point of contact. The rope splits into two independent segments at that point.

10. **Post-cut behavior**: After cutting:
    - Both halves become independent and droop under gravity (state 2).
    - Any tension that was being transmitted is immediately lost.
    - Objects that were being held up by the rope (e.g., a bucket hanging from a rope through a pulley) will fall.
    - Objects that were being pulled by the rope will stop being pulled.
    - The cut is permanent and irreversible during simulation.

11. **Cutting tool hierarchy**:
    | Tool | Cuts Rope | Cuts Steel Cable |
    |---|---|---|
    | Scissors | Yes | No |
    | Hedge Trimmers | Yes | No |
    | Tin Snips | Yes | Yes |

12. **Scissors activation**: Scissors must be closed to cut. They close when an object presses on the handle or sharp end. A rope passing through the scissors' cutting zone is cut when the scissors transition to the closed state.

13. **Hedge trimmers and tin snips**: These cut on contact -- they do not require an activation mechanism like scissors do.

### Common Rope/Cable Puzzle Patterns

14. **Counterweight**: Attach a heavy object to one end and a lighter target to the other, threaded through a ceiling pulley. The heavy object falls, pulling the lighter object upward.

15. **Delayed release**: Attach a leaky bucket to one end. As water drains, the bucket becomes lighter, eventually allowing the weight on the other end to pull it down.

16. **Lava lamp drawstring**: Attach rope to the lava lamp's drawstring, thread through a pulley, and attach a weight. When the weight drops, it pulls the drawstring, turning on the lava lamp.

17. **Gun trigger pull**: Attach rope to the gun trigger, thread through pulleys to redirect the pull direction, and attach to a falling object. The object's weight pulls the trigger, firing the gun.

18. **Mandrill shade**: Attach rope to the mandrill motor's shade. Thread through pulleys and attach to a weight. When the weight drops, the shade opens, revealing the banana, and the monkey starts pedaling.

19. **Cut-to-release**: A rope holds an object in place (e.g., a ball on a platform). Scissors or other cutting tool in the machine's chain reaction cut the rope, releasing the object to fall.
