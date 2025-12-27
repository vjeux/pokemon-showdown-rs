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
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = pokemon_pos;

    // const types = pokemon.getTypes();
    let types = battle.get_types(pokemon, false);

    // let type = types[0];
    let mut type_str = types.get(0).cloned().unwrap_or_else(|| "Normal".to_string());

    // if (type === 'Bird') type = '???';
    if type_str == "Bird" {
        type_str = "???".to_string();
    }

    // if (type === '???' && types[1]) type = types[1];
    if type_str == "???" {
        if let Some(second_type) = types.get(1) {
            type_str = second_type.clone();
        }
    }

    // move.type = type;
    let active_move = match &mut battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };
    active_move.move_type = type_str;

    EventResult::Continue
}

