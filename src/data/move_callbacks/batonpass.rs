//! Baton Pass Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (!this.canSwitch(target.side) || target.volatiles['commanded']) {
///         this.attrLastMove('[still]');
///         this.add('-fail', target);
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

