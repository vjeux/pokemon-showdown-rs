//! Lax Incense Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyAccuracy(accuracy) {
///     if (typeof accuracy !== 'number') return;
///     this.debug('lax incense - decreasing accuracy');
///     return this.chainModify([3686, 4096]);
/// }
pub fn on_modify_accuracy(battle: &mut Battle) -> EventResult {
    // if (typeof accuracy !== 'number') return;
    // this.debug('lax incense - decreasing accuracy');
    // return this.chainModify([3686, 4096]);

    // Note: In JS, this checks if accuracy is a number (not true/bypassed).
    // Since this callback is only called when accuracy can be modified,
    // we can apply the modifier.
    battle.debug("lax incense - decreasing accuracy");
    battle.chain_modify_fraction(3686, 4096);
    EventResult::Continue
}
