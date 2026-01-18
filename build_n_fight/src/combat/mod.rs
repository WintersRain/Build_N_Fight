//! Combat module - Weapons, damage, and projectiles
//!
//! Modular weapon system with mount points.
//! Weapons can target different Z-levels based on type.

use bevy::prelude::*;

mod damage;
mod mount;
mod projectile;
mod weapon;

pub use damage::*;
pub use mount::*;
pub use projectile::*;
pub use weapon::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(Update, (
                update_weapons,
                update_projectiles,
                process_damage,
            ));
    }
}
