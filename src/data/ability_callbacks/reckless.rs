//! Reckless Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (move.recoil || move.hasCrashDamage) {
///         this.debug('Reckless boost');
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

