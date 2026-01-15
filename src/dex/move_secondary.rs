//! Move secondary effect data

use serde::{Deserialize, Serialize};

/// Move secondary effect data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// JavaScript equivalent: MoveSecondary/SecondaryEffect (sim/dex-moves.ts)
/// SecondaryEffect extends HitEffect with: chance, ability, kingsrock, self
pub struct MoveSecondary {
    /// Chance of effect occurring (percentage)
    /// JavaScript: chance?: number
    #[serde(default)]
    pub chance: Option<i32>,
    /// Status to inflict
    /// JavaScript: status?: string
    #[serde(default)]
    pub status: Option<String>,
    /// Stat boosts to apply
    /// JavaScript: boosts?: SparseBoostsTable
    #[serde(default)]
    pub boosts: Option<crate::dex_data::BoostsTable>,
    /// Volatile status to inflict
    /// JavaScript: volatileStatus?: string
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status: Option<String>,
    /// Side condition to apply
    /// JavaScript: sideCondition?: string
    #[serde(rename = "sideCondition", default)]
    pub side_condition: Option<String>,
    /// Slot condition to apply
    /// JavaScript: slotCondition?: string
    #[serde(rename = "slotCondition", default)]
    pub slot_condition: Option<String>,
    /// Pseudo weather to apply
    /// JavaScript: pseudoWeather?: string
    #[serde(rename = "pseudoWeather", default)]
    pub pseudo_weather: Option<String>,
    /// Terrain to apply
    /// JavaScript: terrain?: string
    #[serde(default)]
    pub terrain: Option<String>,
    /// Weather to apply
    /// JavaScript: weather?: string
    #[serde(default)]
    pub weather: Option<String>,
    /// Self-targeted secondary effect (applied to user instead of target)
    /// JavaScript: self?: HitEffect
    #[serde(rename = "self", default)]
    pub self_secondary: Option<Box<MoveSecondary>>,
    /// Ability data (runtime only, not from JSON)
    /// JavaScript: ability?: Ability
    #[serde(default)]
    pub ability: Option<crate::dex_data::ID>,
    /// King's Rock effect flag (runtime only, not from JSON)
    /// JavaScript: kingsrock?: boolean
    #[serde(default)]
    pub kingsrock: Option<bool>,
    /// Whether this secondary has an onHit callback (runtime only, not from JSON)
    /// Used to distinguish move's original secondaries (with onHit) from item-added secondaries (without onHit)
    #[serde(default)]
    pub has_on_hit: bool,
}
