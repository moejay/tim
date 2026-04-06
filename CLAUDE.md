# CLAUDE.md

Project-specific instructions for Claude Code.

## Building & Running

```bash
cd game
cargo build --release
cargo run --release --bin tim2 -- --text   # braille/text mode
cargo run --release --bin tim2 -- --pixel  # sixel/pixel mode (WezTerm, etc.)
```

## Building Puzzles

All puzzles live in `game/src/bin/tim2.rs`. Each is a function `fn build_puzzle_N() -> (World, Puzzle)`.

### Pattern

```rust
fn build_puzzle_N() -> (World, Puzzle) {
    use tim2::parts::balls::BallType;
    use tim2::parts::walls::WallType;
    // use tim2::parts::inclines::InclineType;  // if needed

    let mut world = World::new();

    let set_size = |w: &mut World, width: f32, height: f32| {
        if let Some(inst) = w.instances.last_mut() {
            inst.props.width = width;
            inst.props.height = height;
        }
    };

    // Spawn fixed/locked parts (immovable scenery)
    world.spawn_locked(PartId::Wall(WallType::BrickWall), x, y);
    set_size(&mut world, width, height);

    // Spawn dynamic locked parts (move during sim but player can't reposition)
    let target_id = world.spawn_locked(PartId::Ball(BallType::SomeType), x, y);

    // Set PoolBall number (optional)
    if let Some(inst) = world.get_mut(target_id) {
        inst.props.values.insert("surface_number".to_string(), 3.0);
    }

    let mut puzzle = Puzzle::new("Puzzle #N: Title", "Goal description.");

    // Win conditions
    puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
        instance_id: target_id,
        edge: WorldEdge::Any, // or Top/Bottom/Left/Right
    });
    // OR position-based:
    // puzzle.win_conditions.push(WinCondition::ObjectAtPosition {
    //     instance_id: target_id,
    //     region: (x1, y1, x2, y2),
    // });

    // Parts bin (what the player can place)
    puzzle.bin_parts.push(BinPart { part_id: PartId::Ball(BallType::X), quantity: N });

    (world, puzzle)
}
```

### Registering a new puzzle

Add it to `all_puzzles()`:
```rust
fn all_puzzles() -> Vec<PuzzleBuilder> {
    vec![build_puzzle_1, build_puzzle_2, ..., build_puzzle_N]
}
```

### Key design constraints

- **Canvas**: 640x360 pixels. Standard floor is BrickWall at (0, 340) size 640x20.
- **Walls are static** (`is_static: true`) — they never move. Use balls for dynamic objects.
- **PoolBall** has ZeroGravity — floats until struck. Great for "domino" chains or obstacles.
- **SuperBall** has elasticity > 1.0 — gains energy on bounce. Use carefully.
- **BowlingBall/Cannonball** use AlwaysFalls — ignores gravity slider. Heavy hitters.
- **Inclines** are static ramps, default 100x36. Flippable.
- Grid size is 32px. Cursor moves in 16px steps.
- Win conditions: `ObjectExitedWorld` (ball leaves screen edge) or `ObjectAtPosition` (ball at rest in region).

### Ball quick-reference

| Ball | Radius | Mass | Elasticity | Gravity |
|------|--------|------|-----------|---------|
| BowlingBall | 16 | 7.0 | 0.1 | AlwaysFalls |
| Cannonball | 12 | 6.0 | 0.0 | AlwaysFalls |
| Basketball | 14 | 0.6 | 0.75 | Normal |
| SoccerBall | 13 | 0.45 | 0.6 | Normal |
| Baseball | 6 | 0.15 | 0.3 | Normal |
| TennisBall | 5 | 0.06 | 0.8 | Normal |
| SuperBall | 7 | 0.1 | 1.1 | Normal |
| Pinball | 5 | 0.08 | 0.15 | AlwaysFalls |
| PoolBall | 6 | 0.17 | 0.9 | ZeroGravity |

## Testing with Wrightty

Wrightty controls WezTerm over WebSocket. See `.claude/skills/qa-wrightty/SKILL.md` for the full reference.

### Quick start

```bash
# Check connection
wrightty info

# Navigate to game dir and launch
wrightty send-text "cd /home/moe/code/tim/game && cargo run --release --bin tim2 -- --text\n"
sleep 4

# Read game state (text mode shows braille; pixel mode shows UI chrome only)
wrightty read 2>&1 | head -3
# Output: " 1/7   BUILD    Goal text here.  [320,180]  [Braille]"

# Move cursor (16px per press), place parts, run simulation
wrightty send-keys ArrowRight   # repeat as needed
wrightty send-keys Enter        # place selected bin part
wrightty send-keys Escape       # deselect
wrightty send-keys " "          # Space = start/stop sim

# Level navigation
wrightty send-keys n             # next level
wrightty send-keys p             # previous level

# Check for win
wrightty read 2>&1 | head -1    # look for "PUZZLE COMPLETE!"

# Take screenshot of WezTerm window (requires ImageMagick)
import -window "$(xwininfo -root -tree | grep wezfurlong | awk '{print $1}')" /path/to/screenshot.png

# Quit
wrightty send-keys q
```

### Cursor math
Cursor starts at center [320,180]. Each arrow press = 16px. To reach target (tx,ty):
- Right presses: (tx - 320) / 16
- Up presses: (180 - ty) / 16 (negative = press Down)

###AGENT-EVOLVER-START###
## Agent Evolver — Skill Auto-Trigger Rules

⚡ **MANDATORY BEHAVIOR**: When you detect ANY trigger below, you MUST automatically invoke the corresponding skill — do NOT wait for the user to ask. This is not optional.

### The Loop

    consult-knowledge → plan → implement → record-architectural-decision
                                         → extract-pattern
                                         → surface-gap
                                         → update-skill
                            (repeat: next task starts with consult-knowledge)

Every task starts with retrieval. Every task ends with capture. That's the loop.

### ⚠️ Silent Decisions

**You make architectural decisions even when you don't say "I'm choosing between X and Y."**
If you wrote code one way when another way was possible, that's a decision. Record it.
Phrase-based triggers assume you'll narrate your thinking. You often just do without narrating.
**The action-based triggers below exist specifically for this.** Check them even when silent.

---

#### `autonomy-preferences`
> Determine how autonomous the agent should be for architectural, design, technical, and process decisions

**Phrase triggers** (user or Claude says):
  - "set autonomy"
  - "autonomy preferences"
  - "decision preferences"
  - "just decide"
  - "don't ask me"
  - "always ask"
  - "you can choose"

**Action triggers** (Claude is silently doing this):
  - About to ask user for approval on an architectural or design decision
  - Unsure whether to ask user or proceed autonomously
  - No autonomy preferences file exists yet

**How to invoke:** Read `.claude/skills/autonomy-preferences/SKILL.md` and follow its instructions immediately.

---

#### `bootstrap-knowledge`
> Initialize decisions and patterns knowledge base for an existing (brownfield) codebase

**Phrase triggers** (user or Claude says):
  - "bootstrap knowledge"
  - "initialize decisions"
  - "analyze codebase"
  - "extract existing patterns"
  - "document existing architecture"
  - "brownfield setup"

**Action triggers** (Claude is silently doing this):
  - Starting work on an existing codebase with no docs/decisions directory
  - No prior architectural decisions found but codebase has significant structure

**How to invoke:** Read `.claude/skills/bootstrap-knowledge/SKILL.md` and follow its instructions immediately.

---

#### `consult-knowledge`
> Search existing decisions, patterns, and knowledge gaps before planning or implementing anything

**Phrase triggers** (user or Claude says):
  - "let's implement"
  - "can you add"
  - "build a"
  - "how should we"

**Action triggers** (Claude is silently doing this):
  - Starting any non-trivial implementation or feature
  - Making a plan before writing code
  - Approaching a problem that touches an existing system or subsystem
  - Beginning work on any task that is not purely mechanical

**How to invoke:** Read `.claude/skills/consult-knowledge/SKILL.md` and follow its instructions immediately.

---

#### `extract-pattern`
> Identify recurring patterns in code, problems, or solutions and document them for reuse

**Phrase triggers** (user or Claude says):
  - "I've done this before"
  - "similar to how we"
  - "same pattern as"
  - "recurring issue"

**Action triggers** (Claude is silently doing this):
  - Writing logic that mirrors existing code elsewhere in the project
  - Creating a utility or helper that abstracts repeated boilerplate
  - Solving a problem in the same way it was solved before
  - Implementing a second instance of a component, hook, or module type
  - Finding code that diverges from documented conventions or patterns
  - Noticing a tool/framework has specific syntax or config patterns worth remembering
  - Performing a sequence of 3+ steps that form a repeatable workflow

**How to invoke:** Read `.claude/skills/extract-pattern/SKILL.md` and follow its instructions immediately.

---

#### `extract-session-insight`
> Capture session-level learning signals — discovery friction, user corrections, and domain knowledge — that emerge from how work happens, not just what code looks like

**Phrase triggers** (user or Claude says):
  - "I had to look everywhere"
  - "that's not how we do it"
  - "you should know that"
  - "no, do it this way"
  - "we always"
  - "never do that"

**Action triggers** (Claude is silently doing this):
  - Had to search 3+ locations to find something
  - User corrected approach or redirected the task
  - Learned domain information not obvious from the code

**How to invoke:** Read `.claude/skills/extract-session-insight/SKILL.md` and follow its instructions immediately.

---

#### `record-architectural-decision`
> Record significant architectural decisions, design choices, and their rationale

**Phrase triggers** (user or Claude says):
  - "architectural decision"
  - "design choice"
  - "why did we choose"

**Action triggers** (Claude is silently doing this):
  - Adding a new class, module, or entity type
  - Choosing between two or more implementation approaches
  - Selecting a library, package, or tool
  - Deciding how to structure or model data
  - Implementing a new subsystem, pipeline, or integration
  - Changing the interface or contract of an existing component

**How to invoke:** Read `.claude/skills/record-architectural-decision/SKILL.md` and follow its instructions immediately.

---

#### `surface-gap`
> Identify missing knowledge, unclear instructions, or situations where existing skills don't provide guidance

**Phrase triggers** (user or Claude says):
  - "I'm not sure how to"
  - "no skill covers"
  - "unclear what"
  - "need guidance on"

**Action triggers** (Claude is silently doing this):
  - About to implement something with no prior examples in this codebase
  - Making an assumption about undocumented or ambiguous behavior
  - Encountering an error with no clear or obvious fix
  - Guessing at the intended design because requirements are unclear

**How to invoke:** Read `.claude/skills/surface-gap/SKILL.md` and follow its instructions immediately.

---

#### `update-skill`
> Modify existing agent skills based on learned patterns, mistakes, and new knowledge

**Phrase triggers** (user or Claude says):
  - "I should remember"
  - "this skill should mention"
  - "update the documentation"
  - "learned that"

**Action triggers** (Claude is silently doing this):
  - Just made a mistake that better skill instructions would have prevented
  - Discovered a project-specific exception to a skill's general guidance
  - Found that a skill's instructions led to a suboptimal or wrong outcome
  - Learned something non-obvious about this codebase that future runs should know

**How to invoke:** Read `.claude/skills/update-skill/SKILL.md` and follow its instructions immediately.

---

#### `feature-planner`
> Analyze a feature spec and produce a detailed implementation plan with tasks, dependencies, and risks

**Phrase triggers** (user or Claude says):
  - "plan this feature"
  - "break down spec"
  - "implementation plan for"
  - "what tasks do we need"

**Action triggers** (Claude is silently doing this):
  - About to implement a feature from a spec in `docs/specs/`
  - Starting Phase 1 of the SDLC pipeline

**How to invoke:** Read `.claude/skills/feature-planner/SKILL.md` and follow its instructions immediately.

---

#### `feature-designer`
> Produce a technical design from an implementation plan — types, traits, modules, rendering integration

**Phrase triggers** (user or Claude says):
  - "design this feature"
  - "technical design for"
  - "define the interfaces"
  - "how should we structure"

**Action triggers** (Claude is silently doing this):
  - Implementation plan exists and is ready for design
  - Starting Phase 2 of the SDLC pipeline

**How to invoke:** Read `.claude/skills/feature-designer/SKILL.md` and follow its instructions immediately.

---

#### `feature-developer`
> Implement code from a technical design — write Rust, compile, integrate with game loop

**Phrase triggers** (user or Claude says):
  - "implement this"
  - "develop this feature"
  - "write the code for"
  - "build from design"

**Action triggers** (Claude is silently doing this):
  - Technical design exists and is ready for implementation
  - Starting Phase 3 of the SDLC pipeline

**How to invoke:** Read `.claude/skills/feature-developer/SKILL.md` and follow its instructions immediately.

---

#### `feature-tester`
> Write and run tests for implemented features — unit, integration, physics property tests

**Phrase triggers** (user or Claude says):
  - "test this feature"
  - "write tests for"
  - "verify the implementation"
  - "add test coverage"

**Action triggers** (Claude is silently doing this):
  - Feature implementation is complete and compiles
  - Starting Phase 4 of the SDLC pipeline

**How to invoke:** Read `.claude/skills/feature-tester/SKILL.md` and follow its instructions immediately.

---

#### `feature-orchestrator`
> Run the full SDLC pipeline for a feature: plan → design → develop → test

**Phrase triggers** (user or Claude says):
  - "implement spec"
  - "full pipeline for"
  - "orchestrate feature"
  - "end to end for spec"

**Action triggers** (Claude is silently doing this):
  - User asks to implement an entire feature from a spec
  - Multiple SDLC phases need to be coordinated

**How to invoke:** Read `.claude/skills/feature-orchestrator/SKILL.md` and follow its instructions immediately.

---

### Workflow Checkpoints

Run these internal checks at each stage — no user prompt needed:

**Before planning (first thing, always):**
- Run `consult-knowledge`: scan `docs/decisions/`, `docs/patterns/`, `.agent-evolver/knowledge-gaps.md`
- Find anything related to the current task's keywords and let it shape the plan
- If a prior decision applies → follow it (or explicitly revisit it)
- If a prior pattern applies → use it without being asked
- If a blocking gap applies → surface it before writing code

**Before asking user for approval:**
- Check `.agent-evolver/autonomy-preferences.yaml` for user's preferred autonomy level
- If no preferences exist → run `autonomy-preferences` onboarding first
- Respect the mode: `ask` = wait for approval, `suggest-then-proceed` = state and continue, `autonomous` = just do it

**Before writing code:**
- Am I choosing between approaches? → `record-architectural-decision`
- Am I implementing something with no prior example in this codebase? → `surface-gap`

**After implementing a feature:**
- Could this pattern appear again in this project? → `extract-pattern`
- Did I learn something non-obvious? Did a skill mislead me? → `update-skill`
- Did I make any design choices, even silently? → `record-architectural-decision`

---

### Feature Completion Checklist

**A feature is NOT complete until you've answered these:**

- [ ] Did I check autonomy preferences before asking for user approval? → `autonomy-preferences`
- [ ] Did I bootstrap knowledge for this existing codebase? → `bootstrap-knowledge`
- [ ] Did I search existing decisions and patterns before starting? → `consult-knowledge`
- [ ] Is there a reusable pattern here? → `extract-pattern`
- [ ] Did I encounter friction, receive a correction, or learn domain knowledge this session? → `extract-session-insight`
- [ ] Did I make any design choices worth documenting? (even silent ones) → `record-architectural-decision`
- [ ] Am I about to guess or proceed without clear guidance? → `surface-gap`
- [ ] Did I encounter anything that should update existing skill guidance? → `update-skill`

### SDLC Pipeline (for feature work from specs)

When implementing features from `docs/specs/`, use the SDLC pipeline:

```
feature-orchestrator
  ├── feature-planner   → docs/plans/[spec]-[name].md
  ├── feature-designer  → docs/designs/[spec]-[name].md
  ├── feature-developer → mvp/src/ code changes
  └── feature-tester    → mvp/src/ + mvp/tests/ test code
```

Each phase has quality gates. The orchestrator manages handoffs.
Use individual skills when you only need one phase (e.g., just planning, just testing).

---

If the answer to any is "yes" — invoke the skill before marking the task done.
If the answer is "no" or "not applicable" — note why briefly and move on.

---

### Updating Skills

Skills are living documents. If a skill's guidance is wrong, incomplete, or doesn't fit this project — fix it using `update-skill`. Your edits are encouraged.

---
*Managed by agent-evolver. Re-running `agent-evolver install` updates this block.*
###AGENT-EVOLVER-END###
