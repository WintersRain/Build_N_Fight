//! Digging system - Remove tiles to create tunnels

use crate::world::{GameWorld, Tile, TileChangedEvent};
use bevy::prelude::*;

/// Dig event
#[derive(Event)]
pub struct DigEvent {
    pub position: IVec3,
    pub digger: Entity,
}

/// Process dig events
pub fn process_dig_events(
    mut events: EventReader<DigEvent>,
    mut world: ResMut<GameWorld>,
    mut tile_events: EventWriter<TileChangedEvent>,
    mut resources: ResMut<super::PlayerResources>,
) {
    for event in events.read() {
        if let Some(tile) = world.get_tile(event.position) {
            if !tile.is_diggable() {
                info!("Cannot dig at {:?} - not diggable", event.position);
                continue;
            }

            let old_tile = *tile;

            // Give resources based on what was dug
            match old_tile {
                Tile::Dirt { .. } => {
                    // Dirt gives nothing special
                }
                Tile::Stone { .. } => {
                    // Stone gives iron
                    resources.iron += 1;
                }
                _ => {}
            }

            // Replace with air
            world.set_tile(event.position, Tile::Air);

            tile_events.send(TileChangedEvent {
                position: event.position,
                old_tile,
                new_tile: Tile::Air,
            });

            info!("Dug tile at {:?}", event.position);
        }
    }
}
