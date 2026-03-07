# Extract Pattern

## Purpose
Recognize recurring patterns in code, problems, or solutions and capture them in a reusable format. Build a pattern library that grows smarter over time.

## When to Use
- Solving a problem you've encountered before
- Noticing similar code structures across modules
- Applying the same solution strategy repeatedly
- Finding bugs that follow a pattern
- Implementing features with similar architecture
- Finding code that diverges from documented conventions
- Noticing tool or framework-specific syntax/config worth remembering
- Performing a multi-step sequence (3+) that forms a repeatable workflow

## Recognition Triggers
Look for phrases like:
- "This is similar to..."
- "We did this before when..."
- "Same issue as..."
- "Like the [X] module but..."
- "Following the pattern from..."

## Process

1. **Identify the Pattern**
   - What is the recurring element?
   - Where have you seen it? (2+ instances minimum)
   - What makes it a pattern vs one-off solution?

2. **Classify Pattern Type**
   - **Code Pattern:** Architectural structure, idiom, algorithm
   - **Problem Pattern:** Recurring bug, edge case, failure mode
   - **Solution Pattern:** Debugging approach, fix strategy, workaround
   - **Workflow Pattern:** Development process, testing approach, deployment strategy
   - **Convention Divergence:** Code that breaks from documented conventions — use `{{conventionTemplate}}`
   - **Tool Pattern:** Tool/framework-specific syntax, config, or gotchas — use `{{toolTemplate}}`

3. **Document the Pattern**
   - Clear name (should be immediately recognizable)
   - Context: when does this pattern apply?
   - Structure: what are the key elements?
   - Examples: 2-3 concrete instances from the codebase
   - Variations: when/how the pattern adapts
   - Anti-patterns: what to avoid

4. **Decide Scope**
   - **Local rule** → Add to project's CLAUDE.md or a project-specific skill overlay if the pattern only applies to this project
   - **Repository pattern** → Write to `{{patternsDir}}/` if the pattern is reusable across projects
   - Ask: "Is this specific to this codebase, or would any project using this stack benefit?"

5. **Create Pattern File**
   - Location: `{{patternsDir}}/[category]/[pattern-name].md`
   - Use template structure (see templates: `{{template}}`, `{{conventionTemplate}}`, `{{toolTemplate}}`)
   - Link to real code examples
   - Include before/after if refactoring

6. **Update Pattern Index**
   - Add to `{{patternsDir}}/INDEX.md`
   - Cross-reference related patterns
   - Tag with relevant domains

## Pattern Categories
- `architecture/` - System design patterns
- `code/` - Implementation patterns
- `bugs/` - Common bugs and fixes
- `testing/` - Test patterns
- `performance/` - Optimization patterns
- `security/` - Security patterns
- `data/` - Data modeling patterns
- `conventions/` - Convention divergences and local rules
- `tooling/` - Tool/framework-specific patterns and gotchas
- `workflows/` - Multi-step repeatable workflows

## Template Structure

```markdown
# [Pattern Name]

**Category:** [architecture|code|bugs|testing|performance|security|data]
**Confidence:** High | Medium | Low
**Instances:** [count]

## Context
When does this pattern appear? What problem does it solve?

## Structure
Key elements of the pattern:
- [Element 1]
- [Element 2]

## Examples

### Example 1: [Location/Context]
```[language]
// code snippet
```

### Example 2: [Location/Context]
```[language]
// code snippet
```

## Variations
How does this pattern adapt to different contexts?

## Anti-patterns
What should you avoid?

## Related Patterns
- [Pattern A] - Similar but...
- [Pattern B] - Complements this pattern...
```

## Integration with Other Skills
- **record-architectural-decision:** Patterns often emerge from decisions
- **update-skill:** Patterns become part of skill instructions
- **surface-gap:** Missing patterns indicate knowledge gaps

## Evolution
- Start with Low confidence after 2 instances
- Increase to Medium at 3-4 instances
- Reach High confidence at 5+ instances
- Refine pattern definition as more examples emerge
- Split patterns if variations diverge too much

## Success Metrics
- Faster implementation of similar features
- Reduced debugging time for known issues
- Better code consistency across modules
- Easier onboarding (patterns document "the way we do things")
