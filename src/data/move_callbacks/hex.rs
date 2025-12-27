//! Hex Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (target.status || target.hasAbility('comatose')) {
///         this.debug('BP doubled from status condition');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.status || target.hasAbility('comatose')) {
    let has_status = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.status.is_some()
    };

    let has_comatose = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_ability(&ID::from("comatose"))
    };

    if has_status || has_comatose {
        // this.debug('BP doubled from status condition');
        battle.debug("BP doubled from status condition");

        // return move.basePower * 2;
        let base_power = battle.active_move.as_ref().map(|m| m.base_power).unwrap_or(0);
        return EventResult::Int(base_power * 2);
    }

    // return move.basePower;
    let base_power = battle.active_move.as_ref().map(|m| m.base_power).unwrap_or(0);
    EventResult::Int(base_power)
}

