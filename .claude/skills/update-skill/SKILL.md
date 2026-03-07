# Update Skill

## Purpose
Enable agents to modify their own skills based on learned patterns, mistakes, and new knowledge. Create a feedback loop where experience improves future performance.

## When to Use
- After discovering a better approach than what's documented
- When a skill's instructions are incomplete or unclear
- After making a mistake that could be prevented
- When learning project-specific conventions
- When patterns suggest skill improvements

## Process

1. **Identify Improvement Trigger**
   - **Mistake:** "I should have checked X first"
   - **Discovery:** "A better way is Y"
   - **Gap:** "The skill didn't mention Z"
   - **Refinement:** "This step should be more specific"
   - **Pattern:** "This happens often enough to add to the skill"

2. **Determine Target Skill**
   - Which skill is this relevant to?
   - Is this a new skill or modification of existing?
   - Does this apply to one project or universally?

3. **Formulate the Change**
   - **Add:** New step, tool, pattern, example
   - **Modify:** Clarify existing instruction, update process
   - **Remove:** Obsolete step, incorrect guidance
   - **Reorder:** Improve logical flow

4. **Scope the Update**
   - **Universal:** Applies to all projects (update base skill)
   - **Project-specific:** Only for this codebase (add to project skill variant)
   - **Temporary:** Experimental, might revert (add as "experimental" section)

5. **Apply the Update**
   ```bash
   # For universal skills (in ~/.agent-evolver/skills or agent-specific dir)
   edit {{skillsDir}}/[skill-name]/SKILL.md
   
   # For project-specific extensions
   edit {{projectRoot}}/.agent-evolver/skills/[skill-name]/SKILL.md
   ```

6. **Log the Change**
   - Append to `{{changesLog}}`
   - Format: `[DATE] [SKILL] [TYPE] - [DESCRIPTION]`
   - Example: `2026-02-16 record-decision ADD - Include git commit reference section`

7. **Validate the Update**
   - Does the change make the skill clearer?
   - Would past-you have avoided a mistake with this update?
   - Is the language specific and actionable?
   - Does it conflict with other skills?

## Update Types

### Add New Step
When you discover a necessary step that wasn't documented.

**Before:**
```markdown
1. Create the file
2. Commit the changes
```

**After:**
```markdown
1. Create the file
2. **Run precommit checks** (format → compile → test)
3. Commit the changes
```

### Add Example
When an example would clarify abstract instructions.

**Before:**
```markdown
Use descriptive variable names.
```

**After:**
```markdown
Use descriptive variable names.

**Examples:**
- ❌ `d` → ✅ `createdAt`
- ❌ `arr` → ✅ `activeUsers`
```

### Add Anti-pattern
When you made a mistake worth preventing.

**After the mistake:**
```markdown
## Anti-patterns
- ❌ **Don't use `rm`** - Use `trash` instead (recoverable)
- ❌ **Don't merge without CI** - Wait for all checks to pass
```

### Add Context/Rationale
When you needed to understand "why" to do it right.

**Before:**
```markdown
Use advisory locks for fee cap updates.
```

**After:**
```markdown
Use advisory locks for fee cap updates.

**Why:** Without locks, concurrent transactions can race and exceed the cap.
**Example:** Two $30 charges at the same time against a $50 cap = $60 (overage).
```

### Refine Vague Instruction
When "do X" wasn't clear enough in practice.

**Before:**
```markdown
Test thoroughly before committing.
```

**After:**
```markdown
Run precommit checks before every commit:
1. `mix format` - Auto-format code
2. `mix compile --warnings-as-errors` - Catch compilation issues
3. `mix test [relevant_tests]` - Verify functionality
```

## Project-Specific Skills

When a skill needs project-specific extensions:

1. **Create project skill overlay:**
   ```
   {{projectRoot}}/.agent-evolver/skills/[skill-name]/SKILL.md
   ```

2. **Reference base skill:**
   ```markdown
   # [Skill Name] - Project Extensions
   
   **Base skill:** ~/.agent-evolver/skills/[skill-name]/SKILL.md
   
   ## Project-Specific Additions
   [additions here]
   ```

3. **Agent loads both:** Base skill + project overlay

## Change Log Format

```
YYYY-MM-DD [SKILL-NAME] [ADD|MODIFY|REMOVE] - Description
```

**Examples:**
```
2026-02-16 record-decision ADD - Git commit reference section
2026-02-16 extract-pattern MODIFY - Clarified confidence levels
2026-02-16 precommit-checks REMOVE - Obsolete linting step
2026-02-15 update-skill ADD - Project-specific skill variant process
```

## Integration Points
- **extract-pattern:** Patterns often become skill updates
- **record-architectural-decision:** Decisions inform skill changes
- **surface-gap:** Gaps become new skills or skill extensions

## Safety Rails

### Always Ask Before:
- Removing entire sections (might be used elsewhere)
- Changing core logic of widely-used skills
- Updates that contradict existing patterns

### Safe to Do Freely:
- Adding examples
- Clarifying vague instructions
- Adding anti-patterns from mistakes
- Expanding "when to use" sections
- Adding project-specific overlays

## Success Metrics
- Skills become more specific over time
- Fewer repeated mistakes
- Faster execution of familiar tasks
- Skills reflect actual workflow, not ideal workflow
- New agents onboard faster with improved skills
