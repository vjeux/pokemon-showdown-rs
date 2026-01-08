//! Skill Link Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex::Multihit;

/// onModifyMove(move) {
///     if (move.multihit && Array.isArray(move.multihit) && move.multihit.length) {
///         move.multihit = move.multihit[1];
///     }
///     if (move.multiaccuracy) {
///         delete move.multiaccuracy;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.multihit && Array.isArray(move.multihit) && move.multihit.length)
    // In JavaScript, multihit can be a number or [min, max] array
    // When it's an array (Range in Rust), we set it to the max value (index 1)

    if let Some(active_move) = active_move {
        // Check if this is a multi-hit range
        if let Some(Multihit::Range(_min, max)) = active_move.multi_hit {
            // move.multihit = move.multihit[1]
            // Set to the maximum value from the range
            active_move.multi_hit = Some(Multihit::Fixed(max));
        }

        // if (move.multiaccuracy)
        //     delete move.multiaccuracy
        // Set multi_accuracy to false to disable per-hit accuracy checks
        // This makes moves like Triple Axel always hit all times
        active_move.multi_accuracy = false;
    }

    EventResult::Continue
}

