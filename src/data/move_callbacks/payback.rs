//! Payback Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (target.newlySwitched || this.queue.willMove(target)) {
///         this.debug('Payback NOT boosted');
///         return move.basePower;
///     }
///     this.debug('Payback damage boost');
///     return move.basePower * 2;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.newlySwitched || this.queue.willMove(target)) {
    let newly_switched = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.newly_switched
    };

    // TODO: Implement queue_will_move method in Battle
    let will_move = false; // battle.queue_will_move(target);

    if newly_switched || will_move {
        // this.debug('Payback NOT boosted');
        // (debug is typically not needed in Rust implementation)

        // return move.basePower;
        let base_power = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.base_power
        };
        return EventResult::Number(base_power);
    }

    // this.debug('Payback damage boost');
    // (debug is typically not needed in Rust implementation)

    // return move.basePower * 2;
    let base_power = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.base_power
    };

    EventResult::Number(base_power * 2)
}

