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
pub fn on_base_power(_battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

