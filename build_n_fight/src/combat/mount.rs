//! Mount point system for modular weapons
//!
//! Structures can have mount points where weapons attach.
//! Click to select, click to swap weapons.

use super::*;
use bevy::prelude::*;

/// A mount point on a structure
#[derive(Component)]
pub struct MountPoint {
    /// Currently mounted weapon entity (if any)
    pub mounted_weapon: Option<Entity>,
    /// What weapon categories can be mounted here
    pub allowed_categories: Vec<WeaponCategory>,
}

impl MountPoint {
    pub fn new(allowed: Vec<WeaponCategory>) -> Self {
        Self {
            mounted_weapon: None,
            allowed_categories: allowed,
        }
    }

    /// Universal mount point (accepts any weapon)
    pub fn universal() -> Self {
        Self::new(vec![
            WeaponCategory::Ballista,
            WeaponCategory::Mortar,
            WeaponCategory::Flamer,
            WeaponCategory::BombDrop,
            WeaponCategory::Crossbow,
        ])
    }

    /// Check if a weapon category can be mounted here
    pub fn can_mount(&self, category: WeaponCategory) -> bool {
        self.allowed_categories.contains(&category)
    }

    /// Check if this mount point has a weapon
    pub fn is_occupied(&self) -> bool {
        self.mounted_weapon.is_some()
    }
}

/// Player's weapon inventory
#[derive(Resource, Default)]
pub struct WeaponInventory {
    pub available: hashbrown::HashMap<WeaponCategory, u32>,
}

impl WeaponInventory {
    pub fn add(&mut self, category: WeaponCategory, count: u32) {
        *self.available.entry(category).or_insert(0) += count;
    }

    pub fn remove(&mut self, category: WeaponCategory) -> bool {
        if let Some(count) = self.available.get_mut(&category) {
            if *count > 0 {
                *count -= 1;
                return true;
            }
        }
        false
    }

    pub fn count(&self, category: WeaponCategory) -> u32 {
        *self.available.get(&category).unwrap_or(&0)
    }
}

/// Mount a weapon to a mount point
pub fn mount_weapon(
    commands: &mut Commands,
    mount_entity: Entity,
    mount_point: &mut MountPoint,
    weapon: Weapon,
) -> Option<Entity> {
    if !mount_point.can_mount(weapon.category) {
        return None;
    }

    // Despawn old weapon if any
    if let Some(old_weapon) = mount_point.mounted_weapon {
        commands.entity(old_weapon).despawn();
    }

    // Spawn new weapon as child of mount
    let weapon_entity = commands
        .spawn((weapon, Transform::default()))
        .set_parent(mount_entity)
        .id();

    mount_point.mounted_weapon = Some(weapon_entity);
    Some(weapon_entity)
}

/// Unmount a weapon from a mount point
pub fn unmount_weapon(
    commands: &mut Commands,
    mount_point: &mut MountPoint,
) -> Option<WeaponCategory> {
    if let Some(weapon_entity) = mount_point.mounted_weapon.take() {
        // TODO: Get weapon category before despawning
        commands.entity(weapon_entity).despawn();
        // Return the category so it can be added back to inventory
        Some(WeaponCategory::Ballista) // Placeholder
    } else {
        None
    }
}
