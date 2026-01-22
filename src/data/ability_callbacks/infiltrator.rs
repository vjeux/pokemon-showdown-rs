//! Infiltrator Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyMove(move) {
///     move.infiltrates = true;
/// }
pub fn on_modify_move(_battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // move.infiltrates = true;
    if let Some(active_move) = active_move {
        active_move.infiltrates = true;
    }

    EventResult::Continue
}

