//! Away team system - High risk exploration
//!
//! Send teams underground to:
//! - Scout
//! - Mine resources
//! - Seal tunnels
//! - Plant explosives
//!
//! WARNING: If they die, ants get biomass!

use bevy::prelude::*;

/// Away team component
#[derive(Component)]
pub struct AwayTeam {
    pub members: Vec<Entity>,
    pub mission: Mission,
    pub supplies: Supplies,
}

/// What the team is doing
#[derive(Debug, Clone)]
pub enum Mission {
    Scout { target_area: IVec3 },
    Mine { target_deposit: IVec3 },
    SealTunnel { tunnel_pos: IVec3 },
    PlantExplosive { target: IVec3, fuse_time: f32 },
    Return,
}

/// Team supplies
#[derive(Debug, Clone, Default)]
pub struct Supplies {
    pub explosives: u32,
    pub sealant: u32,
    pub ammo: u32,
    pub food: f32,  // Time before they need to return
}

impl AwayTeam {
    pub fn new(members: Vec<Entity>, mission: Mission) -> Self {
        Self {
            members,
            mission,
            supplies: Supplies::default(),
        }
    }

    pub fn with_supplies(mut self, supplies: Supplies) -> Self {
        self.supplies = supplies;
        self
    }

    /// Check if team is out of supplies
    pub fn needs_resupply(&self) -> bool {
        self.supplies.food <= 0.0
    }

    /// Team size
    pub fn size(&self) -> usize {
        self.members.len()
    }
}

/// System to update away teams
pub fn update_away_teams(
    mut teams: Query<&mut AwayTeam>,
    time: Res<Time>,
) {
    for mut team in teams.iter_mut() {
        // Consume food over time
        team.supplies.food -= time.delta_secs();

        // If out of food, force return
        if team.needs_resupply() {
            team.mission = Mission::Return;
        }

        // TODO: Mission execution logic
        match &team.mission {
            Mission::Scout { target_area } => {
                // Move toward target, look for threats
            }
            Mission::Mine { target_deposit } => {
                // Move to deposit, dig, collect resources
            }
            Mission::SealTunnel { tunnel_pos } => {
                // Move to tunnel, use sealant
            }
            Mission::PlantExplosive { target, fuse_time } => {
                // Move to target, plant, run!
            }
            Mission::Return => {
                // Head back to base
            }
        }
    }
}
