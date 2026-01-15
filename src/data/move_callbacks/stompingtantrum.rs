//! Stomping Tantrum Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::battle_actions::MoveResult;

/// basePowerCallback(pokemon, target, move) {
///     if (pokemon.moveLastTurnResult === false) {
///         this.debug('doubling Stomping Tantrum BP due to previous move failure');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (pokemon.moveLastTurnResult === false)
    // IMPORTANT: In JavaScript, `moveLastTurnResult === false` only matches explicit false,
    // NOT null or undefined. MoveResult::Failed represents explicit false.
    // MoveResult::Null (from moves blocked by Protect) does NOT trigger this.
    if pokemon.move_last_turn_result == MoveResult::Failed {
        let bp = active_move.base_power * 2;
        // this.debug('doubling Stomping Tantrum BP due to previous move failure');
        battle.debug("doubling Stomping Tantrum BP due to previous move failure");
        return EventResult::Number(bp);
    }

    EventResult::Number(active_move.base_power)
}
