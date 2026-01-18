//! Breach point management
//!
//! When a wall is destroyed, it becomes a breach point.
//! Swarm leaders can claim breach points to coordinate attacks.

use bevy::prelude::*;

/// Collection of active breach points
#[derive(Resource, Default)]
pub struct BreachPoints {
    pub points: Vec<BreachPoint>,
}

/// A breach in the player's defenses
#[derive(Debug, Clone)]
pub struct BreachPoint {
    /// Position of the breach
    pub position: IVec3,
    /// Swarm leader that claimed this breach (if any)
    pub claimed_by: Option<Entity>,
    /// Number of reinforcements requested
    pub reinforcement_requests: u32,
    /// Time since breach was created
    pub age: f32,
}

impl BreachPoint {
    pub fn new(position: IVec3) -> Self {
        Self {
            position,
            claimed_by: None,
            reinforcement_requests: 0,
            age: 0.0,
        }
    }

    /// Check if this breach is claimed
    pub fn is_claimed(&self) -> bool {
        self.claimed_by.is_some()
    }

    /// Claim this breach for a swarm leader
    pub fn claim(&mut self, leader: Entity) {
        self.claimed_by = Some(leader);
    }

    /// Request reinforcements at this breach
    pub fn request_reinforcements(&mut self, count: u32) {
        self.reinforcement_requests += count;
    }
}

impl BreachPoints {
    /// Find nearest unclaimed breach to a position
    pub fn nearest_unclaimed(&self, pos: IVec3) -> Option<&BreachPoint> {
        self.points
            .iter()
            .filter(|b| !b.is_claimed())
            .min_by_key(|b| {
                let diff = b.position - pos;
                diff.x.abs() + diff.y.abs() + diff.z.abs()
            })
    }

    /// Find breach by position
    pub fn at_position(&self, pos: IVec3) -> Option<&BreachPoint> {
        self.points.iter().find(|b| b.position == pos)
    }

    /// Find breach by position (mutable)
    pub fn at_position_mut(&mut self, pos: IVec3) -> Option<&mut BreachPoint> {
        self.points.iter_mut().find(|b| b.position == pos)
    }

    /// Add a new breach point
    pub fn add(&mut self, position: IVec3) {
        if self.at_position(position).is_none() {
            self.points.push(BreachPoint::new(position));
        }
    }
}

/// Event fired when a new breach is created
#[derive(Event)]
pub struct BreachCreatedEvent {
    pub position: IVec3,
}

/// System to manage breach points
pub fn manage_breach_points(
    mut breach_points: ResMut<BreachPoints>,
    mut breach_events: EventReader<BreachCreatedEvent>,
    time: Res<Time>,
) {
    // Add new breaches from events
    for event in breach_events.read() {
        breach_points.add(event.position);
        info!("Breach created at {:?}", event.position);
    }

    // Update age of all breaches
    for breach in &mut breach_points.points {
        breach.age += time.delta_secs();
    }
}
