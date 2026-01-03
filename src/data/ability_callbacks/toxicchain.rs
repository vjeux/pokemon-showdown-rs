//! Toxic Chain Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceDamagingHit(damage, target, source, move) {
///     // Despite not being a secondary, Shield Dust / Covert Cloak block Toxic Chain's effect
///     if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
/// 
///     if (this.randomChance(3, 10)) {
///         target.trySetStatus('tox', source);
///     }
/// }
pub fn on_source_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // 30% chance to badly poison the target after damaging them
    if let (Some(target), Some(_source)) = (target_pos, source_pos) {
        // TODO: Check for Shield Dust ability and Covert Cloak item when those are implemented

        if battle.random_chance(3, 10) {
            // Try to set toxic (badly poisoned) status on the target
            crate::pokemon::Pokemon::try_set_status(battle, target, crate::ID::from("tox"), None);
        }
    }
    EventResult::Continue
}

