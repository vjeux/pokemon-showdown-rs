//! Revelation Dance Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     const types = pokemon.getTypes();
///     let type = types[0];
///     if (type === 'Bird') type = '???';
///     if (type === '???' && types[1]) type = types[1];
///     move.type = type;
/// }
pub fn on_modify_type(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    

    // const types = pokemon.getTypes();
    // let type = types[0];
    // if (type === 'Bird') type = '???';
    // if (type === '???' && types[1]) type = types[1];
    // move.type = type;
    let types = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_types(false)
    };

    let mut move_type = types.get(0).cloned().unwrap_or(String::from("Normal"));

    if move_type.as_str() == "Bird" {
        move_type = String::from("???");
    }

    if move_type.as_str() == "???" {
        if let Some(second_type) = types.get(1) {
            move_type = second_type.clone();
        }
    }

    if let Some(active_move) = &mut battle.active_move {
        active_move.move_type = move_type;
    }

    EventResult::Continue
}

