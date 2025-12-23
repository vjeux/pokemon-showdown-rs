//! Data-driven Ability Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines abilities as data structures with event handlers,
//! following the pattern from data/abilities.ts in the JS codebase.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;
use crate::event::{EventType, AbilityFlags, HandlerPriority};

/// Ability definition - data-driven ability with event handlers
#[derive(Debug, Clone)]
pub struct AbilityDef {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
    /// Rating for competitive viability (0-5)
    pub rating: f32,
    /// Ability flags
    pub flags: AbilityFlags,
    /// Whether this is a suppressing ability (Mold Breaker, etc.)
    pub is_suppressing: bool,
    /// Type immunity granted (e.g., "Ground" for Levitate)
    pub type_immunity: Option<String>,
    /// Status immunity granted (e.g., "par" for Limber)
    pub status_immunity: Option<Vec<String>>,
    /// Weather immunity granted (e.g., "sandstorm" for Sand Veil)
    pub weather_immunity: Option<String>,
    /// Weather set on switch-in
    pub on_switch_in_weather: Option<String>,
    /// Terrain set on switch-in
    pub on_switch_in_terrain: Option<String>,
    /// Stat boost to apply on switch-in (stat, stages, target_foe)
    pub on_switch_in_boost: Option<(String, i8, bool)>,
    /// Absorb type for healing (e.g., "Water" for Water Absorb)
    pub absorb_type: Option<String>,
    /// Heal fraction when absorbing (default 0.25)
    pub absorb_heal: Option<f64>,
    /// Boost when absorbing instead of healing (stat, stages)
    pub absorb_boost: Option<(String, i8)>,
    /// Residual boost (stat, stages) - e.g., Speed Boost
    pub residual_boost: Option<(String, i8)>,
    /// Residual heal fraction (e.g., 0.125 for Poison Heal)
    pub residual_heal: Option<f64>,
    /// Block indirect damage (Magic Guard)
    pub block_indirect_damage: bool,
    /// Contact damage fraction dealt to attacker (Iron Barbs, Rough Skin)
    pub contact_damage: Option<f64>,
    /// Contact status chance (Static, Poison Point, etc.)
    pub contact_status: Option<(String, u8)>,
    /// Stat multiplier when statused (stat, multiplier) - e.g., Guts
    pub status_stat_boost: Option<(String, f64)>,
    /// STAB multiplier override (Adaptability: 2.0)
    pub stab_multiplier: Option<f64>,
    /// Base power boost condition and multiplier (e.g., Technician)
    pub base_power_boost: Option<BasePowerBoost>,
    /// Priority for specific events
    pub priorities: HashMap<EventType, i32>,
}

/// Condition for base power boost
#[derive(Debug, Clone)]
pub enum BasePowerBoost {
    /// Boost moves with BP <= threshold by multiplier
    LowPower { threshold: u32, multiplier: f64 },
    /// Boost specific move type by multiplier
    MoveType { move_type: String, multiplier: f64 },
    /// Boost if move has specific flag
    MoveFlag { flag: String, multiplier: f64 },
}

impl Default for AbilityDef {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            rating: 0.0,
            flags: AbilityFlags::default(),
            is_suppressing: false,
            type_immunity: None,
            status_immunity: None,
            weather_immunity: None,
            on_switch_in_weather: None,
            on_switch_in_terrain: None,
            on_switch_in_boost: None,
            absorb_type: None,
            absorb_heal: None,
            absorb_boost: None,
            residual_boost: None,
            residual_heal: None,
            block_indirect_damage: false,
            contact_damage: None,
            contact_status: None,
            status_stat_boost: None,
            stab_multiplier: None,
            base_power_boost: None,
            priorities: HashMap::new(),
        }
    }
}

impl AbilityDef {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: ID::new(id),
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Check if ability grants type immunity
    pub fn grants_type_immunity(&self, move_type: &str) -> bool {
        self.type_immunity.as_ref().map_or(false, |t| t.eq_ignore_ascii_case(move_type))
    }

    /// Check if ability grants status immunity
    pub fn grants_status_immunity(&self, status: &str) -> bool {
        self.status_immunity.as_ref().map_or(false, |statuses| {
            statuses.iter().any(|s| s.eq_ignore_ascii_case(status))
        })
    }

    /// Check if ability absorbs a type
    pub fn absorbs_type(&self, move_type: &str) -> bool {
        self.absorb_type.as_ref().map_or(false, |t| t.eq_ignore_ascii_case(move_type))
    }

    /// Get the handler priority for an event
    pub fn priority_for(&self, event: &EventType) -> Option<HandlerPriority> {
        let handles = match event {
            EventType::SwitchIn => {
                self.on_switch_in_weather.is_some() ||
                self.on_switch_in_terrain.is_some() ||
                self.on_switch_in_boost.is_some()
            }
            EventType::TryHit | EventType::TryImmunity => {
                self.type_immunity.is_some() || self.absorb_type.is_some()
            }
            EventType::TrySetStatus => self.status_immunity.is_some(),
            EventType::Residual => {
                self.residual_boost.is_some() ||
                self.residual_heal.is_some() ||
                self.block_indirect_damage
            }
            EventType::DamagingHit => {
                self.contact_damage.is_some() || self.contact_status.is_some()
            }
            EventType::ModifyAtk | EventType::ModifySpA => {
                self.status_stat_boost.is_some()
            }
            EventType::BasePower => self.base_power_boost.is_some(),
            _ => false,
        };

        if handles {
            let priority = self.priorities.get(event).copied().unwrap_or(0);
            Some(HandlerPriority::with_priority(priority))
        } else {
            None
        }
    }
}

/// Static registry of all abilities
pub static ABILITIES: Lazy<HashMap<ID, AbilityDef>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // === Weather setters ===
    map.insert(ID::new("drizzle"), AbilityDef {
        id: ID::new("drizzle"),
        name: "Drizzle".to_string(),
        rating: 4.0,
        on_switch_in_weather: Some("raindance".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("drought"), AbilityDef {
        id: ID::new("drought"),
        name: "Drought".to_string(),
        rating: 4.0,
        on_switch_in_weather: Some("sunnyday".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("sandstream"), AbilityDef {
        id: ID::new("sandstream"),
        name: "Sand Stream".to_string(),
        rating: 4.0,
        on_switch_in_weather: Some("sandstorm".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("snowwarning"), AbilityDef {
        id: ID::new("snowwarning"),
        name: "Snow Warning".to_string(),
        rating: 4.0,
        on_switch_in_weather: Some("snow".to_string()),
        ..Default::default()
    });

    // === Terrain setters ===
    map.insert(ID::new("electricsurge"), AbilityDef {
        id: ID::new("electricsurge"),
        name: "Electric Surge".to_string(),
        rating: 4.0,
        on_switch_in_terrain: Some("electricterrain".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("grassysurge"), AbilityDef {
        id: ID::new("grassysurge"),
        name: "Grassy Surge".to_string(),
        rating: 4.0,
        on_switch_in_terrain: Some("grassyterrain".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("psychicsurge"), AbilityDef {
        id: ID::new("psychicsurge"),
        name: "Psychic Surge".to_string(),
        rating: 4.0,
        on_switch_in_terrain: Some("psychicterrain".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("mistysurge"), AbilityDef {
        id: ID::new("mistysurge"),
        name: "Misty Surge".to_string(),
        rating: 3.5,
        on_switch_in_terrain: Some("mistyterrain".to_string()),
        ..Default::default()
    });

    // === Type immunities ===
    map.insert(ID::new("levitate"), AbilityDef {
        id: ID::new("levitate"),
        name: "Levitate".to_string(),
        rating: 3.5,
        type_immunity: Some("Ground".to_string()),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("flashfire"), AbilityDef {
        id: ID::new("flashfire"),
        name: "Flash Fire".to_string(),
        rating: 3.5,
        type_immunity: Some("Fire".to_string()),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    // === Absorbing abilities ===
    map.insert(ID::new("waterabsorb"), AbilityDef {
        id: ID::new("waterabsorb"),
        name: "Water Absorb".to_string(),
        rating: 3.5,
        absorb_type: Some("Water".to_string()),
        absorb_heal: Some(0.25),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("voltabsorb"), AbilityDef {
        id: ID::new("voltabsorb"),
        name: "Volt Absorb".to_string(),
        rating: 3.5,
        absorb_type: Some("Electric".to_string()),
        absorb_heal: Some(0.25),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dryskin"), AbilityDef {
        id: ID::new("dryskin"),
        name: "Dry Skin".to_string(),
        rating: 3.0,
        absorb_type: Some("Water".to_string()),
        absorb_heal: Some(0.25),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("motordrive"), AbilityDef {
        id: ID::new("motordrive"),
        name: "Motor Drive".to_string(),
        rating: 3.0,
        absorb_type: Some("Electric".to_string()),
        absorb_boost: Some(("spe".to_string(), 1)),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("lightningrod"), AbilityDef {
        id: ID::new("lightningrod"),
        name: "Lightning Rod".to_string(),
        rating: 3.5,
        absorb_type: Some("Electric".to_string()),
        absorb_boost: Some(("spa".to_string(), 1)),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("stormdrain"), AbilityDef {
        id: ID::new("stormdrain"),
        name: "Storm Drain".to_string(),
        rating: 3.5,
        absorb_type: Some("Water".to_string()),
        absorb_boost: Some(("spa".to_string(), 1)),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("sapsipper"), AbilityDef {
        id: ID::new("sapsipper"),
        name: "Sap Sipper".to_string(),
        rating: 3.5,
        absorb_type: Some("Grass".to_string()),
        absorb_boost: Some(("atk".to_string(), 1)),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    // === Switch-in stat modifiers ===
    map.insert(ID::new("intimidate"), AbilityDef {
        id: ID::new("intimidate"),
        name: "Intimidate".to_string(),
        rating: 3.5,
        on_switch_in_boost: Some(("atk".to_string(), -1, true)),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    // === Residual abilities ===
    map.insert(ID::new("speedboost"), AbilityDef {
        id: ID::new("speedboost"),
        name: "Speed Boost".to_string(),
        rating: 4.5,
        residual_boost: Some(("spe".to_string(), 1)),
        ..Default::default()
    });

    map.insert(ID::new("poisonheal"), AbilityDef {
        id: ID::new("poisonheal"),
        name: "Poison Heal".to_string(),
        rating: 4.0,
        residual_heal: Some(0.125),
        ..Default::default()
    });

    map.insert(ID::new("magicguard"), AbilityDef {
        id: ID::new("magicguard"),
        name: "Magic Guard".to_string(),
        rating: 4.0,
        block_indirect_damage: true,
        ..Default::default()
    });

    // === Contact retaliation ===
    map.insert(ID::new("ironbarbs"), AbilityDef {
        id: ID::new("ironbarbs"),
        name: "Iron Barbs".to_string(),
        rating: 2.5,
        contact_damage: Some(1.0 / 8.0),
        ..Default::default()
    });

    map.insert(ID::new("roughskin"), AbilityDef {
        id: ID::new("roughskin"),
        name: "Rough Skin".to_string(),
        rating: 2.5,
        contact_damage: Some(1.0 / 8.0),
        ..Default::default()
    });

    map.insert(ID::new("static"), AbilityDef {
        id: ID::new("static"),
        name: "Static".to_string(),
        rating: 2.0,
        contact_status: Some(("par".to_string(), 30)),
        ..Default::default()
    });

    map.insert(ID::new("poisonpoint"), AbilityDef {
        id: ID::new("poisonpoint"),
        name: "Poison Point".to_string(),
        rating: 1.5,
        contact_status: Some(("psn".to_string(), 30)),
        ..Default::default()
    });

    map.insert(ID::new("flamebody"), AbilityDef {
        id: ID::new("flamebody"),
        name: "Flame Body".to_string(),
        rating: 2.0,
        contact_status: Some(("brn".to_string(), 30)),
        ..Default::default()
    });

    // === Status-boosting abilities ===
    map.insert(ID::new("guts"), AbilityDef {
        id: ID::new("guts"),
        name: "Guts".to_string(),
        rating: 3.5,
        status_stat_boost: Some(("atk".to_string(), 1.5)),
        ..Default::default()
    });

    map.insert(ID::new("marvelscale"), AbilityDef {
        id: ID::new("marvelscale"),
        name: "Marvel Scale".to_string(),
        rating: 2.5,
        status_stat_boost: Some(("def".to_string(), 1.5)),
        ..Default::default()
    });

    map.insert(ID::new("quickfeet"), AbilityDef {
        id: ID::new("quickfeet"),
        name: "Quick Feet".to_string(),
        rating: 2.5,
        status_stat_boost: Some(("spe".to_string(), 1.5)),
        ..Default::default()
    });

    // === STAB modifiers ===
    map.insert(ID::new("adaptability"), AbilityDef {
        id: ID::new("adaptability"),
        name: "Adaptability".to_string(),
        rating: 4.0,
        stab_multiplier: Some(2.0),
        ..Default::default()
    });

    // === Base power modifiers ===
    map.insert(ID::new("technician"), AbilityDef {
        id: ID::new("technician"),
        name: "Technician".to_string(),
        rating: 3.5,
        base_power_boost: Some(BasePowerBoost::LowPower { threshold: 60, multiplier: 1.5 }),
        ..Default::default()
    });

    map.insert(ID::new("ironfist"), AbilityDef {
        id: ID::new("ironfist"),
        name: "Iron Fist".to_string(),
        rating: 3.0,
        base_power_boost: Some(BasePowerBoost::MoveFlag { flag: "punch".to_string(), multiplier: 1.2 }),
        ..Default::default()
    });

    map.insert(ID::new("strongjaw"), AbilityDef {
        id: ID::new("strongjaw"),
        name: "Strong Jaw".to_string(),
        rating: 3.5,
        base_power_boost: Some(BasePowerBoost::MoveFlag { flag: "bite".to_string(), multiplier: 1.5 }),
        ..Default::default()
    });

    map.insert(ID::new("megalauncher"), AbilityDef {
        id: ID::new("megalauncher"),
        name: "Mega Launcher".to_string(),
        rating: 3.5,
        base_power_boost: Some(BasePowerBoost::MoveFlag { flag: "pulse".to_string(), multiplier: 1.5 }),
        ..Default::default()
    });

    // === Status immunities ===
    map.insert(ID::new("limber"), AbilityDef {
        id: ID::new("limber"),
        name: "Limber".to_string(),
        rating: 2.0,
        status_immunity: Some(vec!["par".to_string()]),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("immunity"), AbilityDef {
        id: ID::new("immunity"),
        name: "Immunity".to_string(),
        rating: 2.0,
        status_immunity: Some(vec!["psn".to_string(), "tox".to_string()]),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("waterveil"), AbilityDef {
        id: ID::new("waterveil"),
        name: "Water Veil".to_string(),
        rating: 2.0,
        status_immunity: Some(vec!["brn".to_string()]),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("magmaarmor"), AbilityDef {
        id: ID::new("magmaarmor"),
        name: "Magma Armor".to_string(),
        rating: 1.0,
        status_immunity: Some(vec!["frz".to_string()]),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("insomnia"), AbilityDef {
        id: ID::new("insomnia"),
        name: "Insomnia".to_string(),
        rating: 2.0,
        status_immunity: Some(vec!["slp".to_string()]),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("vitalspirit"), AbilityDef {
        id: ID::new("vitalspirit"),
        name: "Vital Spirit".to_string(),
        rating: 2.0,
        status_immunity: Some(vec!["slp".to_string()]),
        flags: AbilityFlags { breakable: true, ..Default::default() },
        ..Default::default()
    });

    // === Mold Breaker variants ===
    map.insert(ID::new("moldbreaker"), AbilityDef {
        id: ID::new("moldbreaker"),
        name: "Mold Breaker".to_string(),
        rating: 3.5,
        is_suppressing: true,
        ..Default::default()
    });

    map.insert(ID::new("teravolt"), AbilityDef {
        id: ID::new("teravolt"),
        name: "Teravolt".to_string(),
        rating: 3.5,
        is_suppressing: true,
        ..Default::default()
    });

    map.insert(ID::new("turboblaze"), AbilityDef {
        id: ID::new("turboblaze"),
        name: "Turboblaze".to_string(),
        rating: 3.5,
        is_suppressing: true,
        ..Default::default()
    });

    map
});

/// Get ability definition by ID
pub fn get_ability(id: &ID) -> Option<&'static AbilityDef> {
    ABILITIES.get(id)
}

/// Check if an ability grants immunity to a type
pub fn ability_grants_type_immunity(ability_id: &ID, move_type: &str) -> bool {
    get_ability(ability_id).map_or(false, |a| a.grants_type_immunity(move_type))
}

/// Check if an ability absorbs a type (for healing or stat boost)
pub fn ability_absorbs_type(ability_id: &ID, move_type: &str) -> bool {
    get_ability(ability_id).map_or(false, |a| a.absorbs_type(move_type))
}

/// Check if an ability grants status immunity
pub fn ability_grants_status_immunity(ability_id: &ID, status: &str) -> bool {
    get_ability(ability_id).map_or(false, |a| a.grants_status_immunity(status))
}

/// Check if an ability is a suppressing ability (Mold Breaker, etc.)
pub fn ability_is_suppressing(ability_id: &ID) -> bool {
    get_ability(ability_id).map_or(false, |a| a.is_suppressing)
}

/// Check if an ability can be suppressed by Mold Breaker
pub fn ability_is_breakable(ability_id: &ID) -> bool {
    get_ability(ability_id).map_or(false, |a| a.flags.breakable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levitate_immunity() {
        let levitate = ID::new("levitate");
        assert!(ability_grants_type_immunity(&levitate, "Ground"));
        assert!(!ability_grants_type_immunity(&levitate, "Fire"));
    }

    #[test]
    fn test_water_absorb() {
        let water_absorb = ID::new("waterabsorb");
        assert!(ability_absorbs_type(&water_absorb, "Water"));
        assert!(!ability_absorbs_type(&water_absorb, "Fire"));

        let ability = get_ability(&water_absorb).unwrap();
        assert_eq!(ability.absorb_heal, Some(0.25));
    }

    #[test]
    fn test_intimidate() {
        let intimidate = ID::new("intimidate");
        let ability = get_ability(&intimidate).unwrap();
        assert_eq!(ability.on_switch_in_boost, Some(("atk".to_string(), -1, true)));
    }

    #[test]
    fn test_mold_breaker() {
        let mold_breaker = ID::new("moldbreaker");
        assert!(ability_is_suppressing(&mold_breaker));
        assert!(!ability_is_breakable(&mold_breaker));

        let levitate = ID::new("levitate");
        assert!(ability_is_breakable(&levitate));
    }

    #[test]
    fn test_status_immunity() {
        let limber = ID::new("limber");
        assert!(ability_grants_status_immunity(&limber, "par"));
        assert!(!ability_grants_status_immunity(&limber, "brn"));

        let immunity = ID::new("immunity");
        assert!(ability_grants_status_immunity(&immunity, "psn"));
        assert!(ability_grants_status_immunity(&immunity, "tox"));
    }
}
