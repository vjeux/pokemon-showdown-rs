//! Effect structs for secondary effects

use serde::{Deserialize, Serialize};
use crate::dex_data::{BoostsTable, ID};

/// Secondary effect data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: SecondaryEffect (sim/dex-moves.ts)
pub struct SecondaryEffect {
    /// Chance of the effect occurring (percentage)
    pub chance: Option<i32>,
    /// Stat boosts to apply
    pub boosts: Option<BoostsTable>,
    /// Status condition to inflict
    pub status: Option<String>,
    /// Volatile status to inflict
    pub volatile_status: Option<String>,
    /// Side condition to apply
    pub side_condition: Option<String>,
    /// Slot condition to apply
    pub slot_condition: Option<String>,
    /// Pseudo-weather to apply
    pub pseudo_weather: Option<String>,
    /// Terrain to apply
    pub terrain: Option<String>,
    /// Weather to apply
    pub weather: Option<String>,
    /// Ability data
    pub ability: Option<ID>,
    /// King's Rock effect flag
    pub kingsrock: Option<bool>,
    /// Self-targeting effect
    pub self_effect: bool,
}

/// Self effect data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: HitEffect (in SecondaryEffect.self field)
pub struct SelfEffect {
    /// Stat boosts to apply
    pub boosts: Option<BoostsTable>,
    /// Status condition to inflict
    pub status: Option<String>,
    /// Volatile status to inflict
    #[serde(rename = "volatileStatus")]
    pub volatile_status: Option<String>,
    /// Side condition to apply
    #[serde(rename = "sideCondition")]
    pub side_condition: Option<String>,
    /// Slot condition to apply
    #[serde(rename = "slotCondition")]
    pub slot_condition: Option<String>,
    /// Pseudo-weather to apply
    #[serde(rename = "pseudoWeather")]
    pub pseudo_weather: Option<String>,
    /// Terrain to apply
    pub terrain: Option<String>,
    /// Weather to apply
    pub weather: Option<String>,
    /// Chance of the effect occurring (percentage)
    pub chance: Option<i32>,
}

/// HitEffect enum for passing to moveHit
/// JavaScript uses HitEffect as a parameter to moveHit
#[derive(Debug, Clone)]
pub enum HitEffect<'a> {
    /// Reference to an ActiveMove
    Move(&'a super::ActiveMove),
    /// Reference to a MoveSecondary (SecondaryEffect)
    Secondary(&'a crate::dex::MoveSecondary),
}

impl<'a> HitEffect<'a> {
    /// Get boosts from the hit effect
    pub fn boosts(&self) -> Option<&BoostsTable> {
        match self {
            HitEffect::Move(m) => m.boosts.as_ref(),
            HitEffect::Secondary(s) => s.boosts.as_ref(),
        }
    }

    /// Get status from the hit effect
    pub fn status(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.status.as_ref(),
            HitEffect::Secondary(s) => s.status.as_ref(),
        }
    }

    /// Get volatile_status from the hit effect
    pub fn volatile_status(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.volatile_status.as_ref(),
            HitEffect::Secondary(s) => s.volatile_status.as_ref(),
        }
    }

    /// Get side_condition from the hit effect
    pub fn side_condition(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.side_condition.as_ref(),
            HitEffect::Secondary(s) => s.side_condition.as_ref(),
        }
    }

    /// Get slot_condition from the hit effect
    pub fn slot_condition(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.slot_condition.as_ref(),
            HitEffect::Secondary(s) => s.slot_condition.as_ref(),
        }
    }

    /// Get pseudo_weather from the hit effect
    pub fn pseudo_weather(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.pseudo_weather.as_ref(),
            HitEffect::Secondary(s) => s.pseudo_weather.as_ref(),
        }
    }

    /// Get terrain from the hit effect
    pub fn terrain(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.terrain.as_ref(),
            HitEffect::Secondary(s) => s.terrain.as_ref(),
        }
    }

    /// Get weather from the hit effect
    pub fn weather(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.weather.as_ref(),
            HitEffect::Secondary(s) => s.weather.as_ref(),
        }
    }

    /// Check if this is a self effect (secondary only)
    pub fn is_self_effect(&self) -> bool {
        match self {
            HitEffect::Move(_) => false,
            HitEffect::Secondary(s) => s.self_secondary.is_some(),
        }
    }

    /// Get heal fraction (only available on ActiveMove)
    pub fn heal(&self) -> Option<(i32, i32)> {
        match self {
            HitEffect::Move(m) => m.heal,
            HitEffect::Secondary(_) => None,
        }
    }

    /// Get force_switch (only available on ActiveMove)
    pub fn force_switch(&self) -> bool {
        match self {
            HitEffect::Move(m) => m.force_switch,
            HitEffect::Secondary(_) => false,
        }
    }

    /// Get self_destruct (only available on ActiveMove)
    pub fn self_destruct(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.self_destruct.as_ref(),
            HitEffect::Secondary(_) => None,
        }
    }

    /// Get self_switch (only available on ActiveMove)
    pub fn self_switch(&self) -> Option<&String> {
        match self {
            HitEffect::Move(m) => m.self_switch.as_ref(),
            HitEffect::Secondary(_) => None,
        }
    }

    /// Get self_effect (only available on ActiveMove)
    pub fn self_effect_data(&self) -> Option<&crate::dex::MoveSecondary> {
        match self {
            HitEffect::Move(m) => m.self_effect.as_ref(),
            HitEffect::Secondary(_) => None,
        }
    }

    /// Get the ID (for move, returns move.id; for secondary, no ID available)
    pub fn id(&self) -> Option<&ID> {
        match self {
            HitEffect::Move(m) => Some(&m.id),
            HitEffect::Secondary(_) => None,
        }
    }

    /// Get as ActiveMove if this is a Move variant
    pub fn as_move(&self) -> Option<&super::ActiveMove> {
        match self {
            HitEffect::Move(m) => Some(m),
            HitEffect::Secondary(_) => None,
        }
    }
}
