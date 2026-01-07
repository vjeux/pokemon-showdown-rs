//! Confusion Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target, source, sourceEffect) {
///     if (sourceEffect?.id === 'lockedmove') {
///         this.add('-start', target, 'confusion', '[fatigue]');
///     } else if (sourceEffect?.effectType === 'Ability') {
///         this.add('-start', target, 'confusion', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else {
///         this.add('-start', target, 'confusion');
///     }
///     const min = sourceEffect?.id === 'axekick' ? 3 : 2;
///     this.effectState.time = this.random(min, 6);
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // Get target ident
    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // if (sourceEffect?.id === 'lockedmove')
    let is_lockedmove = battle.effect.as_ref().map(|e| e.as_str() == "lockedmove").unwrap_or(false);

    // else if (sourceEffect?.effectType === 'Ability')
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_lockedmove {
        // this.add('-start', target, 'confusion', '[fatigue]');
        battle.add("-start", &[Arg::String(target_ident), Arg::Str("confusion"), Arg::Str("[fatigue]")]);
    } else if is_ability {
        // this.add('-start', target, 'confusion', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
        let ability_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
            .map(|ab| ab.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let source_ident = battle.active_pokemon
            .and_then(|(side_idx, poke_idx)| battle.pokemon_at(side_idx, poke_idx))
            .map(|p| p.get_slot())
            .unwrap_or_else(|| String::new());

        battle.add(
            "-start",
            &[
                Arg::String(target_ident),
                Arg::Str("confusion"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-start', target, 'confusion');
        battle.add("-start", &[Arg::String(target_ident), Arg::Str("confusion")]);
    }

    // const min = sourceEffect?.id === 'axekick' ? 3 : 2;
    let min = if battle.effect.as_ref().map(|e| e.as_str() == "axekick").unwrap_or(false) {
        3
    } else {
        2
    };

    // this.effectState.time = this.random(min, 6);
    let time = battle.random_with_range(min, 6);

    // Set time in effect state using with_effect_state
    // In JavaScript: this.effectState.time = value
    battle.with_effect_state(|state| {
        state.data.insert("time".to_string(), serde_json::Value::Number(serde_json::Number::from(time)));
    });

    EventResult::Continue
}

/// onEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEnd(target) {
///     this.add('-end', target, 'confusion');
/// }
/// ```
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-end', target, 'confusion');
    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-end", &[Arg::String(target_ident), Arg::Str("confusion")]);

    EventResult::Continue
}

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeMovePriority: 3,
/// onBeforeMove(pokemon) {
///     pokemon.volatiles['confusion'].time--;
///     if (!pokemon.volatiles['confusion'].time) {
///         pokemon.removeVolatile('confusion');
///         return;
///     }
///     this.add('-activate', pokemon, 'confusion');
///     if (!this.randomChance(33, 100)) {
///         return;
///     }
///     this.activeTarget = pokemon;
///     const damage = this.actions.getConfusionDamage(pokemon, 40);
///     if (typeof damage !== 'number') throw new Error("Confusion damage not dealt");
///     const activeMove = { id: this.toID('confused'), effectType: 'Move', type: '???' };
///     this.damage(damage, pokemon, pokemon, activeMove as ActiveMove);
///     return false;
/// }
/// ```
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // pokemon.volatiles['confusion'].time--;
    let time_before = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let confusion_id = ID::from("confusion");
        pokemon.volatiles.get(&confusion_id)
            .and_then(|v| v.data.get("time"))
            .and_then(|t| t.as_i64())
            .unwrap_or(0)
    };

    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let confusion_id = ID::from("confusion");
        if let Some(volatile) = pokemon.volatiles.get_mut(&confusion_id) {
            if let Some(time_val) = volatile.data.get_mut("time") {
                if let Some(time) = time_val.as_i64() {
                    *time_val = serde_json::Value::Number(serde_json::Number::from(time - 1));
                }
            }
        }
    }

    // if (!pokemon.volatiles['confusion'].time)
    let time_is_zero = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let confusion_id = ID::from("confusion");
        let time_after = pokemon.volatiles.get(&confusion_id)
            .and_then(|v| v.data.get("time"))
            .and_then(|t| t.as_i64())
            .unwrap_or(-1);

        time_after == 0
    };

    if time_is_zero {
        // pokemon.removeVolatile('confusion');
        let confusion_id = ID::from("confusion");
        crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &confusion_id);
        // return;
        return EventResult::Continue;
    }

    // this.add('-activate', pokemon, 'confusion');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[Arg::String(pokemon_ident.clone()), Arg::Str("confusion")]);

    // if (!this.randomChance(33, 100))
    let hit_self = battle.random_chance(33, 100);
    if !hit_self {
        // return;
        return EventResult::Continue;
    }

    // this.activeTarget = pokemon;
    battle.active_target = Some(pokemon_pos);

    // const damage = this.actions.getConfusionDamage(pokemon, 40);
    // Inline the confusion damage calculation
    // JavaScript: getConfusionDamage(pokemon, basePower)
    let damage = {
        // Extract all needed data from pokemon first
        let (atk_stat, def_stat, atk_boost, def_boost, level) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (
                pokemon.stored_stats.atk,
                pokemon.stored_stats.def,
                pokemon.boosts.atk,
                pokemon.boosts.def,
                pokemon.level as f64
            )
        };

        // const attack = pokemon.calculateStat('atk', pokemon.boosts['atk']);
        // Inline boost calculation for attack
        let attack = {
            let clamped_boost = atk_boost.clamp(-6, 6);
            let boost_table: [f64; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
            if clamped_boost >= 0 {
                atk_stat as f64 * boost_table[clamped_boost as usize]
            } else {
                atk_stat as f64 / boost_table[(-clamped_boost) as usize]
            }
        };

        // const defense = pokemon.calculateStat('def', pokemon.boosts['def']);
        // Inline boost calculation for defense
        let defense = {
            let clamped_boost = def_boost.clamp(-6, 6);
            let boost_table: [f64; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
            if clamped_boost >= 0 {
                def_stat as f64 * boost_table[clamped_boost as usize]
            } else {
                def_stat as f64 / boost_table[(-clamped_boost) as usize]
            }
        };

        // const basePower = 40;
        let base_power = 40.0;

        // const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50) + 2;
        let base_damage = {
            let step1 = battle.trunc(2.0 * level / 5.0 + 2.0, None);
            let step2 = battle.trunc(step1 as f64 * base_power * attack, None);
            let step3 = battle.trunc(step2 as f64 / defense, None);
            let step4 = battle.trunc(step3 as f64 / 50.0, None);
            step4 + 2
        };

        // Damage is 16-bit context in self-hit confusion damage
        // let damage = tr(baseDamage, 16);
        let damage_16bit = battle.trunc(base_damage as f64, Some(16)) as i32;

        // damage = this.battle.randomizer(damage);
        let damage_randomized = battle.randomizer(damage_16bit);

        // return Math.max(1, damage);
        damage_randomized.max(1)
    };

    // this.damage(damage, pokemon, pokemon, activeMove as ActiveMove);
    battle.damage(damage, Some(pokemon_pos), Some(pokemon_pos), None, false);

    // return false;
    EventResult::Boolean(false)
}

