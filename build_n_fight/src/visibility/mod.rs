//! Visibility module - Fog of war and sonar
//!
//! Underground is hidden until explored.
//! Sonar devices reveal contacts (but not what they are).

use bevy::prelude::*;

mod fog;
mod sonar;

pub use fog::*;
pub use sonar::*;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FogOfWar>()
            .add_event::<SonarPingEvent>()
            .add_systems(Update, (update_fog, update_sonar));
    }
}
