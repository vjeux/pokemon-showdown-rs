//! Technician Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     const basePowerAfterMultiplier = this.modify(basePower, this.event.modifier);
///     this.debug(`Base Power: ${basePowerAfterMultiplier}`);
///     if (basePowerAfterMultiplier <= 60) {
///         this.debug('Technician boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(_battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

