//! Skill Swap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     const targetAbility = target.getAbility();
///     const sourceAbility = source.getAbility();
///     if (sourceAbility.flags['failskillswap'] || targetAbility.flags['failskillswap'] || target.volatiles['dynamax']) {
///         return false;
///     }
///     const sourceCanBeSet = this.runEvent('SetAbility', source, source, this.effect, targetAbility);
///     if (!sourceCanBeSet) return sourceCanBeSet;
///     const targetCanBeSet = this.runEvent('SetAbility', target, source, this.effect, sourceAbility);
///     if (!targetCanBeSet) return targetCanBeSet;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // onTryHit(target, source) {
    //     const targetAbility = target.getAbility();
    //     const sourceAbility = source.getAbility();
    //     if (sourceAbility.flags['failskillswap'] || targetAbility.flags['failskillswap'] || target.volatiles['dynamax']) {
    //         return false;
    //     }
    //     const sourceCanBeSet = this.runEvent('SetAbility', source, source, this.effect, targetAbility);
    //     if (!sourceCanBeSet) return sourceCanBeSet;
    //     const targetCanBeSet = this.runEvent('SetAbility', target, source, this.effect, sourceAbility);
    //     if (!targetCanBeSet) return targetCanBeSet;
    // }
    let target = target_pos;
    let source = source_pos;

    // const targetAbility = target.getAbility();
    // const sourceAbility = source.getAbility();
    let (target_ability_id, source_ability_id) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target_pokemon.ability.clone(), source_pokemon.ability.clone())
    };

    // if (sourceAbility.flags['failskillswap'] || targetAbility.flags['failskillswap'] || target.volatiles['dynamax']) {
    //     return false;
    // }
    let source_fails = {
        let source_ability = battle.dex.get_ability_by_id(&source_ability_id);
        if let Some(ref ability) = source_ability {
            ability.flags.get("failskillswap").copied().unwrap_or(0) != 0
        } else {
            false
        }
    };

    let target_fails = {
        let target_ability = battle.dex.get_ability_by_id(&target_ability_id);
        if let Some(ref ability) = target_ability {
            ability.flags.get("failskillswap").copied().unwrap_or(0) != 0
        } else {
            false
        }
    };

    let has_dynamax = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("dynamax"))
    };

    if source_fails || target_fails || has_dynamax {
        return EventResult::Boolean(false);
    }

    // const sourceCanBeSet = this.runEvent('SetAbility', source, source, this.effect, targetAbility);
    // if (!sourceCanBeSet) return sourceCanBeSet;
    let source_can_be_set = battle.run_event_for_ability("SetAbility", source, Some(source), Some(&target_ability_id));
    if !source_can_be_set {
        return EventResult::Boolean(false);
    }

    // const targetCanBeSet = this.runEvent('SetAbility', target, source, this.effect, sourceAbility);
    // if (!targetCanBeSet) return targetCanBeSet;
    let target_can_be_set = battle.run_event_for_ability("SetAbility", target, Some(source), Some(&source_ability_id));
    if !target_can_be_set {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit(target, source, move) {
///     const targetAbility = target.getAbility();
///     const sourceAbility = source.getAbility();
///     if (target.isAlly(source)) {
///         this.add('-activate', source, 'move: Skill Swap', '', '', `[of] ${target}`);
///     } else {
///         this.add('-activate', source, 'move: Skill Swap', targetAbility, sourceAbility, `[of] ${target}`);
///     }
///     this.singleEvent('End', sourceAbility, source.abilityState, source);
///     this.singleEvent('End', targetAbility, target.abilityState, target);
///     source.ability = targetAbility.id;
///     target.ability = sourceAbility.id;
///     source.abilityState = this.initEffectState({ id: this.toID(source.ability), target: source });
///     target.abilityState = this.initEffectState({ id: this.toID(target.ability), target });
///     source.volatileStaleness = undefined;
///     if (!target.isAlly(source)) target.volatileStaleness = 'external';
///     this.singleEvent('Start', targetAbility, source.abilityState, source);
///     this.singleEvent('Start', sourceAbility, target.abilityState, target);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // onHit(target, source, move) {
    //     const targetAbility = target.getAbility();
    //     const sourceAbility = source.getAbility();
    //     if (target.isAlly(source)) {
    //         this.add('-activate', source, 'move: Skill Swap', '', '', `[of] ${target}`);
    //     } else {
    //         this.add('-activate', source, 'move: Skill Swap', targetAbility, sourceAbility, `[of] ${target}`);
    //     }
    //     this.singleEvent('End', sourceAbility, source.abilityState, source);
    //     this.singleEvent('End', targetAbility, target.abilityState, target);
    //     source.ability = targetAbility.id;
    //     target.ability = sourceAbility.id;
    //     source.abilityState = this.initEffectState({ id: this.toID(source.ability), target: source });
    //     target.abilityState = this.initEffectState({ id: this.toID(target.ability), target });
    //     source.volatileStaleness = undefined;
    //     if (!target.isAlly(source)) target.volatileStaleness = 'external';
    //     this.singleEvent('Start', targetAbility, source.abilityState, source);
    //     this.singleEvent('Start', sourceAbility, target.abilityState, target);
    // }
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetAbility = target.getAbility();
    // const sourceAbility = source.getAbility();
    let (target_ability_id, source_ability_id) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target_pokemon.ability.clone(), source_pokemon.ability.clone())
    };

    // if (target.isAlly(source)) {
    //     this.add('-activate', source, 'move: Skill Swap', '', '', `[of] ${target}`);
    // } else {
    //     this.add('-activate', source, 'move: Skill Swap', targetAbility, sourceAbility, `[of] ${target}`);
    // }
    let is_ally = battle.is_ally(target, source);

    let source_arg = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(source_pokemon)
    };

    let target_arg = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(target_pokemon)
    };

    if is_ally {
        battle.add("-activate", &[
            source_arg.clone(),
            "move: Skill Swap".into(),
            "".into(),
            "".into(),
            format!("[of] {}", target_arg).into(),
        ]);
    } else {
        let target_ability_data = battle.dex.get_ability_by_id(&target_ability_id);
        let source_ability_data = battle.dex.get_ability_by_id(&source_ability_id);

        let target_ability_name = target_ability_data.map(|a| a.name.clone()).unwrap_or_default();
        let source_ability_name = source_ability_data.map(|a| a.name.clone()).unwrap_or_default();

        battle.add("-activate", &[
            source_arg.clone(),
            "move: Skill Swap".into(),
            target_ability_name.into(),
            source_ability_name.into(),
            format!("[of] {}", target_arg).into(),
        ]);
    }

    // this.singleEvent('End', sourceAbility, source.abilityState, source);
    // this.singleEvent('End', targetAbility, target.abilityState, target);
    battle.single_event("End", &source_ability_id, Some(source), None, None);
    battle.single_event("End", &target_ability_id, Some(target), None, None);

    // source.ability = targetAbility.id;
    // target.ability = sourceAbility.id;
    battle.swap_abilities(source, target);

    // source.abilityState = this.initEffectState({ id: this.toID(source.ability), target: source });
    // target.abilityState = this.initEffectState({ id: this.toID(target.ability), target });
    battle.init_ability_state(source);
    battle.init_ability_state(target);

    // source.volatileStaleness = undefined;
    // if (!target.isAlly(source)) target.volatileStaleness = 'external';
    battle.set_volatile_staleness(source, None);
    if !is_ally {
        battle.set_volatile_staleness(target, Some("external"));
    }

    // this.singleEvent('Start', targetAbility, source.abilityState, source);
    // this.singleEvent('Start', sourceAbility, target.abilityState, target);
    battle.single_event("Start", &target_ability_id, Some(source), None, None);
    battle.single_event("Start", &source_ability_id, Some(target), None, None);

    EventResult::Continue
}

