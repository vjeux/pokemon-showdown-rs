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
pub fn on_source_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // 30% chance to poison the target when attacker makes contact
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // Despite not being a secondary, Shield Dust / Covert Cloak block Poison Touch's effect
        // if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
        let has_shield_dust_or_cloak = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.has_ability(battle, &["shielddust"]) || target_pokemon.has_item(battle, &["covertcloak"])
        };

        if has_shield_dust_or_cloak {
            return EventResult::Continue;
        }

        // IMPORTANT: Use the ActiveMove directly to get the correct flags (including inherited flags for G-Max moves)
        // Note: For on_source_damaging_hit, target is the defender and source is the attacker (ability holder)
        // So source attacks target, we check contact with (source, target) for the attacker/defender relationship
        if battle.check_move_makes_contact_with_active_move(active_move, source, target, false) {
            if battle.random_chance(3, 10) {
                // target.trySetStatus('psn', source);
                // Note: source (the Poison Touch Pokemon/attacker) is the source of the poison status
                // This is important for Synchronize to know who to pass the status back to
                crate::pokemon::Pokemon::try_set_status(battle, target, crate::ID::from("psn"), Some(source), None);
            }
        }
    }
    EventResult::Continue
}

