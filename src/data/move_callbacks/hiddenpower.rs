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
    _active_move: Option<&crate::battle_actions::ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;

    // move.type = pokemon.hpType || 'Dark';
    let hp_type = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if pokemon_pokemon.hp_type.is_empty() {
            String::from("Dark")
        } else {
            pokemon_pokemon.hp_type.clone()
        }
    };

    if let Some(ref current_move) = battle.active_move {
        current_move.borrow_mut().move_type = hp_type;
    }

    EventResult::Continue
}
