//! Render module - ASCII and simple 2D rendering
//!
//! Start with ASCII, upgrade to sprites later.

use bevy::prelude::*;

mod ascii;
mod camera;
mod ui;

pub use ascii::*;
pub use camera::*;
pub use ui::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderSettings>()
            .add_systems(Update, (render_world_ascii, render_ui));
    }
}

/// Render settings
#[derive(Resource)]
pub struct RenderSettings {
    pub tile_size: f32,
    pub show_grid: bool,
    pub ascii_mode: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            tile_size: 16.0,
            show_grid: true,
            ascii_mode: true,  // Start with ASCII
        }
    }
}
