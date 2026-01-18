//! Basic UI rendering

use crate::player::{BuildMode, PlayerResources};
use crate::world::CurrentZLevel;
use bevy::prelude::*;

/// System to render basic UI info
pub fn render_ui(
    resources: Res<PlayerResources>,
    build_mode: Res<BuildMode>,
    current_z: Res<CurrentZLevel>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Only show on U key press
    if !keyboard.just_pressed(KeyCode::KeyU) {
        return;
    }

    println!("\n=== STATUS ===");
    println!("Z-Level: {}", current_z.level);
    println!("Resources: {} tungsten, {} iron, {} wood",
        resources.tungsten, resources.iron, resources.wood);
    println!("Build Mode: {} (Selected: {:?})",
        if build_mode.active { "ON" } else { "OFF" },
        build_mode.selected);
    println!("\nControls:");
    println!("  WASD/Arrows - Move camera");
    println!("  [ ] - Change Z-level");
    println!("  B - Toggle build mode");
    println!("  1-6 - Select build type (in build mode)");
    println!("  R - Refresh ASCII view");
    println!("  U - Show this status");
}
