//! Chunk-based world storage for efficient tile access

use super::Tile;
use bevy::prelude::*;

pub const CHUNK_SIZE: usize = 16;

/// A 16x16x16 chunk of tiles
#[derive(Clone)]
pub struct Chunk {
    tiles: Box<[[[Tile; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    /// Dirty flag for flow field recalculation
    pub flow_field_dirty: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            tiles: Box::new([[[Tile::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
            flow_field_dirty: true,
        }
    }

    /// Create a chunk filled with a specific tile
    pub fn filled(tile: Tile) -> Self {
        Self {
            tiles: Box::new([[[tile; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
            flow_field_dirty: true,
        }
    }

    /// Get a tile at local chunk position
    pub fn get_tile(&self, pos: UVec3) -> &Tile {
        &self.tiles[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    /// Set a tile at local chunk position
    pub fn set_tile(&mut self, pos: UVec3, tile: Tile) {
        self.tiles[pos.x as usize][pos.y as usize][pos.z as usize] = tile;
        self.flow_field_dirty = true;
    }

    /// Iterate over all tiles with their local positions
    pub fn iter_tiles(&self) -> impl Iterator<Item = (UVec3, &Tile)> {
        (0..CHUNK_SIZE).flat_map(move |x| {
            (0..CHUNK_SIZE).flat_map(move |y| {
                (0..CHUNK_SIZE).map(move |z| {
                    (
                        UVec3::new(x as u32, y as u32, z as u32),
                        &self.tiles[x][y][z],
                    )
                })
            })
        })
    }
}
