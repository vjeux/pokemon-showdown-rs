//! Poison Touch Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceDamagingHit(damage, target, source, move) {
///     // Despite not being a secondary, Shield Dust / Covert Cloak block Poison Touch's effect
///     if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
///     if (this.checkMoveMakesContact(move, target, source)) {
///         if (this.randomChance(3, 10)) {
///             target.trySetStatus('psn', source);
///         }
///     }
/// }
pub fn on_source_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // 30% chance to poison the target when attacker makes contact
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // TODO: Check for Shield Dust ability and Covert Cloak item when those are implemented

        if battle.check_move_makes_contact(&crate::ID::from(move_id), target, source, false) {
            if battle.random_chance(3, 10) {
                // Try to set poison status on the target
                crate::pokemon::Pokemon::try_set_status(battle, target, crate::ID::from("psn"), None);
            }
        }
    }
    EventResult::Continue
}

