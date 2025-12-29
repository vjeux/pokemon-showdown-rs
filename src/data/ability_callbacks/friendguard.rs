//! Friend Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyModifyDamage(damage, source, target, move) {
///     if (target !== this.effectState.target && target.isAlly(this.effectState.target)) {
///         this.debug('Friend Guard weaken');
///         return this.chainModify(0.75);
///     }
/// }
pub fn on_any_modify_damage(battle: &mut Battle, damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

