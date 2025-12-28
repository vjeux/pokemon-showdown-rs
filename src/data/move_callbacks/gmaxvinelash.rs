//! G-Max Vine Lash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'G-Max Vine Lash');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', targetSide, 'G-Max Vine Lash');
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };

                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sidestart", &[side_arg, "G-Max Vine Lash".into()]);
            }
        }

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (!target.hasType('Grass')) this.damage(target.baseMaxhp / 6, target);
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!target.hasType('Grass')) this.damage(target.baseMaxhp / 6, target);
        let has_grass_type = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.has_type("grass")
        };

        if !has_grass_type {
            // this.damage(target.baseMaxhp / 6, target);
            let damage_amount = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.base_maxhp / 6
            };

            battle.damage(damage_amount, Some(target), None, None, false);
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'G-Max Vine Lash');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'G-Max Vine Lash');
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };

                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sideend", &[side_arg, "G-Max Vine Lash".into()]);
            }
        }

        EventResult::Continue
    }
}
