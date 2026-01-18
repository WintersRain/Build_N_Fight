//! Input handling

use super::*;
use crate::world::CurrentZLevel;
use bevy::prelude::*;

/// System to handle player input
pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut build_mode: ResMut<BuildMode>,
    mut current_z: ResMut<CurrentZLevel>,
    // TODO: Add cursor position tracking for build/dig commands
) {
    // Toggle build mode with B
    if keyboard.just_pressed(KeyCode::KeyB) {
        build_mode.active = !build_mode.active;
        info!("Build mode: {}", if build_mode.active { "ON" } else { "OFF" });
    }

    // Cycle build types with 1-6
    if build_mode.active {
        if keyboard.just_pressed(KeyCode::Digit1) {
            build_mode.selected = BuildableType::WoodWall;
            info!("Selected: Wood Wall");
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            build_mode.selected = BuildableType::StoneWall;
            info!("Selected: Stone Wall");
        }
        if keyboard.just_pressed(KeyCode::Digit3) {
            build_mode.selected = BuildableType::MetalWall;
            info!("Selected: Metal Wall");
        }
        if keyboard.just_pressed(KeyCode::Digit4) {
            build_mode.selected = BuildableType::WoodFloor;
            info!("Selected: Wood Floor");
        }
        if keyboard.just_pressed(KeyCode::Digit5) {
            build_mode.selected = BuildableType::StoneFloor;
            info!("Selected: Stone Floor");
        }
        if keyboard.just_pressed(KeyCode::Digit6) {
            build_mode.selected = BuildableType::Turret;
            info!("Selected: Turret");
        }
    }

    // Z-level navigation with [ and ]
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        current_z.level -= 1;
        info!("Z-level: {}", current_z.level);
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        current_z.level += 1;
        info!("Z-level: {}", current_z.level);
    }

    // TODO: Mouse click to build/dig at cursor position
}
