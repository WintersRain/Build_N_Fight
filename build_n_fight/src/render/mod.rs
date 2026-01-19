//! Render module - Visual tile rendering
//!
//! Renders tiles as colored squares in the game window.

use bevy::prelude::*;

use crate::world::{CurrentZLevel, GameWorld, Tile};

mod camera;

pub use camera::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderSettings>()
            .add_systems(Startup, setup_tile_sprites)
            .add_systems(Update, (
                update_tile_sprites,
                handle_camera_input,
                update_ui_text,
            ));
    }
}

/// Render settings
#[derive(Resource)]
pub struct RenderSettings {
    pub tile_size: f32,
    pub view_size: i32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            tile_size: 20.0,
            view_size: 32,
        }
    }
}

/// Marker for tile sprites
#[derive(Component)]
pub struct TileSprite {
    pub world_pos: IVec3,
}

/// UI text component
#[derive(Component)]
pub struct UiText;

/// Setup tile sprites for the visible area
fn setup_tile_sprites(
    mut commands: Commands,
    settings: Res<RenderSettings>,
) {
    let size = settings.view_size;
    let tile_size = settings.tile_size;

    // Create sprites for each tile position
    for x in 0..size {
        for y in 0..size {
            let world_x = (x as f32 - size as f32 / 2.0) * tile_size;
            let world_y = (y as f32 - size as f32 / 2.0) * tile_size;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.2, 0.2),
                    custom_size: Some(Vec2::splat(tile_size - 1.0)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
                TileSprite {
                    world_pos: IVec3::new(x, y, 0),
                },
            ));
        }
    }

    // Create UI text
    commands.spawn((
        Text2d::new("Z: 0 | WASD: move | []: Z-level | B: build"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 340.0, 10.0),
        UiText,
    ));

    info!("Created {} tile sprites", size * size);
}

/// Update tile sprite colors based on world state
fn update_tile_sprites(
    world: Res<GameWorld>,
    current_z: Res<CurrentZLevel>,
    mut sprites: Query<(&mut Sprite, &mut TileSprite)>,
) {
    let z = current_z.level;

    for (mut sprite, mut tile_sprite) in sprites.iter_mut() {
        // Update the Z-level being displayed
        tile_sprite.world_pos.z = z;
        let pos = tile_sprite.world_pos;

        let color = if let Some(tile) = world.get_tile(pos) {
            tile_to_color(tile, z)
        } else {
            // Ungenerated area
            Color::srgb(0.1, 0.1, 0.15)
        };

        sprite.color = color;
    }
}

/// Convert tile to display color
fn tile_to_color(tile: &Tile, z: i32) -> Color {
    match tile {
        Tile::Air => {
            if z >= 0 {
                // Surface air - sky blue
                Color::srgb(0.4, 0.6, 0.8)
            } else {
                // Underground air (tunnel) - dark
                Color::srgb(0.15, 0.12, 0.1)
            }
        }
        Tile::Dirt { hp, max_hp } => {
            let health = *hp as f32 / *max_hp as f32;
            Color::srgb(0.4 * health + 0.2, 0.25 * health + 0.1, 0.1)
        }
        Tile::Stone { hp, max_hp } => {
            let health = *hp as f32 / *max_hp as f32;
            let gray = 0.3 + 0.3 * health;
            Color::srgb(gray, gray, gray)
        }
        Tile::Wall { hp, max_hp, material } => {
            let health = *hp as f32 / *max_hp as f32;
            match material {
                crate::world::BuildMaterial::Wood => {
                    Color::srgb(0.5 * health + 0.2, 0.3 * health + 0.1, 0.1)
                }
                crate::world::BuildMaterial::Stone => {
                    Color::srgb(0.5 * health + 0.2, 0.5 * health + 0.2, 0.4 * health + 0.2)
                }
                crate::world::BuildMaterial::Metal => {
                    Color::srgb(0.4 * health + 0.2, 0.4 * health + 0.2, 0.5 * health + 0.3)
                }
            }
        }
        Tile::Floor { material, .. } => {
            match material {
                crate::world::BuildMaterial::Wood => Color::srgb(0.45, 0.3, 0.15),
                crate::world::BuildMaterial::Stone => Color::srgb(0.4, 0.4, 0.35),
                crate::world::BuildMaterial::Metal => Color::srgb(0.35, 0.35, 0.4),
            }
        }
        Tile::Rubble => Color::srgb(0.35, 0.3, 0.25),
        Tile::AntStructure { structure_type, .. } => {
            match structure_type {
                crate::world::AntStructureType::Tunnel => Color::srgb(0.3, 0.2, 0.15),
                crate::world::AntStructureType::Nest => Color::srgb(0.5, 0.2, 0.2),
                crate::world::AntStructureType::Storage => Color::srgb(0.4, 0.35, 0.2),
            }
        }
    }
}

/// Handle camera movement and Z-level changes
fn handle_camera_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut current_z: ResMut<CurrentZLevel>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };

    // Camera movement
    let speed = 200.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * speed * time.delta_secs();
    }

    // Z-level changes
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        current_z.level -= 1;
        info!("Z-level: {}", current_z.level);
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        current_z.level += 1;
        info!("Z-level: {}", current_z.level);
    }
}

/// Update UI text with current state
fn update_ui_text(
    current_z: Res<CurrentZLevel>,
    mut text_query: Query<&mut Text2d, With<UiText>>,
    camera: Query<&Transform, With<Camera2d>>,
) {
    let Ok(mut text) = text_query.get_single_mut() else {
        return;
    };

    let cam_pos = camera.get_single().map(|t| t.translation).unwrap_or_default();

    **text = format!(
        "Z: {} | Pos: ({:.0}, {:.0}) | WASD: move | []: Z-level",
        current_z.level,
        cam_pos.x,
        cam_pos.y
    );
}
