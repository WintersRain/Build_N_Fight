//! Projectile system
//!
//! Projectiles travel from weapon to target.
//! Some are instant (hitscan), some have travel time.

use bevy::prelude::*;

/// Projectile component
#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub target: IVec3,
    pub aoe_radius: Option<f32>,
    /// Who fired this (for friendly fire prevention)
    pub source: Entity,
}

impl Projectile {
    pub fn new(damage: f32, speed: f32, target: IVec3, source: Entity) -> Self {
        Self {
            damage,
            speed,
            target,
            aoe_radius: None,
            source,
        }
    }

    pub fn with_aoe(mut self, radius: f32) -> Self {
        self.aoe_radius = Some(radius);
        self
    }
}

/// System to update projectile movement
pub fn update_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
    // TODO: Add damage event writer
) {
    let dt = time.delta_secs();

    for (entity, mut transform, projectile) in projectiles.iter_mut() {
        let current = transform.translation;
        let target = projectile.target.as_vec3();
        let direction = target - current;
        let distance = direction.length();

        if distance < 1.0 {
            // Reached target - deal damage and despawn
            // TODO: Send damage event
            commands.entity(entity).despawn();
        } else {
            // Move toward target
            let movement = direction.normalize() * projectile.speed * dt;
            transform.translation += movement;
        }
    }
}
