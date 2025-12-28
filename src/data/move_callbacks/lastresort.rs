//! Last Resort Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.moveSlots.length < 2) return false; // Last Resort fails unless the user knows at least 2 moves
///     let hasLastResort = false; // User must actually have Last Resort for it to succeed
///     for (const moveSlot of source.moveSlots) {
///         if (moveSlot.id === 'lastresort') {
///             hasLastResort = true;
///             continue;
///         }
///         if (!moveSlot.used) return false;
///     }
///     return hasLastResort;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (source.moveSlots.length < 2) return false; // Last Resort fails unless the user knows at least 2 moves
    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if source_pokemon.move_slots.len() < 2 {
        return EventResult::Boolean(false);
    }

    // let hasLastResort = false; // User must actually have Last Resort for it to succeed
    // for (const moveSlot of source.moveSlots) {
    //     if (moveSlot.id === 'lastresort') {
    //         hasLastResort = true;
    //         continue;
    //     }
    //     if (!moveSlot.used) return false;
    // }
    let mut has_last_resort = false;
    for move_slot in &source_pokemon.move_slots {
        if move_slot.id == ID::from("lastresort") {
            has_last_resort = true;
            continue;
        }
        if !move_slot.used {
            return EventResult::Boolean(false);
        }
    }

    // return hasLastResort;
    EventResult::Boolean(has_last_resort)
}

