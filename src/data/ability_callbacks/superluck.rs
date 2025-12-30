//! Super Luck Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyCritRatio(critRatio) {
///     return critRatio + 1;
/// }
pub fn on_modify_crit_ratio(_battle: &mut Battle, crit_ratio: i32, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    EventResult::Number(crit_ratio + 1)
}

