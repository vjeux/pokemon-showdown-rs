//! Compound Eyes Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onSourceModifyAccuracy(accuracy) {
///     if (typeof accuracy !== 'number') return;
///     this.debug('compoundeyes - enhancing accuracy');
///     return this.chainModify([5325, 4096]);
/// }
pub fn on_source_modify_accuracy(battle: &mut Battle, _accuracy: i32, _target_pos: (usize, usize), _source_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    battle.chain_modify_fraction(5325, 4096);
    EventResult::Continue
}

