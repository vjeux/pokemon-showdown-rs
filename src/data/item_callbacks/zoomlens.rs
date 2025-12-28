//! Zoom Lens Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyAccuracy(accuracy, target) {
///     if (typeof accuracy === 'number' && !this.queue.willMove(target)) {
///         this.debug('Zoom Lens boosting accuracy');
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_source_modify_accuracy(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (typeof accuracy === 'number' && !this.queue.willMove(target)) {
    //     this.debug('Zoom Lens boosting accuracy');
    //     return this.chainModify([4915, 4096]);
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // !this.queue.willMove(target)
    let will_move = battle.queue.will_move(target.0, target.1);
    if will_move.is_none() {
        // this.debug('Zoom Lens boosting accuracy');
        battle.debug("Zoom Lens boosting accuracy");

        // return this.chainModify([4915, 4096]);
        return EventResult::Number(battle.chain_modify_fraction(4915, 4096));
    }

    EventResult::Continue
}
