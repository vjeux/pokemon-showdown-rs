//! Guardian of Alola Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// damageCallback(pokemon, target) {
///     const hp75 = Math.floor(target.getUndynamaxedHP() * 3 / 4);
///     if (
///         target.volatiles['protect'] || target.volatiles['banefulbunker'] || target.volatiles['kingsshield'] ||
///         target.volatiles['spikyshield'] || target.side.getSideCondition('matblock')
///     ) {
///         this.add('-zbroken', target);
///         return this.clampIntRange(Math.ceil(hp75 / 4 - 0.5), 1);
///     }
///     return this.clampIntRange(hp75, 1);
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Calculate 75% of target's undynamaxed HP
    let hp75 = (target.get_undynamaxed_hp() * 3) / 4;

    // Check for protect-like volatiles
    let has_protect = target.has_volatile("protect");
    let has_banefulbunker = target.has_volatile("banefulbunker");
    let has_kingsshield = target.has_volatile("kingsshield");
    let has_spikyshield = target.has_volatile("spikyshield");

    // Check for matblock side condition
    let has_matblock = if let Some(side) = battle.sides.get(target_pos.0) {
        side.has_side_condition("matblock")
    } else {
        false
    };

    if has_protect || has_banefulbunker || has_kingsshield || has_spikyshield || has_matblock {
        // Z-Move breaks through protection but at reduced power
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.add("-zbroken", &[Arg::Pokemon(target)]);

        // Reduced damage: ceil(hp75 / 4 - 0.5) = floor((hp75 / 4) + 0.5) = floor((hp75 + 2) / 4)
        let reduced_damage = (hp75 + 2) / 4;
        let damage = battle.clamp_int_range(reduced_damage, 1);
        return EventResult::Number(damage);
    }

    // Normal damage: 75% of target's HP
    let damage = battle.clamp_int_range(hp75, 1);
    EventResult::Number(damage)
}

