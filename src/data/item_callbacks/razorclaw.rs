//! Razor Claw Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyCritRatio(critRatio) {
///     return critRatio + 1;
/// }
pub fn on_modify_crit_ratio(battle: &mut Battle) -> EventResult {
    // return critRatio + 1;
    // TODO: Need critRatio parameter passed to this callback and ability to return modified value
    // This callback should increase the critical hit ratio by 1 stage
    EventResult::Continue
}
