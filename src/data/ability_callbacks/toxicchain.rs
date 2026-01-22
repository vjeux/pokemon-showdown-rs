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
pub fn on_source_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // 30% chance to badly poison the target after damaging them
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // Despite not being a secondary, Shield Dust / Covert Cloak block Toxic Chain's effect
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

        if battle.random_chance(3.0, 10) {
            // target.trySetStatus('tox', source);
            // Note: source (the Toxic Chain Pokemon/attacker) is the source of the toxic status
            // This is important for Synchronize to know who to pass the status back to
            crate::pokemon::Pokemon::try_set_status(battle, target, crate::ID::from("tox"), Some(source), None);
        }
    }
    EventResult::Continue
}

