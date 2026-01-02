//! Simple Beam Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;
use crate::Pokemon;

/// onTryHit(target) {
///     if (target.getAbility().flags['cantsuppress'] || target.ability === 'simple' || target.ability === 'truant') {
///         return false;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    // onTryHit(target) {
    //     if (target.getAbility().flags['cantsuppress'] || target.ability === 'simple' || target.ability === 'truant') {
    //         return false;
    //     }
    // }
    let target = target_pos;

    // if (target.getAbility().flags['cantsuppress'] || target.ability === 'simple' || target.ability === 'truant') {
    let (cantsuppress, is_simple, is_truant) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let ability_data = battle.dex.abilities().get_by_id(&target_pokemon.ability);
        let cantsuppress = if let Some(ability) = ability_data {
            ability.flags.get("cantsuppress").copied().unwrap_or(0) != 0
        } else {
            false
        };

        let is_simple = target_pokemon.ability == ID::from("simple");
        let is_truant = target_pokemon.ability == ID::from("truant");

        (cantsuppress, is_simple, is_truant)
    };

    if cantsuppress || is_simple || is_truant {
        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     const oldAbility = target.setAbility('simple');
///     if (!oldAbility) return oldAbility as false | null;
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // onHit(target, source) {
    //     const oldAbility = target.setAbility('simple');
    //     if (!oldAbility) return oldAbility as false | null;
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const oldAbility = target.setAbility('simple');
    let old_ability = Pokemon::set_ability(battle, target, ID::from("simple"), None, None, false, false);

    // if (!oldAbility) return oldAbility as false | null;
    if old_ability == ID::from("") {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
