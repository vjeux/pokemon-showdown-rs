// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex::MoveSecondary;

impl Dex {

    /// Convert MoveSecondary to SecondaryEffect
    pub fn convert_secondary(secondary: &MoveSecondary) -> crate::battle_actions::SecondaryEffect {
        // If there's a self_secondary field, use it for the effect and mark as self-targeting
        // Otherwise, use the top-level fields for target-effect
        let (boosts, status, volatile_status, self_effect) = if let Some(ref self_sec) = secondary.self_secondary {
            // Self-targeting effect: use boosts/status from self field
            (
                self_sec.boosts,
                self_sec.status.clone(),
                self_sec.volatile_status.clone(),
                true,  // This is a self-effect
            )
        } else {
            // Target effect: use top-level boosts/status
            (
                secondary.boosts,
                secondary.status.clone(),
                secondary.volatile_status.clone(),
                false,  // This is a target effect
            )
        };

        crate::battle_actions::SecondaryEffect {
            chance: secondary.chance,
            boosts,
            status,
            volatile_status,
            side_condition: None,
            slot_condition: None,
            pseudo_weather: None,
            terrain: None,
            weather: None,
            ability: None,
            kingsrock: None,
            self_effect,
        }
    }
}
