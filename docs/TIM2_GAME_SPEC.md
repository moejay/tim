# The Incredible Machine 2 — Game Feature Specification

## 1. Overview

**Title:** The Incredible Machine 2
**Original Developer:** Jeff Tunnell Productions / Dynamix
**Original Publisher:** Sierra On-Line
**Original Release:** 1994 (DOS, Macintosh); 1995 (Windows as "Version 3.0")
**Genre:** Physics-based puzzle / Rube Goldberg simulation

The Incredible Machine 2 is a physics-based puzzle game where the player constructs elaborate Rube Goldberg-style contraptions from a toolbox of mechanical, electrical, and whimsical parts to achieve specific goals. The game combines creative engineering with real-time physics simulation. Players are given a partially-built machine with missing parts and must place the correct parts in the correct positions to trigger a chain reaction that achieves the stated puzzle goal.

---

## 2. Game Modes

### 2.1 Puzzle Play (Single-Player Campaign)
- 150+ puzzles across progressive difficulty tiers: **Tutorial, Easy, Medium, Hard, Really Hard (Expert)**.
- Each puzzle presents a partially-built machine and a stated goal (e.g. "Pop the balloon", "Get the ball into the basket").
- The player is given a limited set of parts to place on the playfield to complete the machine.
- Fixed parts are immovable; only the player's given parts can be placed and repositioned.
- Not all given parts need to be used — some puzzles include decoy/extra parts to increase difficulty.
- No time limit — the player experiments freely, running the machine as many times as needed.
- Player progress is tracked per sign-in name; solved puzzles are indicated by glowing light bulb icons.
- A "Show Solution" feature reveals Professor Tim's intended solution for any puzzle (lightbulb icon in control panel).
- After solving: options for Same Puzzle, Replay Solution, or Next Puzzle.
- Each puzzle has a password code that can be used to jump to that puzzle.

### 2.2 Professor Tim's Workshop (Level Editor / Freeform)
- Full sandbox mode for building machines from scratch using the entire parts catalog (up to 79 parts per puzzle).
- Build completely playable custom puzzles with defined win conditions.
- Ten-step puzzle building workflow:
  1. Access the Workshop.
  2. Place parts onto the playfield (minimum 5 parts required).
  3. Write a goal description in a text field.
  4. Lock all parts using the padlock button (these become the fixed/immovable parts).
  5. Unlock specific parts to become the player's inventory (appear in the Parts Bin during play).
  6. Select puzzle music from the track list.
  7. Adjust gravity and atmospheric pressure (optional).
  8. Insert hints (optional) — positioned pointing-finger tooltips with custom text.
  9. Program solution recognition using checkered flag handles on relevant parts.
  10. Save the puzzle (8-character filenames, directory navigation supported).
- Solution programming for non-moving parts: designate desired final state (e.g. "occupied" vs "vacant").
- Solution programming for moving parts: define exit location (Off Top, Off Bottom, Off Screen) or a specific position rectangle; can program physical state changes (ignited, launched, etc.).
- Scenery system: decorative non-functional elements (trees, clouds, buildings, creatures, landscapes, space objects) with front/back layering.
- Right-click to duplicate parts (unlimited copies in Workshop).
- Any previously solved puzzle can be loaded and modified.
- Parts Bin editor: remove all parts with bomb button, then selectively add parts for the player to use.

### 2.3 Head-to-Head (Two-Player Hotseat)
- Competitive mode where two players alternate turns on the same puzzle.
- 50 dedicated two-player puzzles, plus the ability to play on any single-player puzzle.
- Setup options:
  - Games per match: 1–9.
  - Time per turn: 10–90 seconds (individual timers per player).
  - Player name entry.
- Turn rules:
  - First player places one part from the bin.
  - Subsequent turns: place one part from the bin OR move one existing part (not both).
  - Parts cannot be returned to the bin once placed.
  - "End Turn" button or timer expiration ends a turn.
  - Green flag can be pressed mid-turn to test the machine.
  - First player to achieve the goal wins the round.
- Turn indicator: red background = Player 1, blue = Player 2.
- Scoreboard tracks wins per player across the match.
- Strategic depth: blocking opponent moves, creating deceptive setups, avoiding advantageous positioning for the opponent.

### 2.4 Guided Tour
- Automated tutorial that demonstrates game mechanics, part functions, and controls.
- Walk-through mode with step-by-step puzzle construction guided by Professor Tim.

---

## 3. Physics Engine

The game simulates a deterministic, real-time 2D physics environment. All simulations run without randomness, ensuring reproducible results.

### 3.1 Global Physics Parameters (Adjustable in Workshop and Freeform)

| Parameter | Effect |
|---|---|
| **Gravity** | Slider controlling fall speed and weight influence. Higher = faster falling. Can be set to "lunar" for reduced gravity. At weak gravity, light objects (basketballs, tennis balls, balloons) rise instead of fall. |
| **Atmospheric Pressure** | Affects object movement density. Higher pressure slows movement and can cause low-density objects to float. Lower pressure causes balloons to fall. |

Combined gravity and pressure settings create unique puzzle environments (e.g. "moon" levels where balloons fall and objects behave differently).

### 3.2 Simulated Physics Properties

- **Gravity** — pulls objects downward at adjustable strength; each object has a gravity response (some rise, some fall, some float).
- **Momentum** — determines force of impacts; heavier objects exert more force. Objects retain momentum through bounces and rolls.
- **Collision Detection** — handles bounces, rolls, triggers, and chain reactions. Objects interact on contact.
- **Elasticity** — dictates rebound height after collisions. Varies per object (super ball gains height; cannonball has zero bounce).
- **Friction** — affects sliding and rolling behavior, especially on inclines. Caution walls have high friction.
- **Density** — interacts with atmospheric pressure to determine buoyancy.
- **Air Resistance** — atmospheric pressure affects object travel distance and speed.

### 3.3 Per-Object Physics (Programmable Ball)
The Programmable Ball allows custom tuning of:
- Mass
- Elasticity
- Density
- Friction

### 3.4 Gravity Response Categories
Objects fall into three gravity response categories:
- **Falls under all gravity:** Bowling ball, cannonball, pinball
- **Falls under normal/strong gravity, rises under weak:** Basketball, baseball, tennis ball, cheese
- **Rises under normal/weak gravity, falls under strong:** Balloon
- **Zero gravity (floats until struck):** Pool ball (8-ball)

---

## 4. Complete Parts Catalog

### 4.1 Balls

| Part | Weight | Bounce | Gravity | Special |
|---|---|---|---|---|
| **Bowling Ball** | Heaviest | Minimal | Falls (all) | Largest ball; can push heavy objects |
| **Cannonball** | Heavy | None | Falls (all) | Also fired from cannon; no bounce at all |
| **Basketball** | Medium | High | Falls (normal+), rises (weak) | Good for bouncing puzzles |
| **Soccer Ball** | Medium | Medium | Falls (normal+), rises (weak) | — |
| **Baseball** | Light | Low | Falls (normal+), rises (weak) | Small; fits through tight gaps |
| **Tennis Ball** | Lightest | High | Falls (normal+), rises (weak) | Smallest ball; very bouncy |
| **Super Ball (Rubber Ball)** | Light | Extreme | Falls (normal+), rises (weak) | Gains height with each bounce; chaotic trajectory on angled surfaces |
| **Pinball** | Hard/heavy | Minimal | Falls (all) | — |
| **Pool Ball (8-Ball)** | — | — | Zero gravity (floats) | Stationary until struck; moves like a pool ball on a table; unaffected by gravity; programmable surface number (0–9); bounces off surfaces until momentum is lost |
| **Programmable Ball** | Custom | Custom | Custom | Adjustable sliders for mass, elasticity, density, friction |

### 4.2 Walls, Floors & Barriers

| Part | Resizable | Destructible by Dynamite | Notes |
|---|---|---|---|
| **Brick Wall** | Yes | Yes | Standard solid wall/floor |
| **Yellow Brick Wall** | Yes | Yes | Decorative variant |
| **Cinder Block Wall** | Yes | Yes | Heavy construction variant |
| **Greco-Roman Wall** | Yes | No | Classical-themed |
| **Wooden Wall (Wood)** | Yes | Yes | Standard wood barrier/floor |
| **Log Wall** | Yes | No | Rustic variant |
| **Caution Wall** | Yes | No | High friction, explosion-resistant |
| **Sand Wall** | Yes | No | Textured variant |
| **Pipe Wall (Metal Piping)** | Yes | No | Indestructible metal surface |
| **Curved Pipe Wall (Metal Piping Curve)** | Yes | No | Four-direction rotation; indestructible; used for corners and lips |
| **Grass Floor (Piece of Earth)** | No | No | Natural ground surface; indestructible |
| **Scaffold Barrier** | No | No | Construction-style barrier |
| **Wooden Barrier** | No | No | Small wooden obstruction |
| **Lattice Archway** | No | No | Decorative arch |
| **Marble Archway** | No | No | Classical arch |

### 4.3 Inclines & Ramps

| Part | Notes |
|---|---|
| **Brick Incline (Wooden Slant)** | Stretchable to adjust angle/steepness; multiple size settings from very steep (short) to nearly level (long); can be flipped for left/right slope direction |
| **Yellow Brick Incline** | Stretchable to adjust angle |
| **Granite Incline** | Stretchable to adjust angle |

All inclines are indestructible by dynamite.

### 4.4 Pipe System

| Part | Function |
|---|---|
| **Large Pipes** | Configurable straight pipe sections for routing objects |
| **T-Connectors** | Junction pieces for branching pipe routes |
| **Large Curved Pipe** | Direction control within pipe systems |
| **Accelerator Tubes** | Speed and direction modifier for objects in pipes |

### 4.5 Mechanical Parts

| Part | Function | Details |
|---|---|---|
| **Gears (Metal Cog)** | Interlocking toothed wheels; transfer rotational motion | Can pop balloons when spinning; must be driven by another spinning part or connected via belt; adjacent gears interlock automatically |
| **Pulley** | Redirects rope/cable paths | Allows re-angling rope direction; essential for connecting distant parts; rope threads through pulley to change direction |
| **Belt (Fan Belt)** | Connects rotating parts together | Limited stretch distance; requires proximity between connected components; the connecting link between any two spinning parts |
| **Teeter-Totter** | Seesaw/lever; tips when weight is applied to one side | Rope can attach to either end; can catapult objects; can be used as a ramp/deflector; a core puzzle-solving element |
| **Conveyor Belt** | Moves objects along its surface in one direction | Resizable (multiple length settings); must be connected to a spinning part via belt to operate; direction depends on driving part's rotation |
| **Trampoline** | Bounces objects upward on contact | Objects maintain their angle of approach; falling straight down bounces straight up; angled impacts produce angled bounces |
| **Jack-in-the-Box** | Springs open, catapulting objects on its lid | Must be driven by a spinning part connected via belt; can launch objects significant distances |
| **Windmill** | Rotates when blown by air (fan, bellows/bike pump, teapot steam) | Can be flipped to change rotation direction; drives other parts via belt |
| **Mandrill Motor** | Motorized rotating part with a shade/blind mechanism | Monkey rides when shade is open (banana visible); shade can be attached to rope and pulled open; if hit on head, monkey stops pedaling briefly; replaced Monkey Bike |
| **Mouse Exercise Wheel** | Spinning wheel powered by a frightened mouse | Activates when the mouse inside is startled by an object hitting the wheel or landing nearby; drives other parts via belt; mouse direction (left/right facing) determines spin direction |
| **Trans-Roto-Matic** | Converts translational motion to rotational motion | — |
| **Roto-Trans-Converter** | Converts rotational motion to translational motion | — |
| **Tipsy Trailer** | Tilting platform triggered by weight | — |
| **Scissors** | Cuts rope when closed; pops balloons on contact | Two states: open and closed; closes when an object presses on the handle or sharp end; can be positioned along a rope line |
| **Hedge Trimmers** | Cuts rope on contact | — |
| **Tin Snips** | Cuts rope or steel cable on contact | Only tool that cuts steel cable |
| **Automatic Boxing Glove** | Punches objects when triggered | Fires outward when its flat end is touched by any object; sends movable objects flying in the direction it faces; can be aimed left or right |
| **Vacuum Cleaner** | Sucks up nearby gravity-affected objects | Must be plugged into a powered outlet; sucks any object that "feels gravity"; placing a surface over its opening traps objects against it without sucking them in |
| **Pinball Bumper** | Bounces objects away on contact | Sends touching objects flying in the opposite direction, like a real pinball bumper |
| **Tack** | Pops balloons; can form walkways | Sharp point pops balloons on contact; multiple tacks placed together create a walking surface; can form curved ramps |

### 4.6 Electrical System

| Part | Function |
|---|---|
| **Generator** | Wheel-driven power source with outlet; must be connected to a spinning part via belt to generate power |
| **Electrical Outlet (Always-On)** | Provides constant power; electric devices plug directly into it |
| **Electric Switch & Outlet (Light Switch Outlet)** | Has a physical switch that must be flipped by a falling/moving object to activate; switch can be oriented up or down |
| **Solar Panel (Light-Powered Outlet)** | Activated by a nearby light source (flashlight, lava lamp, or candle); provides power when illuminated |
| **Laser-Activated Plug** | Color-programmable; activates when hit by a specific laser color |
| **Electric Fan** | Blows continuous directional airflow when powered; can push objects and spin windmills; must be plugged in |
| **Electric Motor** | Spins a gear/cog when powered; drives other parts via belt; can be reversed; must be plugged in |
| **Toaster** | Launches toast after programmable delay (light/medium/burnt); requires power |
| **Can Opener** | Opens cans when powered |
| **Electric Mixer** | Rotates when powered |

**Connection Rules:**
- The outlet must be placed on the screen first, then the electric device is connected to it.
- An outlet has two plug sockets — two devices can share one outlet.
- A plugged-in device shows a black circle on the outlet.

### 4.7 Laser System

| Part | Function |
|---|---|
| **Laser (Red)** | Emits a red laser beam |
| **Laser (Green)** | Emits a green laser beam |
| **Laser (Blue)** | Emits a blue laser beam |
| **Laser Mixer** | Blends laser beams (e.g. red + blue = purple) |
| **Angled Mirror** | Four-position deflection; redirects laser beams |
| **Laser Detector** | Receives any color laser; indicates status via lights |
| **Laser-Activated Plug** | Color-specific activation (see Electrical) |

**Laser interactions:** Pop balloons, ignite fuses, light candles, activate mechanisms.

### 4.8 Rope & Cable System

| Part | Properties |
|---|---|
| **Rope** | Standard flexible connector between parts; cuttable with scissors, hedge trimmers, or tin snips; threads through pulleys to change direction |
| **Steel Cable** | Stronger variant; only cuttable with tin snips |

**Rope/Cable attachment points:** Teeter-totter (either end), boat, laundry basket, bucket, leaky bucket, boat cleat, Captain Z Super Phazer, balloon, hot air balloon, lava lamp, mandrill motor (shade), tipsy trailer, match on spring, pulley, trans-roto-matic, roto-trans-converter, remote control bomb, gun (trigger), light with drawstring, dynamite plunger, jack-in-the-box shade, bottomless birdcage/laundry basket.

### 4.9 Animals & Characters

| Part | Behavior |
|---|---|
| **Pokey the Cat** | Walks in a direction; chases mice and is attracted to broken fishbowls; if hit by an object, shrieks with fur standing up; will not move until stimulated by a mouse or broken fishbowl in proximity; heavier than bucket |
| **Mort the Mouse** | Runs from Pokey the Cat; runs toward cheese; eaten by alligator; tiny enough to fit through small gaps; activates mouse exercise wheels by proximity |
| **Cheese** | Attracts Mort the Mouse; Mort runs directly toward it when in vicinity; does not get consumed — both sit still when Mort reaches it |
| **Edison Alligator** | Stationary; eats Mort the Mouse and Mel; will NOT eat Pokey; tail bounces objects (shakes up and down during simulation — objects dropped on tail bounce back up); jaw snaps at passing objects and flings them; signature laugh animation; replaced Ernie the Alligator |
| **Mel Schlemming** | Walks in a set direction until hitting a wall (turns around), reaching his house (enters and stops), or dying; dies from: falling too far, being eaten by alligator, hitting a wall too fast; programmable: walk/run/stationary; multiple Mels can exist simultaneously |
| **Mel's House** | Mel enters when he walks past it; house lights up with smoke from chimney; Mel visible through window; toggle between suburban house and log cabin appearance |
| **Bill's Fish Tank (Bob the Fish)** | Fish in a bowl; does nothing normally; when bowl is broken (by impact), the broken bowl attracts Pokey the Cat toward it; fish dies (stops flapping) without water; replaced Bob's Fish Bowl |
| **Monkey (on Mandrill Motor)** | Rides bicycle when shade/blind is opened (banana visible); shade must be pulled open via rope; if bonked on head, stops pedaling briefly with stars above head |
| **Leprechaun** | Decorative/interactive character (replaced Irish Shamrock) |

### 4.10 Ignition & Pyrotechnic Parts

| Part | Function | Details |
|---|---|---|
| **Magnifying Glass** | Focuses light into a concentrated beam | Does not interact with physics objects (untouchable during simulation); requires a light source (flashlight or lava lamp) aimed at it; can light any fuse, wick, or candle; must be oriented correctly (flippable); if flashlight is flipped, magnifying glass must also be flipped |
| **Flashlight** | Emits a beam of light | Must be turned on by having a moving object dropped onto its switch; provides light for magnifying glass and solar panel; can be oriented left or right |
| **Lava Lamp (Light with Drawstring)** | Provides light when drawstring is pulled | Drawstring must be pulled by weighted rope (attach rope to drawstring, route through pulleys, attach to weighted object); can power solar panels and provide light for magnifying glass; replaced Drawstring Light |
| **Candle** | Provides flame; ignites fuses on contact | Lit by magnifying glass + light source, laser, or another flame; can be blown out by fan or bellows; the only easily controllable moving light source; feels gravity; can be placed on conveyor belts to move while lit; used to heat teapots |
| **Dynamite** | Explodes when fuse is lit | Feels gravity; destroys brick and wood surfaces; does NOT destroy inclines/ramps, metal piping, metal piping curves, or earth/grass; chain-reaction: nearby dynamite explodes too; sends nearby objects flying |
| **Dynamite Plunger** | Explodes when plunger handle is pressed down | Pressure on handle (dropping object or pulling via rope) detonates; does NOT chain-react from nearby dynamite explosions; distinct from regular dynamite |
| **Cannon** | Fires a cannonball when fuse is lit | Requires magnifying glass + light source to ignite; fires in the direction it faces; cannonball follows gravity after firing |
| **Rocket** | Launches upward when fuse is lit | Feels gravity before ignition; after launch, the burning fuse end can ignite other objects (fuses, candles) and pop balloons on contact |
| **Fireworks** | Programmable explosion colors (three color options) | Decorative explosion effect |
| **Remote Control Bomb** | Attachable via rope; detonates on signal | — |
| **Match on Spring** | Spring-loaded ignition source | — |
| **Fuse** | Burns along its length; transfers ignition | Connects ignition sources to explosives |

**Ignition chain:** Light source (flashlight/lava lamp) → magnifying glass → fuse/wick/candle → dynamite/cannon/rocket. Alternatively: laser beam directly ignites fuses, candles, and pops balloons.

### 4.11 Air & Fluid Parts

| Part | Function |
|---|---|
| **Bike Pump (Bellows)** | Produces a burst of air when handle is compressed by a falling object or rising balloon; blows objects and spins windmills; directional (left or right); replaced Bellows |
| **Electric Fan** | Continuous directional airflow when powered; pushes objects and spins windmills |
| **Bucket (Pail)** | Catches and holds objects (balls, animals, cannonballs); must be attached to rope to hang; feels gravity; can be hit by bullets; weight changes when objects land inside |
| **Leaky Bucket** | Programmable water leak rate; heavier initially, lighter as it drains; weight differential creates rope/pulley dynamics |
| **Teapot** | Produces steam in an up-right or up-left direction when heated from below (usually by a lit candle); steam pushes the teapot backward (recoil); feels gravity; steam stream can spin windmills and push objects |
| **Hot Air Balloon** | Rises when heated; attachable via rope |
| **Balloon** | Four programmable appearance designs (identical behavior); rises under normal/weak gravity; falls under strong gravity; popped by: spinning gears, scissors, tacks, dynamite, candles, rockets, guns, lasers; can be held by rope attached to another part |

### 4.12 Gadgets & Specialty Items

| Part | Function |
|---|---|
| **Captain Z Super Phazer** | Ray gun; programmable blast quantity |
| **Santa Lamp** | Decorative lamp (replaced Christmas Tree) |
| **Egg Timer** | Programmable delay before spring-loaded arm deploys |
| **Eye Hook** | Attachment point for ropes |
| **Boat Cleat (Metal Loop)** | Stable, immovable rope attachment point; when rope needs a fixed anchor, tie it here; replaced Metal Loop |
| **Laundry Basket (Bottomless Birdcage)** | Container with no floor; can be dropped onto animals to trap them (e.g. trapping Pokey the Cat to prevent him from reaching Mort); attachable to rope; heavier than bucket; replaced Bird Cage |
| **Jack-in-the-Box** | Catapults objects placed on its lid when wound/turned; must be turned by a spinning part connected via belt; direction of catapult depends on orientation |
| **Gun (Revolver)** | Fires a bullet when trigger is pulled via rope; bullet travels instantly in the aimed direction; hits movable objects (cats, fish bowls, balls, Mel); rope attaches to trigger and must pull from behind the handle; can use pulleys to redirect rope angle; the gun body itself can be used as a ramp/deflector |
| **Anti-Gravity Pad** | Reverses gravity for objects on it |

### 4.13 Containers & Structures

| Part | Function |
|---|---|
| **Boxes (5 variants)** | Glass, wooden, wicker, metal, cardboard — varying sizes; programmable |
| **Mel's House** | Mel enters when walking past; lights up with chimney smoke; toggle between suburban house and log cabin |
| **Bill's Fish Tank** | Contains fish; when broken by impact, attracts Pokey the Cat; replaced Bob's Fish Bowl |
| **Mouse Hole** | Destination for Mort the Mouse in some puzzles |

### 4.14 Display & Information Parts

| Part | Function |
|---|---|
| **Message Computer** | Displays alphabet letters, numerals 0–9, symbols |
| **Color Block** | 44 color options for scenery/background blocks |

---

## 5. User Interface

### 5.1 Main Playfield
- 2D side-view workspace where the machine is built and run.
- Scrollable/expandable area for larger machines.
- Background color selectable from palette menu.

### 5.2 Control Panel (Left Side)

| Button | Function |
|---|---|
| **Green Flag** | Starts/runs the machine simulation |
| **Checkered Flag** | Indicates running state (animates when machine is active) |
| **Load** | Load puzzles |
| **Music** | Select and control background music (track list, volume, restart) |
| **Globe** | Adjust gravity and atmospheric pressure (Workshop only) |
| **Broom** | Remove all unlocked parts from playfield |
| **Question Mark** | Context-sensitive help — click then click any element |
| **Lightbulb** | Show Professor Tim's solution (Puzzle Play) |
| **Hint** | Activate hints placed by puzzle creator |
| **Scenery Toggle** | Show/hide scenery parts |

### 5.3 Parts Bin (Right Side)
- Scrollable sidebar displaying available parts with quantity indicators.
- Click a part and drag onto the playfield to place it.
- Right-click a placed part to duplicate (unlimited in Workshop, limited in Puzzle Play).
- Parts organized into categories navigable via keyboard shortcuts.
- In Workshop: "Adjust Parts Bin" mode accessible by clicking the parts bin; bomb button clears all parts; individual parts can be added/removed.

### 5.4 Part Interaction Handles
When a part is selected, contextual handles appear:

| Handle | Icon | Function |
|---|---|---|
| **Recycle** | Trash can / bottle | Returns part to the bin (or drag to parts bin) |
| **Flip** | Curved arrows (bottom left/right) | Rotates or mirrors the part; on metal piping curves, rotates 90 degrees clockwise/counter-clockwise |
| **Size** | Double arrows (sides/top/bottom) | Stretches or shrinks the part horizontally/vertically |
| **Programming** | Computer icon | Opens programming interface for configurable parts |
| **Info** | Magnifying glass | Shows part description and behavior |
| **Checkered Flag** | Flag (Workshop only) | Programs solution recognition state |
| **Padlock** | Lock (Workshop only) | Locks/unlocks parts; locked parts behave like fixed puzzle parts |

### 5.5 Puzzle Status Display
- Puzzle goal text displayed prominently (clickable placeholder in Workshop: "Click here to enter description").
- Light bulb indicators: empty = unsolved, glowing = solved.
- Animated checkered flag when machine is running.

### 5.6 Keyboard Shortcuts

| Key | Action |
|---|---|
| R | Jump to Balls category |
| W | Jump to Walls category |
| F | Jump to Floors category |
| P | Jump to Pipes category |
| I | Jump to Inclines category |
| A | Jump to Arches category |
| O | Jump to Pool Stuff category |
| G | Jump to Gears & Belts category |
| ALT+S | Screenshot (saves .LBM file) |

---

## 6. Puzzle Goal Types

Puzzles can define win conditions based on various object states and outcomes:

### 6.1 Object Destination Goals
- Get a ball into a specific container/basket/hoop/pipe basket.
- Move an object to a designated location (e.g. "Put the bowling ball inside the brick walls").
- Launch an object off-screen (top, bottom, sides) — e.g. "Make the bowling ball fall off the bottom of the screen."
- Get Mel to his house.
- Guide Mort the Mouse to his mouse hole.

### 6.2 State Change Goals
- Pop a balloon (or pop all balloons, or pop specific balloons).
- Light a candle (or all candles).
- Ignite dynamite / explode all dynamite.
- Launch a rocket off the top of the screen.
- Fire a cannon.
- Fire a gun / shoot all guns.
- Turn on all fans / lights.
- Break Bob's fishbowl.
- Make a gear/windmill turn.
- Boil water in a teapot.
- Open the Jack-in-the-Box.

### 6.3 Creature Interaction Goals
- Feed Mort the Mouse to Pokey the Cat (let cat catch mouse).
- Feed animals to the alligator.
- Trap Pokey the Cat in a cage.
- Trap Mort the Mouse in a cage.
- Save Mort from the cats (guide him to safety).
- Get Pokey home to his house.
- Exercise the monkey (make him ride his bicycle).
- Make all mice run in their exercise wheels simultaneously.
- Vacuum up objects.

### 6.4 Prevention / Protection Goals
- Don't let any balloons pop ("Save the balloons").
- Save Bob the Fish (prevent his bowl from breaking).
- Don't let the cat eat any mice.
- Protect specific objects from destruction.

### 6.5 Complex/Compound Goals
- Multiple simultaneous conditions (e.g. "Put the cannonball in the bucket AND the bucket in the hole").
- Sorting goals (e.g. "Put baseballs in the left container and tennis balls in the right").
- Sequence-dependent goals requiring specific ordering and timing.
- Goals requiring specific physics states (object at rest in a position, object moving through an area).

### 6.6 Solution Recognition System (Workshop)
- **Non-moving parts:** Define desired final state (occupied, vacant, activated, etc.).
- **Moving parts:** Define exit trajectory (Off Top, Off Bottom, Off Screen) or target position rectangle.
- **State triggers:** Ignited, launched, popped, destroyed, opened.
- **Advanced:** Part count verification, delay timing, position designation.

---

## 7. Scoring & Progression

### 7.1 Single-Player
- Sign-in system tracks individual player progress with separate records per username.
- Solved puzzles marked with glowing light bulb indicators.
- Two bonus countdown timers per level — completing before timers expire adds points to score.
- Puzzles unlock progressively through difficulty tiers.
- Password system: each solved puzzle reveals a password to jump to the next puzzle.

### 7.2 Head-to-Head
- Match-based scoring: first player to solve wins the round.
- Configurable matches (1–9 games per match).
- Scoreboard tracks cumulative wins per player.

---

## 8. Audio & Visual Design

### 8.1 Graphics
- Comic-book-style 2D art with improved sprites over the original game.
- Dynamic part animations:
  - **Idle:** Cat meowing, alligator tail shaking, mouse running in wheel, monkey riding.
  - **Action:** Explosions, bouncing, flowing water/steam, rocket trails, candle flames, spinning gears.
  - **Reaction:** Cat shrieking (fur standing up when hit), monkey seeing stars (when bonked), Mel entering house (lights, chimney smoke, visible through window), alligator laughing after eating.
- Ambient animations (toggleable): idle part wiggling and twinkling effects.
- Environmental scenery with front/back layering.

### 8.2 Sound
- Sound effects for motion, impact, activation, and destruction providing satisfying feedback.
- Character-specific audio: alligator laugh, cat meows/shrieks, monkey chatter, Mel's footsteps.
- Selectable background music with scrollable track list and volume control.
- CD version features: voice-over narration, expanded soundtrack (blues, progressive rock, hip-hop, hoedown), multi-language support (English, French, German, Spanish).

---

## 9. Player Preferences

Toggleable settings accessible from the options menu:

| Setting | Effect |
|---|---|
| **Ambient Animation** | Idle part wiggling/twinkling |
| **Scenery Parts Display** | Show/hide decorative scenery |
| **Background Animation** | Enable/disable animated backgrounds |
| **Info Handles** | Show/hide magnifying glass tooltips |
| **Sound Effects** | Enable/disable sound effects |

---

## 10. Part Interaction Matrix

Key interaction rules governing how parts affect each other:

| Interaction | Description |
|---|---|
| **Weight on Lever** | Objects falling on teeter-totter catapult items on the other end; rope can connect both ends to distant parts |
| **Air on Windmill** | Bike pump, fan, or teapot steam spins windmills; windmill can be flipped to reverse rotation direction |
| **Belt Drive** | Rotating parts (gears, windmill, electric motor, mouse exercise wheel, mandrill motor) transfer motion via belt to other rotating parts (conveyor, jack-in-the-box, gears, generator) |
| **Rope + Pulley** | Rope threaded through pulleys redirects force direction; creates mechanical advantage systems; essential for connecting distant parts |
| **Cat sees Mouse** | Pokey chases Mort; Mort runs from Pokey; Pokey is faster than Mort |
| **Cat sees Broken Bowl** | Pokey is attracted to broken fish bowls and will walk toward them; used to lure Pokey |
| **Mouse sees Cheese** | Mort runs directly toward cheese when in vicinity |
| **Alligator eats** | Edison Alligator eats Mort the Mouse and Mel; will NOT eat Pokey the Cat |
| **Alligator tail** | Objects dropped on alligator's bouncing tail are bounced back up; tail shakes continuously during simulation |
| **Flame ignites Fuse** | Candle, rocket trail, or focused light (magnifying glass + flashlight/lava lamp) ignites fuses, wicks, and candles |
| **Light on Solar Panel** | Flashlight, lava lamp, or candle activates solar panel to produce power |
| **Light + Magnifying Glass** | Light source aimed through magnifying glass creates focused beam that ignites fuses/wicks; both must face the same direction |
| **Laser on Mirror** | Angled mirror redirects laser beam in one of four configurable directions |
| **Laser Mixing** | Laser mixer combines colored beams (red + blue = purple, etc.) |
| **Electricity Chain** | Generator/switch/solar panel → outlet → powered device (fan, electric motor, vacuum, toaster, can opener, mixer) |
| **Two Devices per Outlet** | Each outlet has two sockets; two electric devices can share one power source |
| **Gravity on Balloon** | Normal/weak gravity: balloons rise. Strong gravity: balloons fall. Balloon weight is near-zero. |
| **Heat on Teapot** | Lit candle placed beneath teapot causes it to produce steam and slide backward from recoil |
| **Heat on Hot Air Balloon** | Candle/flame beneath balloon causes it to rise |
| **Object in Pipe** | Objects entering pipe openings travel through connected pipe sections |
| **Cutting Rope** | Scissors (when closed), hedge trimmers, or tin snips sever rope; only tin snips cut steel cable |
| **Explosion Radius** | Dynamite destroys brick and wood surfaces; does NOT destroy inclines, metal piping, metal piping curves, or earth/grass. Nearby dynamite chain-reacts. Dynamite plunger does NOT chain-react. |
| **Spinning Gear pops Balloon** | A gear that is actively spinning will pop any balloon that contacts it |
| **Gun Trigger** | Rope attached to gun trigger, pulled by a weighted or moving object (through pulleys if needed), fires the gun |
| **Boxing Glove Trigger** | Any object touching the flat end of the automatic boxing glove causes it to punch outward |
| **Mouse Exercise Activation** | Any object falling near or hitting the mouse exercise wheel scares the mouse inside, making it run and spin the wheel |
| **Mandrill Shade** | Monkey only pedals when its shade is open (banana visible); shade can be opened by pulling via rope attached to it |
| **Vacuum Suction** | Powered vacuum sucks in any gravity-affected object in its vicinity; placing a surface over the opening traps objects without consuming them |
| **Object as Ramp** | Many non-obvious parts can serve as ramps/deflectors: teeter-totters, bellows, boxing gloves, guns, fans, electric motors, flashlights, metal piping curves, vacuums, tacks, pinball bumpers |

---

## 11. Puzzle Design Patterns

Common puzzle design patterns observed across the 150+ levels:

| Pattern | Description |
|---|---|
| **Chain Reaction** | A single initial event (e.g. ball drop) triggers a cascade of interactions leading to the goal |
| **Timing Puzzle** | Multiple events must happen in a specific time sequence; player must add delays or speed up paths |
| **Mirror/Copy** | Player must duplicate an existing setup on the opposite side of the screen |
| **Decoy Parts** | Extra parts are provided that are not needed, misleading the player |
| **Creature Routing** | Using animal behaviors (cat chasing mouse, mouse chasing cheese) to route characters to destinations |
| **Multi-Path** | Multiple independent chains must all succeed simultaneously |
| **Bouncing/Aiming** | Precise placement of trampolines, ramps, or bumpers to route an object to a target |
| **Power Chain** | Building a chain of power generation: spinning part → belt → generator → outlet → electric device |
| **Ignition Chain** | Building a chain to light something: light source → magnifying glass → fuse → explosive |
| **Protection** | Preventing an event from happening (blocking bullets, trapping cats, redirecting objects) |
| **Altered Physics** | Puzzles with modified gravity/pressure (moon levels, high-pressure levels) that change object behaviors |

---

## 12. Educational Value

The game implicitly teaches:
- **Classical mechanics**: gravity, momentum, force, friction, elasticity, weight.
- **Simple machines**: levers (teeter-totter), pulleys, inclined planes (ramps), gears, wheels, wedges.
- **Energy transfer**: kinetic, potential, electrical, thermal, pneumatic (air pressure).
- **Cause and effect**: sequential chain reactions and logical reasoning.
- **Engineering thinking**: iterative design, testing, debugging, creative problem-solving.
- **Optics**: laser reflection, color mixing, light focusing through magnifying glass.
- **Electrical circuits**: power sources, switches, powered devices, outlets.
- **Pneumatics/fluid dynamics**: air pressure, steam generation, wind power.
- **Biology/behavior**: animal stimulus-response patterns (cat-mouse, mouse-cheese).

---

## Sources

- [The Incredible Machine 2 - Wikipedia](https://en.wikipedia.org/wiki/The_Incredible_Machine_2)
- [The Incredible Machine 2 - MobyGames](https://www.mobygames.com/game/1605/the-incredible-machine-2/)
- [The Incredible Machine 2 - Dynamix Wiki](https://dynamix.fandom.com/wiki/The_Incredible_Machine_2)
- [The Incredible Machine 2 - Grokipedia](https://grokipedia.com/page/the_incredible_machine_2)
- [The Incredible Machine 2 Parts - Fandom Wiki](https://the-incredible-machine.fandom.com/wiki/The_Incredible_Machine_2/Parts)
- [The Incredible Machine 2 - Internet Archive (Manual)](https://archive.org/stream/Incredible_Machine_2_-_Manual/Incredible_Machine_2_-_Manual_djvu.txt)
- [The Even More Incredible Machine - Walkthrough by Mike8787 (GameFAQs)](https://gamefaqs.gamespot.com/pc/564703-the-even-more-incredible-machine/faqs/18514)
- [The Incredible Machine - DOSGames.com](https://dosgames.com/game/the-incredible-machine-2/)
