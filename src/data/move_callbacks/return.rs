//! Return Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon) {
///     return Math.floor((pokemon.happiness * 10) / 25) || 1;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // return Math.floor((pokemon.happiness * 10) / 25) || 1;
    let happiness = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.happiness as u32
    };

    let base_power = ((happiness * 10) / 25).max(1);

    EventResult::Number(base_power as i32)
}
