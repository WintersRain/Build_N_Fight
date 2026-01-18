//! Build N' Fight - Asymmetric Underground Warfare Tower Defense
//!
//! A 2D voxel tower defense game where the player defends a surface base
//! against an ant ecology that expands, evolves, and eventually discovers
//! and assaults them from below.

use bevy::prelude::*;

mod ai;
mod combat;
mod flow;
mod player;
mod render;
mod visibility;
mod world;

/// Game states for managing different phases
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Build N' Fight".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        // Game state
        .init_state::<GameState>()
        // Core game plugins
        .add_plugins((
            world::WorldPlugin,
            flow::FlowPlugin,
            ai::AiPlugin,
            combat::CombatPlugin,
            player::PlayerPlugin,
            visibility::VisibilityPlugin,
            render::RenderPlugin,
        ))
        // Startup systems
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    info!("Build N' Fight initialized");
}
