//! Role Play Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onTryHit(target, source) {
///     if (target.ability === source.ability) return false;
///     if (target.getAbility().flags['failroleplay'] || source.getAbility().flags['cantsuppress']) return false;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    // JavaScript: onTryHit(target, source) - target comes first, source second
    let target = target_pos;
    let source = source_pos;

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
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    let target = target_pos;
    let source = match source_pos {
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

    let old_ability = Pokemon::set_ability(battle, source, target_ability, None, None, false, false);

    // if (!oldAbility) return oldAbility as false | null;
    if old_ability == ID::from("") {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
