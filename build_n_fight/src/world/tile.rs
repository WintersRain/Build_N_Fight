//! Tile types and their properties
//!
//! Key insight: Wall traversal cost = Wall HP
//! Damaged walls naturally attract more ants.

use bevy::prelude::*;

/// BuildMaterial types for constructed tiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BuildMaterial {
    #[default]
    Wood,
    Stone,
    Metal,
}

impl BuildMaterial {
    pub fn base_hp(&self) -> u16 {
        match self {
            BuildMaterial::Wood => 50,
            BuildMaterial::Stone => 150,
            BuildMaterial::Metal => 300,
        }
    }
}

/// Tile types in the world
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Tile {
    #[default]
    Air,

    /// Natural dirt - diggable
    Dirt {
        hp: u16,
        max_hp: u16,
    },

    /// Natural stone - harder to dig
    Stone {
        hp: u16,
        max_hp: u16,
    },

    /// Player-built wall
    Wall {
        hp: u16,
        max_hp: u16,
        material: BuildMaterial,
    },

    /// Player-built floor (blocks vertical movement)
    Floor {
        hp: u16,
        max_hp: u16,
        material: BuildMaterial,
    },

    /// Collapsed wall/structure
    Rubble,

    /// Ant-built structure
    AntStructure {
        hp: u16,
        structure_type: AntStructureType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AntStructureType {
    #[default]
    Tunnel,
    Nest,
    Storage,
}

impl Tile {
    /// Get the traversal cost for flow field pathfinding
    /// Key insight: Cost = HP, so damaged tiles are preferred paths
    pub fn traversal_cost(&self) -> u32 {
        match self {
            Tile::Air => 1,
            Tile::Dirt { hp, .. } => *hp as u32,
            Tile::Stone { hp, .. } => (*hp as u32) * 2, // Stone is harder
            Tile::Wall { hp, .. } => *hp as u32,
            Tile::Floor { hp, .. } => *hp as u32,
            Tile::Rubble => 3, // Easy to traverse
            Tile::AntStructure { .. } => 1, // Ants move freely through their structures
        }
    }

    /// Can this tile be traversed at all?
    pub fn is_passable(&self) -> bool {
        match self {
            Tile::Air => true,
            Tile::Rubble => true,
            Tile::AntStructure { .. } => true,
            _ => false, // Must dig/destroy to pass
        }
    }

    /// Can this tile be dug through?
    pub fn is_diggable(&self) -> bool {
        matches!(self, Tile::Dirt { .. } | Tile::Stone { .. })
    }

    /// Can this tile be attacked/destroyed?
    pub fn is_destructible(&self) -> bool {
        matches!(
            self,
            Tile::Dirt { .. }
                | Tile::Stone { .. }
                | Tile::Wall { .. }
                | Tile::Floor { .. }
                | Tile::AntStructure { .. }
        )
    }

    /// Get current HP if applicable
    pub fn hp(&self) -> Option<u16> {
        match self {
            Tile::Dirt { hp, .. } => Some(*hp),
            Tile::Stone { hp, .. } => Some(*hp),
            Tile::Wall { hp, .. } => Some(*hp),
            Tile::Floor { hp, .. } => Some(*hp),
            Tile::AntStructure { hp, .. } => Some(*hp),
            _ => None,
        }
    }

    /// Apply damage to tile, returns true if destroyed
    pub fn damage(&mut self, amount: u16) -> bool {
        match self {
            Tile::Dirt { hp, .. }
            | Tile::Stone { hp, .. }
            | Tile::Wall { hp, .. }
            | Tile::Floor { hp, .. }
            | Tile::AntStructure { hp, .. } => {
                if *hp <= amount {
                    *self = Tile::Rubble;
                    true
                } else {
                    *hp -= amount;
                    false
                }
            }
            _ => false,
        }
    }

    /// ASCII representation for debug rendering
    pub fn to_ascii(&self) -> char {
        match self {
            Tile::Air => '.',
            Tile::Dirt { .. } => ',',
            Tile::Stone { .. } => '#',
            Tile::Wall { material, .. } => match material {
                BuildMaterial::Wood => '=',
                BuildMaterial::Stone => 'H',
                BuildMaterial::Metal => 'M',
            },
            Tile::Floor { .. } => '_',
            Tile::Rubble => '%',
            Tile::AntStructure { structure_type, .. } => match structure_type {
                AntStructureType::Tunnel => 'o',
                AntStructureType::Nest => 'O',
                AntStructureType::Storage => 's',
            },
        }
    }
}
