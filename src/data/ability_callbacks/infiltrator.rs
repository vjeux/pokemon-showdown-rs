//! Infiltrator Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     move.infiltrates = true;
/// }
pub fn on_modify_move(battle: &mut Battle, _move_id: &str) -> EventResult {
    // move.infiltrates = true;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.infiltrates = true;
    }

    EventResult::Continue
}

