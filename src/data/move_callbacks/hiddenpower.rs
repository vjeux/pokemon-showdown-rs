//! Hidden Power Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     move.type = pokemon.hpType || 'Dark';
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _move_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;

    // move.type = pokemon.hpType || 'Dark';
    let hp_type = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon
            .hp_type
            .clone()
            .unwrap_or_else(|| String::from("Dark"))
    };

    if let Some(ref mut current_move) = battle.active_move {
        current_move.move_type = hp_type;
    }

    EventResult::Continue
}
