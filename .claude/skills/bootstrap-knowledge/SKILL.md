# Bootstrap Knowledge

## Purpose

Initialize the knowledge base (decisions and patterns) for an existing codebase. This skill analyzes a brownfield project to extract and document architectural decisions and patterns that are already evident in the code.

---

## When to Use

- Starting work on an existing codebase that has no `docs/decisions/` or `docs/patterns/`
- Onboarding to a new project and wanting to understand its architecture
- Before major refactoring to document the current state
- When the team wants to formalize tribal knowledge

---

## Process

### Step 1: Analyze Project Structure

Scan the codebase to understand its shape:

```
- What languages/frameworks are used?
- What's the directory structure? (monorepo, src/, app/, lib/, etc.)
- Are there config files that reveal choices? (package.json, tsconfig, docker, CI/CD)
- What dependencies are installed?
- Is there existing documentation? (README, docs/, wiki)
```

### Step 2: Identify Evident Decisions

Look for decisions that are already made (even if not documented):

| Signal | Likely Decision |
|--------|-----------------|
| `package.json` with React | Frontend framework choice |
| `tsconfig.json` | TypeScript over JavaScript |
| `prisma/` or `drizzle/` | ORM choice |
| `docker-compose.yml` | Containerization strategy |
| `.github/workflows/` | CI/CD platform choice |
| `src/api/` with REST routes | API style (REST vs GraphQL) |
| `tailwind.config.js` | Styling approach |
| `jest.config.js` or `vitest.config.ts` | Testing framework |
| Monorepo structure | Multi-package architecture |

### Step 3: Detect Existing Patterns

Look for recurring structures in the code:

| Pattern Signal | What to Document |
|----------------|------------------|
| Multiple files with same structure | Component/module pattern |
| Consistent naming conventions | Naming pattern |
| Repeated error handling approach | Error handling pattern |
| Common data fetching pattern | Data access pattern |
| Shared utility functions | Helper/utility patterns |
| Test file organization | Testing patterns |

### Step 4: Create Directory Structure

```bash
mkdir -p docs/decisions
mkdir -p docs/patterns
mkdir -p .agent-evolver
```

### Step 5: Generate Initial Records

For each identified decision, create a record using the `record-architectural-decision` template:

```
docs/decisions/
├── INDEX.md
├── 0001-framework-choice.md
├── 0002-database-choice.md
├── 0003-api-style.md
└── ...
```

For each identified pattern, create a record using the `extract-pattern` template:

```
docs/patterns/
├── INDEX.md
├── architecture/
│   └── module-structure.md
├── code/
│   └── error-handling.md
└── ...
```

### Step 6: Create Bootstrap Summary

Write a summary to `.agent-evolver/bootstrap-summary.md`:

```markdown
# Bootstrap Summary

**Date:** YYYY-MM-DD
**Codebase:** [project name]

## Decisions Documented
- [x] Framework: React + TypeScript
- [x] Database: PostgreSQL with Prisma
- [x] API: REST with Express
- [ ] Authentication: (needs investigation)

## Patterns Identified
- [x] Component structure pattern
- [x] API route pattern
- [ ] State management pattern (needs more examples)

## Gaps Identified
- No clear deployment strategy documented
- Testing patterns unclear
- Error handling inconsistent

## Recommended Next Steps
1. Interview team about undocumented decisions
2. Watch for patterns as you work
3. Document decisions as they're made going forward
```

---

## Questions to Ask the User

Before bootstrapping, consider asking:

1. **Are there any major decisions you remember making but didn't document?**
2. **Who else should I talk to about how things work?**
3. **Are there any "we tried X but switched to Y" stories?**
4. **What patterns do you consider "the right way" in this codebase?**

---

## What NOT to Do

- **Don't fabricate rationale** — If you don't know why a decision was made, mark it as "Rationale: Unknown (pre-dates documentation)"
- **Don't document everything** — Focus on significant architectural choices, not every library
- **Don't block on completeness** — Start with what's obvious, refine over time
- **Don't assume patterns are intentional** — Some repetition is accidental, not a pattern

---

## Confidence Levels

Mark bootstrapped records with confidence:

| Confidence | Meaning |
|------------|---------|
| **Evident** | Clear from code/config (e.g., React is used) |
| **Inferred** | Likely based on structure (e.g., appears to follow MVC) |
| **Uncertain** | Needs team confirmation |

---

## Integration with Other Skills

After bootstrapping:

- **consult-knowledge** will now have decisions/patterns to reference
- **record-architectural-decision** should be used for NEW decisions
- **extract-pattern** should be used as new patterns emerge
- **surface-gap** can document areas that need clarification

---

## Success Metrics

- Agent can answer "why is it done this way?" for major architectural choices
- New contributors can onboard faster
- Future decisions can reference prior decisions
- Patterns are followed consistently
