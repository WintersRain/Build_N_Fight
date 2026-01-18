//! Ant ecology - The ants have their own game
//!
//! Nests gather resources, expand, and tech up.
//! Biomass from kills = tech advancement.
//! Human body = MASSIVE tech spike.

use super::*;
use bevy::prelude::*;

/// Ant nest component
#[derive(Component)]
pub struct AntNest {
    /// Current biomass (main resource)
    pub biomass: u32,
    /// Tech level (unlocks new ant types)
    pub tech_level: u8,
    /// Current awareness of player
    pub awareness: AwarenessState,
    /// Ants queued for production
    pub production_queue: Vec<AntCaste>,
    /// Maximum population
    pub max_population: u32,
    /// Current population
    pub population: u32,
}

/// Awareness state machine
#[derive(Debug, Clone, PartialEq, Default)]
pub enum AwarenessState {
    #[default]
    Unaware,                          // Expanding normally, no threat detected
    Suspicious { last_seen: IVec3 },  // Scout saw something, investigating
    Aware,                            // Confirmed player exists, preparing assault
    Aggressive,                       // Full assault mode
}

/// Event fired when awareness state changes
#[derive(Event)]
pub struct AwarenessChangedEvent {
    pub nest: Entity,
    pub old_state: AwarenessState,
    pub new_state: AwarenessState,
}

/// Biomass rewards
pub mod biomass_rewards {
    pub const MINOR_ANT: u32 = 2;
    pub const PLAYER_WORKER: u32 = 10;
    pub const PLAYER_SOLDIER: u32 = 25;
    pub const PLAYER_HUMAN: u32 = 500; // MASSIVE boost!
}

/// Tech thresholds
pub mod tech_thresholds {
    pub const LEVEL_1: u32 = 100;   // Unlock Median ants
    pub const LEVEL_2: u32 = 300;   // Unlock Major ants
    pub const LEVEL_3: u32 = 600;   // Unlock smarter tunneling
    pub const LEVEL_4: u32 = 1000;  // Unlock Siege ants
    pub const LEVEL_5: u32 = 2000;  // Unlock special abilities
}

impl AntNest {
    pub fn new() -> Self {
        Self {
            biomass: 50, // Start with some biomass
            tech_level: 0,
            awareness: AwarenessState::Unaware,
            production_queue: Vec::new(),
            max_population: 100,
            population: 10,
        }
    }

    /// Add biomass to the nest
    pub fn add_biomass(&mut self, amount: u32) {
        self.biomass += amount;
        self.check_tech_advancement();
    }

    /// Check if we've hit a tech threshold
    fn check_tech_advancement(&mut self) {
        let new_level = match self.biomass {
            b if b >= tech_thresholds::LEVEL_5 => 5,
            b if b >= tech_thresholds::LEVEL_4 => 4,
            b if b >= tech_thresholds::LEVEL_3 => 3,
            b if b >= tech_thresholds::LEVEL_2 => 2,
            b if b >= tech_thresholds::LEVEL_1 => 1,
            _ => 0,
        };

        if new_level > self.tech_level {
            info!("Nest advanced to tech level {}!", new_level);
            self.tech_level = new_level;
        }
    }

    /// Get available ant castes at current tech level
    pub fn available_castes(&self) -> Vec<AntCaste> {
        let mut castes = vec![AntCaste::Minor, AntCaste::Scout];

        if self.tech_level >= 1 {
            castes.push(AntCaste::Median);
        }
        if self.tech_level >= 2 {
            castes.push(AntCaste::Major);
        }
        if self.tech_level >= 4 {
            castes.push(AntCaste::Siege);
        }

        castes
    }

    /// Queue production of an ant
    pub fn queue_production(&mut self, caste: AntCaste) -> bool {
        if self.biomass >= caste.biomass_cost() {
            self.biomass -= caste.biomass_cost();
            self.production_queue.push(caste);
            true
        } else {
            false
        }
    }

    /// Escalate awareness based on scout reports
    pub fn escalate_awareness(&mut self, discoveries: &[Discovery]) {
        for discovery in discoveries {
            match discovery {
                Discovery::PlayerUnit { position, .. } => {
                    match &self.awareness {
                        AwarenessState::Unaware => {
                            self.awareness = AwarenessState::Suspicious {
                                last_seen: *position,
                            };
                        }
                        AwarenessState::Suspicious { .. } => {
                            self.awareness = AwarenessState::Aware;
                        }
                        _ => {}
                    }
                }
                Discovery::PlayerStructure { .. } => {
                    // Structure = definitely player base
                    if !matches!(self.awareness, AwarenessState::Aggressive) {
                        self.awareness = AwarenessState::Aware;
                    }
                }
                _ => {}
            }
        }
    }
}

impl Default for AntNest {
    fn default() -> Self {
        Self::new()
    }
}

/// System to update ecology (production, expansion, etc.)
pub fn update_ecology(
    mut nests: Query<&mut AntNest>,
    mut scout_events: EventReader<ScoutReturnedEvent>,
    mut awareness_events: EventWriter<AwarenessChangedEvent>,
    time: Res<Time>,
) {
    // Handle scout returns
    for event in scout_events.read() {
        if let Ok(mut nest) = nests.get_mut(event.nest) {
            let old_awareness = nest.awareness.clone();
            nest.escalate_awareness(&event.discoveries);

            if nest.awareness != old_awareness {
                awareness_events.send(AwarenessChangedEvent {
                    nest: event.nest,
                    old_state: old_awareness,
                    new_state: nest.awareness.clone(),
                });
            }
        }
    }

    // TODO: Production ticks, resource gathering, expansion
}
