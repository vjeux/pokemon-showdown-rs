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
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // if (pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) move.category = 'Physical';
    let (atk, spa) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let atk = pokemon_pokemon.get_stat("atk", false);
        let spa = pokemon_pokemon.get_stat("spa", false);
        (atk, spa)
    };

    if atk > spa {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.category = String::from("Physical");
        }
    }

    EventResult::Continue
}

