# Skill: feature-orchestrator

**Purpose:** Orchestrate the full SDLC pipeline for a feature: plan → design → develop → test. Manages the handoff between phases, tracks progress, and ensures quality gates are met.

---

## Inputs

- **Spec path**: Path to the feature spec (e.g., `docs/specs/03-parts-mechanical/v1.md`)
- **Phase** (optional): Start from a specific phase if prior phases are done (e.g., "from design")
- **Scope** (optional): Subset of the spec to implement

---

## Pipeline Phases

### Phase 1: Plan (feature-planner)

**Invoke:** Read `.claude/skills/feature-planner/SKILL.md` and follow its process.

**Quality gate before proceeding:**
- [ ] Plan exists at `docs/plans/[spec-number]-[feature-name].md`
- [ ] All tasks have acceptance criteria
- [ ] Dependencies on other specs are identified and resolved (or flagged as blockers)
- [ ] Risks are documented
- [ ] No blocking knowledge gaps (check via surface-gap)

**Output:** Implementation plan document

---

### Phase 2: Design (feature-designer)

**Invoke:** Read `.claude/skills/feature-designer/SKILL.md` and follow its process.

**Quality gate before proceeding:**
- [ ] Design exists at `docs/designs/[spec-number]-[feature-name].md`
- [ ] All new types have Rust definitions with field types
- [ ] All public interfaces have function signatures
- [ ] Both pixel and text renderer approaches are specified
- [ ] Physics integration is defined (if applicable)
- [ ] Design is consistent with existing code patterns

**Output:** Technical design document

---

### Phase 3: Develop (feature-developer)

**Invoke:** Read `.claude/skills/feature-developer/SKILL.md` and follow its process.

**Use the Agent tool** to run development in a focused context:
- Launch a general-purpose agent with the full developer skill prompt
- Pass it the design document path and task scope
- The agent implements code, runs cargo check, and reports results

**Quality gate before proceeding:**
- [ ] `cargo check` passes with no errors
- [ ] `cargo clippy` has no warnings (default lint level)
- [ ] All files from the design's change summary are implemented
- [ ] Both renderers updated for any visual changes
- [ ] No unresolved TODO/FIXME comments left behind

**Output:** Working code committed (or ready to commit)

---

### Phase 4: Test (feature-tester)

**Invoke:** Read `.claude/skills/feature-tester/SKILL.md` and follow its process.

**Quality gate:**
- [ ] `cargo test` passes with no failures
- [ ] Each acceptance criterion from the plan is either:
  - Covered by an automated test, OR
  - Documented as "manual verification needed" with instructions
- [ ] No bugs found that block the feature

**Output:** Test suite, coverage report

---

## Orchestration Rules

### Sequential by Default
Phases run sequentially: plan → design → develop → test. Each phase's output feeds the next.

### Parallel Tasks Within Development
During Phase 3, if the plan identifies independent tasks, use parallel Agent tool calls:
- Launch separate agents for independent tasks
- Wait for all to complete before integration wiring

### Failure Handling
- **Compilation failure in develop**: Fix immediately, don't proceed to test
- **Test failure**: Determine if bug in code or test, fix, re-run
- **Design assumption wrong**: Update the design doc, note the deviation, continue
- **Spec ambiguity found**: Surface it (surface-gap skill), make a reasonable choice, document it

### Progress Tracking
After each phase, produce a brief status update:

```
## Feature: [Name] — Phase [N/4] Complete

✓ Plan: [done/in-progress/blocked]
✓ Design: [done/in-progress/blocked]
✓ Develop: [done/in-progress/blocked]
✓ Test: [done/in-progress/blocked]

Next: [what happens next]
Blockers: [any blockers]
```

---

## Multi-Feature Orchestration

When working on multiple specs:

1. **Respect the dependency graph** from `docs/specs/INDEX.md`
2. **Plan all features first** before designing any — this catches cross-feature conflicts early
3. **Design features that share interfaces together** — e.g., if two part types use the same collision system
4. **Develop in dependency order** — upstream features first
5. **Test incrementally** — don't wait for all features to test

---

## Directory Structure

The orchestrator creates/uses:
```
docs/
├── plans/          # Phase 1 output
│   ├── 01-physics-engine.md
│   └── ...
├── designs/        # Phase 2 output
│   ├── 01-physics-engine.md
│   └── ...
├── specs/          # Input (existing)
│   ├── INDEX.md
│   └── ...
└── decisions/      # Side output from any phase
    └── ...
```

---

## Integration with Agent-Evolver

Before starting ANY feature:
1. Run `consult-knowledge` — check for prior decisions and patterns
2. Check `autonomy-preferences` — know how autonomous to be

After completing ANY feature:
1. Run `extract-pattern` if reusable patterns emerged
2. Run `record-architectural-decision` for significant choices
3. Run `update-skill` if any skill instructions were suboptimal
4. Run `surface-gap` for any unresolved ambiguities

---

## Quick Start

To orchestrate a single feature end-to-end:

```
User: "Implement spec 03 (mechanical parts)"

Orchestrator:
1. consult-knowledge (prior decisions/patterns)
2. feature-planner on docs/specs/03-parts-mechanical/v1.md
3. [quality gate check]
4. feature-designer from the plan
5. [quality gate check]
6. feature-developer from the design (parallel agents for independent tasks)
7. [quality gate check]
8. feature-tester on the implementation
9. [quality gate check]
10. extract-pattern + record-architectural-decision + update-skill
11. Status report to user
```
