//! Traversal cost field - "How hard is it to get there?"
//!
//! Key insight: Traversal cost = tile HP
//! Damaged walls have lower cost, naturally attracting more ants.

use crate::world::{GameWorld, TileChangedEvent};
use bevy::prelude::*;

/// Traversal cost field resource
#[derive(Resource, Default)]
pub struct TraversalField {
    /// Cost to traverse each tile (position -> cost)
    pub costs: hashbrown::HashMap<IVec3, u32>,
    /// Direction to flow toward goal from each tile
    pub flow_directions: hashbrown::HashMap<IVec3, IVec3>,
    /// Whether the field needs recalculation
    pub dirty: bool,
}

impl TraversalField {
    /// Get traversal cost for a position
    pub fn cost(&self, pos: IVec3) -> u32 {
        *self.costs.get(&pos).unwrap_or(&u32::MAX)
    }

    /// Get the flow direction from a position (normalized direction toward goal)
    pub fn flow_direction(&self, pos: IVec3) -> Option<IVec3> {
        self.flow_directions.get(&pos).copied()
    }

    /// Mark field as needing recalculation
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

/// System to update traversal field when tiles change
pub fn update_traversal_field(
    mut field: ResMut<TraversalField>,
    world: Res<GameWorld>,
    mut tile_events: EventReader<TileChangedEvent>,
) {
    // Mark dirty if any tiles changed
    for event in tile_events.read() {
        // Only recalculate if the cost changed
        let old_cost = event.old_tile.traversal_cost();
        let new_cost = event.new_tile.traversal_cost();
        if old_cost != new_cost {
            field.dirty = true;
            // Update the single tile cost immediately
            field.costs.insert(event.position, new_cost);
        }
    }

    // Full recalculation if dirty
    // TODO: Implement proper flow field algorithm (Dijkstra from goals)
    if field.dirty {
        recalculate_flow_field(&mut field, &world);
        field.dirty = false;
    }
}

/// Recalculate the entire flow field using Dijkstra from goals
fn recalculate_flow_field(field: &mut TraversalField, world: &GameWorld) {
    // TODO: Implement proper flow field calculation
    // 1. Find all goal positions (player structures, breach points)
    // 2. Dijkstra outward from goals
    // 3. Store direction toward lowest-cost neighbor at each tile

    // For now, just update costs from world
    field.costs.clear();
    for (chunk_pos, chunk) in world.chunks.iter() {
        for (local_pos, tile) in chunk.iter_tiles() {
            let world_pos = *chunk_pos * 16 + local_pos.as_ivec3();
            field.costs.insert(world_pos, tile.traversal_cost());
        }
    }
}
