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
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.borrow().id.clone(),
        None => return EventResult::Continue,
    };

    // Get the move data
    let move_data = match battle.dex.moves().get_by_id(&move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // const damagedByTarget = pokemon.attackedBy.some(
    //     p => p.source === target && p.damage > 0 && p.thisTurn
    // );
    let damaged_by_target = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        pokemon.attacked_by.iter().any(|attacker| {
            attacker.source == target && attacker.damage > 0 && attacker.this_turn
        })
    };

    // if (damagedByTarget) {
    //     this.debug(`BP doubled for getting hit by ${target}`);
    //     return move.basePower * 2;
    // }
    if damaged_by_target {
        debug_elog!("[AVALANCHE] BP doubled for getting hit by {:?}", target);
        return EventResult::Number(move_data.base_power * 2);
    }

    // return move.basePower;
    EventResult::Number(move_data.base_power)
}
