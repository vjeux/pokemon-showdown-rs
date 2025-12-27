//! Power Split Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     const newatk = Math.floor((target.storedStats.atk + source.storedStats.atk) / 2);
///     target.storedStats.atk = newatk;
///     source.storedStats.atk = newatk;
///     const newspa = Math.floor((target.storedStats.spa + source.storedStats.spa) / 2);
///     target.storedStats.spa = newspa;
///     source.storedStats.spa = newspa;
///     this.add('-activate', source, 'move: Power Split', `[of] ${target}`);
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

