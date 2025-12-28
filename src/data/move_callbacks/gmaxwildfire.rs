//! G-Max Wildfire Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'G-Max Wildfire');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        use crate::dex_data::ID;

        // this.add('-sidestart', targetSide, 'G-Max Wildfire');
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
            let side_arg = crate::battle::Arg::Side(side_index);
            battle.add("-sidestart", &[side_arg, "G-Max Wildfire".into()]);
                    }
        }

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (!target.hasType('Fire')) this.damage(target.baseMaxhp / 6, target);
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!target.hasType('Fire')) this.damage(target.baseMaxhp / 6, target);
        let has_fire_type = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.has_type("fire")
        };

        if !has_fire_type {
            // this.damage(target.baseMaxhp / 6, target);
            let damage_amount = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.base_maxhp / 6
            };

            battle.damage(damage_amount, target, None, None);
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'G-Max Wildfire');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'G-Max Wildfire');
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
            let side_arg = crate::battle::Arg::Side(side_index);
            battle.add("-sideend", &[side_arg, "G-Max Wildfire".into()]);
                    }
        }

        EventResult::Continue
    }
}
