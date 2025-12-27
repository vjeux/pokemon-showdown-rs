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
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let types = pokemon.get_types();
    let mut move_type = types.first().map(|s| s.as_str()).unwrap_or("Normal");

    if move_type == "Bird" {
        move_type = "???";
    }

    if move_type == "???" && types.len() > 1 {
        move_type = types[1].as_str();
    }

    // TODO: Need battle.dex.getActiveMove(move_id) to get mutable move reference
    // For now, this is a placeholder - we need ActiveMove mutation system
    battle.debug(&format!("Revelation Dance type: {}", move_type));

    EventResult::Continue
}

