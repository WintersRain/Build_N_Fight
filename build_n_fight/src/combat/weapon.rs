//! Weapon definitions and behavior
//!
//! Weapons are data-driven and mountable.
//! Each weapon type has different targeting capabilities.

use bevy::prelude::*;

/// Weapon categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponCategory {
    Ballista,   // High damage, single target, horizontal only
    Mortar,     // AoE, can target below, slow
    Flamer,     // Cone damage, short range, good for breaches
    BombDrop,   // Drops explosive into tunnel, requires LOS down
    Crossbow,   // Medium damage, fast, horizontal
}

/// Targeting angle capabilities
#[derive(Debug, Clone, Copy, Default)]
pub struct TargetingAngles {
    pub horizontal: bool,   // Can target same Z-level
    pub above: bool,        // Can target higher Z
    pub below: bool,        // Can target lower Z (key for vertical combat!)
}

impl TargetingAngles {
    pub const HORIZONTAL_ONLY: Self = Self {
        horizontal: true,
        above: false,
        below: false,
    };

    pub const ALL_DIRECTIONS: Self = Self {
        horizontal: true,
        above: true,
        below: true,
    };

    pub const BELOW_ONLY: Self = Self {
        horizontal: false,
        above: false,
        below: true,
    };
}

/// Weapon component
#[derive(Component, Clone)]
pub struct Weapon {
    pub category: WeaponCategory,
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,             // Shots per second
    pub aoe_radius: Option<f32>,    // None = single target
    pub targeting: TargetingAngles,
    pub ammo_type: Option<AmmoType>,
    pub ammo_capacity: u32,
    pub current_ammo: u32,
    /// Time until next shot
    pub cooldown: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmmoType {
    Bolt,       // Ballista/Crossbow
    Shell,      // Mortar
    Fuel,       // Flamer
    Bomb,       // BombDrop
}

impl Weapon {
    pub fn ballista() -> Self {
        Self {
            category: WeaponCategory::Ballista,
            damage: 50.0,
            range: 15.0,
            fire_rate: 0.5,
            aoe_radius: None,
            targeting: TargetingAngles::HORIZONTAL_ONLY,
            ammo_type: Some(AmmoType::Bolt),
            ammo_capacity: 20,
            current_ammo: 20,
            cooldown: 0.0,
        }
    }

    pub fn mortar() -> Self {
        Self {
            category: WeaponCategory::Mortar,
            damage: 30.0,
            range: 20.0,
            fire_rate: 0.25,
            aoe_radius: Some(3.0),
            targeting: TargetingAngles::ALL_DIRECTIONS,
            ammo_type: Some(AmmoType::Shell),
            ammo_capacity: 10,
            current_ammo: 10,
            cooldown: 0.0,
        }
    }

    pub fn flamer() -> Self {
        Self {
            category: WeaponCategory::Flamer,
            damage: 10.0,  // Per tick
            range: 5.0,
            fire_rate: 10.0,  // Continuous
            aoe_radius: Some(2.0),  // Cone approximated as AoE
            targeting: TargetingAngles::HORIZONTAL_ONLY,
            ammo_type: Some(AmmoType::Fuel),
            ammo_capacity: 100,
            current_ammo: 100,
            cooldown: 0.0,
        }
    }

    pub fn bomb_drop() -> Self {
        Self {
            category: WeaponCategory::BombDrop,
            damage: 100.0,
            range: 1.0,  // Must be directly above
            fire_rate: 0.1,
            aoe_radius: Some(4.0),
            targeting: TargetingAngles::BELOW_ONLY,
            ammo_type: Some(AmmoType::Bomb),
            ammo_capacity: 5,
            current_ammo: 5,
            cooldown: 0.0,
        }
    }

    /// Check if weapon can fire
    pub fn can_fire(&self) -> bool {
        self.cooldown <= 0.0 && self.current_ammo > 0
    }

    /// Fire the weapon, returns true if successful
    pub fn fire(&mut self) -> bool {
        if self.can_fire() {
            self.current_ammo -= 1;
            self.cooldown = 1.0 / self.fire_rate;
            true
        } else {
            false
        }
    }

    /// Update cooldown
    pub fn update(&mut self, dt: f32) {
        if self.cooldown > 0.0 {
            self.cooldown -= dt;
        }
    }

    /// Reload ammo
    pub fn reload(&mut self, amount: u32) {
        self.current_ammo = (self.current_ammo + amount).min(self.ammo_capacity);
    }

    /// Can this weapon target a position relative to itself?
    pub fn can_target(&self, relative_pos: IVec3) -> bool {
        let dz = relative_pos.z;

        if dz == 0 && self.targeting.horizontal {
            return true;
        }
        if dz > 0 && self.targeting.above {
            return true;
        }
        if dz < 0 && self.targeting.below {
            return true;
        }

        false
    }
}

/// System to update weapons (cooldowns, auto-targeting)
pub fn update_weapons(
    mut weapons: Query<(&mut Weapon, &Transform)>,
    time: Res<Time>,
    // TODO: Add targeting queries
) {
    let dt = time.delta_secs();

    for (mut weapon, _transform) in weapons.iter_mut() {
        weapon.update(dt);

        // TODO: Auto-targeting based on weapon type and available targets
    }
}
