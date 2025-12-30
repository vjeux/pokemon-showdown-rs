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
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    // onTryHit(target) {
    //     if (this.activePerHalf === 1) return false;
    // }

    // if (this.activePerHalf === 1) return false;
    let active_per_half = battle.active_per_half;

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
            pokemon_data.get_slot()
        };

        battle.add(
            "-singleturn",
            &[pokemon_arg.into(), "move: Spotlight".into()],
        );

        EventResult::Continue
    }

    /// onFoeRedirectTarget(target, source, source2, move) {
    ///     if (this.validTarget(this.effectState.target, source, move.target)) {
    ///         this.debug("Spotlight redirected target of move");
    ///         return this.effectState.target;
    ///     }
    /// }
    pub fn on_foe_redirect_target(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
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
        let effect_state_target = match &battle.current_effect_state {
            Some(state) => match state.target {
                Some(target) => target,
                None => return EventResult::Continue,
            },
            None => return EventResult::Continue,
        };

        let move_data = battle.dex.moves().get_by_id(&ID::from(move_id));
        let move_target = move_data.map(|m| m.target.clone()).unwrap_or_default();

        let is_valid = battle.valid_target(effect_state_target, source, &move_target);

        if is_valid {
            // this.debug("Spotlight redirected target of move");
            battle.debug("Spotlight redirected target of move");

            // return this.effectState.target;
            return EventResult::Position(effect_state_target);
        }

        EventResult::Continue
    }
}
