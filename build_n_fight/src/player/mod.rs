//! Player module - Building, digging, away teams
//!
//! Player actions and unit control.

use bevy::prelude::*;

mod away_team;
mod building;
mod digging;
mod input;

pub use away_team::*;
pub use building::*;
pub use digging::*;
pub use input::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerResources>()
            .init_resource::<BuildMode>()
            .add_event::<BuildEvent>()
            .add_event::<DigEvent>()
            .add_systems(Update, (
                handle_input,
                process_build_events,
                process_dig_events,
                update_away_teams,
            ));
    }
}

/// Player's resources
#[derive(Resource, Default)]
pub struct PlayerResources {
    pub tungsten: u32,
    pub iron: u32,
    pub wood: u32,
}

impl PlayerResources {
    pub fn new() -> Self {
        Self {
            tungsten: 100,
            iron: 200,
            wood: 500,
        }
    }
}
