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
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // 30% chance to poison attacker on contact
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // IMPORTANT: Use the ActiveMove directly to get the correct flags (including inherited flags for G-Max moves)
        if battle.check_move_makes_contact_with_active_move(active_move, source, target, false) {
            if battle.random_chance(3, 10) {
                // Try to set poison status on the attacker
                crate::pokemon::Pokemon::try_set_status(battle, source, crate::ID::from("psn"), None);
            }
        }
    }
    EventResult::Continue
}

