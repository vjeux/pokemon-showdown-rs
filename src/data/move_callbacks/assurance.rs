//! Assurance Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (target.hurtThisTurn) {
///         this.debug('BP doubled on damaged target');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the target position
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the target pokemon
    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
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

    // if (target.hurtThisTurn) {
    if target_pokemon.hurt_this_turn.is_some() {
        // this.debug('BP doubled on damaged target');
        battle.debug("BP doubled on damaged target");
        // return move.basePower * 2;
        return EventResult::Int(move_data.base_power * 2);
    }

    // return move.basePower;
    EventResult::Int(move_data.base_power)
}

