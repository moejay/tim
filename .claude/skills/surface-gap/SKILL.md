# Surface Gap

## Purpose
Recognize and document situations where existing skills don't provide adequate guidance. Make uncertainty visible so it can be addressed through new skills or skill improvements.

## Core Principle
**Uncertainty is data.** When you don't know what to do, that's valuable information about what needs to be learned or documented.

## When to Use
- Encountering an unfamiliar problem
- Existing skills are vague or incomplete
- Multiple approaches seem equally valid
- Project-specific context is missing
- You're about to guess or make assumptions

## Gap Types

### 1. Missing Skill
No skill addresses this situation at all.

**Example:**
- "How do I handle database migrations in this project?"
- "What's the code review process here?"
- "How should I structure test fixtures?"

### 2. Incomplete Skill
A skill exists but lacks critical detail.

**Example:**
- Skill says "test thoroughly" but doesn't specify what to test
- Skill mentions "consider security" but doesn't say how
- Skill lists steps but missing context for when to use them

### 3. Ambiguous Guidance
Multiple skills provide conflicting or unclear advice.

**Example:**
- One skill says "ask before external actions," another says "be autonomous"
- Unclear whether to prioritize speed vs thoroughness
- Conflicting naming conventions

### 4. Missing Context
Project-specific information not captured anywhere.

**Example:**
- Which database is used? What ORM?
- What's the branching strategy?
- Who reviews PRs? How long to wait?
- What's the deployment process?

### 5. Edge Cases
Common scenario not covered by existing skills.

**Example:**
- Skill covers happy path but not error handling
- Instructions for new projects, but not ongoing maintenance
- Process for simple changes, but not complex refactors

## Process

### 1. Recognize the Gap
When you think:
- "I'm not sure..."
- "Should I... or should I...?"
- "What's the right way to...?"
- "I don't have enough information to..."

**Stop and acknowledge:** This is a gap.

### 2. Characterize the Gap
- **Type:** Missing skill, incomplete, ambiguous, context, edge case
- **Severity:** Blocking, slowing, minor
- **Scope:** Universal or project-specific
- **Frequency:** How often will this come up?

### 3. Document the Gap
Add to `{{gapsLog}}`:

```markdown
## [Date] - [Gap Title]

**Type:** [missing-skill|incomplete|ambiguous|context|edge-case]
**Severity:** [blocking|slowing|minor]
**Scope:** [universal|project-specific]

### Situation
[What were you trying to do?]

### Current Guidance
[What do existing skills say, if anything?]

### What's Missing
[What information/guidance would have helped?]

### Temporary Resolution
[What did you decide to do? Why?]

### Proposed Action
[New skill? Update existing? Ask human?]
```

### 4. Take Immediate Action
Based on severity:

**Blocking:**
1. Document the gap
2. Ask human for guidance
3. Once resolved, create/update skill

**Slowing:**
1. Document the gap
2. Make best judgment based on similar situations
3. Note decision and reasoning
4. Propose skill update

**Minor:**
1. Document the gap
2. Proceed with reasonable default
3. Batch with other minor gaps for later review

### 5. Follow Up
- After resolution, create new skill or update existing
- Link gap log entry to the resulting skill change
- Mark gap as "resolved" with reference to solution

## Gap Log Format

```markdown
# Knowledge Gaps

## Resolved
[Gaps that led to skills or updates]

## Active
[Current gaps awaiting resolution]

## Tracking
[Low-priority gaps to watch for patterns]
```

## Example Gap Entry

```markdown
## 2026-02-16 - Database Transaction Testing

**Type:** incomplete
**Severity:** slowing
**Scope:** project-specific

### Situation
Writing integration tests for purchase flow. Unclear how to handle database transactions in tests vs production.

### Current Guidance
"Test thoroughly" - but no guidance on test database isolation, fixtures, or async task handling.

### What's Missing
- How to set up test database
- Fixture patterns for complex objects (products, accounts, transactions)
- How to test async email delivery without actually sending
- Whether to use database sandboxing or transactions in tests

### Temporary Resolution
Examined existing tests. Found pattern:
- Tests use `async: true` by default (database sandbox)
- Async tasks (like email) must use `async: false` to share connection
- Fixtures created via `Fixtures` modules in `test/support/fixtures/`

### Proposed Action
Create project-specific skill: "integration-testing-patterns"
- Document test database setup
- Explain async/sync test modes
- Show fixture patterns
- Cover common pitfalls (sandbox + async tasks)

### Resolution
✅ 2026-02-16 - Created `test-patterns/integration-tests.md`
```

## Integration Points
- **record-architectural-decision:** Decisions fill context gaps
- **extract-pattern:** Patterns reveal missing or unclear guidance
- **update-skill:** Gaps drive skill improvements

## Red Flags (When to Surface Gaps Immediately)

### Security Uncertainty
- Handling credentials or secrets
- Authentication/authorization decisions
- Data privacy concerns
- API key management

### Irreversible Actions
- Deleting production data
- Publishing public content
- Sending external communications
- Financial transactions

### External Impact
- Actions visible to users
- Changes affecting other people's work
- Public-facing modifications

## Success Metrics
- Fewer moments of uncertainty over time
- Gaps lead to actionable improvements
- Repeated gaps get resolved (not just documented)
- Skills become more comprehensive
- Faster execution as gaps fill in

## Anti-patterns

❌ **Don't:**
- Guess when you should ask
- Proceed silently when uncertain
- Document gap but not follow up
- Treat every minor uncertainty as blocking

✅ **Do:**
- Surface uncertainty early
- Characterize severity accurately
- Propose concrete solutions
- Track patterns across gaps
- Close the loop (gap → resolution → skill)
