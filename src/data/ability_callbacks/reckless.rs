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
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // if (move.recoil || move.hasCrashDamage) {
    //     this.debug('Reckless boost');
    //     return this.chainModify([4915, 4096]);
    // }

    if let Some(ref active_move) = battle.active_move {
        // Check if move has recoil or crash damage
        if active_move.recoil.is_some() || active_move.has_crash_damage.unwrap_or(false) {
            eprintln!("Reckless boost");
            return EventResult::Number(battle.chain_modify_fraction(4915, 4096));
        }
    }

    EventResult::Continue
}

