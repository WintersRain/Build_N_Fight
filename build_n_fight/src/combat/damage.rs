//! Damage system
//!
//! Central damage processing.
//! Tracks deaths for biomass rewards.

use crate::ai::{Ant, AntNest, biomass_rewards};
use crate::world::{GameWorld, Tile, TileChangedEvent};
use bevy::prelude::*;

/// Damage event
#[derive(Event)]
pub struct DamageEvent {
    pub target: DamageTarget,
    pub amount: f32,
    pub source: Option<Entity>,
    pub position: IVec3,
}

/// What can take damage
#[derive(Clone)]
pub enum DamageTarget {
    Entity(Entity),
    Tile(IVec3),
}

/// Death event (for biomass tracking)
#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
    pub position: IVec3,
    pub killed_by: Option<Entity>,
    pub biomass_value: u32,
}

/// Health component for entities
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: f32) -> bool {
        self.current -= amount;
        self.current <= 0.0
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// System to process damage events
pub fn process_damage(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
    mut death_events: EventWriter<DeathEvent>,
    mut tile_events: EventWriter<TileChangedEvent>,
    mut world: ResMut<GameWorld>,
    mut health_query: Query<(&mut Health, &Transform, Option<&Ant>)>,
) {
    for event in damage_events.read() {
        match &event.target {
            DamageTarget::Entity(entity) => {
                if let Ok((mut health, transform, ant)) = health_query.get_mut(*entity) {
                    let died = health.damage(event.amount);

                    if died {
                        // Calculate biomass value
                        let biomass = if ant.is_some() {
                            biomass_rewards::MINOR_ANT
                        } else {
                            biomass_rewards::PLAYER_WORKER
                        };

                        death_events.send(DeathEvent {
                            entity: *entity,
                            position: transform.translation.as_ivec3(),
                            killed_by: event.source,
                            biomass_value: biomass,
                        });

                        commands.entity(*entity).despawn();
                    }
                }
            }

            DamageTarget::Tile(pos) => {
                if let Some(tile) = world.get_tile(*pos) {
                    let mut tile = *tile;
                    let old_tile = tile;

                    if tile.damage(event.amount as u16) {
                        // Tile destroyed
                        tile_events.send(TileChangedEvent {
                            position: *pos,
                            old_tile,
                            new_tile: Tile::Rubble,
                        });
                    } else {
                        // Tile damaged but not destroyed
                        tile_events.send(TileChangedEvent {
                            position: *pos,
                            old_tile,
                            new_tile: tile,
                        });
                    }

                    world.set_tile(*pos, tile);
                }
            }
        }
    }
}
