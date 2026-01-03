//! Poison Point Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target)) {
///         if (this.randomChance(3, 10)) {
///             source.trySetStatus('psn', target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // 30% chance to poison attacker on contact
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        if battle.check_move_makes_contact(&crate::ID::from(move_id), source, target, false) {
            if battle.random_chance(3, 10) {
                // Try to set poison status on the attacker
                crate::pokemon::Pokemon::try_set_status(battle, source, crate::ID::from("psn"), None);
            }
        }
    }
    EventResult::Continue
}

