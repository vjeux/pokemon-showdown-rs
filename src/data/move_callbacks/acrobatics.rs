//! Acrobatics Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (!pokemon.item) {
///         this.debug("BP doubled for no item");
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => &active_move.id,
        None => return EventResult::Continue,
    };

    // Get the move data and extract base_power before mutable borrow
    let base_power = match battle.dex.get_move_by_id(move_id) {
        Some(m) => m.base_power,
        None => return EventResult::Continue,
    };

    // if (!pokemon.item)
    if pokemon.item.is_empty() {
        // this.debug("BP doubled for no item");
        battle.debug("BP doubled for no item");
        // return move.basePower * 2;
        return EventResult::Number(base_power * 2);
    }

    // return move.basePower;
    EventResult::Number(base_power)
}

