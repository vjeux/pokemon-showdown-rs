//! Water Spout Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     const bp = move.basePower * pokemon.hp / pokemon.maxhp;
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the pokemon (user of the move)
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => &active_move.id,
        None => return EventResult::Continue,
    };

    // Get the move data
    let move_data = match battle.dex.get_move_by_id(move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Calculate base power based on current HP ratio
    let bp = move_data.base_power * pokemon.hp / pokemon.maxhp;
    // Note: JS has this.debug call which we don't have infrastructure for yet
    // this.debug(`BP: ${bp}`);
    EventResult::Number(bp)
}
