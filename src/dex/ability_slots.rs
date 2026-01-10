//! Abilities structure (mapping slot to ability name)

use serde::{Deserialize, Serialize};

/// Abilities structure (mapping slot to ability name)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: Abilities (sim/dex-species.ts)
/// 4 fields in JavaScript
pub struct AbilitySlots {
    /// Regular ability slot 0
    /// JavaScript: "0": string
    #[serde(rename = "0", default)]
    pub slot0: Option<String>,
    /// Regular ability slot 1 (second ability)
    /// JavaScript: "1"?: string
    #[serde(rename = "1", default)]
    pub slot1: Option<String>,
    /// Hidden ability
    /// JavaScript: "H"?: string
    #[serde(rename = "H", default)]
    pub hidden: Option<String>,
    /// Special ability (event-only)
    /// JavaScript: "S"?: string
    #[serde(rename = "S", default)]
    pub special: Option<String>,
}
