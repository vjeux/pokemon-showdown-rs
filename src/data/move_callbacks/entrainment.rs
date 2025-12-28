//! Entrainment Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     if (target === source || target.volatiles['dynamax']) return false;
///     if (
///         target.ability === source.ability ||
///         target.getAbility().flags['cantsuppress'] || target.ability === 'truant' ||
///         source.getAbility().flags['noentrain']
///     ) {
///         return false;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;
    let source = source_pos;

    // if (target === source || target.volatiles['dynamax']) return false;
    if target == source {
        return EventResult::Boolean(false);
    }

    let has_dynamax = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("dynamax"))
    };

    if has_dynamax {
        return EventResult::Boolean(false);
    }

    // if (
    //     target.ability === source.ability ||
    //     target.getAbility().flags['cantsuppress'] || target.ability === 'truant' ||
    //     source.getAbility().flags['noentrain']
    // ) {
    //     return false;
    // }
    let (target_ability, source_ability) = {
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

    if target_ability == source_ability {
        return EventResult::Boolean(false);
    }

    let target_ability_data = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ability_id = &target_pokemon.ability;
        match battle.dex.get_ability(ability_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        }
    };

    if target_ability_data.flags.contains_key("cantsuppress") {
        return EventResult::Boolean(false);
    }

    if target_ability == ID::from("truant") {
        return EventResult::Boolean(false);
    }

    let source_ability_data = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ability_id = &source_pokemon.ability;
        match battle.dex.get_ability(ability_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        }
    };

    if source_ability_data.flags.contains_key("noentrain") {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     const oldAbility = target.setAbility(source.ability, source);
///     if (!oldAbility) return oldAbility as false | null;
///     if (!target.isAlly(source)) target.volatileStaleness = 'external';
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const oldAbility = target.setAbility(source.ability, source);
    let source_ability = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.ability.clone()
    };

    let old_ability = {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.set_ability(source_ability)
    };

    // if (!oldAbility) return oldAbility as false | null;
    if old_ability.as_str().is_empty() {
        // oldAbility is empty (falsy in JS), return null
        return EventResult::Stop;
    }

    // if (!target.isAlly(source)) target.volatileStaleness = 'external';
    let is_ally = battle.is_ally(target, source);
    if !is_ally {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatile_staleness = Some("external".to_string());
    }

    EventResult::Continue
}

