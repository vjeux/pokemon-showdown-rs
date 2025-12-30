use crate::*;
use crate::dex::MoveSecondary;

impl Dex {

    /// Convert MoveSecondary to SecondaryEffect
    pub fn convert_secondary(secondary: &MoveSecondary) -> crate::battle_actions::SecondaryEffect {
        crate::battle_actions::SecondaryEffect {
            chance: secondary.chance,
            boosts: secondary
                .boosts
                .as_ref()
                .map(Self::convert_boosts_hash_to_table),
            status: secondary.status.clone(),
            volatile_status: secondary.volatile_status_secondary.clone(),
            self_effect: false,
        }
    }
}
