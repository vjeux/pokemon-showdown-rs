//! Follow Me Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     return this.activePerHalf > 1;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // return this.activePerHalf > 1;
    let active_per_half = battle.active_per_half;

    EventResult::Boolean(active_per_half > 1)
}

pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     if (effect?.id === 'zpower') {
    ///         this.add('-singleturn', target, 'move: Follow Me', '[zeffect]');
    ///     } else {
    ///         this.add('-singleturn', target, 'move: Follow Me');
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (effect?.id === 'zpower') {
        //     this.add('-singleturn', target, 'move: Follow Me', '[zeffect]');
        // } else {
        //     this.add('-singleturn', target, 'move: Follow Me');
        // }
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        if let Some(effect) = effect_id {
            if effect == "zpower" {
                battle.add("-singleturn", &[target_arg, "move: Follow Me".into(), "[zeffect]".into()]);
            } else {
                battle.add("-singleturn", &[target_arg, "move: Follow Me".into()]);
            }
        } else {
            battle.add("-singleturn", &[target_arg, "move: Follow Me".into()]);
        }

        EventResult::Continue
    }

    /// onFoeRedirectTarget(target, source, source2, move) {
    ///     if (!this.effectState.target.isSkyDropped() && this.validTarget(this.effectState.target, source, move.target)) {
    ///         if (move.smartTarget) move.smartTarget = false;
    ///         this.debug("Follow Me redirected target of move");
    ///         return this.effectState.target;
    ///     }
    /// }
    pub fn on_foe_redirect_target(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // if (!this.effectState.target.isSkyDropped() && this.validTarget(this.effectState.target, source, move.target)) {
        // Get the effect state target (the Pokemon with Follow Me active)
        let effect_state_target = match &battle.current_effect_state {
            Some(effect_state) => match effect_state.target {
                Some(pos) => pos,
                None => return EventResult::Continue,
            },
            None => return EventResult::Continue,
        };

        let is_sky_dropped = {
            let target_pokemon = match battle.pokemon_at(effect_state_target.0, effect_state_target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.is_sky_dropped()
        };

        if is_sky_dropped {
            return EventResult::Continue;
        }

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get move target type
        let move_target = match battle.dex.get_move_by_id(&crate::dex_data::ID::from(move_id)) {
            Some(move_data) => move_data.target.clone(),
            None => return EventResult::Continue,
        };

        let is_valid_target = battle.valid_target(effect_state_target, source, move_target.as_str());

        if is_valid_target {
            // if (move.smartTarget) move.smartTarget = false;
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = Some(false);
            }

            // this.debug("Follow Me redirected target of move");
            battle.debug("Follow Me redirected target of move");

            // return this.effectState.target;
            return EventResult::Position(effect_state_target);
        }

        EventResult::Continue
    }
}
