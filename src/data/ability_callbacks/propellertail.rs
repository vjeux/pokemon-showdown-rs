//! Propeller Tail Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     // most of the implementation is in Battle#getTarget
///     move.tracksTarget = move.target !== 'scripted';
/// }
pub fn on_modify_move(battle: &mut Battle, _move_id: &str, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // move.tracksTarget = move.target !== 'scripted';
    if let Some(ref mut active_move) = battle.active_move {
        if active_move.target != "scripted" {
            active_move.tracks_target = true;
        }
    }

    EventResult::Continue
}

