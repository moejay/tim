use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::parts::balls::BallType;
use crate::parts::inclines::InclineType;
use crate::parts::walls::WallType;
use crate::parts::PartId;
use crate::puzzle::*;
use crate::world::World;

// ── RON-serializable level definition ────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelDef {
    pub title: String,
    pub goal: String,
    pub parts: Vec<PlacedPart>,
    pub win_conditions: Vec<WinConditionDef>,
    pub bin: Vec<BinPartDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedPart {
    pub kind: String,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub width: Option<f32>,
    #[serde(default)]
    pub height: Option<f32>,
    #[serde(default)]
    pub props: HashMap<String, f32>,
    /// A label used by win conditions to reference this part.
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WinConditionDef {
    ExitedWorld {
        label: String,
        #[serde(default = "default_edge")]
        edge: String,
    },
    AtPosition {
        label: String,
        region: (f32, f32, f32, f32),
    },
}

fn default_edge() -> String {
    "Any".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinPartDef {
    pub kind: String,
    pub quantity: u32,
}

// ── Part name resolution ─────────────────────────────────────────

fn resolve_part_id(name: &str) -> Option<PartId> {
    match name {
        // Balls
        "BowlingBall" => Some(PartId::Ball(BallType::BowlingBall)),
        "Cannonball" => Some(PartId::Ball(BallType::Cannonball)),
        "Basketball" => Some(PartId::Ball(BallType::Basketball)),
        "SoccerBall" => Some(PartId::Ball(BallType::SoccerBall)),
        "Baseball" => Some(PartId::Ball(BallType::Baseball)),
        "TennisBall" => Some(PartId::Ball(BallType::TennisBall)),
        "SuperBall" => Some(PartId::Ball(BallType::SuperBall)),
        "Pinball" => Some(PartId::Ball(BallType::Pinball)),
        "PoolBall" => Some(PartId::Ball(BallType::PoolBall)),
        "ProgrammableBall" => Some(PartId::Ball(BallType::ProgrammableBall)),
        // Walls
        "BrickWall" => Some(PartId::Wall(WallType::BrickWall)),
        "YellowBrickWall" => Some(PartId::Wall(WallType::YellowBrickWall)),
        "CinderBlockWall" => Some(PartId::Wall(WallType::CinderBlockWall)),
        "GrecoRomanWall" => Some(PartId::Wall(WallType::GrecoRomanWall)),
        "WoodenWall" => Some(PartId::Wall(WallType::WoodenWall)),
        "LogWall" => Some(PartId::Wall(WallType::LogWall)),
        "CautionWall" => Some(PartId::Wall(WallType::CautionWall)),
        "SandWall" => Some(PartId::Wall(WallType::SandWall)),
        "PipeWall" => Some(PartId::Wall(WallType::PipeWall)),
        "CurvedPipeWall" => Some(PartId::Wall(WallType::CurvedPipeWall)),
        "GrassFloor" => Some(PartId::Wall(WallType::GrassFloor)),
        "ScaffoldBarrier" => Some(PartId::Wall(WallType::ScaffoldBarrier)),
        "WoodenBarrier" => Some(PartId::Wall(WallType::WoodenBarrier)),
        "LatticeArchway" => Some(PartId::Wall(WallType::LatticeArchway)),
        "MarbleArchway" => Some(PartId::Wall(WallType::MarbleArchway)),
        // Inclines
        "BrickIncline" => Some(PartId::Incline(InclineType::BrickIncline)),
        "YellowBrickIncline" => Some(PartId::Incline(InclineType::YellowBrickIncline)),
        "GraniteIncline" => Some(PartId::Incline(InclineType::GraniteIncline)),
        _ => None,
    }
}

fn resolve_edge(name: &str) -> WorldEdge {
    match name {
        "Top" => WorldEdge::Top,
        "Bottom" => WorldEdge::Bottom,
        "Left" => WorldEdge::Left,
        "Right" => WorldEdge::Right,
        _ => WorldEdge::Any,
    }
}

// ── Loading ──────────────────────────────────────────────────────

pub fn load_level_from_str(ron_str: &str) -> Result<LevelDef> {
    ron::from_str(ron_str).context("Failed to parse level RON")
}

pub fn load_level_file(path: &Path) -> Result<LevelDef> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read level file: {}", path.display()))?;
    load_level_from_str(&contents)
}

/// Build a (World, Puzzle) from a LevelDef.
pub fn build_from_def(def: &LevelDef) -> Result<(World, Puzzle)> {
    let mut world = World::new();
    let mut label_map: HashMap<String, crate::world::InstanceId> = HashMap::new();

    for placed in &def.parts {
        let part_id = resolve_part_id(&placed.kind)
            .with_context(|| format!("Unknown part kind: {}", placed.kind))?;

        let id = world.spawn_locked(part_id, placed.x, placed.y);

        // Apply size overrides
        if let Some(inst) = world.get_mut(id) {
            if let Some(w) = placed.width {
                inst.props.width = w;
            }
            if let Some(h) = placed.height {
                inst.props.height = h;
            }
            for (key, val) in &placed.props {
                inst.props.values.insert(key.clone(), *val);
            }
        }

        if let Some(label) = &placed.label {
            label_map.insert(label.clone(), id);
        }
    }

    let mut puzzle = Puzzle::new(&def.title, &def.goal);

    for wc in &def.win_conditions {
        match wc {
            WinConditionDef::ExitedWorld { label, edge } => {
                let id = label_map.get(label)
                    .with_context(|| format!("Win condition references unknown label: {}", label))?;
                puzzle.win_conditions.push(WinCondition::ObjectExitedWorld {
                    instance_id: *id,
                    edge: resolve_edge(edge),
                });
            }
            WinConditionDef::AtPosition { label, region } => {
                let id = label_map.get(label)
                    .with_context(|| format!("Win condition references unknown label: {}", label))?;
                puzzle.win_conditions.push(WinCondition::ObjectAtPosition {
                    instance_id: *id,
                    region: *region,
                });
            }
        }
    }

    for bp in &def.bin {
        let part_id = resolve_part_id(&bp.kind)
            .with_context(|| format!("Unknown bin part kind: {}", bp.kind))?;
        puzzle.bin_parts.push(BinPart {
            part_id,
            quantity: bp.quantity,
        });
    }

    Ok((world, puzzle))
}

/// Load all `.ron` level files from a directory, sorted by filename.
pub fn load_levels_from_dir(dir: &Path) -> Result<Vec<LevelDef>> {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .with_context(|| format!("Failed to read levels directory: {}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "ron"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let mut levels = Vec::new();
    for entry in entries {
        let def = load_level_file(&entry.path())?;
        levels.push(def);
    }

    Ok(levels)
}
