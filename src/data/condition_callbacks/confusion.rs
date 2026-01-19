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
    source_effect_id: Option<&str>,
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
    // JavaScript: sourceEffect is passed as a parameter, not battle.effect (which is the current effect being processed)
    let is_lockedmove = source_effect_id.map(|e| e == "lockedmove").unwrap_or(false);

    // else if (sourceEffect?.effectType === 'Ability')
    // Check if the source effect is an ability by looking it up in the dex
    let is_ability = source_effect_id
        .and_then(|eff_id| battle.dex.abilities().get(eff_id))
        .is_some();

    if is_lockedmove {
        // this.add('-start', target, 'confusion', '[fatigue]');
        battle.add("-start", &[Arg::String(target_ident), Arg::Str("confusion"), Arg::Str("[fatigue]")]);
    } else if is_ability {
        // this.add('-start', target, 'confusion', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
        let ability_name = source_effect_id
            .and_then(|eff_id| battle.dex.abilities().get(eff_id))
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
    // JavaScript: sourceEffect is the parameter passed to onStart, NOT battle.effect
    let min = if source_effect_id.map(|e| e == "axekick").unwrap_or(false) {
        3
    } else {
        2
    };

    // this.effectState.time = this.random(min, 6);
    let time = battle.random_with_range(min, 6);

    // Set time in effect state using with_effect_state
    // In JavaScript: this.effectState.time = value
    battle.with_effect_state(|state| {
        state.time = Some(time);
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
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // pokemon.volatiles['confusion'].time--;
    // JavaScript: this.effectState.time-- (decrement time)
    battle.with_effect_state(|state| {
        if let Some(time) = state.time {
            state.time = Some(time - 1);
        }
    });

    // if (!pokemon.volatiles['confusion'].time)
    // JavaScript: this.effectState.time == 0
    let time_is_zero = battle.with_effect_state_ref(|state| {
        state.time.map(|t| t == 0).unwrap_or(false)
    }).unwrap_or(false);

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
        // Note: Wonder Room swaps Def and SpD before calculating stats
        let (atk_stat, def_stat, atk_boost, def_boost, level) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Wonder Room swaps defenses before calculating anything else
            // JavaScript: if ('wonderroom' in this.battle.field.pseudoWeather) { stat = storedStats['spd'] }
            let def_stat = if battle.field.pseudo_weather.contains_key(&ID::from("wonderroom")) {
                pokemon.stored_stats.spd
            } else {
                pokemon.stored_stats.def
            };

            (
                pokemon.stored_stats.atk,
                def_stat,
                pokemon.boosts.atk,
                pokemon.boosts.def,
                pokemon.level as f64
            )
        };

        // const attack = pokemon.calculateStat('atk', pokemon.boosts['atk']);
        // JavaScript calculateStat:
        //   if (boost >= 0) stat = Math.floor(stat * (2 + boost) / 2);
        //   else stat = Math.floor(stat * 2 / (2 - boost));
        let attack = {
            let clamped_boost = atk_boost.clamp(-6, 6);
            if clamped_boost >= 0 {
                // Math.floor(stat * (2 + boost) / 2)
                (atk_stat as f64 * (2.0 + clamped_boost as f64) / 2.0).floor()
            } else {
                // Math.floor(stat * 2 / (2 - boost))
                (atk_stat as f64 * 2.0 / (2.0 - clamped_boost as f64)).floor()
            }
        };

        // const defense = pokemon.calculateStat('def', pokemon.boosts['def']);
        // JavaScript calculateStat:
        //   if (boost >= 0) stat = Math.floor(stat * (2 + boost) / 2);
        //   else stat = Math.floor(stat * 2 / (2 - boost));
        let defense = {
            let clamped_boost = def_boost.clamp(-6, 6);
            if clamped_boost >= 0 {
                // Math.floor(stat * (2 + boost) / 2)
                (def_stat as f64 * (2.0 + clamped_boost as f64) / 2.0).floor()
            } else {
                // Math.floor(stat * 2 / (2 - boost))
                (def_stat as f64 * 2.0 / (2.0 - clamped_boost as f64)).floor()
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

    // const activeMove = { id: this.toID('confused'), effectType: 'Move', type: '???' };
    // this.damage(damage, pokemon, pokemon, activeMove as ActiveMove);
    // JavaScript creates a fake "move" effect so Disguise.onDamage recognizes it as a move
    let confused_effect = crate::battle::Effect::move_("confused");
    battle.damage(damage, Some(pokemon_pos), Some(pokemon_pos), Some(&confused_effect), false);

    // return false;
    EventResult::Boolean(false)
}

