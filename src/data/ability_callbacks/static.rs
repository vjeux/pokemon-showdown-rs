//! Static Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target)) {
///         if (this.randomChance(3, 10)) {
///             source.trySetStatus('par', target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // 30% chance to paralyze attacker on contact
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        if battle.check_move_makes_contact(&crate::ID::from(move_id), source, target, false) {
            if battle.random_chance(3, 10) {
                // Try to set paralysis status on the attacker
                crate::pokemon::Pokemon::try_set_status(battle, source, crate::ID::from("par"), None);
            }
        }
    }
    EventResult::Continue
}

