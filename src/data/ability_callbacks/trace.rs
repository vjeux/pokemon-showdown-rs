//! Trace Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;
use serde_json::Value;

/// onStart(pokemon) {
///     this.effectState.seek = true;
///     // n.b. only affects Hackmons
///     // interaction with No Ability is complicated: https://www.smogon.com/forums/threads/pokemon-sun-moon-battle-mechanics-research.3586701/page-76#post-7790209
///     if (pokemon.adjacentFoes().some(foeActive => foeActive.ability === 'noability')) {
///         this.effectState.seek = false;
///     }
///     // interaction with Ability Shield is similar to No Ability
///     if (pokemon.hasItem('Ability Shield')) {
///         this.add('-block', pokemon, 'item: Ability Shield');
///         this.effectState.seek = false;
///     }
///     if (this.effectState.seek) {
///         this.singleEvent('Update', this.effect, this.effectState, pokemon);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // this.effectState.seek = true;
    battle.effect_state.data.insert("seek".to_string(), Value::Bool(true));

    // if (pokemon.adjacentFoes().some(foeActive => foeActive.ability === 'noability'))
    let has_no_ability_foe = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let adjacent_foes = pokemon.adjacent_foes(battle);

        adjacent_foes.iter().any(|&foe_pos| {
            let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                Some(p) => p,
                None => return false,
            };
            foe.ability.as_str() == "noability"
        })
    };

    if has_no_ability_foe {
        // this.effectState.seek = false;
        battle.effect_state.data.insert("seek".to_string(), Value::Bool(false));
    }

    // if (pokemon.hasItem('Ability Shield'))
    let has_ability_shield = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_item(battle, &["Ability Shield"])
    };

    if has_ability_shield {
        // this.add('-block', pokemon, 'item: Ability Shield');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-block", &[
            pokemon_slot.as_str().into(),
            "item: Ability Shield".into(),
        ]);

        // this.effectState.seek = false;
        battle.effect_state.data.insert("seek".to_string(), Value::Bool(false));
    }

    // if (this.effectState.seek)
    let should_seek = battle.effect_state.data.get("seek")
        .and_then(|v: &Value| v.as_bool())
        .unwrap_or(false);

    if should_seek {
        // this.singleEvent('Update', this.effect, this.effectState, pokemon);
        // Get the current effect (Trace ability)
        let ability_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.ability.clone()
        };

        battle.single_event("Update", &crate::battle::Effect::ability(ability_id), None, Some(pokemon_pos), None, None, None);
    }

    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (!this.effectState.seek) return;
///
///     const possibleTargets = pokemon.adjacentFoes().filter(
///         target => !target.getAbility().flags['notrace'] && target.ability !== 'noability'
///     );
///     if (!possibleTargets.length) return;
///
///     const target = this.sample(possibleTargets);
///     const ability = target.getAbility();
///     pokemon.setAbility(ability, target);
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.effectState.seek) return;
    let should_seek = battle.effect_state.data.get("seek")
        .and_then(|v: &Value| v.as_bool())
        .unwrap_or(false);

    if !should_seek {
        return EventResult::Continue;
    }

    // const possibleTargets = pokemon.adjacentFoes().filter(...)
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let adjacent_foes = pokemon.adjacent_foes(battle);

    // Filter: target => !target.getAbility().flags['notrace'] && target.ability !== 'noability'
    let mut possible_targets: Vec<(usize, usize)> = Vec::new();

    for &foe_pos in &adjacent_foes {
        let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
            Some(p) => p,
            None => continue,
        };

        // Check: target.ability !== 'noability'
        if foe.ability.as_str() == "noability" {
            continue;
        }

        // Check: !target.getAbility().flags['notrace']
        let has_notrace_flag = battle.dex.abilities()
            .get_by_id(&foe.ability)
            .and_then(|ability_data| ability_data.flags.get("notrace"))
            .map(|v| *v != 0)
            .unwrap_or(false);

        if has_notrace_flag {
            continue;
        }

        possible_targets.push(foe_pos);
    }

    // if (!possibleTargets.length) return;
    if possible_targets.is_empty() {
        return EventResult::Continue;
    }

    // const target = this.sample(possibleTargets);
    println!("[TRACE] About to sample from {} possible targets, PRNG={}", possible_targets.len(), battle.prng.call_count);
    let target_pos = match battle.sample(&possible_targets) {
        Some(&pos) => pos,
        None => return EventResult::Continue,
    };
    println!("[TRACE] After sample, PRNG={}", battle.prng.call_count);

    // const ability = target.getAbility();
    let target_ability = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_ability().clone()
    };

    // pokemon.setAbility(ability, target);
    Pokemon::set_ability(
        battle,
        pokemon_pos,
        target_ability,
        Some(target_pos),
        None,
        false,
        false,
    );

    EventResult::Continue
}

