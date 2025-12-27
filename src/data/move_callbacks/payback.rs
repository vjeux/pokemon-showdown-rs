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
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let move_id = match &battle.active_move {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    let move_data = match battle.dex.get_move_by_id(move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    let base_power = move_data.base_power;

    // Check if target is newly switched or will move this turn
    let newly_switched = target.newly_switched;
    drop(target); // Release borrow before calling queue methods

    let will_move = battle.queue.will_move(target_pos.0, target_pos.1).is_some();

    if newly_switched || will_move {
        // Target moved first or will move - NOT boosted
        battle.debug("Payback NOT boosted");
        EventResult::Number(base_power)
    } else {
        // Target moved after user - boosted
        battle.debug("Payback damage boost");
        EventResult::Number(base_power * 2)
    }
}

