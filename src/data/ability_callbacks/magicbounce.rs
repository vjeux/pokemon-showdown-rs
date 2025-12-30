//! Magic Bounce Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target === source || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
///         return;
///     }
///     const newMove = this.dex.getActiveMove(move.id);
///     newMove.hasBounced = true;
///     newMove.pranksterBoosted = false;
///     this.actions.useMove(newMove, target, { target: source });
///     return null;
/// }
pub fn on_try_hit(_battle: &mut Battle, _target_pos: (usize, usize), _source_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAllyTryHitSide(target, source, move) {
///     if (target.isAlly(source) || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
///         return;
///     }
///     const newMove = this.dex.getActiveMove(move.id);
///     newMove.hasBounced = true;
///     newMove.pranksterBoosted = false;
///     this.actions.useMove(newMove, this.effectState.target, { target: source });
///     move.hasBounced = true; // only bounce once in free-for-all battles
///     return null;
/// }
pub fn on_ally_try_hit_side(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

