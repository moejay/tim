# CLAUDE.md

Project-specific instructions for Claude Code.

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

If the answer to any is "yes" — invoke the skill before marking the task done.
If the answer is "no" or "not applicable" — note why briefly and move on.

---

### Updating Skills

Skills are living documents. If a skill's guidance is wrong, incomplete, or doesn't fit this project — fix it using `update-skill`. Your edits are encouraged.

---
*Managed by agent-evolver. Re-running `agent-evolver install` updates this block.*
###AGENT-EVOLVER-END###
