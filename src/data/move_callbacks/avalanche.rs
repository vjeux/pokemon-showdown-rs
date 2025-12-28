//! Avalanche Move
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
    // Get the pokemon (user of the move)
    let _pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the target
    let _target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => &active_move.id,
        None => return EventResult::Continue,
    };

    // Get the move data
    let move_data = match battle.dex.get_move_by_id(move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // TODO: Infrastructure gap - Pokemon.attackedBy field not yet implemented
    // This should check: pokemon.attackedBy.some(p => p.source === target && p.damage > 0 && p.thisTurn)
    // For now, return base power without modification
    // const damagedByTarget = pokemon.attackedBy.some(
    //     p => p.source === target && p.damage > 0 && p.thisTurn
    // );
    // if (damagedByTarget) {
    //     this.debug(`BP doubled for getting hit by ${target}`);
    //     return move.basePower * 2;
    // }

    // return move.basePower;
    EventResult::Number(move_data.base_power)
}

