//! Building system - Walls, floors, structures

use crate::world::{GameWorld, BuildMaterial, Tile, TileChangedEvent};
use bevy::prelude::*;

/// What we're trying to build
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildableType {
    #[default]
    WoodWall,
    StoneWall,
    MetalWall,
    WoodFloor,
    StoneFloor,
    Turret,
}

impl BuildableType {
    pub fn cost(&self) -> (u32, u32, u32) {
        // (tungsten, iron, wood)
        match self {
            BuildableType::WoodWall => (0, 0, 10),
            BuildableType::StoneWall => (0, 5, 0),
            BuildableType::MetalWall => (5, 10, 0),
            BuildableType::WoodFloor => (0, 0, 5),
            BuildableType::StoneFloor => (0, 3, 0),
            BuildableType::Turret => (2, 5, 5),
        }
    }

    pub fn to_tile(&self) -> Option<Tile> {
        match self {
            BuildableType::WoodWall => Some(Tile::Wall {
                hp: BuildMaterial::Wood.base_hp(),
                max_hp: BuildMaterial::Wood.base_hp(),
                material: BuildMaterial::Wood,
            }),
            BuildableType::StoneWall => Some(Tile::Wall {
                hp: BuildMaterial::Stone.base_hp(),
                max_hp: BuildMaterial::Stone.base_hp(),
                material: BuildMaterial::Stone,
            }),
            BuildableType::MetalWall => Some(Tile::Wall {
                hp: BuildMaterial::Metal.base_hp(),
                max_hp: BuildMaterial::Metal.base_hp(),
                material: BuildMaterial::Metal,
            }),
            BuildableType::WoodFloor => Some(Tile::Floor {
                hp: BuildMaterial::Wood.base_hp(),
                max_hp: BuildMaterial::Wood.base_hp(),
                material: BuildMaterial::Wood,
            }),
            BuildableType::StoneFloor => Some(Tile::Floor {
                hp: BuildMaterial::Stone.base_hp(),
                max_hp: BuildMaterial::Stone.base_hp(),
                material: BuildMaterial::Stone,
            }),
            BuildableType::Turret => None, // Turret is an entity, not a tile
        }
    }
}

/// Current build mode
#[derive(Resource, Default)]
pub struct BuildMode {
    pub active: bool,
    pub selected: BuildableType,
}

/// Build event
#[derive(Event)]
pub struct BuildEvent {
    pub position: IVec3,
    pub buildable: BuildableType,
}

/// Process build events
pub fn process_build_events(
    mut commands: Commands,
    mut events: EventReader<BuildEvent>,
    mut world: ResMut<GameWorld>,
    mut resources: ResMut<super::PlayerResources>,
    mut tile_events: EventWriter<TileChangedEvent>,
) {
    for event in events.read() {
        let (tungsten, iron, wood) = event.buildable.cost();

        // Check resources
        if resources.tungsten < tungsten
            || resources.iron < iron
            || resources.wood < wood
        {
            info!("Not enough resources to build {:?}", event.buildable);
            continue;
        }

        // Check if position is valid (must be air or rubble)
        if let Some(tile) = world.get_tile(event.position) {
            if !matches!(tile, Tile::Air | Tile::Rubble) {
                info!("Cannot build at {:?} - tile occupied", event.position);
                continue;
            }
        }

        // Deduct resources
        resources.tungsten -= tungsten;
        resources.iron -= iron;
        resources.wood -= wood;

        // Place the building
        if let Some(new_tile) = event.buildable.to_tile() {
            let old_tile = *world.get_tile(event.position).unwrap_or(&Tile::Air);
            world.set_tile(event.position, new_tile);

            tile_events.send(TileChangedEvent {
                position: event.position,
                old_tile,
                new_tile,
            });

            info!("Built {:?} at {:?}", event.buildable, event.position);
        } else if matches!(event.buildable, BuildableType::Turret) {
            // Spawn turret entity
            use crate::combat::MountPoint;

            commands.spawn((
                MountPoint::universal(),
                Transform::from_translation(event.position.as_vec3()),
            ));

            info!("Placed turret mount at {:?}", event.position);
        }
    }
}
