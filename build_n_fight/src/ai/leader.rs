//! Swarm leader AI - The only ants that "think"
//!
//! Leaders make decisions about:
//! - Where to attack
//! - When to claim breach points
//! - When to request/provide reinforcements

use super::*;
use crate::flow::{BreachPoints, TargetField, TraversalField};
use bevy::prelude::*;

/// Swarm leader component
#[derive(Component)]
pub struct SwarmLeader {
    pub follower_count: u32,
    pub max_followers: u32,
    pub claimed_breach: Option<Entity>,
    pub state: LeaderState,
}

/// Leader behavior states
#[derive(Debug, Clone, PartialEq, Default)]
pub enum LeaderState {
    #[default]
    Seeking,                    // Looking for breach point or wall to attack
    Assaulting { target: IVec3 },   // Leading followers to attack
    Reinforcing { ally: Entity },    // Sending troops to another leader
    Creating { target: IVec3 },      // Creating new breach point
    Retreating,                 // Falling back (too many losses)
}

impl SwarmLeader {
    pub fn new(max_followers: u32) -> Self {
        Self {
            follower_count: 0,
            max_followers,
            claimed_breach: None,
            state: LeaderState::Seeking,
        }
    }

    /// Check if this leader needs reinforcements
    pub fn needs_reinforcements(&self) -> bool {
        self.follower_count < self.max_followers / 2
    }

    /// Check if this leader can spare troops
    pub fn can_spare_troops(&self) -> bool {
        self.follower_count > self.max_followers * 3 / 4
    }
}

/// System to update leader behavior
pub fn update_leaders(
    mut leaders: Query<(Entity, &mut SwarmLeader, &Transform)>,
    breach_points: Res<BreachPoints>,
    target_field: Res<TargetField>,
    traversal_field: Res<TraversalField>,
) {
    for (entity, mut leader, transform) in leaders.iter_mut() {
        let pos = transform.translation.truncate().as_ivec2().extend(0);

        match &leader.state {
            LeaderState::Seeking => {
                // Priority 1: Claim unclaimed breach if nearby
                if let Some(breach) = breach_points.nearest_unclaimed(pos) {
                    let dist = (breach.position - pos).abs();
                    if dist.x <= 10 && dist.y <= 10 {
                        leader.state = LeaderState::Assaulting {
                            target: breach.position,
                        };
                        continue;
                    }
                }

                // Priority 2: Find high-value target
                if let Some((target_pos, _)) = target_field.highest_value_target(pos, 50) {
                    leader.state = LeaderState::Assaulting { target: target_pos };
                    continue;
                }

                // Priority 3: Wander toward player base (TODO: implement)
            }

            LeaderState::Assaulting { target } => {
                // Check if we've reached the target
                let dist = (*target - pos).abs();
                if dist.x <= 1 && dist.y <= 1 && dist.z <= 1 {
                    // At target - attack or transition
                    // TODO: Implement attack behavior
                }

                // Check if we've lost too many troops
                if leader.needs_reinforcements() {
                    // Request reinforcements but keep attacking
                    // TODO: Broadcast reinforcement request
                }
            }

            LeaderState::Reinforcing { ally } => {
                // Move toward ally's position
                // TODO: Implement reinforcement movement
            }

            LeaderState::Creating { target } => {
                // Digging toward target to create new breach
                // TODO: Implement dig behavior
            }

            LeaderState::Retreating => {
                // Fall back toward nest
                // TODO: Implement retreat behavior
            }
        }
    }
}
