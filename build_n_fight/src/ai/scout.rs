//! Scout AI - Exploration and discovery
//!
//! Scouts explore the world and report back.
//! If they discover the player and RETURN HOME, awareness increases.
//! Kill scouts before they report!

use super::*;
use bevy::prelude::*;

/// Scout-specific component
#[derive(Component)]
pub struct Scout {
    /// Nest this scout belongs to
    pub origin_nest: Entity,
    /// Things discovered during exploration
    pub discoveries: Vec<Discovery>,
    /// Whether scout is returning home
    pub returning: bool,
    /// Path back home (scent trail positions)
    pub home_path: Vec<IVec3>,
}

/// Things a scout can discover
#[derive(Debug, Clone)]
pub enum Discovery {
    PlayerUnit { position: IVec3, unit_type: String },
    PlayerStructure { position: IVec3, structure_type: String },
    SealedTunnel { position: IVec3 },
    Resource { position: IVec3, resource_type: String },
}

impl Scout {
    pub fn new(nest: Entity) -> Self {
        Self {
            origin_nest: nest,
            discoveries: Vec::new(),
            returning: false,
            home_path: Vec::new(),
        }
    }

    /// Record a discovery
    pub fn discover(&mut self, discovery: Discovery) {
        // Important discovery = start returning home
        if matches!(&discovery, Discovery::PlayerUnit { .. } | Discovery::PlayerStructure { .. }) {
            self.returning = true;
        }
        self.discoveries.push(discovery);
    }

    /// Check if scout has important intel
    pub fn has_important_intel(&self) -> bool {
        self.discoveries.iter().any(|d| {
            matches!(d, Discovery::PlayerUnit { .. } | Discovery::PlayerStructure { .. })
        })
    }
}

/// Event fired when a scout returns home with intel
#[derive(Event)]
pub struct ScoutReturnedEvent {
    pub nest: Entity,
    pub discoveries: Vec<Discovery>,
}

/// System to update scout behavior
pub fn update_scouts(
    mut commands: Commands,
    mut scouts: Query<(Entity, &mut Scout, &Transform)>,
    nests: Query<&Transform, With<AntNest>>,
    mut scout_events: EventWriter<ScoutReturnedEvent>,
    mut scent_trails: ResMut<ScentTrails>,
) {
    for (entity, mut scout, transform) in scouts.iter_mut() {
        let pos = transform.translation.truncate().as_ivec2().extend(0);

        if scout.returning {
            // Moving back home
            if let Ok(nest_transform) = nests.get(scout.origin_nest) {
                let nest_pos = nest_transform.translation.truncate().as_ivec2();
                let dist = (nest_pos - pos.truncate()).abs();

                if dist.x <= 1 && dist.y <= 1 {
                    // Reached home! Report discoveries
                    if scout.has_important_intel() {
                        scout_events.send(ScoutReturnedEvent {
                            nest: scout.origin_nest,
                            discoveries: scout.discoveries.clone(),
                        });
                        info!("Scout returned with {} discoveries!", scout.discoveries.len());
                    }
                    // Despawn scout (will be recycled into nest population)
                    commands.entity(entity).despawn();
                }
            }
        } else {
            // Exploring - leave scent trail
            scent_trails.add_scent(pos, scout.origin_nest, 1.0);
            scout.home_path.push(pos);

            // TODO: Exploration logic
            // - Move toward unexplored areas
            // - Check for player presence
            // - Avoid dangers
        }
    }
}
