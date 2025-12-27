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
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

