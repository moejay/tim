# Parts Implementation Checklist

Status legend:
- [x] Implemented and matches spec
- [~] Partially implemented (noted)
- [ ] Not yet implemented

## Summary

| Category | Parts | Physics | States | State Rendering | Properties | Animations | Interactions |
|---|---|---|---|---|---|---|---|
| Balls | 10/10 | 10/10 | 10/10 | 10/10 | 10/10 | 7/10 | 0/10 |
| Walls | 15/15 | 15/15 | 15/15 | 15/15 | 15/15 | 4/4 | 0/15 |
| Inclines | 3/3 | 3/3 | 3/3 | 3/3 | 3/3 | 0/0 | 0/3 |
| Mechanical | 20/20 | 20/20 | 20/20 | 18/20 | 17/20 | 13/14 | 0/20 |
| Electrical | 10/10 | 10/10 | 10/10 | 10/10 | 10/10 | 8/10 | 0/10 |
| Pyrotechnic | 12/12 | 12/12 | 12/12 | 12/12 | 12/12 | 10/12 | 0/12 |
| Animals | 9/9 | 9/9 | 9/9 | 9/9 | 9/9 | 5/5 | 0/9 |
| Gadgets | 12/12 | 12/12 | 12/12 | 12/12 | 12/12 | 6/7 | 0/12 |
| Pipes | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 1/1 | 0/4 |
| Lasers | 7/7 | 7/7 | 7/7 | 7/7 | 7/7 | 4/4 | 0/7 |
| Ropes | 2/2 | 2/2 | 2/2 | 2/2 | 2/2 | 1/1 | 0/2 |
| **TOTAL** | **104/104** | **104/104** | **104/104** | **102/104** | **101/104** | **59/68** | **0/104** |

---

## Balls (10 parts)

### Bowling Ball
- [x] Physics: mass=7.0, elasticity=0.1, density=8.0, friction=0.4, gravity=AlwaysFalls, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims colors to half, Moving draws motion trail
- [x] Pixel rendering: dark sphere [60,60,70]->[30,30,35], 3 finger holes [20,20,25,200], drop shadow, specular [255,255,255,120]
- [x] Properties: none (correct per spec)
- [x] Animation: motion trail with 4 fading afterimages in Moving state
- [~] Animation: AtRest idle phase `(frame*0.05).sin()` computed but unused (matches spec note "reserved for subtle bob")
- [ ] Interaction: rolls on surfaces, destroys on impact (no physics integration yet)
- [x] Text icon: U+2B24, color DARK_GRAY
- [x] Size: 32x32 (radius 16.0)

### Cannonball
- [x] Physics: mass=6.0, elasticity=0.0, density=10.0, friction=0.3, gravity=AlwaysFalls, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: plain dark iron sphere [40,40,45]->[20,20,22], no type-specific detail (correct per spec)
- [x] Properties: none (correct per spec)
- [x] Animation: motion trail with 4 fading afterimages
- [ ] Interaction: fired from Cannon, zero bounce, densest ball
- [x] Text icon: U+25CF, color DARK_GRAY
- [x] Size: 24x24 (radius 12.0)

### Basketball
- [x] Physics: mass=0.6, elasticity=0.75, density=0.5, friction=0.6, gravity=Normal, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: orange sphere [240,150,40]->[180,90,20], horizontal+vertical seam lines [60,30,10,180]
- [x] Properties: none (correct per spec)
- [x] Animation: motion trail with 4 fading afterimages
- [ ] Interaction: Normal gravity, good bounce
- [x] Text icon: U+25CF, color ORANGE
- [x] Size: 28x28 (radius 14.0)

### Soccer Ball
- [x] Physics: mass=0.45, elasticity=0.6, density=0.4, friction=0.5, gravity=Normal, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: white sphere [250,250,250]->[200,200,200], 5 pentagon dots [40,40,40,160] at r*0.5
- [x] Properties: none (correct per spec)
- [x] Animation: motion trail with 4 fading afterimages
- [ ] Interaction: Normal gravity, lowest density ball
- [x] Text icon: U+25CF, color WHITE
- [x] Size: 26x26 (radius 13.0)

### Baseball
- [x] Physics: mass=0.15, elasticity=0.3, density=1.5, friction=0.4, gravity=Normal, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: white sphere [250,245,240]->[220,210,200], 8 red stitch pixels [200,30,30,200] along curved arc
- [x] Properties: none (correct per spec)
- [x] Animation: motion trail with 4 fading afterimages
- [ ] Interaction: small ball fits through gaps
- [x] Text icon: U+25CF, color WHITE
- [x] Size: 12x12 (radius 6.0)

### Tennis Ball
- [x] Physics: mass=0.06, elasticity=0.8, density=0.3, friction=0.7, gravity=Normal, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: yellow-green sphere [200,230,60]->[140,180,30], white AA seam line
- [x] Properties: none (correct per spec)
- [x] Animation: motion trail with 4 fading afterimages
- [ ] Interaction: lightest ball, high bounce, high friction
- [x] Text icon: U+25CF, color YELLOW_GREEN
- [x] Size: 10x10 (radius 5.0)

### Super Ball
- [x] Physics: mass=0.1, elasticity=1.1, density=0.8, friction=0.5, gravity=Normal, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: magenta sphere [220,60,220]->[140,20,140], glow effect r*1.3 [255,100,255]
- [x] Properties: none (correct per spec)
- [x] Animation: motion trail + static glow every frame
- [ ] Interaction: elasticity>1.0 gains energy on bounce
- [x] Text icon: U+25CF, color MAGENTA
- [x] Size: 14x14 (radius 7.0)

### Pinball
- [x] Physics: mass=0.08, elasticity=0.15, density=8.0, friction=0.2, gravity=AlwaysFalls, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: silver sphere [210,210,220]->[150,150,160], no type-specific detail (correct per spec)
- [x] Properties: none (correct per spec)
- [~] Animation: motion trail implemented; no has_animation flag set (spec does not flag as animated)
- [ ] Interaction: AlwaysFalls, very low bounce, slides easily
- [x] Text icon: U+25CF, color SILVER
- [x] Size: 10x10 (radius 5.0)

### Pool Ball
- [x] Physics: mass=0.17, elasticity=0.9, density=2.0, friction=0.3, gravity=ZeroGravity, is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: red sphere [200,40,40]->[120,20,20], white inner circle r*0.45 [255,255,255,220], number text
- [x] Properties: surface_number min=0 max=15 step=1 default=8 label="Number"
- [~] Animation: motion trail implemented; no has_animation flag set
- [ ] Interaction: ZeroGravity float until struck
- [x] Text icon: U+25CF, color RED
- [x] Size: 12x12 (radius 6.0)

### Programmable Ball
- [x] Physics: mass=1.0, elasticity=0.8, density=1.0, friction=0.5, gravity=Custom(1.0), is_static=false
- [x] States: Idle, Moving, AtRest (3/3)
- [x] State rendering: AtRest dims, Moving draws trail
- [x] Pixel rendering: teal sphere [0,190,190]->[0,120,120], 6 hexagonal dots [0,100,100,180] at r*0.5
- [x] Properties: mass (0.01-10.0/0.1/1.0), elasticity (0.0-2.0/0.05/0.8), density (0.1-10.0/0.1/1.0), friction (0.0-2.0/0.05/0.5), gravity_factor (0.0-2.0/0.1/1.0)
- [~] Animation: motion trail implemented; no has_animation flag set
- [ ] Interaction: fully customizable physics via 5 sliders
- [x] Text icon: U+25CF, color TEAL
- [x] Size: 16x16 (radius 8.0)

---

## Walls (15 parts)

### Brick Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact, Destroyed (2/2)
- [x] State rendering: Intact=brick pattern, Destroyed=rubble+dust cloud
- [x] Pixel rendering: base [178,80,50,255], mortar lines [-40,-30,-30], brick_h=8, brick_w=16, running bond pattern
- [x] Destroyed rendering: 12 debris chunks + animated dust cloud with oscillating alpha
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: Yes
- [x] Resizable: Yes
- [x] Animation: dust cloud alpha `(80 + sin(frame*0.1)*30)` in Destroyed state
- [ ] Interaction: chain-reaction with nearby dynamite
- [x] Text icon: U+2588, color RED_BROWN

### Yellow Brick Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact, Destroyed (2/2)
- [x] State rendering: Intact=yellow brick pattern, Destroyed=rubble+dust
- [x] Pixel rendering: base YELLOW [230,200,50,255], same running bond as Brick Wall
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: Yes
- [x] Resizable: Yes
- [x] Animation: same dust cloud oscillation as Brick Wall
- [ ] Interaction: same as Brick Wall
- [x] Text icon: U+2588, color YELLOW

### Cinder Block Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact, Destroyed (2/2)
- [x] State rendering: Intact=cinder block pattern, Destroyed=rubble+dust
- [x] Pixel rendering: base GRAY [160,160,160,255], mortar [120,120,120,255], block_h=16, block_w=32
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: Yes
- [x] Resizable: Yes
- [x] Animation: same dust cloud oscillation
- [ ] Interaction: destructible heavy variant
- [x] Text icon: U+2588, color GRAY

### Greco-Roman Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: generic wall style (base fill + highlight + shadow)
- [x] Pixel rendering: CREAM [240,230,200,255], top highlight, bottom shadow
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: No
- [x] Resizable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible classical marble wall
- [x] Text icon: U+2588, color CREAM

### Wooden Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Intact, Destroyed (2/2)
- [x] State rendering: Intact=wood grain, Destroyed=rubble+dust
- [x] Pixel rendering: vertical gradient [160,110,60]->[120,70,30], grain lines every 4px [BROWN-20,-15,-10, alpha 80]
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: Yes
- [x] Resizable: Yes
- [x] Animation: dust cloud in Destroyed state
- [ ] Interaction: higher friction (0.5), destructible
- [x] Text icon: U+2588, color BROWN

### Log Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: log cross-section pattern
- [x] Pixel rendering: DARK_BROWN [100,60,30,255], log cross-sections every 16px, outer ring r=6 [80,50,25,180], inner ring r=3 [60,35,15,200]
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: No
- [x] Resizable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible, higher friction
- [x] Text icon: U+2588, color DARK_BROWN

### Caution Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.9, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: yellow/black diagonal stripes
- [x] Pixel rendering: yellow base [230,200,50,255], diagonal black stripes [30,30,30,200], stripe_w=12
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: No
- [x] Resizable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: highest friction wall (0.9), indestructible
- [x] Text icon: U+2588, color YELLOW

### Sand Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: generic wall style (base + highlight + shadow)
- [x] Pixel rendering: TAN [210,180,140,255], top highlight, bottom shadow
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: No
- [x] Resizable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible, sandy texture
- [x] Text icon: U+2588, color TAN

### Pipe Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.2, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: generic wall style
- [x] Pixel rendering: STEEL [140,150,160,255], top highlight, bottom shadow
- [x] Properties: width (16-256/16/64), height (16-256/16/32)
- [x] Destructible by dynamite: No
- [x] Resizable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible, low friction (0.2)
- [x] Text icon: U+2588, color STEEL

### Curved Pipe Wall
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.2, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: quarter-circle arc from corner
- [x] Pixel rendering: 90-degree arc, 4-pixel thickness, corner position by rotation (0-3), color STEEL
- [x] Properties: rotation (0-3/1/0)
- [x] Destructible by dynamite: No
- [x] Resizable: No
- [x] Animation: None (correct per spec)
- [ ] Interaction: 4 rotations, low friction (0.2)
- [x] Text icon: U+256E, color STEEL

### Grass Floor
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: grass top with dirt below
- [x] Pixel rendering: dirt [120,80,40,255] at y+h/3, grass [50,160,50,255] at top, blade tips every 3px [30,180,30,200]
- [x] Properties: none (correct per spec -- not resizable)
- [x] Resizable: No
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible natural ground, fixed size
- [x] Text icon: U+2594, color GREEN
- [x] Size: 64x16

### Scaffold Barrier
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: generic wall style (GRAY + highlight + shadow)
- [x] Pixel rendering: GRAY [160,160,160,255], top [200,200,200], bottom [120,120,120]
- [x] Properties: none (correct per spec -- not resizable)
- [x] Resizable: No
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible, fixed-size scaffold
- [x] Text icon: U+256C, color GRAY
- [x] Size: 32x48

### Wooden Barrier
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: generic wall style (BROWN + highlight + shadow)
- [x] Pixel rendering: BROWN [139,90,43,255], top [179,130,83], bottom [99,50,3]
- [x] Properties: none (correct per spec)
- [x] Resizable: No
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible, small fixed-size
- [x] Text icon: U+2573, color BROWN
- [x] Size: 24x24

### Lattice Archway
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: generic wall style (WHITE + highlight + shadow)
- [x] Pixel rendering: WHITE [240,240,240,255], top [255,255,255], bottom [200,200,200]
- [x] Properties: none (correct per spec)
- [x] Resizable: No
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible decorative arch
- [x] Text icon: U+256C, color WHITE
- [x] Size: 48x64

### Marble Archway
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: generic wall style (CREAM + highlight + shadow)
- [x] Pixel rendering: CREAM [240,230,200,255], top [255,255,240], bottom [200,190,160]
- [x] Properties: none (correct per spec)
- [x] Resizable: No
- [x] Animation: None (correct per spec)
- [ ] Interaction: indestructible classical arch
- [x] Text icon: U+03A0, color CREAM
- [x] Size: 48x64

---

## Inclines (3 parts)

### Brick Incline
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: filled triangle with gradient, white AA hypotenuse
- [x] Pixel rendering: gradient RED_BROWN [178,80,50] to darker [-30,-30,-30], flipped triangle orientation
- [x] Properties: size (0-4/1/2) -- 5 stretch levels with correct dimensions
- [x] Flippable: Yes (mirrors slope)
- [x] can_be_ramp: not set in code (spec says Yes)
- [x] Animation: None (correct per spec)
- [ ] Interaction: primary ramp surface, indestructible
- [x] Text icon: U+25E2 (normal) / U+25E3 (flipped), color RED_BROWN
- [x] Size levels: VShort(30x28), Short(60x32), Medium(100x36), Long(150x40), VLong(200x44)

### Yellow Brick Incline
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: filled triangle with gradient, white AA hypotenuse
- [x] Pixel rendering: gradient YELLOW [230,200,50] to darker [-30,-30,-30]
- [x] Properties: size (0-4/1/2) -- same 5 levels
- [x] Flippable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: identical to Brick Incline
- [x] Text icon: U+25E2/U+25E3, color YELLOW

### Granite Incline
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Intact (1/1)
- [x] State rendering: filled triangle with gradient, white AA hypotenuse
- [x] Pixel rendering: gradient GRAY [160,160,160] to darker [-30,-30,-30]
- [x] Properties: size (0-4/1/2) -- same 5 levels
- [x] Flippable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: lower friction (0.3), smoothest ramp
- [x] Text icon: U+25E2/U+25E3, color GRAY

---

## Mechanical (20 parts)

### Gear
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle, Spinning (2/2)
- [x] State rendering: Idle=fixed angle (0), Spinning=teeth rotate at frame*0.08
- [x] Pixel rendering: SILVER body, darker inner ring (r*0.6), 8 teeth (5x5px), axle hub r=3 [80,80,90]
- [x] Properties: radius (16.0-32.0/2.0/20.0)
- [x] Animation: Spinning teeth angle=frame*0.08
- [ ] Interaction: pops balloons, drives belts, interlocking
- [x] Text icon: U+2699, color SILVER

### Pulley
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [~] State rendering: fallback rectangle (spec says GRAY rect + highlight/shadow -- code uses generic fallback)
- [x] Pixel rendering: GRAY rectangle with highlight/shadow
- [x] Properties: none (correct per spec)
- [x] has_rope_point: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: frictionless rope redirection, max 8 per rope
- [x] Text icon: U+25CE, color GRAY
- [x] Size: 24x24

### Belt
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [~] State rendering: fallback rectangle (matches spec for basic belt)
- [x] Pixel rendering: GRAY rectangle with highlight/shadow
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: connects rotating parts, max 200px stretch
- [x] Text icon: U+2500, color GRAY
- [x] Size: 64x8

### Teeter-Totter
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Level, TiltedLeft, TiltedRight (3/3)
- [x] State rendering: board tilts -6/0/+6 px based on state
- [x] Pixel rendering: pivot triangle at bottom center, 2px thick board line with tilt offset, BROWN
- [x] Properties: length (60.0-120.0/10.0/80.0)
- [x] has_rope_point: Yes
- [x] can_be_ramp: Yes
- [x] Animation: tilt based on state transition
- [ ] Interaction: seesaw catapult, rope attachment at ends
- [x] Text icon: U+2550, color BROWN

### Conveyor Belt
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=1.0, gravity=Normal, is_static=true
- [x] States: Idle, Running (2/2)
- [x] State rendering: Idle=static ribs + single faded arrow, Running=animated ribs + scrolling arrows + spinning wheel spokes
- [x] Pixel rendering: belt surface GRAY, ribs every 8px, arrows, end wheels r=5 [100,100,110], wheel spokes when running
- [x] Properties: length (64.0-256.0/64.0/64.0)
- [x] Animation: rib offset=(frame*2)%8, arrow offset=(frame*2)%16, wheel angle=frame*0.2
- [ ] Interaction: must be belt-driven, high friction grips objects
- [x] Text icon: U+25AC, color GRAY

### Trampoline
- [x] Physics: mass=INF, elasticity=1.2, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Idle, Compressed, Releasing (3/3)
- [x] State rendering: fabric position varies (y+h/2-4, +2, -8), color varies (blue shades), energy release lines in Releasing
- [x] Pixel rendering: legs, springs with compress offset, fabric surface 2px thick, energy release lines (3 horizontal lines)
- [x] Properties: none (correct per spec)
- [x] Animation: state-driven animation (Idle->Compressed->Releasing->Idle)
- [ ] Interaction: bounces objects, elasticity>1.0 gains height, max 1500 px/s
- [x] Text icon: U+255A, color BLUE

### Jack-in-the-Box
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Closed, WindingUp, Open (3/3)
- [x] State rendering: Closed=flat lid, WindingUp=vibrating lid+crank, Open=flipped lid+spring+figure head+eyes
- [x] Pixel rendering: magenta box [180,50,180], lid [200,80,200], vibration sin(frame*0.4)*2, crank angle frame*0.15, figure head [255,220,100] r=4, eyes [40,40,40]
- [x] Properties: none (correct per spec)
- [x] Animation: lid vibration, crank rotation (WindingUp)
- [ ] Interaction: belt-driven, catapult ~500 px/s when Open
- [x] Text icon: U+2554, color MAGENTA

### Windmill
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle, Spinning (2/2)
- [x] State rendering: Idle=blades at 0.4 rad, Spinning=blades at frame*0.08
- [x] Pixel rendering: 4 blades from center, blade_len=w/2-4, BROWN, blade width +3px perpendicular, hub r=4 [80,80,90]
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Animation: blade angle=frame*0.08 when Spinning
- [ ] Interaction: spins from air sources, drives belts
- [x] Text icon: U+2731, color BROWN

### Mandrill Motor
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: ShadeClosed, Pedaling, Stunned (3/3)
- [x] State rendering: ShadeClosed=shade covers monkey, Pedaling=animated legs+shade open, Stunned=stars+shade closed
- [x] Pixel rendering: base frame, bicycle wheels r=6, V-frame, monkey head r=6 + torso, pedaling legs frame*0.2, stunned stars frame*0.3 with TAU/3 spacing
- [x] Properties: none (correct per spec)
- [x] Animation: pedaling legs frame*0.2, stunned stars frame*0.3
- [ ] Interaction: shade rope-activated, bonk=2s stun, drives belts
- [x] Text icon: M, color BROWN

### Mouse Exercise Wheel
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle, Spinning (2/2)
- [x] State rendering: Idle=mouse at bottom-left, Spinning=spokes rotate+mouse orbits
- [x] Pixel rendering: wire wheel r=16 (semi-transparent), 8 spokes r=15, hub r=3, mouse r=3 (idle at cx-6,cy+8 / spinning orbiting at angle+1 r=10)
- [x] Properties: none (correct per spec)
- [x] Animation: spoke rotation frame*0.1, mouse orbit (Spinning)
- [ ] Interaction: activates on nearby impact, mouse direction=spin direction
- [x] Text icon: U+25CE, color GRAY

### Trans-Roto-Matic
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [~] State rendering: fallback rectangle (matches spec)
- [x] Pixel rendering: GRAY rectangle with highlight/shadow
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: converts translational to rotational motion
- [x] Text icon: U+229E, color GRAY
- [x] Size: 24x24

### Roto-Trans-Converter
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [~] State rendering: fallback rectangle (matches spec)
- [x] Pixel rendering: GRAY rectangle with highlight/shadow
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: converts rotational to translational motion
- [x] Text icon: U+229F, color GRAY
- [x] Size: 24x24

### Tipsy Trailer
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [~] State rendering: fallback rectangle (matches spec)
- [x] Pixel rendering: BROWN rectangle with highlight/shadow
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: tilting platform on wheels
- [x] Text icon: U+2550, color BROWN
- [x] Size: 48x24

### Scissors
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Open, Closed (2/2)
- [x] State rendering: Open=blades at 0.3 rad, Closed=blades at 0.0
- [x] Pixel rendering: pivot at (cx,cy), blades from pivot, SILVER [200,200,210], pivot circle r=2 [150,150,160], handles [180,60,60]
- [x] Properties: none (correct per spec)
- [x] Animation: None (state-driven, correct per spec)
- [ ] Interaction: cuts rope (not steel cable), pops balloons when closed
- [x] Text icon: U+2702, color SILVER

### Hedge Trimmers
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: blades continuously animate
- [x] Pixel rendering: pivot at (x+8,y+6), blades open=|sin(frame*0.15)*0.15|, GREEN [50,160,50], pivot r=2 [80,80,90], handle 8x6 [100,70,40]
- [x] Properties: none (correct per spec)
- [~] Animation: blade oscillation is rendered but has_animation returns false (spec says always active)
- [ ] Interaction: always active, cuts rope (not steel cable)
- [x] Text icon: U+2704, color GREEN

### Tin Snips
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: blades continuously animate
- [x] Pixel rendering: pivot at (x+8,y+6), blades open=|sin(frame*0.12)*0.2|, SILVER [200,200,210], pivot r=2 [150,150,160], handle 8x6 [180,180,190]
- [x] Properties: none (correct per spec)
- [~] Animation: blade oscillation rendered but has_animation returns false (spec says always active)
- [ ] Interaction: ONLY tool cutting both rope AND steel cable
- [x] Text icon: U+2704, color SILVER

### Boxing Glove
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Retracted, Punching (2/2)
- [x] State rendering: Retracted=glove at y+8 with 6 coils, Punching=extends 12px up with 3 coils + motion blur
- [x] Pixel rendering: spring zigzag coils, red glove gradient r=10 [230,50,50]->[180,30,30], motion blur lines [255,200,200] with decreasing alpha
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Animation: state-driven (not continuous frame animation)
- [ ] Interaction: triggered punch ~1200 px/s, sends objects flying
- [x] Text icon: B, color RED

### Vacuum Cleaner
- [x] Physics: mass=INF, elasticity=0.3, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=static body, On=suction lines+vibration
- [x] Pixel rendering: BLUE body [50,100,220,200], motor housing [60,90,180], hose [80,80,90], suction lines scrolling rightward, vibration sin(frame*0.5)*1.0
- [x] Properties: none (correct per spec)
- [x] Requires power: Yes
- [x] can_be_ramp: Yes
- [x] Animation: suction line scrolling, body vibration (On)
- [ ] Interaction: sucks objects in ~60px radius when powered
- [x] Text icon: V, color BLUE

### Pinball Bumper
- [x] Physics: mass=INF, elasticity=1.5, density=100.0, friction=0.1, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: glow blink pattern (frame%10<3 || state==0)
- [x] Pixel rendering: gradient circle [255,255,200]->[200,180,50] r=12, glow ring r=16 [255,255,100]
- [x] Properties: none (correct per spec)
- [x] can_be_ramp: Yes
- [x] Animation: glow blink 3-frame-on / 7-frame-off cycle
- [ ] Interaction: bounces objects ~800 px/s, elasticity 1.5
- [x] Text icon: U+25C9, color YELLOW

### Tack
- [x] Physics: mass=INF, elasticity=0.0, density=100.0, friction=0.8, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: static rendering
- [x] Pixel rendering: triangle point (x+4,y)->(x+1,y+6)->(x+7,y+6) [200,200,210,255], base rect 6x2 [180,180,190,255]
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: pops balloons, high friction walking surface
- [x] Text icon: U+25B4, color SILVER
- [x] Size: 8x8

---

## Electrical (10 parts)

### Generator
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Unpowered, Generating (2/2)
- [x] State rendering: Unpowered=dim wheel, Generating=spinning spokes+bright wheel+glow+sparks
- [x] Pixel rendering: housing [80,80,90], spinning spokes angle=frame*0.15, bright wheel YELLOW, glow r=14 [255,255,100], sparks every 2 frames at frame*0.7, axle hub r=4
- [x] Properties: none (correct per spec)
- [x] Provides power: Yes
- [x] Animation: spoke rotation frame*0.15, spark orbit frame*0.7 (Generating)
- [ ] Interaction: belt-driven, powers outlet with 2 sockets
- [x] Text icon: U+26A1, color YELLOW

### Electrical Outlet
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: On (1/1)
- [x] State rendering: always-on with green LED
- [x] Pixel rendering: faceplate [230,225,220], top border [200,200,200], socket holes 3x4 [40,40,40], green LED [100,255,100,200]
- [x] Properties: none (correct per spec)
- [x] Provides power: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: always-on, 2 sockets
- [x] Text icon: U+25AA, color WHITE

### Switch Outlet
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=switch up+no LED, On=switch down+green LED
- [x] Pixel rendering: faceplate [230,225,220], socket holes at h/2, switch housing [180,180,180], toggle [220,220,220], green LED when On
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Provides power: Yes
- [x] Animation: None (state-driven, correct per spec)
- [ ] Interaction: toggled by falling object, 2 sockets when On
- [x] Text icon: U+25AA, color WHITE

### Solar Panel
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Dark, Powered (2/2)
- [x] State rendering: Dark=dim blue panel, Powered=bright blue+sun reflection glow
- [x] Pixel rendering: panel body varying brightness, grid lines v/8px h/6px [60,100,200,180], top+bottom frames [120,120,130], sun reflection glow r=8 [255,255,200]
- [x] Properties: none (correct per spec)
- [x] Provides power: Yes
- [x] Animation: None (state-driven, correct per spec)
- [ ] Interaction: powered by light source ~80px LOS
- [x] Text icon: U+25A6, color BLUE

### Laser-Activated Plug (Electrical)
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Inactive, Active (2/2)
- [x] State rendering: Inactive=dim, Active=bright+spinning indicator
- [x] Pixel rendering: fallback with brightness toggle, spinning indicator at frame*0.15 r=4 [255,255,200,200]
- [x] Properties: activation_color (0.0-2.0/1.0/0.0) label="Color (0=R/1=G/2=B)"
- [x] Provides power: Yes
- [x] Animation: spinning indicator in Active state
- [ ] Interaction: color-programmable, powers 2 sockets when matching laser hits
- [x] Text icon: U+25C9, color RED

### Electric Fan
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=static blades at half brightness, On=spinning blades+airflow lines
- [x] Pixel rendering: 5 blades from center r=10, Off=dim BLUE/2, On=full BLUE angle=frame*0.3, airflow lines 3 sets with directional flip, hub r=3 [80,80,90]
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Requires power: Yes
- [x] can_be_ramp: Yes
- [x] Animation: blade rotation frame*0.3, airflow line scrolling (On)
- [ ] Interaction: directional airflow ~120px, blows out candles, spins windmills
- [x] Text icon: U+274A, color BLUE

### Electric Motor
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=no axle animation, On=spinning axle+vibration
- [x] Pixel rendering: GRAY body, motor circle r=8 [100,100,110], axle line angle=frame*0.2 length 7, vibration stripe sin(frame*0.5)*1.0, hub r=3
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Requires power: Yes
- [x] can_be_ramp: Yes
- [x] Animation: axle rotation frame*0.2, body vibration (On)
- [ ] Interaction: continuous rotation, drives belts
- [x] Text icon: U+2299, color GRAY

### Toaster
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, Heating, Popped (3/3)
- [x] State rendering: Off=dark slots, Heating=orange glow slots, Popped=dark slots+flying toast
- [x] Pixel rendering: body gradient [210,210,220]->[170,170,180], lever 3x8 [150,150,160], slots 6x3, glow [255,120,40], toast 4x5 [220,190,120] flying upward
- [x] Properties: delay (0.0-2.0/1.0/1.0) label="Delay (0=light/1=med/2=burnt)"
- [x] Requires power: Yes
- [x] Animation: toast launch Y offset=-(frame*2)%16 (Popped)
- [ ] Interaction: launches toast ~600 px/s after delay
- [x] Text icon: U+25AC, color SILVER

### Can Opener
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=dim (brightness 150), On=full brightness+spinning indicator
- [x] Pixel rendering: SILVER rect with brightness toggle, spinning indicator at frame*0.15
- [x] Properties: none (correct per spec)
- [x] Requires power: Yes
- [x] Animation: spinning indicator (On)
- [ ] Interaction: opens cans when powered
- [x] Text icon: U+22A1, color SILVER

### Electric Mixer
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=dim, On=full brightness+spinning indicator
- [x] Pixel rendering: SILVER rect with brightness toggle, spinning indicator at frame*0.15
- [x] Properties: none (correct per spec)
- [x] Requires power: Yes
- [x] Animation: spinning indicator (On)
- [ ] Interaction: rotating beaters, drives belts
- [x] Text icon: U+229B, color SILVER

---

## Pyrotechnic (12 parts)

### Magnifying Glass
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: static magnifying glass
- [x] Pixel rendering: lens circle r=7 [200,220,240,120], 36-point rim [140,140,150,255], handle 2 lines [120,80,40,255]
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: focuses light into ignition beam
- [x] Text icon: U+25CE, color WHITE

### Flashlight
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=dark lens [200,200,180], On=bright lens [255,255,220]+beam+glow
- [x] Pixel rendering: body gradient [200,200,210]->[160,160,170], lens rect, beam 16px with spreading width and fading alpha, lens glow r=6 [255,255,180]
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Animation: beam projection with fading alpha (On)
- [ ] Interaction: object-activated, ~200px beam, powers Solar Panels
- [x] Text icon: U+25BA, color YELLOW

### Lava Lamp
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Off, On (2/2)
- [x] State rendering: Off=dark glass [50,25,70,150], On=bright purple [100,50,150,200]+blobs+glow
- [x] Pixel rendering: base [60,60,70], cap [100,100,110], blob 1 r=3 [220,110,220,220] bob=sin(frame*0.05)*6, blob 2 r=2 [240,130,240,200] bob=sin(frame*0.03+2)*4, glow r=8 [180,80,180]
- [x] Properties: none (correct per spec)
- [x] Animation: blob 1 oscillation, blob 2 oscillation (On)
- [ ] Interaction: drawstring-activated, omnidirectional ~100px light
- [x] Text icon: U+2565, color PURPLE

### Candle
- [x] Physics: mass=0.05, elasticity=0.1, density=0.8, friction=0.4, gravity=Normal, is_static=false
- [x] States: Unlit, Lit, Extinguished (3/3)
- [x] State rendering: Unlit=bare body, Lit=flame+glow, Extinguished=charred wick+smoke
- [x] Pixel rendering: wax 4x10 [240,230,200], wick [60,50,40], flame draw_flame(6.0), glow r=10 [255,200,80], charred wick [40,35,30], smoke wisp [140,140,150]
- [x] Properties: none (correct per spec)
- [x] Animation: flame flicker (Lit), smoke rise (Extinguished)
- [ ] Interaction: lit by magnifying glass/laser/flame, blown out by fan/bellows, ignites fuses
- [x] Text icon: U+2565, color YELLOW

### Dynamite
- [x] Physics: mass=0.3, elasticity=0.1, density=1.5, friction=0.5, gravity=Normal, is_static=false
- [x] States: Idle, FuseLit, Exploded (3/3)
- [x] State rendering: Idle/FuseLit=intact stick, Exploded=debris ring+glow
- [x] Pixel rendering: red stick 8x16 [210,40,30], label band 8x3 [240,220,180], fuse line [120,80,40], flame draw_flame(4.0), 8 debris particles, glow r=14 [255,200,50]
- [x] Properties: none (correct per spec)
- [x] Animation: fuse burning (FuseLit), explosion debris orbiting (Exploded)
- [ ] Interaction: ~60px destruction radius, chain-reaction, destroys brick/wood walls
- [x] Text icon: U+256B, color RED

### Dynamite Plunger
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Ready, Pressed (2/2)
- [x] State rendering: Ready=handle up, Pressed=handle lowered+spark
- [x] Pixel rendering: wooden box 16x14 [120,80,40], handle bar 12x3 [80,80,90], rod, spark glow r=5 [255,255,100]
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: instant detonation, does NOT chain-react
- [x] Text icon: U+2564, color RED

### Cannon
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Ready, FuseLit, Fired (3/3)
- [x] State rendering: Ready=fuse glow blink (frame%6<3), FuseLit=flame+glow, Fired=smoke cloud
- [x] Pixel rendering: barrel gradient 32x12, opening 4x10 [40,40,45], wheels r=5 [100,70,40], smoke 5 puffs with sinusoidal drift
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Animation: fuse blink (Ready), fuse flame (FuseLit), smoke dissipation (Fired)
- [ ] Interaction: fires cannonball ~1500 px/s, recoil
- [x] Text icon: U+2550, color GRAY

### Rocket
- [x] Physics: mass=0.2, elasticity=0.1, density=1.2, friction=0.3, gravity=Normal, is_static=false
- [x] States: Idle, FuseLit, Launched (3/3)
- [x] State rendering: Idle/FuseLit=grounded rocket, Launched=ascending with exhaust
- [x] Pixel rendering: red body 8x16 [200,40,40], nose cone triangle, left+right fins [180,30,30], nozzle 6x4 [80,80,90], exhaust 8 particles [255,180,50] with sinusoidal wobble
- [x] Properties: none (correct per spec)
- [x] Animation: fuse flame (FuseLit), ascent fly_y=y-min(frame*3,40) (Launched), exhaust trail
- [ ] Interaction: launches ~2000 px/s, trail ignites objects
- [x] Text icon: U+25B2, color RED

### Fireworks
- [x] Physics: mass=0.2, elasticity=0.1, density=1.2, friction=0.3, gravity=Normal, is_static=false
- [x] States: Idle, FuseLit, Launched, Exploded (4/4)
- [x] State rendering: Idle/FuseLit=grounded, Launched=ascending+spark trail, Exploded=12-point starburst
- [x] Pixel rendering: gray tube 4x16, colored nose cone, spark trail 5 particles, 12-point starburst with custom RGB, burst_r=min(frame*0.8,18)
- [x] Properties: color1 (0-255/25/255), color2 (0-255/25/100), color3 (0-255/25/200)
- [x] Animation: fuse flame (FuseLit), ascent (Launched), starburst expansion+rotation (Exploded)
- [ ] Interaction: cosmetic explosion only, programmable RGB colors
- [x] Text icon: U+25B2, color MAGENTA

### Remote Control Bomb
- [x] Physics: mass=0.4, elasticity=0.1, density=1.5, friction=0.5, gravity=Normal, is_static=false
- [x] States: Idle, FuseLit, Exploded (3/3)
- [x] State rendering: Idle=slow LED blink (20-frame), FuseLit=fast LED blink (6-frame), Exploded=debris+glow
- [x] Pixel rendering: dark red body 12x12 [180,40,40], antenna+tip, LED [255,50,50] with glow r=3, 8 debris particles, glow r=14
- [x] Properties: none (correct per spec)
- [x] Animation: LED blink rate varies by state (Idle/FuseLit), explosion debris (Exploded)
- [ ] Interaction: remote-triggered, same destruction as dynamite
- [x] Text icon: U+2731, color RED

### Match on Spring
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: static match head on spring
- [x] Pixel rendering: 5 spring coils with wobble sin(sy*1.5)*2.0, match head r=3 [200,50,30], match stick [180,150,100]
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: spring-loaded ignition source
- [x] Text icon: U+2191, color RED

### Fuse
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.4, gravity=Normal, is_static=true
- [x] States: Unburnt, Burning, Spent (3/3)
- [x] State rendering: Unburnt=full brown cord, Burning=charred behind spark+flame at burn front, Spent=all charred
- [x] Pixel rendering: wobble sin(fx*0.3)*1.0, unburnt [120,80,40,255], burnt [40,35,30,255], spark glow r=4 [255,220,80], flame at burn front
- [x] Properties: length (16-200/8/64) label="Length (px)"
- [x] Animation: burn progression burn_pos=(frame*1.0)%length, ~60px/s
- [ ] Interaction: burns from ignition end, transfers to connected parts
- [x] Text icon: U+2500, color BROWN

---

## Animals (9 parts)

### Pokey the Cat
- [x] Physics: mass=0.8, elasticity=0.1, density=1.0, friction=0.7, gravity=Normal, is_static=false
- [x] States: Idle, Walking, Chasing, Startled (4/4)
- [x] State rendering: all 4 states render differently (fur color, ear size, eye size, tail speed, leg speed, startle stars)
- [x] Pixel rendering: fur [230,160,60]/startled [255,200,80], body 16x10, head r=6, ears (puffed when startled), eyes (wide when chasing/startled 2x2), tail wave sin(frame*speed)*3, legs sin(frame*speed)*2, startle stars blink frame%6<3
- [x] Properties: none (correct per spec)
- [x] Animation: tail wag (speed varies 0.1/0.25/0.4), leg walk cycle (0.15/0.3), startle flash
- [ ] Interaction: chases mouse 200px LOS, attracted to broken fishbowl, NOT eaten by gator
- [x] Text icon: C, color ORANGE

### Mort the Mouse
- [x] Physics: mass=0.02, elasticity=0.1, density=0.5, friction=0.6, gravity=Normal, is_static=false
- [x] States: Idle, Fleeing, SeekingCheese, Eaten, Safe (5/5)
- [x] State rendering: Active=body+ear+eye+tail+legs, Eaten=fading pixel blink, Safe=eyes peeking
- [x] Pixel rendering: body r=4 [160,160,160], ear r=2 [180,150,150], eye [30,30,30], tail wave, legs animated, eaten pixel blink frame%8<4, safe eyes [200,200,200]
- [x] Properties: none (correct per spec)
- [x] Animation: leg scurry (0.35/0.2), tail wag, eaten poof blink
- [ ] Interaction: flees cat, seeks cheese, activates wheel, eaten by gator, enters mouse hole
- [x] Text icon: m, color GRAY

### Cheese
- [x] Physics: mass=0.1, elasticity=0.2, density=0.8, friction=0.5, gravity=Normal, is_static=false
- [x] States: Idle (1/1)
- [x] State rendering: static cheese wedge
- [x] Pixel rendering: yellow wedge triangle [240,210,60,255], hole 1 r=1.5 [200,170,40], hole 2 r=1.0
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: attracts Mort 150px, NOT consumed
- [x] Text icon: U+25B2, color YELLOW

### Edison Alligator
- [x] Physics: mass=INF, elasticity=0.5, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle, Snapping, Laughing (3/3)
- [x] State rendering: Idle=tail bouncing+jaw gap 2, Snapping=jaw gap 6, Laughing=belly shake+speech bubbles
- [x] Pixel rendering: green body 48x14, head (upper+lower jaw), teeth every 3px, eye r=2 [255,255,50]+pupil, tail sin(frame*0.12)*4, 4 legs, scales every 6px, laughing belly shake sin(frame*0.4)*2, speech bubble blink frame%10<5
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] Animation: tail bounce frame*0.12, jaw snap, laugh shake frame*0.4
- [ ] Interaction: tail bounces ~500 px/s, jaw snaps ~600 px/s, eats mouse/Mel, NOT cat
- [x] Text icon: A, color GREEN

### Mel Schlemming
- [x] Physics: mass=0.5, elasticity=0.1, density=1.0, friction=0.8, gravity=Normal, is_static=false
- [x] States: Walking, Running, Stationary, EnteringHouse, Dead (5/5)
- [x] State rendering: Walking/Running/Stationary=active figure, EnteringHouse=fading, Dead=fallen+X eyes
- [x] Pixel rendering: head r=4 [220,180,150], body 8x10 [60,100,200], legs animated step=sin(frame*speed*0.1)*3, arms, entering house fade, dead X-eyes [60,30,30]
- [x] Properties: speed (0-2/1/1) label="Speed (0=stop/1=walk/2=run)"
- [x] Animation: walk/run cycle, entering house fade-out
- [ ] Interaction: enters house, eaten by gator, dies from fall>150px or impact>200 px/s
- [x] Text icon: M, color BLUE

### Mel's House
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Empty, Occupied (2/2)
- [x] State rendering: Empty=dark windows, Occupied=warm windows+chimney smoke+Mel silhouette
- [x] Pixel rendering: Style 0 (suburban): beige walls, red-brown roof, wood door, windows change color. Style 1 (cabin): dark wood+log lines+dark roof+dark door. Occupied: chimney 4x8, 3 smoke puffs rising with sinusoidal drift, Mel silhouette 3 dark pixels
- [x] Properties: style (0-1/1/0) label="Style (0=suburban/1=cabin)"
- [x] Animation: chimney smoke puffs (Occupied)
- [ ] Interaction: Mel enters, window color changes, chimney smoke
- [x] Text icon: U+2302, color BROWN

### Bill's Fish Tank
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Intact, Broken (2/2)
- [x] State rendering: Intact=bowl+water+swimming fish, Broken=shards+puddle+dead fish
- [x] Pixel rendering: Intact: bowl r=9 [180,220,240,120], water r=7 [100,180,220,100], fish r=2 [255,140,40] swimming sin(frame*0.05)*3, tail pixels. Broken: puddle r=8 [80,150,200,80], 5 glass shards, dead fish upside down
- [x] Properties: none (correct per spec)
- [x] Animation: fish swimming oscillation (Intact)
- [ ] Interaction: breaks at >100 px/s impact, fish dies 3s after, attracts cat
- [x] Text icon: U+25CB, color CYAN

### Mouse Hole
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: dark arch opening
- [x] Pixel rendering: semi-circular filled region [30,30,35,255], 18-point arch border [80,60,40,255]
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: Mort enters and becomes safe
- [x] Text icon: U+25E0, color DARK_GRAY

### Leprechaun
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: static figure
- [x] Pixel rendering: green coat 12x14 [40,160,40], head r=5 [220,180,150], hat top [30,120,30], hat brim 16x2, legs [30,120,30]
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: decorative character, no physics interactions
- [x] Text icon: U+2663, color GREEN

---

## Gadgets (12 parts)

### Captain Z Super Phazer
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Ready, Firing, Empty (3/3)
- [x] State rendering: Ready/Firing=bright barrel, Empty=dim barrel, Firing=beam+glow
- [x] Pixel rendering: body gradient [0,180,200]->[0,120,140], barrel [0,220,240]/dim [0,100,110], grip [0,140,160], beam 20 pixel pairs fading alpha, barrel glow r=6 [0,255,255]
- [x] Properties: shots (1-5/1/3)
- [x] Flippable: Yes
- [x] has_rope_point: Yes
- [x] Animation: beam projection (Firing)
- [ ] Interaction: 1-5 blasts at ~1800 px/s
- [x] Text icon: U+25BA, color CYAN

### Egg Timer
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Ready, Counting, Triggered (3/3)
- [x] State rendering: Ready=sand at top, Counting=sand flowing+falling grain, Triggered=spring arm+all sand at bottom
- [x] Pixel rendering: upper+lower hourglass triangles [220,200,160,200], top+bottom bands [120,120,130], sand [200,180,120], falling grain pixel, spring arm + ball tip r=2
- [x] Properties: delay (1.0-10.0/0.5/3.0)
- [x] Animation: sand flow (frame*0.04)%1.0*8 (Counting)
- [ ] Interaction: programmable delay, spring arm ~400 px/s
- [x] Text icon: U+231B, color TAN

### Eye Hook
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [~] State rendering: fallback rectangle (spec says SILVER rect)
- [x] Pixel rendering: SILVER rect [200,200,210,200]
- [x] Properties: none (correct per spec)
- [x] has_rope_point: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: fixed rope anchor
- [x] Text icon: U+2310, color SILVER

### Boat Cleat
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [~] State rendering: fallback rectangle (spec says SILVER rect)
- [x] Pixel rendering: SILVER rect [200,200,210,200]
- [x] Properties: none (correct per spec)
- [x] has_rope_point: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: immovable rope anchor
- [x] Text icon: U+2229, color SILVER

### Gun (Revolver)
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Loaded, Fired (2/2)
- [x] State rendering: Loaded=normal, Fired=muzzle flash+smoke
- [x] Pixel rendering: body 20x8 [120,120,130], barrel 4x5, grip 6x8 [100,70,40], cylinder r=4 [110,110,120], trigger, muzzle flash glow r=6 [255,255,150], 3 smoke circles
- [x] Properties: none (correct per spec)
- [x] Flippable: Yes
- [x] has_rope_point: Yes
- [x] can_be_ramp: Yes
- [x] Animation: muzzle flash + smoke (Fired)
- [ ] Interaction: rope-triggered, hitscan bullet, single shot
- [x] Text icon: U+25BA, color GRAY

### Anti-Gravity Pad
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1, always active)
- [x] State rendering: pulsing glow with rising field lines
- [x] Pixel rendering: purple pad [100,50,150,255], glow=sin(frame*0.1)*0.3+0.7, field lines phase=(frame*2)%16 every 4px rising, surface glow [160,80,220]
- [x] Properties: none (correct per spec)
- [x] Animation: pulsing glow, rising field lines
- [ ] Interaction: reverses gravity ~16px zone above
- [x] Text icon: U+2261, color PURPLE

### Santa Lamp
- [x] Physics: mass=INF, elasticity=0.2, density=100.0, friction=0.5, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: static lamp with glow
- [x] Pixel rendering: red shade triangle [200,50,40], pole [80,80,90], base 8x4 [80,80,90], light glow r=4 [255,255,200]
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: decorative ambient light
- [x] Text icon: U+2666, color RED

### Laundry Basket
- [x] Physics: mass=1.2, elasticity=0.1, density=0.8, friction=0.5, gravity=Normal, is_static=false
- [x] States: Open, Trapping (2/2)
- [x] State rendering: Open=normal basket, Trapping=shaking+trapped animal blob
- [x] Pixel rendering: sides BROWN, horizontal bands every 6px, trapped animal blob r=4 [200,160,80,120] shake=sin(frame*0.3)*2, rattling sides blink frame%4<2
- [x] Properties: none (correct per spec)
- [x] has_rope_point: Yes
- [x] Animation: rattling (Trapping)
- [ ] Interaction: traps animals, bottomless, heavier than bucket
- [x] Text icon: U+2554, color BROWN

### Bucket
- [x] Physics: mass=0.5, elasticity=0.1, density=1.0, friction=0.5, gravity=Normal, is_static=false
- [x] States: Empty, Filled (2/2)
- [~] State rendering: Empty/Filled render identically (mass change is internal -- matches spec)
- [x] Pixel rendering: tapered shape (w*0.6+w*0.4*t), SILVER fill, semicircular handle 10-point arc
- [x] Properties: none (correct per spec)
- [x] has_rope_point: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: catches objects, mass increases, rope attachment
- [x] Text icon: U, color SILVER

### Leaky Bucket
- [x] Physics: mass=1.5, elasticity=0.1, density=1.5, friction=0.5, gravity=Normal, is_static=false
- [x] States: Full, Draining, Empty (3/3)
- [~] State rendering: drip renders in states 0+1 (code shows drip always for LeakyBucket variant); no state-based drip suppression for Empty
- [x] Pixel rendering: tapered GRAY body, handle arc, drip=(frame*0.15)%8 two pixels [100,150,220]
- [x] Properties: leak_rate (0-2/1/1) label="Leak (0=slow/1=med/2=fast)"
- [x] has_rope_point: Yes
- [x] Animation: drip cycle (Full/Draining)
- [ ] Interaction: mass 1.5->0.3, drives rope/pulley dynamics
- [x] Text icon: U, color GRAY

### Balloon
- [x] Physics: mass=0.01, elasticity=0.3, density=0.01, friction=0.1, gravity=Buoyant, is_static=false
- [x] States: Inflated, Popped (2/2)
- [x] State rendering: Inflated=bobbing balloon+string, Popped=fragments+dangling string
- [x] Pixel rendering: 4 design colors (red/blue/green/yellow), gradient circle r=9, specular r=2.5 [255,255,255,80], knot, 8-pixel string with wobble. Popped: 6 fragments expanding, dangling string
- [x] Properties: design (1-4/1/1)
- [x] Animation: bobbing sin(frame*0.06)*2, string wobble sin(sy*0.5)*1, pop fragments expanding
- [ ] Interaction: buoyant rise, popped by gear/scissors/tack/dynamite/candle/laser/gun
- [x] Text icon: U+25CB, color BLUE

### Hot Air Balloon
- [x] Physics: mass=0.5, elasticity=0.1, density=0.1, friction=0.1, gravity=Normal, is_static=false
- [x] States: Cold, Heating, Rising (3/3)
- [x] State rendering: Cold=no flame, Heating=flame in basket, Rising=ascending+flame
- [x] Pixel rendering: envelope gradient r=14 [220,60,40]->[180,40,30], 3 yellow stripes [240,220,60,150], ropes, basket 12x8 [140,100,50], flame draw_flame(5.0), rise_offset=-min(frame*0.5,20)
- [x] Properties: none (correct per spec)
- [x] Animation: rise offset (Rising), basket flame (Heating/Rising)
- [ ] Interaction: rises when heated, carries objects
- [x] Text icon: U+25EF, color RED

---

## Pipes (4 parts)

### Straight Pipe
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.2, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: metallic appearance
- [x] Pixel rendering: H-orientation: gradient [180,180,190]->[120,120,130] length x 16px, center highlight [210,210,220,120], left+right rims 2x18 [140,140,150]. V-orientation: same colors rotated
- [x] Properties: length (32-192/32/64), orientation (0-1/1/0)
- [x] can_be_ramp: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: objects travel through, exterior serves as ramp
- [x] Text icon: U+2550, color GRAY

### T-Connector
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.2, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: metallic T-junction
- [x] Pixel rendering: main body 24x16 GRAY, branch stub 8x8 depends on rotation (0=up,1=right,2=down,3=left), center highlight [200,200,210,100]
- [x] Properties: rotation (0-3/1/0)
- [x] can_be_ramp: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: 3-way junction, momentum+gravity determines exit
- [x] Text icon: U+2566, color GRAY

### Curved Pipe
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.2, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: metallic curved pipe
- [x] Pixel rendering: 90-degree arc, 8 thickness layers, center at (x+12,y+12), lighter inner edge (shade 180) + darker outer edge (shade 140), rotation offsets arc by 90 degrees
- [x] Properties: rotation (0-3/1/0)
- [x] can_be_ramp: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: 90-degree bend, redirects objects
- [x] Text icon: U+256E, color GRAY

### Accelerator Tube
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.2, gravity=Normal, is_static=true
- [x] States: Idle, Accelerating (2/2)
- [x] State rendering: Idle=dim cyan+static arrows, Accelerating=bright glowing+fast scrolling arrows+object streak
- [x] Pixel rendering: gradient body [0,base*0.8,base]->[0,base*0.5,base*0.6], inner glow pulsing alpha=sin(frame*0.2)*0.2+0.8, arrows scale with speed_mult, object streak r=3 [255,255,200,150]
- [x] Properties: speed_mult (1.5-3.0/0.5/2.0), direction (0-3/1/0)
- [x] can_be_ramp: Yes
- [x] Animation: inner glow pulse, arrow scrolling (speed scales with speed_mult), object streak
- [ ] Interaction: multiplies velocity by speed_mult in configured direction
- [x] Text icon: U+21D2, color CYAN

---

## Lasers (7 parts)

### Red Laser
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Off, Emitting (2/2)
- [x] State rendering: Off=dim lens, Emitting=bright lens+glow+beam
- [x] Pixel rendering: emitter body 16x12 [80,80,90], dim lens [85,13,13,200], bright lens [255,40,40], lens glow r=6, beam 40px with center+upper/lower fringe, alpha fading 220-bx*4, flicker sin(frame*0.3+bx*0.1)*20
- [x] Properties: rotation (0-3/1/0)
- [x] Animation: beam flicker (Emitting)
- [ ] Interaction: ignites fuses/candles, pops balloons, reflects off mirrors, activates red plugs
- [x] Text icon: U+2500, color RED

### Green Laser
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Off, Emitting (2/2)
- [x] State rendering: identical structure to Red Laser with green color [40,255,40]
- [x] Pixel rendering: same as Red Laser with green values
- [x] Properties: rotation (0-3/1/0)
- [x] Animation: beam flicker (Emitting)
- [ ] Interaction: activates green plugs
- [x] Text icon: U+2500, color GREEN

### Blue Laser
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Off, Emitting (2/2)
- [x] State rendering: identical structure to Red Laser with blue color [40,80,255]
- [x] Pixel rendering: same as Red Laser with blue values
- [x] Properties: rotation (0-3/1/0)
- [x] Animation: beam flicker (Emitting)
- [ ] Interaction: activates blue plugs
- [x] Text icon: U+2500, color BLUE

### Angled Mirror
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: static mirror surface
- [x] Pixel rendering: AA diagonal line [220,220,230] (rotation 0/2=backslash, 1/3=forward-slash), base 8x2 [120,120,130], center highlight [255,255,255,180]
- [x] Properties: rotation (0-3/1/0)
- [x] Animation: None (correct per spec)
- [ ] Interaction: redirects laser beams 90 degrees, preserves color
- [x] Text icon: U+2571, color WHITE

### Laser Mixer
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Idle (1/1)
- [x] State rendering: static prism with input dots
- [x] Pixel rendering: two triangles (diamond shape) [220,220,240,200], input dots R/G/B on left side, center glow r=6 [255,255,255]
- [x] Properties: none (correct per spec)
- [x] Animation: None (correct per spec)
- [ ] Interaction: additive color mixing, output beam exits right side
- [x] Text icon: U+25C7, color WHITE

### Laser Detector
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Idle, Triggered (2/2)
- [x] State rendering: Idle=red LED blink (frame%40<5), Triggered=green LED+glow
- [x] Pixel rendering: housing 12x12 [80,80,90], sensor well r=4 [40,40,50], red LED r=1.5 [200,50,50,180] blinking, green LED r=2 [100,255,100]+glow r=6
- [x] Properties: none (correct per spec)
- [x] Animation: red LED blink every 40 frames (Idle)
- [ ] Interaction: triggers on any laser beam color, does not discriminate
- [x] Text icon: U+25C9, color WHITE

### Laser-Activated Plug (Lasers)
- [x] Physics: mass=INF, elasticity=0.1, density=100.0, friction=0.3, gravity=Normal, is_static=true
- [x] States: Inactive, Active (2/2)
- [x] State rendering: Inactive=dim lens, Active=bright lens+glow+power indicator
- [x] Pixel rendering: outlet body 12x18 [220,215,210], socket holes 3x4 [40,40,40], dim lens r=3 (1/3 brightness), bright lens r=3 full color, glow r=5, power indicator [100,255,100,200]
- [x] Properties: activation_color (0-2/1/0) label="Color (0=R/1=G/2=B)"
- [x] Provides power: Yes
- [x] Animation: None (correct per spec)
- [ ] Interaction: color-specific activation, provides power when correct laser hits
- [x] Text icon: U+25C9, color WHITE

---

## Ropes (2 parts)

### Rope
- [x] Physics: mass=0.01, elasticity=0.0, density=0.5, friction=0.3, gravity=Normal, is_static=false
- [x] States: Slack, Taut, Cut (3/3)
- [x] State rendering: Slack=deep catenary sag 0.15, Taut=nearly straight 0.02+vibration, Cut=two drooping halves+frayed ends
- [x] Pixel rendering: BROWN [139,90,43], shadow pixel [119,70,23,200] (2px thick), slack sag=sin(t*PI)*length*0.15, taut vib=sin(frame*0.5+i*0.3)*0.5, cut halves with quadratic sag t^2*20, frayed ends [139,90,43,120]
- [x] Properties: length (16-600/16/64)
- [x] Animation: taut vibration (state 1)
- [ ] Interaction: tension-only force transmission, cut by scissors/hedge trimmers/tin snips, max 8 pulleys
- [x] Text icon: U+2502, color BROWN

### Steel Cable
- [x] Physics: mass=0.01, elasticity=0.0, density=0.5, friction=0.3, gravity=Normal, is_static=false
- [x] States: Slack, Taut, Cut (3/3)
- [x] State rendering: Slack=stiffer sag 0.05, Taut=nearly straight 0.02+vibration, Cut=two drooping halves+frayed ends
- [x] Pixel rendering: SILVER [200,200,210], single pixel wide (no shadow pixel), slack sag=sin(t*PI)*length*0.05, taut vib=sin(frame*0.5+i*0.3)*0.5, cut halves dimmed [100,100,105,200]
- [x] Properties: length (16-600/16/64)
- [x] Animation: taut vibration (state 1)
- [ ] Interaction: ONLY cut by tin snips, immune to scissors/hedge trimmers, max 8 pulleys
- [x] Text icon: U+2502, color SILVER

---

## Detailed Summary Counts

| Category | Total Parts | Physics Match | States Match | State Rendering | Properties Match | Animations Done | Interactions Done |
|---|---|---|---|---|---|---|---|
| Balls | 10 | 10 [x] | 10 [x] | 10 [x] | 10 [x] | 7 [x] 3 [~] | 0 [x] 10 [ ] |
| Walls | 15 | 15 [x] | 15 [x] | 15 [x] | 15 [x] | 4 [x] 0 [~] | 0 [x] 15 [ ] |
| Inclines | 3 | 3 [x] | 3 [x] | 3 [x] | 3 [x] | N/A (none needed) | 0 [x] 3 [ ] |
| Mechanical | 20 | 20 [x] | 20 [x] | 16 [x] 4 [~] | 17 [x] 3 [~] | 11 [x] 2 [~] | 0 [x] 20 [ ] |
| Electrical | 10 | 10 [x] | 10 [x] | 10 [x] | 10 [x] | 8 [x] 0 [~] | 0 [x] 10 [ ] |
| Pyrotechnic | 12 | 12 [x] | 12 [x] | 12 [x] | 12 [x] | 10 [x] 0 [~] | 0 [x] 12 [ ] |
| Animals | 9 | 9 [x] | 9 [x] | 9 [x] | 9 [x] | 5 [x] 0 [~] | 0 [x] 9 [ ] |
| Gadgets | 12 | 12 [x] | 12 [x] | 10 [x] 2 [~] | 12 [x] | 6 [x] 1 [~] | 0 [x] 12 [ ] |
| Pipes | 4 | 4 [x] | 4 [x] | 4 [x] | 4 [x] | 1 [x] 0 [~] | 0 [x] 4 [ ] |
| Lasers | 7 | 7 [x] | 7 [x] | 7 [x] | 7 [x] | 4 [x] 0 [~] | 0 [x] 7 [ ] |
| Ropes | 2 | 2 [x] | 2 [x] | 2 [x] | 2 [x] | 1 [x] 0 [~] | 0 [x] 2 [ ] |

### Overall Totals

| Dimension | Done [x] | Partial [~] | Missing [ ] | Total |
|---|---|---|---|---|
| Physics values | 104 | 0 | 0 | 104 |
| State definitions | 104 | 0 | 0 | 104 |
| State-aware rendering | 98 | 6 | 0 | 104 |
| Properties | 98 | 3 | 3 | 104 |
| Animations | 59 | 6 | 3 | 68 |
| Interactions/AI | 0 | 0 | 104 | 104 |

### Key Gaps

1. **Interactions/AI**: Zero parts have runtime interaction logic implemented. All 104 parts need physics collision, trigger, and behavior code.
2. **Partial animations**: Hedge Trimmers and Tin Snips render blade oscillation but `has_animation` returns false. Balls (Pinball, Pool Ball, Programmable Ball) render motion trails but lack `has_animation` flag.
3. **Fallback renderers**: Pulley, Belt, Trans-Roto-Matic, Roto-Trans-Converter, Tipsy Trailer, Eye Hook, and Boat Cleat use the generic rectangle fallback renderer. These match the spec (which describes fallback rendering for these parts), but could benefit from more detailed visuals.
4. **Leaky Bucket**: Drip animation renders regardless of state (should stop in Empty state).
5. **Inclines**: `can_be_ramp` trait method is not overridden (defaults to false), but spec says inclines serve as ramps. This matters for physics integration.
