//! After You Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// onHit(target) {
///     if (this.activePerHalf === 1) return false; // fails in singles
///     const action = this.queue.willMove(target);
///     if (action) {
///         this.queue.prioritizeAction(action);
///         this.add('-activate', target, 'move: After You');
///     } else {
///         return false;
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Fails in singles (when only 1 pokemon per side is active)
    if battle.active_per_half == 1 {
        return EventResult::Bool(false);
    }

    // Check if target has a queued move action
    let has_action = battle.queue.will_move(target_pos.0, target_pos.1).is_some();

    if has_action {
        // Prioritize the target's action (move it to front of queue)
        battle.queue.prioritize_action(target_pos.0, target_pos.1);

        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.add("-activate", &[Arg::Pokemon(target), Arg::Str("move: After You")]);

        EventResult::Continue
    } else {
        // Target has no queued action
        EventResult::Bool(false)
    }
}

