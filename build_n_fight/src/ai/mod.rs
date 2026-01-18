//! AI module - Ant behavior and ecology
//!
//! Hierarchical AI:
//! - Leaders think and make decisions
//! - Followers just follow their leader
//!
//! Ecology simulation:
//! - Ants gather resources, expand, tech up
//! - Biomass from kills = tech advancement
//! - Awareness state machine drives escalation

use bevy::prelude::*;

mod ecology;
mod follower;
mod leader;
mod scent;
mod scout;
mod tunnel_queue;

pub use ecology::*;
pub use follower::*;
pub use leader::*;
pub use scent::*;
pub use scout::*;
pub use tunnel_queue::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TunnelNetwork>()
            .init_resource::<ScentTrails>()
            .add_event::<ScoutReturnedEvent>()
            .add_event::<AwarenessChangedEvent>()
            .add_systems(Update, (
                update_leaders,
                update_followers,
                update_scouts,
                update_tunnel_queues,
                update_ecology,
                update_scent_trails,
            ));
    }
}

/// Ant caste determines size, strength, and role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum AntCaste {
    #[default]
    Minor,   // Small, fast, weak
    Median,  // Medium, balanced
    Major,   // Large, slow, strong (big heads!)
    Scout,   // Fast, fragile, leaves scent trails
    Siege,   // Massive head, breaks walls
}

impl AntCaste {
    pub fn base_hp(&self) -> u16 {
        match self {
            AntCaste::Minor => 10,
            AntCaste::Median => 25,
            AntCaste::Major => 50,
            AntCaste::Scout => 5,
            AntCaste::Siege => 100,
        }
    }

    pub fn base_damage(&self) -> u16 {
        match self {
            AntCaste::Minor => 2,
            AntCaste::Median => 5,
            AntCaste::Major => 10,
            AntCaste::Scout => 1,
            AntCaste::Siege => 25,
        }
    }

    pub fn move_speed(&self) -> f32 {
        match self {
            AntCaste::Minor => 1.2,
            AntCaste::Median => 1.0,
            AntCaste::Major => 0.7,
            AntCaste::Scout => 1.5,
            AntCaste::Siege => 0.4,
        }
    }

    /// Biomass cost to produce
    pub fn biomass_cost(&self) -> u32 {
        match self {
            AntCaste::Minor => 5,
            AntCaste::Median => 15,
            AntCaste::Major => 30,
            AntCaste::Scout => 8,
            AntCaste::Siege => 60,
        }
    }
}

/// Basic ant component
#[derive(Component)]
pub struct Ant {
    pub caste: AntCaste,
    pub hp: u16,
    pub max_hp: u16,
    pub home_nest: Entity,
}

impl Ant {
    pub fn new(caste: AntCaste, nest: Entity) -> Self {
        let hp = caste.base_hp();
        Self {
            caste,
            hp,
            max_hp: hp,
            home_nest: nest,
        }
    }
}
