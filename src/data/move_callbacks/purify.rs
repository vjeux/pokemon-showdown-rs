//! Purify Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (!target.cureStatus()) {
///         this.add('-fail', source);
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
///     this.heal(Math.ceil(source.maxhp * 0.5), source);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

