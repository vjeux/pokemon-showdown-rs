//! Long Reach Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     delete move.flags['contact'];
/// }
pub fn on_modify_move(battle: &mut Battle, _move_id: &str, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // delete move.flags['contact'];
    if let Some(ref mut active_move) = battle.active_move {
        active_move.flags.contact = false;
    }

    EventResult::Continue
}

