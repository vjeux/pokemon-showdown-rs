//! Condition Data

use crate::dex_data::ID;
use serde::{Deserialize, Serialize};

/// Condition data - for status, volatiles, side conditions, etc.
/// This mirrors the JS "condition" object pattern
/// JavaScript equivalent: ConditionData (sim/dex-conditions.ts)
/// 13 fields in JavaScript
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionData {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
    /// Number of turns the condition lasts (None = indefinite)
    pub duration: Option<i32>,
    /// Can this condition be passed by Baton Pass?
    pub no_copy: bool,
    /// Counter used by some conditions (e.g., Perish Song)
    pub counter_max: Option<i32>,
    /// Status this condition applies (e.g., "slp" for Rest)
    pub status: Option<String>,
    /// Residual order (when to run onResidual)
    pub residual_order: Option<i32>,
    /// Volatile status to add
    pub volatile_status: Option<String>,
    /// Side condition to add
    pub side_condition: Option<String>,
    /// Slot condition to add
    pub slot_condition: Option<String>,
    /// Weather to set
    pub weather: Option<String>,
    /// Terrain to set
    pub terrain: Option<String>,
    /// Pseudo-weather to set
    pub pseudo_weather: Option<String>,
}

impl ConditionData {
    pub fn new(id: &str) -> Self {
        Self {
            id: ID::new(id),
            name: id.to_string(),
            ..Default::default()
        }
    }

    pub fn with_duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }
}
