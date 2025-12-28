//! Bright Powder Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAccuracy(accuracy) {
///     if (typeof accuracy !== 'number') return;
///     this.debug('brightpowder - decreasing accuracy');
///     return this.chainModify([3686, 4096]);
/// }
pub fn on_modify_accuracy(battle: &mut Battle) -> EventResult {
    // In Rust, the accuracy parameter would be in the event modifier
    // The typeof check is implicit in the type system

    // this.debug('brightpowder - decreasing accuracy');
    battle.debug("brightpowder - decreasing accuracy");

    // return this.chainModify([3686, 4096]);
    battle.chain_modify_fraction(3686, 4096);

    EventResult::Continue
}
