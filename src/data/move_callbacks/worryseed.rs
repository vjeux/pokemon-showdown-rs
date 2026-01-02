//! Worry Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onTryImmunity(target) {
///     // Truant and Insomnia have special treatment; they fail before
///     // checking accuracy and will double Stomping Tantrum's BP
///     if (target.ability === 'truant' || target.ability === 'insomnia') {
///         return false;
///     }
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.ability === 'truant' || target.ability === 'insomnia') {
    //     return false;
    // }
    let ability_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.ability.clone()
    };

    if ability_id == ID::from("truant") || ability_id == ID::from("insomnia") {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

/// onTryHit(target) {
///     if (target.getAbility().flags['cantsuppress']) {
///         return false;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let target = target_pos;

    // if (target.getAbility().flags['cantsuppress']) {
    //     return false;
    // }
    let cant_suppress = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let target_ability = target_pokemon.get_ability();
        let ability_data = battle.dex.abilities().get_by_id(&target_ability);

        match ability_data {
            Some(ability_data) => ability_data.flags.contains_key("cantsuppress"),
            None => false,
        }
    };

    if cant_suppress {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     const oldAbility = target.setAbility('insomnia');
///     if (!oldAbility) return oldAbility as false | null;
///     if (target.status === 'slp') target.cureStatus();
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const oldAbility = target.setAbility('insomnia');
    // if (!oldAbility) return oldAbility as false | null;
    let old_ability = Pokemon::set_ability(battle, target, ID::from("insomnia"), None, None, false, false);

    if old_ability.is_empty() {
        return EventResult::Stop; // return null/false
    }

    // if (target.status === 'slp') target.cureStatus();
    let has_sleep = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        target_pokemon.status == ID::from("slp")
    };

    if has_sleep {
        Pokemon::cure_status(battle, target, false);
    }

    EventResult::Continue
}
