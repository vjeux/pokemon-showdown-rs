//! Scope Lens Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyCritRatio(critRatio) {
///     return critRatio + 1;
/// }
pub fn on_modify_crit_ratio(_battle: &mut Battle, _pokemon_pos: (usize, usize), crit_ratio: i32) -> EventResult {
    // return critRatio + 1;
    EventResult::Number(crit_ratio + 1)
}
