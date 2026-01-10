//! Ability data from the Dex

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ability data from the Dex
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: AbilityData (sim/dex-abilities.ts)
/// 20+ fields in JavaScript (many are callbacks)
pub struct AbilityData {
    /// Ability number
    /// JavaScript: num: number
    #[serde(default)]
    pub num: i32,
    /// Ability name
    /// JavaScript: name: string
    pub name: String,
    /// Full description
    /// JavaScript: desc?: string
    #[serde(default)]
    pub desc: Option<String>,
    /// Short description
    /// JavaScript: shortDesc?: string
    #[serde(rename = "shortDesc", default)]
    pub short_desc: Option<String>,
    /// Competitive rating
    /// JavaScript: rating?: number
    #[serde(default)]
    pub rating: Option<f64>,
    /// Ability flags
    /// JavaScript: flags?: { [k: string]: 1 }
    #[serde(default)]
    pub flags: HashMap<String, i32>,
    /// Effect type
    /// JavaScript: effectType?: string
    #[serde(default)]
    pub effect_type: Option<String>,
    /// Nonstandard status (Past, Future, Unobtainable, etc.)
    /// JavaScript: isNonstandard?: Nonstandard | null
    /// TODO: Rust uses Option<String>, JavaScript uses Nonstandard union type
    #[serde(rename = "isNonstandard", default)]
    pub is_nonstandard: Option<String>,
    /// Whether this ability suppresses weather
    /// JavaScript: suppressWeather?: boolean
    #[serde(rename = "suppressWeather", default)]
    pub suppress_weather: Option<bool>,
    /// Extra fields (like onResidualOrder, onResidualSubOrder, etc.)
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`]
    /// Note: JavaScript has many callback methods (onStart, onEnd, etc.) that cannot be stored in data
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl AbilityData {
    /// Get effect type
    /// JavaScript equivalent: ability.effectType (always 'Ability')
    /// In JavaScript, this is set in the Ability constructor: this.effectType = 'Ability'
    pub fn effect_type(&self) -> &'static str {
        "Ability"
    }
}
