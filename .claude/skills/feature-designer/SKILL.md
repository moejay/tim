# Skill: feature-designer

**Purpose:** Produce a technical design document from an implementation plan. Defines types, traits, module boundaries, data flow, and rendering integration. This is the second phase of the SDLC pipeline.

---

## Inputs

- **Plan path**: Path to the implementation plan (e.g., `docs/plans/01-physics-engine.md`)
- **Spec path**: Path to the original spec for reference

---

## Process

### 1. Review Plan & Spec

- Read the implementation plan (output of feature-planner)
- Re-read the original spec for detailed requirements
- Read all existing files listed in the plan's code audit

### 2. Design Data Structures

For each new type or modification to existing types:
- Define the Rust struct/enum with field types and visibility
- Document invariants and constraints
- Show how it integrates with `GameState` (the central state struct)
- Consider serialization needs (for puzzle save/load)

### 3. Design Module Interfaces

For each module (new or modified):
- Public function signatures with full type signatures
- Trait definitions if polymorphism is needed
- Error handling strategy (Result types, when to panic vs. propagate)
- How the module is called from the game loop (`main.rs`)

### 4. Design Rendering Integration

TIM2 has a dual-renderer architecture. For every visual element:
- Define how it renders in PixelRenderer (image crate primitives on 640×360 canvas)
- Define how it renders in TextRenderer (ratatui widgets on terminal cells)
- Specify the `Renderer` trait additions needed
- Consider both renderers as first-class — text mode is not a fallback

### 5. Design Physics Integration

If the feature involves physical objects:
- How it interacts with `update_physics()` in `physics.rs`
- Collision shapes (circle, AABB, line segment, polygon)
- New collision functions needed
- Integration with the existing collision resolution pattern

### 6. Design State Transitions

- How the feature affects game modes (Normal, Place, Edit, Run)
- Input handling additions in `input.rs`
- State changes and undo integration
- Win condition changes if applicable

### 7. Produce the Design Document

Write to `docs/designs/[spec-number]-[feature-name].md`.

---

## Output Template

```markdown
# Technical Design: [Feature Name]

**Plan:** [path to plan]
**Spec:** [path to spec]
**Date:** [YYYY-MM-DD]

## Architecture Overview
[1 paragraph + ASCII diagram showing how this feature fits into the existing architecture]

## Data Structures

### [StructName]
```rust
// Location: mvp/src/[file].rs
#[derive(Clone, Debug)]
pub struct StructName {
    pub field: Type,  // explanation
}
```

**Invariants:**
- [invariant 1]

**Integration with GameState:**
- [how it's stored/accessed]

## Module Interfaces

### [module_name] (`mvp/src/[file].rs`)

```rust
/// [Brief description]
pub fn function_name(params) -> ReturnType {
    // [pseudocode or key logic notes]
}
```

## Rendering

### Pixel Renderer
- [What to draw, using which imageproc/image primitives]
- [Colors, coordinates, layering]

### Text Renderer
- [What Unicode characters/ratatui widgets to use]
- [Layout considerations]

### Renderer Trait Changes
```rust
// Additions to mvp/src/render/mod.rs
fn render_[feature](&mut self, state: &GameState);
```

## Physics Integration
- [Collision shapes and functions]
- [Force interactions]
- [Update order considerations]

## Input & State
- [New keybindings or mode changes]
- [Undo implications]

## File Changes Summary
| File | Change Type | Description |
|------|-------------|-------------|
| `mvp/src/file.rs` | modify | [what changes] |
| `mvp/src/new.rs` | create | [purpose] |
```

---

## Design Principles (TIM2-Specific)

1. **Pixel coordinates are truth**: All physics and placement in 512×360 pixel space. Renderers adapt.
2. **Dual renderer parity**: Every visual element must work in both pixel and text renderers.
3. **Flat architecture**: Prefer `Vec<Part>` and match statements over deep trait hierarchies. Keep it simple until complexity forces otherwise.
4. **Game loop integration**: New features plug into the existing 60fps fixed-timestep loop in `main.rs`. Don't create separate update loops.
5. **PartKind extensibility**: New parts are enum variants on `PartKind`. Each needs: dimensions, icon_char, label, collision, and dual rendering.

---

## Integration with Other Skills

- **feature-planner**: Provides the plan this skill designs from
- **feature-developer**: Takes this design as the implementation blueprint
- **record-architectural-decision**: Design choices made here should be recorded
- **extract-pattern**: If the design follows or creates a reusable pattern, document it
