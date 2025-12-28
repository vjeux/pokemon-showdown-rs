//! Bolt Beak Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (target.newlySwitched || this.queue.willMove(target)) {
///         this.debug('Bolt Beak damage boost');
///         return move.basePower * 2;
///     }
///     this.debug('Bolt Beak NOT boosted');
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the active move
    let move_id = match &battle.active_move {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    // Get the move data
    let move_data = match battle.dex.get_move_by_id(move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (target.newlySwitched || this.queue.willMove(target)) {
    let should_boost = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Check if target newly switched
        let newly_switched = target_pokemon.newly_switched;

        // Check if target will move
        let will_move = battle.queue.will_move(target.0, target.1).is_some();

        newly_switched || will_move
    };

    if should_boost {
        // this.debug('Bolt Beak damage boost');
        battle.debug("Bolt Beak damage boost");
        // return move.basePower * 2;
        return EventResult::Number(move_data.base_power * 2);
    }

    // this.debug('Bolt Beak NOT boosted');
    battle.debug("Bolt Beak NOT boosted");
    // return move.basePower;
    EventResult::Number(move_data.base_power)
}

