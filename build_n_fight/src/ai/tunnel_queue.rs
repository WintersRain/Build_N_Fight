//! Tunnel queue system - LOD for off-screen ants
//!
//! Off-screen ants don't exist as full entities.
//! They're queue entries moving through tunnel segments.
//! Breaking a tunnel mid-segment = surprise emergence!

use super::*;
use bevy::prelude::*;

/// Network of tunnels with ant queues
#[derive(Resource, Default)]
pub struct TunnelNetwork {
    pub segments: Vec<TunnelSegment>,
}

/// A segment of tunnel with ants queued inside
pub struct TunnelSegment {
    /// Start position of tunnel segment
    pub start: IVec3,
    /// End position (emergence point)
    pub end: IVec3,
    /// Ants in this segment, moving from start to end
    pub queue: Vec<QueuedAnt>,
    /// Movement rate through this tunnel (units per second)
    pub move_rate: f32,
    /// Is this segment intact?
    pub intact: bool,
}

/// An ant in a tunnel queue (not a full entity)
pub struct QueuedAnt {
    /// What caste of ant
    pub caste: AntCaste,
    /// Progress through segment (0.0 = start, 1.0 = end)
    pub progress: f32,
    /// Which leader this ant belongs to (if any)
    pub leader: Option<Entity>,
    /// Home nest
    pub nest: Entity,
}

impl TunnelSegment {
    pub fn new(start: IVec3, end: IVec3, move_rate: f32) -> Self {
        Self {
            start,
            end,
            queue: Vec::new(),
            move_rate,
            intact: true,
        }
    }

    /// Add an ant to the queue (at the start)
    pub fn enqueue(&mut self, ant: QueuedAnt) {
        self.queue.push(QueuedAnt {
            progress: 0.0,
            ..ant
        });
    }

    /// Update ant progress, return ants that reached the end
    pub fn update(&mut self, dt: f32) -> Vec<QueuedAnt> {
        let mut emerged = Vec::new();

        for ant in &mut self.queue {
            ant.progress += self.move_rate * dt;
        }

        // Remove and return ants that reached the end
        self.queue.retain(|ant| {
            if ant.progress >= 1.0 {
                emerged.push(QueuedAnt {
                    caste: ant.caste,
                    progress: ant.progress,
                    leader: ant.leader,
                    nest: ant.nest,
                });
                false
            } else {
                true
            }
        });

        emerged
    }

    /// Break the tunnel at a progress point, ejecting ants before that point
    pub fn break_at(&mut self, break_progress: f32) -> (IVec3, Vec<QueuedAnt>) {
        self.intact = false;

        // Calculate world position of break
        let t = break_progress;
        let break_pos = IVec3::new(
            ((1.0 - t) * self.start.x as f32 + t * self.end.x as f32) as i32,
            ((1.0 - t) * self.start.y as f32 + t * self.end.y as f32) as i32,
            ((1.0 - t) * self.start.z as f32 + t * self.end.z as f32) as i32,
        );

        // Ants before the break point get ejected here (SURPRISE!)
        let mut ejected = Vec::new();
        self.queue.retain(|ant| {
            if ant.progress < break_progress {
                ejected.push(QueuedAnt {
                    caste: ant.caste,
                    progress: ant.progress,
                    leader: ant.leader,
                    nest: ant.nest,
                });
                false
            } else {
                // Ants past the break continue to original destination
                true
            }
        });

        (break_pos, ejected)
    }
}

impl TunnelNetwork {
    /// Find segment by start position
    pub fn segment_at(&self, pos: IVec3) -> Option<&TunnelSegment> {
        self.segments.iter().find(|s| s.start == pos)
    }

    /// Find segment by start position (mutable)
    pub fn segment_at_mut(&mut self, pos: IVec3) -> Option<&mut TunnelSegment> {
        self.segments.iter_mut().find(|s| s.start == pos)
    }

    /// Add a new tunnel segment
    pub fn add_segment(&mut self, start: IVec3, end: IVec3, move_rate: f32) {
        self.segments.push(TunnelSegment::new(start, end, move_rate));
    }
}

/// System to update tunnel queues
pub fn update_tunnel_queues(
    mut commands: Commands,
    mut network: ResMut<TunnelNetwork>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for segment in &mut network.segments {
        if !segment.intact {
            continue; // Broken segments don't process normally
        }

        // Update all ants in queue
        let emerged = segment.update(dt);

        // Spawn emerged ants as real entities at the end position
        for ant in emerged {
            commands.spawn((
                Ant::new(ant.caste, ant.nest),
                Transform::from_translation(segment.end.as_vec3()),
                // Will get Follower component if they have a leader
            ));
        }
    }
}
