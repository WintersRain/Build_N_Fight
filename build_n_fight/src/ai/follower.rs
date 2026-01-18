//! Follower AI - Almost zero cost behavior
//!
//! Followers don't think. They just:
//! 1. Follow their leader
//! 2. Attack what's in front of them
//! 3. Die heroically

use super::*;
use bevy::prelude::*;

/// Follower component - just tracks which leader to follow
#[derive(Component)]
pub struct Follower {
    pub leader: Entity,
    /// Offset from leader position (for formation)
    pub offset: Vec2,
    /// Current state
    pub state: FollowerState,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FollowerState {
    #[default]
    Following,      // Moving toward leader
    Attacking,      // Attacking nearby target
    Dying,          // Death animation
}

impl Follower {
    pub fn new(leader: Entity, offset: Vec2) -> Self {
        Self {
            leader,
            offset,
            state: FollowerState::Following,
        }
    }
}

/// System to update follower positions (trivial - just follow leader)
pub fn update_followers(
    mut followers: Query<(&Follower, &mut Transform), Without<SwarmLeader>>,
    leaders: Query<&Transform, With<SwarmLeader>>,
) {
    for (follower, mut transform) in followers.iter_mut() {
        if let Ok(leader_transform) = leaders.get(follower.leader) {
            // Target position is leader position + offset
            let target = leader_transform.translation.truncate() + follower.offset;

            // Simple move toward target
            let current = transform.translation.truncate();
            let direction = target - current;

            if direction.length() > 1.0 {
                let move_speed = 50.0; // TODO: Get from ant caste
                let movement = direction.normalize() * move_speed;
                transform.translation.x += movement.x * 0.016; // Assuming ~60fps
                transform.translation.y += movement.y * 0.016;
            }
        }
    }
}

/// Spawn followers for a leader
pub fn spawn_followers(
    commands: &mut Commands,
    leader: Entity,
    nest: Entity,
    caste: AntCaste,
    count: u32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for i in 0..count {
        // Arrange in rough formation around leader
        let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
        let radius = 10.0 + rng.gen::<f32>() * 5.0;
        let offset = Vec2::new(angle.cos() * radius, angle.sin() * radius);

        commands.spawn((
            Ant::new(caste, nest),
            Follower::new(leader, offset),
            Transform::default(),
        ));
    }
}
