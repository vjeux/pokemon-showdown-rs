//! Knock Off Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source, target, move) {
///     const item = target.getItem();
///     if (!this.singleEvent('TakeItem', item, target.itemState, target, target, move, item)) return;
///     if (item.id) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterHit(target, source) {
///     if (source.hp) {
///         const item = target.takeItem();
///         if (item) {
///             this.add('-enditem', target, item.name, '[from] move: Knock Off', `[of] ${source}`);
///         }
///     }
/// }
pub fn on_after_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

