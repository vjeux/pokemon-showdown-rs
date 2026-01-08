//! Stalwart Ability
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
pub fn on_modify_move(battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // move.tracksTarget = move.target !== 'scripted';
    if let Some(active_move) = active_move {
        active_move.tracks_target = active_move.target != "scripted";
    }

    EventResult::Continue
}

