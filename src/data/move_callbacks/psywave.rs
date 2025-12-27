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
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Generate random number between 50 and 150 (inclusive)
    let random_multiplier = battle.random(50, 151);
    let damage = (random_multiplier * pokemon.level) / 100;

    EventResult::Number(damage)
}

