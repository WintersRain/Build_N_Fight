//! Z-level management for layer-based viewing

use bevy::prelude::*;

/// Resource tracking the currently viewed Z-level
#[derive(Resource, Default)]
pub struct CurrentZLevel {
    pub level: i32,
}

/// System to handle Z-level switching with [ and ] keys
pub fn handle_z_level_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_z: ResMut<CurrentZLevel>,
) {
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        current_z.level -= 1;
        info!("Z-level: {}", current_z.level);
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        current_z.level += 1;
        info!("Z-level: {}", current_z.level);
    }
}
