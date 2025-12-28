//! Revenge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     const damagedByTarget = pokemon.attackedBy.some(
///         p => p.source === target && p.damage > 0 && p.thisTurn
///     );
///     if (damagedByTarget) {
///         this.debug(`BP doubled for getting hit by ${target}`);
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const damagedByTarget = pokemon.attackedBy.some(
    //     p => p.source === target && p.damage > 0 && p.thisTurn
    // );
    let damaged_by_target = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.attacked_by.iter().any(|p| {
            p.source == Some(target) && p.damage > 0 && p.this_turn
        })
    };

    // if (damagedByTarget) {
    if damaged_by_target {
        // this.debug(`BP doubled for getting hit by ${target}`);
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };
        battle.debug(&format!("BP doubled for getting hit by {}", target_arg));

        // return move.basePower * 2;
        let base_power = {
            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            active_move.base_power
        };
        return EventResult::Number(base_power * 2);
    }

    // return move.basePower;
    let base_power = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.base_power
    };

    EventResult::Number(base_power)
}

