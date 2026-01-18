//! World module - Tile-based world with Z-levels
//!
//! The world is a 3D grid rendered as 2D layers. Each tile has HP which
//! doubles as traversal cost for flow field pathfinding.

use bevy::prelude::*;

mod chunk;
mod tile;
mod z_level;

pub use chunk::*;
pub use tile::*;
pub use z_level::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWorld>()
            .init_resource::<CurrentZLevel>()
            .add_event::<TileChangedEvent>()
            .add_systems(Startup, setup_world);
    }
}

/// The game world resource
#[derive(Resource, Default)]
pub struct GameWorld {
    pub chunks: hashbrown::HashMap<IVec3, Chunk>,
    pub surface_z: i32,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            chunks: hashbrown::HashMap::new(),
            surface_z: 0,
        }
    }

    /// Get a tile at world position
    pub fn get_tile(&self, pos: IVec3) -> Option<&Tile> {
        let chunk_pos = pos.div_euclid(IVec3::splat(CHUNK_SIZE as i32));
        let local_pos = pos.rem_euclid(IVec3::splat(CHUNK_SIZE as i32));
        self.chunks
            .get(&chunk_pos)
            .map(|c| c.get_tile(local_pos.as_uvec3()))
    }

    /// Set a tile at world position
    pub fn set_tile(&mut self, pos: IVec3, tile: Tile) {
        let chunk_pos = pos.div_euclid(IVec3::splat(CHUNK_SIZE as i32));
        let local_pos = pos.rem_euclid(IVec3::splat(CHUNK_SIZE as i32));
        self.chunks
            .entry(chunk_pos)
            .or_insert_with(Chunk::new)
            .set_tile(local_pos.as_uvec3(), tile);
    }
}

/// Event fired when a tile changes (for flow field recalculation)
#[derive(Event)]
pub struct TileChangedEvent {
    pub position: IVec3,
    pub old_tile: Tile,
    pub new_tile: Tile,
}

fn setup_world(mut world: ResMut<GameWorld>) {
    // Generate initial world with surface and underground
    // Surface at Z=0, underground at Z=-1, Z=-2, etc.

    let world_size = 32; // 32x32 tiles for now

    for x in 0..world_size {
        for y in 0..world_size {
            // Surface layer - mostly air with some walls
            world.set_tile(IVec3::new(x, y, 0), Tile::Air);

            // Underground layers - dirt and stone
            world.set_tile(
                IVec3::new(x, y, -1),
                Tile::Dirt {
                    hp: 50,
                    max_hp: 50,
                },
            );
            world.set_tile(
                IVec3::new(x, y, -2),
                Tile::Stone {
                    hp: 100,
                    max_hp: 100,
                },
            );
        }
    }

    info!("World generated: {}x{} tiles, 3 Z-levels", world_size, world_size);
}
