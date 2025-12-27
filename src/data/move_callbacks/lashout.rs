//! Lash Out Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source) {
///     if (source.statsLoweredThisTurn) {
///         this.debug('lashout buff');
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

