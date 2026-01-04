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
/// Fields match JavaScript data fields (not callback implementations)
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
    /// Can be passed by Baton Pass (inverse of noCopy)
    pub baton_passable: bool,
}


impl Default for ConditionDef {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            condition_type: ConditionType::Volatile,
            duration: None,
            baton_passable: false,
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

    // Load all conditions from JSON
    for (condition_id, json_cond) in json_conditions {
        let id = ID::new(&condition_id);

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

        // Create condition from JSON data
        map.insert(
            id.clone(),
            ConditionDef {
                id,
                name: json_cond.name,
                condition_type,
                duration: json_cond.duration,
                baton_passable: !json_cond.no_copy.unwrap_or(false),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burn() {
        let burn = get_condition(&ID::new("brn")).unwrap();
        assert_eq!(burn.name, "brn");
        assert_eq!(burn.condition_type, ConditionType::Status);
    }

    #[test]
    fn test_paralysis() {
        let par = get_condition(&ID::new("par")).unwrap();
        assert_eq!(par.name, "par");
        assert_eq!(par.condition_type, ConditionType::Status);
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
        assert_eq!(sandstorm.name, "Sandstorm");
        assert_eq!(sandstorm.condition_type, ConditionType::Weather);
        assert_eq!(sandstorm.duration, Some(5));
    }
}
