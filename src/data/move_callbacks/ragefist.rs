//! Rage Fist Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon) {
///     return Math.min(350, 50 + 50 * pokemon.timesAttacked);
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // return Math.min(350, 50 + 50 * pokemon.timesAttacked);
    let power = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_power = 50 + 50 * pokemon_pokemon.times_attacked;
        std::cmp::min(350, base_power)
    };

    EventResult::Number(power)
}
