//! Target value field - "What's worth attacking?"
//!
//! Higher value = more attractive target
//! Used to decide WHERE to go, not HOW to get there.

use bevy::prelude::*;

/// Target value field resource
#[derive(Resource, Default)]
pub struct TargetField {
    /// Value of each target position
    pub values: hashbrown::HashMap<IVec3, u32>,
}

/// Target value constants
pub mod target_values {
    pub const KEEP: u32 = 100;
    pub const BARRACKS: u32 = 40;
    pub const ARMORY: u32 = 30;
    pub const WALL_DEFENDER: u32 = 60;
    pub const BREACH_POINT: u32 = 80;
    pub const DAMAGED_WALL: u32 = 50;
    pub const GATE: u32 = 45;
}

impl TargetField {
    /// Get target value for a position
    pub fn value(&self, pos: IVec3) -> u32 {
        *self.values.get(&pos).unwrap_or(&0)
    }

    /// Set target value for a position
    pub fn set_value(&mut self, pos: IVec3, value: u32) {
        if value > 0 {
            self.values.insert(pos, value);
        } else {
            self.values.remove(&pos);
        }
    }

    /// Find highest value target within range
    pub fn highest_value_target(&self, from: IVec3, max_range: i32) -> Option<(IVec3, u32)> {
        self.values
            .iter()
            .filter(|(pos, _)| {
                let diff = **pos - from;
                diff.x.abs() <= max_range && diff.y.abs() <= max_range && diff.z.abs() <= max_range
            })
            .max_by_key(|(_, value)| *value)
            .map(|(pos, value)| (*pos, *value))
    }
}

/// System to update target field based on structures and units
pub fn update_target_field(
    _field: ResMut<TargetField>,
    // TODO: Query player structures and units to populate values
) {
    // This will be populated by:
    // - Player structures (Keep, Barracks, etc.)
    // - Damaged walls (higher value for more damaged)
    // - Wall defenders (units on walls)
    // - Breach points (see breach.rs)
}
