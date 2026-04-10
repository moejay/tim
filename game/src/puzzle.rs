use crate::parts::PartId;
use crate::world::{InstanceId, World};

/// A part available in the player's parts bin.
#[derive(Debug, Clone)]
pub struct BinPart {
    pub part_id: PartId,
    pub quantity: u32,
}

/// Win condition types for a puzzle.
#[derive(Debug, Clone)]
pub enum WinCondition {
    /// A specific instance must be within a rectangular region.
    ObjectAtPosition {
        instance_id: InstanceId,
        region: (f32, f32, f32, f32), // (x1, y1, x2, y2)
    },
    /// A specific instance must have exited the world bounds.
    ObjectExitedWorld {
        instance_id: InstanceId,
        edge: WorldEdge,
    },
    /// A specific instance must be in a given state.
    ObjectInState {
        instance_id: InstanceId,
        state_index: usize,
    },
    /// All conditions must be met.
    AllOf(Vec<WinCondition>),
    /// Any condition must be met.
    AnyOf(Vec<WinCondition>),
}

/// Which edge an object exited from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldEdge {
    Top,
    Bottom,
    Left,
    Right,
    Any,
}

/// Result of evaluating win conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PuzzleStatus {
    /// Simulation not started or in progress.
    Playing,
    /// All win conditions met.
    Won,
}

/// A complete puzzle definition.
pub struct Puzzle {
    pub title: String,
    pub goal_text: String,
    pub win_conditions: Vec<WinCondition>,
    pub bin_parts: Vec<BinPart>,
}

impl Puzzle {
    pub fn new(title: &str, goal_text: &str) -> Self {
        Self {
            title: title.to_string(),
            goal_text: goal_text.to_string(),
            win_conditions: Vec::new(),
            bin_parts: Vec::new(),
        }
    }
}

/// Evaluate win conditions against current world state.
pub fn evaluate(world: &World, conditions: &[WinCondition]) -> PuzzleStatus {
    if conditions.is_empty() {
        return PuzzleStatus::Playing;
    }
    for cond in conditions {
        if !check_condition(world, cond) {
            return PuzzleStatus::Playing;
        }
    }
    PuzzleStatus::Won
}

fn check_condition(world: &World, cond: &WinCondition) -> bool {
    match cond {
        WinCondition::ObjectAtPosition { instance_id, region } => {
            if let Some(inst) = world.get(*instance_id) {
                let cx = inst.x + inst.props.width / 2.0;
                let cy = inst.y + inst.props.height / 2.0;
                // Must be at rest (vy near zero)
                let at_rest = inst.vy.abs() < 2.0 && inst.vx.abs() < 2.0;
                cx >= region.0 && cx <= region.2 && cy >= region.1 && cy <= region.3 && at_rest
            } else {
                false
            }
        }
        WinCondition::ObjectExitedWorld { instance_id, edge } => {
            if let Some(inst) = world.get(*instance_id) {
                let canvas_w = crate::constants::CANVAS_W as f32;
                let canvas_h = crate::constants::CANVAS_H as f32;
                match edge {
                    WorldEdge::Bottom => inst.y > canvas_h,
                    WorldEdge::Top => inst.y + inst.props.height < 0.0,
                    WorldEdge::Left => inst.x + inst.props.width < 0.0,
                    WorldEdge::Right => inst.x > canvas_w,
                    WorldEdge::Any => {
                        inst.y > canvas_h
                            || inst.y + inst.props.height < 0.0
                            || inst.x + inst.props.width < 0.0
                            || inst.x > canvas_w
                    }
                }
            } else {
                // Instance removed = treat as exited
                true
            }
        }
        WinCondition::ObjectInState { instance_id, state_index } => {
            if let Some(inst) = world.get(*instance_id) {
                inst.props.current_state == *state_index
            } else {
                false
            }
        }
        WinCondition::AllOf(conditions) => conditions.iter().all(|c| check_condition(world, c)),
        WinCondition::AnyOf(conditions) => conditions.iter().any(|c| check_condition(world, c)),
    }
}
