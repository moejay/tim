# Skill: feature-planner

**Purpose:** Analyze a feature spec and produce a detailed implementation plan with task breakdown, dependency mapping, risk assessment, and effort estimation. This is the first phase of the SDLC pipeline.

---

## Inputs

- **Spec path**: Path to the feature spec (e.g., `docs/specs/01-physics-engine/v1.md`)
- **Scope override** (optional): Subset of the spec to plan (e.g., "just the gravity system")

---

## Process

### 1. Read & Understand the Spec

- Read the full spec file
- Read `docs/specs/INDEX.md` for the dependency graph
- Identify which other specs this feature depends on and which depend on it

### 2. Audit Existing Code

- Search `mvp/src/` for any existing implementation related to the spec
- Map what already exists vs. what's new work
- Identify files that will be modified vs. created
- Read existing files that will be touched to understand current structure

### 3. Break Down into Tasks

Create an ordered task list. Each task should be:
- **Atomic**: completable in a single coding session
- **Testable**: has clear acceptance criteria
- **Ordered**: respects internal dependencies

Task format:
```markdown
### Task N: [Short Title]
- **Type**: new-module | modify-existing | refactor | integration
- **Files**: [list of files to create/modify]
- **Depends on**: [Task numbers or "none"]
- **Description**: [What to build, 2-3 sentences]
- **Acceptance criteria**:
  - [ ] [Criterion 1]
  - [ ] [Criterion 2]
- **Risk**: low | medium | high — [why]
```

### 4. Identify Risks & Open Questions

- Things the spec doesn't fully define
- Performance concerns
- Integration challenges with existing code
- Breaking changes to current behavior

### 5. Produce the Plan

Write the plan to `docs/plans/[spec-number]-[feature-name].md` using the template below.

---

## Output Template

```markdown
# Implementation Plan: [Feature Name]

**Spec:** [path to spec]
**Date:** [YYYY-MM-DD]
**Status:** Draft

## Summary
[2-3 sentences on what this feature adds and the implementation approach]

## Dependency Analysis

### Upstream (this feature needs)
- [Spec/feature] — [status: implemented | partial | missing]

### Downstream (needs this feature)
- [Spec/feature] — [impact if we change interface]

### Existing Code Audit
| File | Relevance | Action |
|------|-----------|--------|
| `mvp/src/file.rs` | [what it does now] | modify / extend / no change |

## Task Breakdown

### Task 1: [Title]
...

### Task 2: [Title]
...

## Risks & Open Questions
1. [Risk/question]
2. [Risk/question]

## Implementation Order
[Recommended sequence: which tasks can parallelize, which are sequential]
```

---

## Integration with Other Skills

- **consult-knowledge**: Run BEFORE this skill to check for prior decisions/patterns
- **feature-designer**: Takes this plan as input for the next SDLC phase
- **surface-gap**: If the spec has ambiguities, surface them here rather than guessing
- **record-architectural-decision**: If the plan makes significant choices, record them

---

## Guidelines

- Plans should be pragmatic, not theoretical. Reference actual file paths and function names.
- Prefer extending existing modules over creating new ones where it makes sense.
- Keep the MVP mindset — plan the simplest path that satisfies the spec.
- Flag scope creep: if the spec asks for more than what's reasonable in one pass, recommend phasing.
- The plan is a living document — it can be updated as implementation proceeds.
