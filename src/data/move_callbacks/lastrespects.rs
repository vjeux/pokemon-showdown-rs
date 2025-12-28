//! Last Respects Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     return 50 + 50 * pokemon.side.totalFainted;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // return 50 + 50 * pokemon.side.totalFainted;
    let total_fainted = {
        let side = match battle.sides.get(pokemon.0) {
            Some(s) => s,
            None => return EventResult::Continue,
        };
        side.total_fainted
    };

    let base_power = 50 + 50 * total_fainted as i32;
    EventResult::Number(base_power)
}

