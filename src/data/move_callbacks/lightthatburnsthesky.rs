//! Light That Burns the Sky Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon) {
///     if (pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) move.category = 'Physical';
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) move.category = 'Physical';
    let (atk, spa) = (
        battle.get_pokemon_stat(pokemon, crate::dex_data::StatID::Atk, false, true),
        battle.get_pokemon_stat(pokemon, crate::dex_data::StatID::SpA, false, true),
    );

    if atk > spa {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.borrow_mut().category = String::from("Physical");
        }
    }

    EventResult::Continue
}
