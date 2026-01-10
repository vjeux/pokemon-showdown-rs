//! Condition data from the Dex

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of condition
/// JavaScript equivalent: EffectType for conditions (sim/dex-conditions.ts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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

/// Condition data from the Dex
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: ConditionData (sim/dex-conditions.ts)
/// Fields match JavaScript data fields (not callback implementations)
pub struct ConditionData {
    /// Condition name
    /// JavaScript: name?: string
    /// Note: Optional because embedded conditions in moves don't always have names
    #[serde(default)]
    pub name: Option<String>,
    /// Effect type
    /// JavaScript: effectType?: 'Status' | 'Weather' | 'Terrain' | 'SideCondition' | 'SlotCondition' | 'PseudoWeather'
    #[serde(rename = "effectType", default)]
    pub effect_type: Option<String>,
    /// Duration in turns (None = indefinite or until cured)
    /// JavaScript: duration?: number
    #[serde(default)]
    pub duration: Option<i32>,
    /// Cannot be passed by Baton Pass
    /// JavaScript: noCopy?: boolean
    #[serde(rename = "noCopy", default)]
    pub no_copy: bool,
    /// Counter maximum (for stall, etc.)
    /// JavaScript: counterMax?: number
    #[serde(rename = "counterMax", default)]
    pub counter_max: Option<i32>,
    /// Affects fainted Pokemon
    /// JavaScript: affectsFainted?: boolean
    #[serde(rename = "affectsFainted", default)]
    pub affects_fainted: bool,
    /// Extra fields (like onResidualOrder, callback flags, etc.)
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`]
    /// Note: JavaScript has many callback methods that cannot be stored in data
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ConditionData {
    /// Get the condition type
    pub fn condition_type(&self) -> ConditionType {
        match self.effect_type.as_deref() {
            Some("Status") => ConditionType::Status,
            Some("Weather") => ConditionType::Weather,
            Some("Terrain") => ConditionType::Terrain,
            Some("SideCondition") => ConditionType::SideCondition,
            Some("SlotCondition") => ConditionType::SlotCondition,
            Some("PseudoWeather") => ConditionType::PseudoWeather,
            _ => ConditionType::Volatile,
        }
    }

    /// Get effect type string
    /// JavaScript equivalent: condition.effectType
    /// Returns the effectType as a string: 'Status', 'Weather', 'Terrain', or 'Condition'
    pub fn effect_type(&self) -> &str {
        match self.effect_type.as_deref() {
            Some("Status") => "Status",
            Some("Weather") => "Weather",
            Some("Terrain") => "Terrain",
            Some("SideCondition") => "Condition",
            Some("SlotCondition") => "Condition",
            Some("PseudoWeather") => "Condition",
            Some(other) => other,
            None => "Condition",
        }
    }

    /// Can be passed by Baton Pass (inverse of noCopy)
    pub fn baton_passable(&self) -> bool {
        !self.no_copy
    }
}
