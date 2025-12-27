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
    let pokemon = pokemon_pos;

    // if (pokemon.side.faintedLastTurn) {
    //     this.debug('Boosted for a faint last turn');
    //     return this.chainModify(2);
    // }
    let fainted_last_turn = {
        let side = match battle.sides.get(pokemon.0) {
            Some(s) => s,
            None => return EventResult::Continue,
        };
        side.fainted_last_turn
    };

    if fainted_last_turn {
        battle.debug("Boosted for a faint last turn");
        return EventResult::ChainModify(2);
    }

    EventResult::Continue
}

