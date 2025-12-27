//! Water Shuriken Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (pokemon.species.name === 'Greninja-Ash' && pokemon.hasAbility('battlebond') &&
///         !pokemon.transformed) {
///         return move.basePower + 5;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
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

    // TODO: Add hasAbility('battlebond') check when ability system is ready
    if pokemon.species == "Greninja-Ash" && !pokemon.transformed {
        EventResult::Number(move_data.base_power + 5)
    } else {
        EventResult::Number(move_data.base_power)
    }
}

