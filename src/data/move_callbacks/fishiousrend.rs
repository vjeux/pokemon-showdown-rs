//! Fishious Rend Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (target.newlySwitched || this.queue.willMove(target)) {
///         this.debug('Fishious Rend damage boost');
///         return move.basePower * 2;
///     }
///     this.debug('Fishious Rend NOT boosted');
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get current move's base power
    let base_power = match &battle.active_move {
        Some(move_id) => {
            match battle.dex.get_move_by_id(move_id) {
                Some(move_data) => move_data.base_power,
                None => return EventResult::Continue,
            }
        }
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

    let will_move = battle.queue.will_move(target);

    if newly_switched || will_move {
        // this.debug('Fishious Rend damage boost');
        battle.debug("Fishious Rend damage boost");

        // return move.basePower * 2;
        return EventResult::Number(base_power * 2);
    }

    // this.debug('Fishious Rend NOT boosted');
    battle.debug("Fishious Rend NOT boosted");

    // return move.basePower;
    EventResult::Number(base_power)
}

