//! Retaliate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon) {
///     if (pokemon.side.faintedLastTurn) {
///         this.debug('Boosted for a faint last turn');
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Check if a teammate fainted last turn
    let fainted_last_turn = if let Some(side) = battle.sides.get(pokemon_pos.0) {
        side.fainted_last_turn.is_some()
    } else {
        return EventResult::Continue;
    };

    if fainted_last_turn {
        battle.debug("Boosted for a faint last turn");
        return EventResult::Number(battle.chain_modify(2.0));
    }

    EventResult::Continue
}

