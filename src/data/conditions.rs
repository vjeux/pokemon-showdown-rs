//! Data-driven Condition Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines conditions (status effects, volatiles, side conditions,
//! weather, terrain) as data structures following the JS architecture.

use crate::dex_data::ID;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde::Deserialize;

/// Type of condition
/// JavaScript equivalent: EffectType for conditions (sim/dex-conditions.ts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConditionType {
    /// Non-volatile status (burn, paralysis, poison, sleep, freeze)
    Status,
    /// Volatile status (confusion, taunt, encore, etc.)
    #[default]
    Volatile,
    /// Side condition (stealth rock, spikes, reflect, etc.)
    SideCondition,
    /// Slot condition (Wish, Healing Wish)
    SlotCondition,
    /// Weather (rain, sun, sand, hail)
    Weather,
    /// Terrain (electric, grassy, psychic, misty)
    Terrain,
    /// Pseudo-weather (Trick Room, Magic Room, etc.)
    PseudoWeather,
}

/// Condition definition
/// JavaScript equivalent: ConditionData (sim/dex-conditions.ts)
/// ~30 fields in JavaScript
#[derive(Debug, Clone)]
pub struct ConditionDef {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
    /// Condition type
    pub condition_type: ConditionType,
    /// Duration in turns (None = indefinite or until cured)
    pub duration: Option<i32>,
    /// Maximum duration (for variable durations like sleep)
    pub max_duration: Option<i32>,
    /// Minimum duration
    pub min_duration: Option<i32>,
    /// Can be passed by Baton Pass
    pub baton_passable: bool,
    /// Prevents switching
    pub traps: bool,
    /// Maximum layers (for stacking conditions like Spikes)
    pub max_layers: Option<u8>,

    // === Status effects ===
    /// Residual damage per turn (fraction of max HP)
    pub residual_damage: Option<f64>,
    /// Residual damage increases each turn (Toxic)
    pub escalating_damage: bool,
    /// Speed reduction (paralysis = 0.5)
    pub speed_modifier: Option<f64>,
    /// Attack reduction (burn = 0.5 for physical)
    pub attack_modifier: Option<f64>,
    /// Chance to skip turn (paralysis = 25, sleep = 100 until wake)
    pub skip_turn_chance: Option<u8>,
    /// Prevents certain moves (Taunt prevents status moves)
    pub move_restriction: Option<MoveRestriction>,

    // === Volatile effects ===
    /// Damage on each turn (trapped, seeded, etc.)
    pub volatile_damage: Option<f64>,
    /// Heal on each turn (Aqua Ring, Ingrain)
    pub volatile_heal: Option<f64>,
    /// Confusion self-hit chance
    pub confusion_chance: Option<u8>,
    /// Stat boosts while active
    pub stat_boosts: Option<Vec<(String, i8)>>,
    /// Protection from all moves this turn
    pub protection: bool,

    // === Side conditions ===
    /// Entry hazard damage type
    pub hazard_type: Option<String>,
    /// Entry hazard damage (fraction based on type effectiveness for SR)
    pub hazard_damage: Option<f64>,
    /// Screen reduction (Reflect, Light Screen, Aurora Veil)
    pub screen_reduction: Option<f64>,
    /// Applies status on entry (Toxic Spikes)
    pub entry_status: Option<String>,
    /// Speed reduction on entry (Sticky Web)
    pub entry_speed_drop: Option<i8>,

    // === Weather effects ===
    /// Boosts type damage by multiplier
    pub type_boost: Option<(String, f64)>,
    /// Weakens type damage by multiplier
    pub type_weaken: Option<(String, f64)>,
    /// Residual damage to non-immune types
    pub weather_damage: Option<f64>,
    /// Types immune to weather damage
    pub immune_types: Option<Vec<String>>,
    /// Abilities immune to weather damage
    pub immune_abilities: Option<Vec<String>>,

    // === Terrain effects ===
    /// Grounded Pokemon only
    pub grounded_only: bool,
    /// Prevents status
    pub prevents_status: Option<Vec<String>>,
    /// Blocks priority moves against grounded Pokemon
    pub blocks_priority: bool,
}

/// Move restriction types for conditions
/// TODO: Not in JavaScript - Rust-specific enum for organizing move restrictions
/// JavaScript uses individual condition properties and checks
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveRestriction {
    /// Taunt: No status moves
    NoStatusMoves,
    /// Encore: Must repeat last move
    EncoreMove(ID),
    /// Disable: Cannot use specific move
    DisabledMove(ID),
    /// Torment: Cannot use same move twice
    NoRepeat,
    /// Heal Block: No healing moves
    NoHealing,
    /// Imprison: Cannot use moves known by imprisoner
    Imprison,
    /// Choice lock: Must use same move
    ChoiceLock(ID),
}

impl Default for ConditionDef {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            condition_type: ConditionType::Volatile,
            duration: None,
            max_duration: None,
            min_duration: None,
            baton_passable: false,
            traps: false,
            max_layers: None,
            residual_damage: None,
            escalating_damage: false,
            speed_modifier: None,
            attack_modifier: None,
            skip_turn_chance: None,
            move_restriction: None,
            volatile_damage: None,
            volatile_heal: None,
            confusion_chance: None,
            stat_boosts: None,
            protection: false,
            hazard_type: None,
            hazard_damage: None,
            screen_reduction: None,
            entry_status: None,
            entry_speed_drop: None,
            type_boost: None,
            type_weaken: None,
            weather_damage: None,
            immune_types: None,
            immune_abilities: None,
            grounded_only: false,
            prevents_status: None,
            blocks_priority: false,
        }
    }
}

impl ConditionDef {
    pub fn new(id: &str, name: &str, condition_type: ConditionType) -> Self {
        Self {
            id: ID::new(id),
            name: name.to_string(),
            condition_type,
            ..Default::default()
        }
    }
}

/// JSON structure from conditions.json
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonCondition {
    name: String,
    #[serde(default)]
    effect_type: Option<String>,
    #[serde(default)]
    duration: Option<i32>,
    #[serde(default)]
    counter_max: Option<i32>,
    #[serde(default)]
    no_copy: Option<bool>,
    #[serde(default)]
    affects_fainted: Option<bool>,
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}

/// Load conditions from JSON file
fn load_conditions_from_json() -> HashMap<ID, ConditionDef> {
    const CONDITIONS_JSON: &str = include_str!("../../data/conditions.json");

    let json_conditions: HashMap<String, JsonCondition> =
        serde_json::from_str(CONDITIONS_JSON)
            .expect("Failed to parse conditions.json");

    let mut map = HashMap::new();

    // ==========================================
    // HARDCODED DETAILED CONDITIONS
    // These have detailed metadata not in JSON
    // ==========================================

    // STATUS CONDITIONS (non-volatile)
    map.insert(
        ID::new("brn"),
        ConditionDef {
            id: ID::new("brn"),
            name: "Burn".to_string(),
            condition_type: ConditionType::Status,
            residual_damage: Some(1.0 / 16.0),
            attack_modifier: Some(0.5), // Physical moves deal half damage
            ..Default::default()
        },
    );

    map.insert(
        ID::new("par"),
        ConditionDef {
            id: ID::new("par"),
            name: "Paralysis".to_string(),
            condition_type: ConditionType::Status,
            speed_modifier: Some(0.5),
            skip_turn_chance: Some(25),
            ..Default::default()
        },
    );

    map.insert(
        ID::new("slp"),
        ConditionDef {
            id: ID::new("slp"),
            name: "Sleep".to_string(),
            condition_type: ConditionType::Status,
            min_duration: Some(1),
            max_duration: Some(3),
            skip_turn_chance: Some(100), // Can't move while asleep
            ..Default::default()
        },
    );

    map.insert(
        ID::new("frz"),
        ConditionDef {
            id: ID::new("frz"),
            name: "Freeze".to_string(),
            condition_type: ConditionType::Status,
            skip_turn_chance: Some(100), // Can't move while frozen (20% thaw chance handled in callbacks)
            ..Default::default()
        },
    );

    map.insert(
        ID::new("psn"),
        ConditionDef {
            id: ID::new("psn"),
            name: "Poison".to_string(),
            condition_type: ConditionType::Status,
            residual_damage: Some(1.0 / 8.0),
            ..Default::default()
        },
    );

    map.insert(
        ID::new("tox"),
        ConditionDef {
            id: ID::new("tox"),
            name: "Toxic".to_string(),
            condition_type: ConditionType::Status,
            residual_damage: Some(1.0 / 16.0), // Starts at 1/16, increases each turn
            escalating_damage: true,
            ..Default::default()
        },
    );

    // WEATHER CONDITIONS
    map.insert(
        ID::new("sandstorm"),
        ConditionDef {
            id: ID::new("sandstorm"),
            name: "Sandstorm".to_string(),
            condition_type: ConditionType::Weather,
            duration: Some(5),
            weather_damage: Some(1.0 / 16.0),
            immune_types: Some(vec!["Ground".to_string(), "Rock".to_string(), "Steel".to_string()]),
            ..Default::default()
        },
    );

    map.insert(
        ID::new("raindance"),
        ConditionDef {
            id: ID::new("raindance"),
            name: "RainDance".to_string(),
            condition_type: ConditionType::Weather,
            duration: Some(5),
            type_boost: Some(("Water".to_string(), 1.5)),
            type_weaken: Some(("Fire".to_string(), 0.5)),
            ..Default::default()
        },
    );

    map.insert(
        ID::new("sunnyday"),
        ConditionDef {
            id: ID::new("sunnyday"),
            name: "SunnyDay".to_string(),
            condition_type: ConditionType::Weather,
            duration: Some(5),
            type_boost: Some(("Fire".to_string(), 1.5)),
            type_weaken: Some(("Water".to_string(), 0.5)),
            ..Default::default()
        },
    );

    map.insert(
        ID::new("hail"),
        ConditionDef {
            id: ID::new("hail"),
            name: "Hail".to_string(),
            condition_type: ConditionType::Weather,
            duration: Some(5),
            weather_damage: Some(1.0 / 16.0),
            immune_types: Some(vec!["Ice".to_string()]),
            ..Default::default()
        },
    );

    // VOLATILE CONDITIONS - add basic ones from hardcode, rest from JSON
    map.insert(
        ID::new("confusion"),
        ConditionDef {
            id: ID::new("confusion"),
            name: "Confusion".to_string(),
            condition_type: ConditionType::Volatile,
            min_duration: Some(1),
            max_duration: Some(4),
            confusion_chance: Some(33),
            ..Default::default()
        },
    );

    map.insert(
        ID::new("flinch"),
        ConditionDef {
            id: ID::new("flinch"),
            name: "Flinch".to_string(),
            condition_type: ConditionType::Volatile,
            duration: Some(1),
            skip_turn_chance: Some(100),
            ..Default::default()
        },
    );

    map.insert(
        ID::new("substitute"),
        ConditionDef {
            id: ID::new("substitute"),
            name: "Substitute".to_string(),
            condition_type: ConditionType::Volatile,
            baton_passable: true,
            // HP stored separately in volatile state
            ..Default::default()
        },
    );

    map.insert(
        ID::new("protect"),
        ConditionDef {
            id: ID::new("protect"),
            name: "Protect".to_string(),
            condition_type: ConditionType::Volatile,
            duration: Some(1),
            protection: true,
            ..Default::default()
        },
    );

    map.insert(
        ID::new("leechseed"),
        ConditionDef {
            id: ID::new("leechseed"),
            name: "Leech Seed".to_string(),
            condition_type: ConditionType::Volatile,
            volatile_damage: Some(1.0 / 8.0),
            // Heals the seeder - handled in battle.rs
            ..Default::default()
        },
    );

    map.insert(
        ID::new("aquaring"),
        ConditionDef {
            id: ID::new("aquaring"),
            name: "Aqua Ring".to_string(),
            condition_type: ConditionType::Volatile,
            volatile_heal: Some(1.0 / 16.0),
            baton_passable: true,
            ..Default::default()
        },
    );

    map.insert(
        ID::new("ingrain"),
        ConditionDef {
            id: ID::new("ingrain"),
            name: "Ingrain".to_string(),
            condition_type: ConditionType::Volatile,
            volatile_heal: Some(1.0 / 16.0),
            traps: true, // Can't switch out
            baton_passable: false,
            ..Default::default()
        },
    );

    map.insert(
        ID::new("perishsong"),
        ConditionDef {
            id: ID::new("perishsong"),
            name: "Perish Song".to_string(),
            condition_type: ConditionType::Volatile,
            duration: Some(4), // Faints when counter reaches 0
            ..Default::default()
        },
    );

    // ==========================================
    // LOAD REMAINING CONDITIONS FROM JSON
    // Any condition in JSON not already in map gets added
    // ==========================================

    for (condition_id, json_cond) in json_conditions {
        let id = ID::new(&condition_id);

        // Skip if already exists (hardcoded version takes priority)
        if map.contains_key(&id) {
            continue;
        }

        // Determine condition type from effectType field
        let condition_type = match json_cond.effect_type.as_deref() {
            Some("Status") => ConditionType::Status,
            Some("Weather") => ConditionType::Weather,
            Some("Terrain") => ConditionType::Terrain,
            Some("SideCondition") => ConditionType::SideCondition,
            Some("SlotCondition") => ConditionType::SlotCondition,
            Some("PseudoWeather") => ConditionType::PseudoWeather,
            _ => ConditionType::Volatile,
        };

        // Create basic condition from JSON
        map.insert(
            id.clone(),
            ConditionDef {
                id,
                name: json_cond.name,
                condition_type,
                duration: json_cond.duration,
                baton_passable: !json_cond.no_copy.unwrap_or(false),
                ..Default::default()
            },
        );
    }

    map
}

/// Static registry of all conditions - loaded from JSON
pub static CONDITIONS: Lazy<HashMap<ID, ConditionDef>> = Lazy::new(load_conditions_from_json);

/// Get a condition definition by ID
pub fn get_condition(id: &ID) -> Option<&'static ConditionDef> {
    CONDITIONS.get(id)
}

/// Check if a condition is a status (non-volatile)
pub fn is_status_condition(id: &ID) -> bool {
    get_condition(id).is_some_and(|c| c.condition_type == ConditionType::Status)
}

/// Check if a condition is a volatile
pub fn is_volatile_condition(id: &ID) -> bool {
    get_condition(id).is_some_and(|c| c.condition_type == ConditionType::Volatile)
}

/// Check if a condition traps the Pokemon
pub fn condition_traps(id: &ID) -> bool {
    get_condition(id).is_some_and(|c| c.traps)
}

/// Get residual damage for a condition
pub fn get_condition_damage(id: &ID) -> Option<f64> {
    get_condition(id).and_then(|c| c.residual_damage.or(c.volatile_damage))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burn() {
        let burn = get_condition(&ID::new("brn")).unwrap();
        assert_eq!(burn.condition_type, ConditionType::Status);
        assert_eq!(burn.residual_damage, Some(1.0 / 16.0));
        assert_eq!(burn.attack_modifier, Some(0.5));
    }

    #[test]
    fn test_paralysis() {
        let par = get_condition(&ID::new("par")).unwrap();
        assert_eq!(par.speed_modifier, Some(0.5));
        assert_eq!(par.skip_turn_chance, Some(25));
    }

    #[test]
    fn test_loaded_from_json() {
        // Test that stall condition is loaded from JSON
        let stall = get_condition(&ID::new("stall")).unwrap();
        assert_eq!(stall.name, "stall");
        assert_eq!(stall.duration, Some(2));
        assert_eq!(stall.condition_type, ConditionType::Volatile);
    }

    #[test]
    fn test_weather_conditions() {
        let sandstorm = get_condition(&ID::new("sandstorm")).unwrap();
        assert_eq!(sandstorm.condition_type, ConditionType::Weather);
        assert_eq!(sandstorm.duration, Some(5));
    }
}
