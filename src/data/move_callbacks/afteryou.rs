//! After You Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
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
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

