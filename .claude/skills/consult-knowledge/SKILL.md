# Skill: consult-knowledge

**Purpose:** Before planning or implementing anything, search existing project knowledge so prior decisions and patterns inform your approach. This is the retrieval half of the self-evolving loop.

---

## When to Run This

Run this skill BEFORE making any plan or writing any code for a non-trivial task. If you skip this step, you risk:
- Contradicting a prior architectural decision
- Reimplementing a pattern that already exists
- Making a mistake that was already documented as a known gap
- Inconsistency with the codebase's established conventions

---

## Steps

### 1. Extract Keywords From the Task

Identify 3–5 keywords or concepts from the current task. Think about:
- The system/module being touched
- The type of operation (auth, data, rendering, API, etc.)
- Any specific patterns or abstractions involved

### 2. Search Existing Decisions (`docs/decisions/`)

Read the directory listing of `docs/decisions/`. For any file whose name contains a matching keyword:
- Read the file
- Note: the Decision, Rationale, and Consequences sections
- Flag if this decision directly applies to the current task

If the directory doesn't exist yet, note that and continue.

### 3. Search Existing Patterns (`docs/patterns/`)

Read the directory listing of `docs/patterns/` (may have subdirectories by category). For any file that matches:
- Read the file
- Note: the pattern name, when to use it, and any examples
- Flag if this pattern should be applied to the current implementation

If the directory doesn't exist yet, note that and continue.

### 4. Check Known Gaps (`.agent-evolver/knowledge-gaps.md`)

Read the knowledge gaps file if it exists. Identify any gaps that:
- Are Blocking or Slowing severity
- Relate to the current task's domain
- Have not been resolved since they were logged

Flag unresolved blocking gaps before proceeding.

### 5. Check Autonomy Preferences (`.agent-evolver/autonomy-preferences.yaml`)

Read the autonomy preferences file if it exists. This tells you how the user wants you to behave when making decisions:

- **ask** → Ask for approval before architectural/design/technical decisions
- **suggest-then-proceed** → State your decision and proceed without waiting
- **autonomous** → Make reasonable decisions without asking

If the file doesn't exist, you should run the `autonomy-preferences` skill to onboard the user.

### 6. Produce a Context Summary

Using the template below, summarize what you found. Be concise — this is an internal planning artifact, not a user-facing document.

Then **use this summary to shape your plan**:
- If a decision applies → follow it (or explicitly decide to revisit it)
- If a pattern applies → use it (or note why you're deviating)
- If a blocking gap applies → surface it to the user before proceeding

---

## Template

Use `templates/context-summary.md` or write inline:

```markdown
## Knowledge Context: [Task Name]

**Task keywords:** keyword1, keyword2, keyword3

### Relevant Decisions
- [decision-file.md] — [one-line summary of what it decided and why it matters here]
- None found

### Relevant Patterns
- [pattern-file.md] — [one-line summary of the pattern and how it applies]
- None found

### Open Knowledge Gaps
- [gap description] — Severity: Blocking/Slowing/Minor — [still open? resolved?]
- None found

### How This Shapes My Plan
[1–3 sentences on how the above should influence the implementation approach]
```

---

## Notes

- If all three sources are empty (no docs yet), note that and proceed — the project is young
- Don't block on missing docs. The point is to use what exists, not to manufacture what doesn't
- If you find a decision that conflicts with what the user asked for, flag it before writing code — don't silently override it
- If you find a pattern that applies, use it without being asked
- The context summary can be brief or skipped entirely if nothing relevant was found — the check is what matters, not the paperwork
