//! Bestow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     if (target.item) {
///         return false;
///     }
///     const myItem = source.takeItem();
///     if (!myItem) return false;
///     if (!this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem) || !target.setItem(myItem)) {
///         source.item = myItem.id;
///         return false;
///     }
///     this.add('-item', target, myItem.name, '[from] move: Bestow', `[of] ${source}`);
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

