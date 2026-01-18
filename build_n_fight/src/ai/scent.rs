//! Scent trail system - Scout communication paths
//!
//! Scouts leave scent trails as they explore.
//! Reinforcements follow these trails to discovered locations.
//! Trails decay over time.

use bevy::prelude::*;

/// Collection of scent trails
#[derive(Resource, Default)]
pub struct ScentTrails {
    /// Scent intensity at each position, per nest
    trails: hashbrown::HashMap<IVec3, hashbrown::HashMap<Entity, f32>>,
}

impl ScentTrails {
    /// Add scent at a position for a nest
    pub fn add_scent(&mut self, pos: IVec3, nest: Entity, intensity: f32) {
        self.trails
            .entry(pos)
            .or_default()
            .entry(nest)
            .and_modify(|i| *i = (*i + intensity).min(10.0))
            .or_insert(intensity);
    }

    /// Get scent intensity at a position for a nest
    pub fn scent_at(&self, pos: IVec3, nest: Entity) -> f32 {
        self.trails
            .get(&pos)
            .and_then(|m| m.get(&nest))
            .copied()
            .unwrap_or(0.0)
    }

    /// Find strongest scent direction from a position
    pub fn strongest_direction(&self, pos: IVec3, nest: Entity) -> Option<IVec3> {
        let neighbors = [
            IVec3::new(1, 0, 0),
            IVec3::new(-1, 0, 0),
            IVec3::new(0, 1, 0),
            IVec3::new(0, -1, 0),
            IVec3::new(0, 0, 1),
            IVec3::new(0, 0, -1),
        ];

        neighbors
            .iter()
            .max_by(|a, b| {
                let scent_a = self.scent_at(pos + **a, nest);
                let scent_b = self.scent_at(pos + **b, nest);
                scent_a.partial_cmp(&scent_b).unwrap()
            })
            .filter(|dir| self.scent_at(pos + **dir, nest) > 0.0)
            .copied()
    }

    /// Decay all scents
    pub fn decay(&mut self, decay_rate: f32) {
        for pos_scents in self.trails.values_mut() {
            pos_scents.retain(|_, intensity| {
                *intensity -= decay_rate;
                *intensity > 0.01
            });
        }
        self.trails.retain(|_, m| !m.is_empty());
    }
}

/// System to update scent trails (decay over time)
pub fn update_scent_trails(mut trails: ResMut<ScentTrails>, time: Res<Time>) {
    // Decay scents slowly
    let decay_rate = 0.01 * time.delta_secs();
    trails.decay(decay_rate);
}
