//! Spotlight Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (this.activePerHalf === 1) return false;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // onTryHit(target) {
    //     if (this.activePerHalf === 1) return false;
    // }

    // if (this.activePerHalf === 1) return false;
    let active_per_half = battle.active_per_half();

    if active_per_half == 1 {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singleturn', pokemon, 'move: Spotlight');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // onStart(pokemon) {
        //     this.add('-singleturn', pokemon, 'move: Spotlight');
        // }
        let pokemon = pokemon_pos;

        // this.add('-singleturn', pokemon, 'move: Spotlight');
        let pokemon_arg = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_data)
        };

        battle.add("-singleturn", &[
            pokemon_arg,
            "move: Spotlight".into(),
        ]);

        EventResult::Continue
    }

    /// onFoeRedirectTarget(target, source, source2, move) {
    ///     if (this.validTarget(this.effectState.target, source, move.target)) {
    ///         this.debug("Spotlight redirected target of move");
    ///         return this.effectState.target;
    ///     }
    /// }
    pub fn on_foe_redirect_target(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // onFoeRedirectTarget(target, source, source2, move) {
        //     if (this.validTarget(this.effectState.target, source, move.target)) {
        //         this.debug("Spotlight redirected target of move");
        //         return this.effectState.target;
        //     }
        // }
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (this.validTarget(this.effectState.target, source, move.target)) {
        let effect_state_target = battle.get_effect_state_target();

        let move_data = battle.dex.get_move_by_id(&ID::from(move_id));
        let move_target = move_data.map(|m| m.target.clone()).unwrap_or_default();

        let is_valid = battle.is_valid_target(effect_state_target, source, &move_target);

        if is_valid {
            // this.debug("Spotlight redirected target of move");
            battle.debug("Spotlight redirected target of move");

            // return this.effectState.target;
            return EventResult::Position(effect_state_target);
        }

        EventResult::Continue
    }
}
