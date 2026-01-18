//! ASCII rendering for prototype
//!
//! Renders the world as ASCII characters.
//! Simple but functional for testing mechanics.

use crate::visibility::{FogOfWar, TileVisibility};
use crate::world::{CurrentZLevel, GameWorld};
use bevy::prelude::*;

/// System to render the world as ASCII (to console for now)
pub fn render_world_ascii(
    world: Res<GameWorld>,
    fog: Res<FogOfWar>,
    current_z: Res<CurrentZLevel>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Only render on R key press (to avoid spam)
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    let z = current_z.level;
    let view_size = 20; // 20x20 view

    println!("\n=== Z-Level {} ===", z);
    println!("[ ] to change level, R to refresh view\n");

    // Build the ASCII grid
    for y in (0..view_size).rev() {
        let mut line = String::new();

        for x in 0..view_size {
            let pos = IVec3::new(x, y, z);
            let visibility = fog.get(pos);

            let ch = match visibility {
                TileVisibility::Unknown => ' ',
                TileVisibility::Revealed | TileVisibility::Visible => {
                    if let Some(tile) = world.get_tile(pos) {
                        tile.to_ascii()
                    } else {
                        '~' // Ungenerated
                    }
                }
                TileVisibility::SonarContact => '?',
            };

            line.push(ch);
            line.push(' '); // Spacing for readability
        }

        println!("{}", line);
    }

    println!("\nLegend: . air  , dirt  # stone  = wood wall  H stone wall  M metal wall");
    println!("        _ floor  % rubble  o tunnel  O nest  ? sonar contact");
}
