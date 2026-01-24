//! Loaded Dice Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.multiaccuracy) {
///         delete move.multiaccuracy;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.multiaccuracy) {
    //     delete move.multiaccuracy;
    // }

    if let Some(active_move) = battle.active_move.as_mut() {
        if active_move.borrow().multi_accuracy {
            active_move.borrow_mut().multi_accuracy = false;
        }
    }

    EventResult::Continue
}
