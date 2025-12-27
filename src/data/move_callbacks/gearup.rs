//! Gear Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitSide(side, source, move) {
///     const targets = side.allies().filter(target => (
///         target.hasAbility(['plus', 'minus']) &&
///         (!target.volatiles['maxguard'] || this.runEvent('TryHit', target, source, move))
///     ));
///     if (!targets.length) return false;
///     let didSomething = false;
///     for (const target of targets) {
///         didSomething = this.boost({ atk: 1, spa: 1 }, target, source, move, false, true) || didSomething;
///     }
///     return didSomething;
/// }
pub fn on_hit_side(battle: &mut Battle, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

