use std::collections::HashMap;

use crate::parts::{PartDef, PartId, PartProps};

/// A unique identifier for a placed part instance in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceId(pub u32);

/// A part placed in the world with position, velocity, and runtime state.
#[derive(Debug, Clone)]
pub struct PartInstance {
    pub id: InstanceId,
    pub part_id: PartId,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub props: PartProps,
    /// Whether this part is locked (fixed in puzzle, not movable by player).
    pub locked: bool,
}

impl PartInstance {
    pub fn new(id: InstanceId, part_id: PartId, x: f32, y: f32) -> Self {
        let def = part_id.part_def();
        let (w, h) = def.default_size();
        let mut values = HashMap::new();
        for prop in def.properties() {
            values.insert(prop.name.clone(), prop.default);
        }
        Self {
            id,
            part_id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            props: PartProps {
                flipped: false,
                width: w,
                height: h,
                values,
                current_state: 0,
            },
            locked: false,
        }
    }

    pub fn def(&self) -> &dyn PartDef {
        self.part_id.part_def()
    }

    /// AABB bounding box: (left, top, right, bottom).
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.x + self.props.width, self.y + self.props.height)
    }
}

/// The game world: all placed parts and global settings.
pub struct World {
    pub instances: Vec<PartInstance>,
    next_id: u32,
    pub gravity: f32,
    pub pressure: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            next_id: 1,
            gravity: 1.0,
            pressure: 1.0,
        }
    }

    pub fn spawn(&mut self, part_id: PartId, x: f32, y: f32) -> InstanceId {
        let id = InstanceId(self.next_id);
        self.next_id += 1;
        self.instances.push(PartInstance::new(id, part_id, x, y));
        id
    }

    pub fn spawn_locked(&mut self, part_id: PartId, x: f32, y: f32) -> InstanceId {
        let id = self.spawn(part_id, x, y);
        if let Some(inst) = self.get_mut(id) {
            inst.locked = true;
        }
        id
    }

    pub fn get(&self, id: InstanceId) -> Option<&PartInstance> {
        self.instances.iter().find(|i| i.id == id)
    }

    pub fn get_mut(&mut self, id: InstanceId) -> Option<&mut PartInstance> {
        self.instances.iter_mut().find(|i| i.id == id)
    }

    pub fn remove(&mut self, id: InstanceId) -> Option<PartInstance> {
        if let Some(pos) = self.instances.iter().position(|i| i.id == id) {
            Some(self.instances.remove(pos))
        } else {
            None
        }
    }

    /// Snapshot all instance positions/velocities/states for sim reset.
    pub fn snapshot(&self) -> Vec<InstanceSnapshot> {
        self.instances
            .iter()
            .map(|inst| InstanceSnapshot {
                id: inst.id,
                x: inst.x,
                y: inst.y,
                vx: inst.vx,
                vy: inst.vy,
                state: inst.props.current_state,
                flipped: inst.props.flipped,
            })
            .collect()
    }

    /// Restore all instances from a snapshot.
    pub fn restore(&mut self, snapshots: &[InstanceSnapshot]) {
        for snap in snapshots {
            if let Some(inst) = self.get_mut(snap.id) {
                inst.x = snap.x;
                inst.y = snap.y;
                inst.vx = snap.vx;
                inst.vy = snap.vy;
                inst.props.current_state = snap.state;
                inst.props.flipped = snap.flipped;
            }
        }
    }

    /// Check if an instance has exited the visible world bounds.
    pub fn is_off_screen(&self, id: InstanceId) -> bool {
        if let Some(inst) = self.get(id) {
            let cw = crate::constants::CANVAS_W as f32;
            let ch = crate::constants::CANVAS_H as f32;
            inst.x + inst.props.width < -50.0
                || inst.x > cw + 50.0
                || inst.y + inst.props.height < -50.0
                || inst.y > ch + 50.0
        } else {
            true // removed = off screen
        }
    }
}

/// Saved position/state for simulation reset.
#[derive(Debug, Clone)]
pub struct InstanceSnapshot {
    pub id: InstanceId,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub state: usize,
    pub flipped: bool,
}
