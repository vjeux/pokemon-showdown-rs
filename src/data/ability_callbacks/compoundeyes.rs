//! Compound Eyes Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyAccuracy(accuracy) {
///     if (typeof accuracy !== 'number') return;
///     this.debug('compoundeyes - enhancing accuracy');
///     return this.chainModify([5325, 4096]);
/// }
pub fn on_source_modify_accuracy(battle: &mut Battle, accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    let modified = battle.chain_modify_fraction(5325, 4096);
    EventResult::Number(modified)
}

