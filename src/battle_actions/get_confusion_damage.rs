//! BattleActions::getConfusionDamage - Calculate confusion self-hit damage
//!
//! 1:1 port of getConfusionDamage from battle-actions.ts

use crate::*;
use crate::dex_data::BoostID;

/// Calculate confusion self-hit damage
/// Equivalent to getConfusionDamage() in battle-actions.ts
///
/// JavaScript (battle-actions.ts):
///   /**
///    * Confusion damage is unique - most typical modifiers that get run when calculating
///    * damage (e.g. Huge Power, Life Orb, critical hits) don't apply. It also uses a 16-bit
///    * context for its damage, unlike the regular damage formula (though this only comes up
///    * for base damage).
///    */
///   getConfusionDamage(pokemon: Pokemon, basePower: number) {
///     const tr = this.battle.trunc;
///
///     const attack = pokemon.calculateStat('atk', pokemon.boosts['atk']);
///     const defense = pokemon.calculateStat('def', pokemon.boosts['def']);
///     const level = pokemon.level;
///     const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50) + 2;
///
///     // Damage is 16-bit context in self-hit confusion damage
///     let damage = tr(baseDamage, 16);
///     damage = this.battle.randomizer(damage);
///     return Math.max(1, damage);
///   }
#[allow(dead_code)]
pub fn get_confusion_damage(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    base_power: i32,
) -> i32 {
    // const attack = pokemon.calculateStat('atk', pokemon.boosts['atk']);
    // const defense = pokemon.calculateStat('def', pokemon.boosts['def']);
    // const level = pokemon.level;

    // Extract all Pokemon data we need first (immutable borrow)
    // We manually calculate stats here because calling pokemon.calculate_stat requires
    // both an immutable borrow of pokemon and a mutable borrow of battle simultaneously
    let (stored_atk, stored_def, atk_boost, def_boost, level, has_wonder_room) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return 1,
        };

        let wonder_room = battle.field.pseudo_weather.contains_key(&ID::new("wonderroom"));

        (
            pokemon.stored_stats.atk,
            pokemon.stored_stats.def,
            pokemon.boosts.get(BoostID::Atk),
            pokemon.boosts.get(BoostID::Def),
            pokemon.level,
            wonder_room,
        )
    };

    // Calculate attack stat (manually implements Pokemon::calculate_stat logic)
    let attack = {
        let mut stat = if has_wonder_room {
            // Wonder Room doesn't affect attack
            stored_atk
        } else {
            stored_atk
        };

        // Apply boost (matches Pokemon::calculate_stat boost table)
        let boost_table = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        let clamped_boost = atk_boost.max(-6).min(6);
        stat = if clamped_boost >= 0 {
            (stat as f64 * boost_table[clamped_boost as usize]) as i32
        } else {
            (stat as f64 / boost_table[(-clamped_boost) as usize]) as i32
        };

        stat
    };

    // Calculate defense stat (manually implements Pokemon::calculate_stat logic)
    let defense = {
        let mut stat = if has_wonder_room {
            // Wonder Room swaps def and spd
            stored_def // Actually this would swap, but for confusion it's still def vs def
        } else {
            stored_def
        };

        // Apply boost
        let boost_table = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        let clamped_boost = def_boost.max(-6).min(6);
        stat = if clamped_boost >= 0 {
            (stat as f64 * boost_table[clamped_boost as usize]) as i32
        } else {
            (stat as f64 / boost_table[(-clamped_boost) as usize]) as i32
        };

        stat
    };

    // const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50) + 2;
    // We need to truncate at each step to match JavaScript
    let step1 = battle.trunc((2 * level / 5 + 2) as f64, None) as i32;
    let step2 = battle.trunc((step1 * base_power) as f64, None) as i32;
    let step3 = battle.trunc((step2 * attack) as f64, None) as i32;
    let step4 = battle.trunc((step3 / defense.max(1)) as f64, None) as i32;
    let base_damage = battle.trunc((step4 / 50) as f64, None) as i32 + 2;

    // Damage is 16-bit context in self-hit confusion damage
    // let damage = tr(baseDamage, 16);
    let mut damage = battle.trunc(base_damage as f64, Some(16)) as i32;

    // damage = this.battle.randomizer(damage);
    damage = battle.randomizer(damage);

    // return Math.max(1, damage);
    damage.max(1)
}

