//! Role Play Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     if (target.ability === source.ability) return false;
///     if (target.getAbility().flags['failroleplay'] || source.getAbility().flags['cantsuppress']) return false;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let source = source_pos;
    let target = target_pos;

    // if (target.ability === source.ability) return false;
    let (target_ability, source_ability) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            target_pokemon.ability.clone(),
            source_pokemon.ability.clone(),
        )
    };

    if target_ability == source_ability {
        return EventResult::Boolean(false);
    }

    // if (target.getAbility().flags['failroleplay'] || source.getAbility().flags['cantsuppress']) return false;
    let target_ability_data = match battle.dex.abilities().get_by_id(&target_ability) {
        Some(a) => a,
        None => return EventResult::Continue,
    };
    let source_ability_data = match battle.dex.abilities().get_by_id(&source_ability) {
        Some(a) => a,
        None => return EventResult::Continue,
    };

    if target_ability_data
        .flags
        .get("failroleplay")
        .copied()
        .unwrap_or(0)
        != 0
        || source_ability_data
            .flags
            .get("cantsuppress")
            .copied()
            .unwrap_or(0)
            != 0
    {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     const oldAbility = source.setAbility(target.ability, target);
///     if (!oldAbility) return oldAbility as false | null;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const oldAbility = source.setAbility(target.ability, target);
    let target_ability = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.ability.clone()
    };

    let old_ability = {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.set_ability(target_ability)
    };

    // if (!oldAbility) return oldAbility as false | null;
    if old_ability == ID::from("") {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
