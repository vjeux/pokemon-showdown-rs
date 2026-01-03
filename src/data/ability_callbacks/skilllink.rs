//! Skill Link Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.multihit && Array.isArray(move.multihit) && move.multihit.length) {
///         move.multihit = move.multihit[1];
///     }
///     if (move.multiaccuracy) {
///         delete move.multiaccuracy;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, _move_id: &str) -> EventResult {
    // if (move.multihit && Array.isArray(move.multihit) && move.multihit.length)
    // In JavaScript, multihit can be a number or [min, max] array
    // In Rust, multi_hit is Option<i32> and multi_hit_type is Option<String>
    // When multi_hit_type is Some (indicating it's a range), we set multi_hit to the max value
    // JavaScript stores ranges as [min, max] and we need to set it to max (index 1)

    if let Some(ref mut active_move) = battle.active_move {
        // Check if this is a multi-hit range (2-5 hits, etc.)
        // multi_hit_type being Some indicates it's a range like "2-5"
        if active_move.multi_hit_type.is_some() {
            // Parse the multi_hit_type to get the maximum value
            if let Some(ref hit_type) = active_move.multi_hit_type {
                // multi_hit_type is like "2-5", we want the max (5)
                if let Some(max_str) = hit_type.split('-').nth(1) {
                    if let Ok(max_hits) = max_str.parse::<i32>() {
                        // move.multihit = move.multihit[1]
                        active_move.multi_hit = Some(max_hits);
                    }
                }
            }
        }

        // if (move.multiaccuracy)
        //     delete move.multiaccuracy
        // TODO: ActiveMove doesn't have a multi_accuracy field yet
        // This is for moves like Triple Axel that can miss on later hits
        // For now, skip this part as the infrastructure doesn't exist
    }

    EventResult::Continue
}

