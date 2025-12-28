//! Psywave Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// damageCallback(pokemon) {
///     return (this.random(50, 151) * pokemon.level) / 100;
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // return (this.random(50, 151) * pokemon.level) / 100;
    let rand = battle.prng.random_range(50, 151);

    let level = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.level
    };

    let damage = (rand * level as i32) / 100;

    EventResult::Number(damage)
}

