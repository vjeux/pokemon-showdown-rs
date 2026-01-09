// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex::MoveSecondary;

impl Dex {

    /// Convert MoveSecondary to SelfEffect
    /// Used when copying move.self from MoveData to ActiveMove.self_effect
    pub fn convert_self_effect(secondary: &MoveSecondary) -> crate::battle_actions::SelfEffect {
        crate::battle_actions::SelfEffect {
            boosts: secondary.boosts,
            status: secondary.status.clone(),
            volatile_status: secondary.volatile_status_secondary.clone(),
            side_condition: secondary.side_condition.clone(),
            slot_condition: secondary.slot_condition.clone(),
            pseudo_weather: secondary.pseudo_weather.clone(),
            terrain: secondary.terrain.clone(),
            weather: secondary.weather.clone(),
            chance: secondary.chance,  // Copy chance field from MoveSecondary
        }
    }
}
