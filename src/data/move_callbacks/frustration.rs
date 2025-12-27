//! Frustration Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon) {
///     return Math.floor(((255 - pokemon.happiness) * 10) / 25) || 1;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // return Math.floor(((255 - pokemon.happiness) * 10) / 25) || 1;
    let happiness = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.happiness
    };

    let bp = ((255 - happiness) * 10) / 25;

    // || 1 means if bp is 0, return 1
    let bp = if bp == 0 { 1 } else { bp };

    EventResult::Int(bp)
}

