//! Infernal Parade Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (target.status || target.hasAbility('comatose')) return move.basePower * 2;
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.status || target.hasAbility('comatose')) return move.basePower * 2;
    let has_status = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.status != ID::from("")
    };

    let has_comatose = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_ability(&["comatose"])
    };

    if has_status || has_comatose {
        let base_power = battle
            .active_move
            .as_ref()
            .map(|m| m.base_power)
            .unwrap_or(0);
        return EventResult::Number(base_power * 2);
    }

    // return move.basePower;
    let base_power = battle
        .active_move
        .as_ref()
        .map(|m| m.base_power)
        .unwrap_or(0);
    EventResult::Number(base_power)
}
