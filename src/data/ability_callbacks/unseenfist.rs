//! Unseen Fist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.flags['contact']) delete move.flags['protect'];
/// }
pub fn on_modify_move(_battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.flags['contact']) delete move.flags['protect'];
    if let Some(active_move) = active_move {
        if active_move.flags.contact {
            // delete move.flags['protect']; - in Rust, we set it to false
            active_move.flags.protect = false;
        }
    }
    EventResult::Continue
}

