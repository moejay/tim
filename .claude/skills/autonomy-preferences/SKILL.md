# Skill: autonomy-preferences

**Purpose:** Determine how autonomous the agent should be when making decisions. This skill manages user preferences for when the agent should ask vs. act autonomously, focusing on high-level decision categories.

---

## When to Run This

Run this skill in these situations:

1. **No preferences file exists** - Run onboarding to ask the user their preferences
2. **About to ask for approval** - Check if the user wants to be asked for this category
3. **User explicitly mentions autonomy** - Update or view current preferences

---

## Decision Categories

| Category | Description | Examples |
|----------|-------------|----------|
| **Architectural** | System design, component boundaries, data flow | Database schema, service boundaries, API architecture |
| **Design** | Interface contracts, naming, structure | API design, type definitions, module organization |
| **Technical** | Implementation approaches, library choices | Framework selection, algorithms, performance trade-offs |
| **UI/UX** | User interface and experience decisions | Layout, styling, interactions, accessibility |
| **Process** | Workflow, git, releases | Branching strategy, commit conventions, release decisions |

---

## Modes

| Mode | Behavior |
|------|----------|
| **ask** | Always ask user for approval before proceeding |
| **suggest-then-proceed** | Show the decision, then proceed without waiting for approval |
| **autonomous** | Make reasonable decisions without asking; user will correct if needed |

---

## Steps

### Step 1: Check if Preferences File Exists

Look for `.agent-evolver/autonomy-preferences.yaml` in the project root.

- **If it exists** → Go to Step 3 (Consult Preferences)
- **If it doesn't exist** → Go to Step 2 (Onboarding)

### Step 2: Onboarding (First Time Setup)

If no preferences file exists, ask the user to set their preferences.

**Use AskUserQuestion with these options:**

```
Question: "How autonomous should I be when making decisions?"

Options:
1. "Ask me first" - I want to approve architectural, design, and technical decisions
2. "Suggest then proceed" - Show me what you're thinking, but don't wait for approval
3. "Fully autonomous" - Make reasonable decisions, I'll correct you if needed
4. "Let me customize" - Set different levels per category
```

**After user responds:**

1. Create `.agent-evolver/` directory if it doesn't exist
2. Create `autonomy-preferences.yaml` using the template below
3. Populate based on user's choice:
   - "Ask me first" → `default_mode: "ask"`
   - "Suggest then proceed" → `default_mode: "suggest-then-proceed"`
   - "Fully autonomous" → `default_mode: "autonomous"`
   - "Let me customize" → Ask follow-up questions per category

### Step 3: Consult Preferences Before Acting

When about to ask for approval or make a decision:

1. Read `.agent-evolver/autonomy-preferences.yaml`
2. Determine which category the decision falls into
3. Check the mode for that category (or use `default_mode` if category is null)
4. Act accordingly:
   - **ask** → Ask the user for approval
   - **suggest-then-proceed** → State what you're doing and proceed
   - **autonomous** → Just do it

### Step 4: Update Preferences (Optional)

If user explicitly requests a change to autonomy settings:

1. Read current preferences
2. Update the relevant field
3. Write back to file
4. Confirm the change to user

---

## Preference File Structure

Location: `.agent-evolver/autonomy-preferences.yaml`

```yaml
version: 1.0.0
created: "YYYY-MM-DD"

# Overall preference (applies unless overridden per-category)
default_mode: "ask"  # ask | suggest-then-proceed | autonomous

# Category-specific overrides (null = use default_mode)
categories:
  architectural: null
  design: null
  technical: null
  process: null

# User's reasoning (captured during onboarding)
reasoning: ""
```

---

## Integration with Other Skills

**consult-knowledge** should:
- Check autonomy preferences to determine how to present decisions

**record-architectural-decision** should:
- Respect autonomy preferences when deciding whether to ask before recording

---

## Notes

- Default to **ask** mode if anything is unclear
- User can always say "just do it" to override for a single decision
- User can always say "wait, ask me first" to override for a single decision
- Preferences persist across sessions via the YAML file
