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
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // move.type = pokemon.hpType || 'Dark';
    let hp_type = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp_type.clone().unwrap_or(ID::from("dark"))
    };

    if let Some(ref current_move_id) = battle.active_move {
        if let Some(current_move) = battle.dex.get_move_by_id_mut(current_move_id) {
            current_move.move_type = hp_type;
        }
    }

    EventResult::Continue
}

