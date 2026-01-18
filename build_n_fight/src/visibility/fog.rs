//! Fog of war system
//!
//! Surface is visible.
//! Underground is hidden until explored.

use bevy::prelude::*;

/// Visibility state for a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TileVisibility {
    #[default]
    Unknown,        // Never seen
    Revealed,       // Seen before, not currently visible
    Visible,        // Currently visible
    SonarContact,   // Something detected, unknown what
}

/// Fog of war resource
#[derive(Resource, Default)]
pub struct FogOfWar {
    /// Visibility state per tile
    visibility: hashbrown::HashMap<IVec3, TileVisibility>,
    /// Surface Z level (always visible)
    pub surface_z: i32,
}

impl FogOfWar {
    /// Get visibility at a position
    pub fn get(&self, pos: IVec3) -> TileVisibility {
        // Surface is always visible
        if pos.z >= self.surface_z {
            return TileVisibility::Visible;
        }

        *self.visibility.get(&pos).unwrap_or(&TileVisibility::Unknown)
    }

    /// Set visibility at a position
    pub fn set(&mut self, pos: IVec3, visibility: TileVisibility) {
        self.visibility.insert(pos, visibility);
    }

    /// Reveal tiles around a position
    pub fn reveal_around(&mut self, center: IVec3, radius: i32) {
        for x in -radius..=radius {
            for y in -radius..=radius {
                for z in -radius..=radius {
                    let pos = center + IVec3::new(x, y, z);
                    if self.get(pos) == TileVisibility::Unknown {
                        self.set(pos, TileVisibility::Revealed);
                    }
                }
            }
        }
    }

    /// Mark tiles as currently visible (for units with vision)
    pub fn mark_visible(&mut self, center: IVec3, radius: i32) {
        for x in -radius..=radius {
            for y in -radius..=radius {
                for z in -1..=1 {  // Limited vertical vision
                    let pos = center + IVec3::new(x, y, z);
                    self.set(pos, TileVisibility::Visible);
                }
            }
        }
    }

    /// Clear visible status (called each frame before recalculating)
    pub fn clear_visible(&mut self) {
        for (_, vis) in self.visibility.iter_mut() {
            if *vis == TileVisibility::Visible {
                *vis = TileVisibility::Revealed;
            }
            if *vis == TileVisibility::SonarContact {
                *vis = TileVisibility::Revealed;
            }
        }
    }
}

/// System to update fog of war
pub fn update_fog(
    mut fog: ResMut<FogOfWar>,
    // TODO: Query player units to update visible areas
) {
    // Clear visibility each frame
    fog.clear_visible();

    // TODO: Mark visible around each player unit
}
