//! Liquid Voice Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (move.flags['sound'] && !pokemon.volatiles['dynamax']) { // hardcode
///         move.type = 'Water';
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.flags['sound'] && !pokemon.volatiles['dynamax'])
    let has_sound_flag = active_move.as_ref().map(|m| m.flags.sound).unwrap_or(false);

    if !has_sound_flag {
        return EventResult::Continue;
    }

    let has_dynamax = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        use crate::dex_data::ID;
        let dynamax_id = ID::from("dynamax");
        pokemon.volatiles.contains_key(&dynamax_id)
    };

    if has_dynamax {
        return EventResult::Continue;
    }

    // move.type = 'Water';
    if let Some(active_move) = active_move {
        active_move.move_type = "Water".to_string();
    }

    EventResult::Continue
}

