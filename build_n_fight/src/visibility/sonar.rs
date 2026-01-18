//! Sonar system - Throwable detection devices
//!
//! Throw sonar balls to detect contacts underground.
//! You see that something is there, but not WHAT.
//! Creates tension: 5 contacts... 80 contacts... OH SHIT!

use super::*;
use crate::ai::Ant;
use bevy::prelude::*;
use rand::Rng;

/// Sonar device component
#[derive(Component)]
pub struct SonarDevice {
    /// How often to ping (seconds)
    pub ping_interval: f32,
    /// Detection radius
    pub ping_radius: f32,
    /// Time since last ping
    pub time_since_ping: f32,
    /// How long before device expires
    pub lifetime: f32,
}

impl SonarDevice {
    pub fn new(radius: f32, interval: f32, lifetime: f32) -> Self {
        Self {
            ping_interval: interval,
            ping_radius: radius,
            time_since_ping: 0.0,
            lifetime,
        }
    }

    /// Standard sonar grenade
    pub fn grenade() -> Self {
        Self::new(10.0, 2.0, 30.0)  // 10 tile radius, ping every 2s, lasts 30s
    }
}

/// A contact detected by sonar
#[derive(Debug, Clone)]
pub struct SonarContact {
    /// Approximate position (has some error)
    pub position: IVec3,
    /// Contact size (number of entities in area)
    pub count: u32,
}

/// Event fired when sonar pings
#[derive(Event)]
pub struct SonarPingEvent {
    pub device_position: IVec3,
    pub contacts: Vec<SonarContact>,
}

/// System to update sonar devices
pub fn update_sonar(
    mut commands: Commands,
    mut sonar_devices: Query<(Entity, &mut SonarDevice, &Transform)>,
    ants: Query<&Transform, With<Ant>>,
    mut fog: ResMut<FogOfWar>,
    mut ping_events: EventWriter<SonarPingEvent>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let mut rng = rand::thread_rng();

    for (entity, mut device, transform) in sonar_devices.iter_mut() {
        device.lifetime -= dt;
        device.time_since_ping += dt;

        // Remove expired devices
        if device.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Check if it's time to ping
        if device.time_since_ping >= device.ping_interval {
            device.time_since_ping = 0.0;

            let device_pos = transform.translation.as_ivec3();
            let radius_sq = device.ping_radius * device.ping_radius;

            // Find all ants in range
            let mut contacts_in_range: Vec<IVec3> = Vec::new();

            for ant_transform in ants.iter() {
                let ant_pos = ant_transform.translation;
                let dist_sq = (ant_pos - transform.translation).length_squared();

                if dist_sq <= radius_sq {
                    contacts_in_range.push(ant_pos.as_ivec3());
                }
            }

            // Group contacts by approximate position (add some error)
            let mut grouped_contacts: Vec<SonarContact> = Vec::new();

            for contact_pos in contacts_in_range {
                // Add position error (sonar isn't precise)
                let error = IVec3::new(
                    rng.gen_range(-2..=2),
                    rng.gen_range(-2..=2),
                    0,
                );
                let approx_pos = contact_pos + error;

                // Mark as sonar contact in fog
                fog.set(approx_pos, TileVisibility::SonarContact);

                // Add to grouped contacts
                if let Some(existing) = grouped_contacts.iter_mut().find(|c| {
                    (c.position - approx_pos).abs().max_element() <= 2
                }) {
                    existing.count += 1;
                } else {
                    grouped_contacts.push(SonarContact {
                        position: approx_pos,
                        count: 1,
                    });
                }
            }

            if !grouped_contacts.is_empty() {
                ping_events.send(SonarPingEvent {
                    device_position: device_pos,
                    contacts: grouped_contacts.clone(),
                });

                // Log for dramatic effect
                let total: u32 = grouped_contacts.iter().map(|c| c.count).sum();
                if total > 50 {
                    warn!("SONAR: {} contacts detected! They're massing!", total);
                } else if total > 10 {
                    info!("Sonar: {} contacts detected", total);
                } else {
                    info!("Sonar: {} contacts", total);
                }
            }
        }
    }
}
