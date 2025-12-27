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
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Last Resort fails unless the user knows at least 2 moves
    if source.move_slots.len() < 2 {
        return EventResult::Bool(false);
    }

    // User must actually have Last Resort for it to succeed
    let mut has_last_resort = false;
    for move_slot in &source.move_slots {
        if move_slot.id.as_str() == "lastresort" {
            has_last_resort = true;
            continue;
        }
        // All other moves must have been used
        if !move_slot.used {
            return EventResult::Bool(false);
        }
    }

    EventResult::Bool(has_last_resort)
}

