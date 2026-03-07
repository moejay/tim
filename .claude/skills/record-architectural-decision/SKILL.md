# Record Architectural Decision

## Purpose
Capture significant architectural decisions, design choices, and their rationale in a structured format that persists across sessions.

## When to Use
- After making a significant architectural choice
- When choosing between multiple approaches
- When establishing patterns or conventions
- When learning hard lessons that shouldn't be repeated

## Process

1. **Detect Decision Context**
   - New pattern established
   - Major library/framework choice
   - Database schema decision
   - API design choice
   - Security/performance tradeoff

2. **Extract Key Information**
   - What decision was made?
   - What alternatives were considered?
   - Why this choice over others?
   - What are the tradeoffs?
   - What constraints influenced the decision?

3. **Create Decision Record**
   - Use template at `{{docsDir}}/YYYY-MM-DD-title.md`
   - Include date, context, decision, rationale, consequences
   - Link to relevant code/issues/PRs if available

4. **Update Index**
   - Add entry to `{{docsDir}}/INDEX.md`
   - Tag with relevant categories (database, api, security, performance, etc.)

## Template Structure

```markdown
# [Short Decision Title]

**Date:** YYYY-MM-DD
**Status:** Accepted | Deprecated | Superseded
**Tags:** database, api, security, performance, tooling

## Context
What is the issue we're trying to solve? What constraints exist?

## Decision
What did we decide to do?

## Alternatives Considered
What other options did we look at?

## Rationale
Why this choice over the alternatives?

## Consequences
- **Positive:** Benefits of this decision
- **Negative:** Costs or limitations we accept
- **Neutral:** Other impacts

## Related Decisions
Links to other decision records that relate to this one.
```

## Integration Points
- Git commit messages referencing decision files
- Code comments linking to decision records
- PR descriptions citing relevant decisions
- README sections pointing to key decisions

## Maintenance
- Review decision records quarterly
- Update status when decisions are superseded
- Archive deprecated decisions but keep history
- Refactor records if context changes significantly

## Success Metrics
- Future agents can understand "why" not just "what"
- Avoid repeating failed approaches
- Faster onboarding for new contributors
- Better context for code reviews
