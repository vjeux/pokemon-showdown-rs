//! Facade Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon) {
///     if (pokemon.status && pokemon.status !== 'slp') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Double base power if user has a status condition that isn't sleep
    if !pokemon.status.is_empty() && pokemon.status.as_str() != "slp" {
        // chainModify(2) means multiply base power by 2
        EventResult::Number(base_power * 2)
    } else {
        EventResult::Continue
    }
}

