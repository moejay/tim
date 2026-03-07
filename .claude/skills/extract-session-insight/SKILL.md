# Extract Session Insight

## Purpose
Capture learning signals that emerge from *how work happens during a session*, not from the code itself. While `extract-pattern` looks at code for reusable patterns, this skill reflects on the session to capture friction, corrections, and domain knowledge that would otherwise be lost.

## When to Use
- You had to search multiple places (3+) before finding what you needed
- The user corrected your approach or redirected the task
- You learned domain-specific information that isn't obvious from the code
- You made an assumption that turned out to be wrong
- Something was harder to find or do than it should have been

## Boundary with Other Skills
- **extract-pattern** = "what's in the code" (code-level observations)
- **extract-session-insight** = "what happened during the session" (session-level observations)
- **surface-gap** = "what's blocking me right now" (prospective)
- **update-skill** = "fix the instructions" (prescriptive)

This skill is **retrospective** — it captures what already happened so it doesn't happen again.

## Insight Types

### 1. Discovery Friction
Something was hard to find. The codebase made you search, retry, or ask multiple questions.

**Signals:**
- Searched 3+ files/directories to locate something
- Had to ask the user where something lives
- Grep/find returned too many false positives
- Documentation pointed to the wrong location

**Capture:**
- What were you looking for?
- Where did you search? (list the attempts)
- Where was it actually found?
- How to make it easier next time (better naming, index file, CLAUDE.md hint)

**Template:** `{{frictionTemplate}}`

### 2. User Correction
The user redirected your approach. You were doing something wrong or suboptimal.

**Signals:**
- User said "no", "not like that", "we don't do it that way"
- User provided an alternative approach
- User pointed out a convention you missed
- User caught an assumption that was wrong

**Capture:**
- What was the original (wrong) approach?
- What did the user correct it to?
- What rule or convention does this imply?
- Where should this rule be documented? (CLAUDE.md, skill, pattern)

**Template:** `{{correctionTemplate}}`

### 3. Domain Knowledge
You learned something about the project, business, or domain that isn't in the code.

**Signals:**
- User explained business logic or domain terminology
- Discovered an implicit requirement not in any spec
- Learned why something is the way it is (historical context)
- Found out about external dependencies or constraints

**Capture:**
- What was learned?
- Who/what was the source?
- In what context does this knowledge matter?
- Where should it be documented?

**Template:** `{{domainTemplate}}`

## Process

1. **Recognize the Signal**
   - At session end, or when a notable event happens, check: did friction, correction, or learning occur?

2. **Classify the Insight**
   - Discovery Friction → something was hard to find
   - User Correction → approach was redirected
   - Domain Knowledge → learned something new about the domain

3. **Capture Using Template**
   - Use the appropriate template
   - Be specific: include file paths, search queries, exact corrections
   - Focus on actionable prevention, not just recording what happened

4. **Decide Where to Act**
   - **Immediate:** Add a CLAUDE.md hint, rename a file, add an index
   - **Pattern file:** Write to `{{insightsDir}}/[type]/` for future reference
   - **Skill update:** If the insight reveals a skill gap, trigger `update-skill`

5. **Create Insight File**
   - Location: `{{insightsDir}}/[friction|corrections|domain]/[descriptive-name].md`
   - Use the appropriate template

## Integration with Other Skills
- **extract-pattern:** Session insights may reveal code patterns worth extracting
- **update-skill:** Corrections often mean a skill's instructions need updating
- **surface-gap:** Friction may indicate a gap that should be surfaced
- **record-architectural-decision:** Domain knowledge may inform architectural decisions

## Success Metrics
- Reduced time searching for things across sessions
- Fewer user corrections over time (same mistakes not repeated)
- Growing domain knowledge base that accelerates future work
- CLAUDE.md and project docs get richer from real friction data
