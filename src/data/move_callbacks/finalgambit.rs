//! Final Gambit Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// damageCallback(pokemon) {
///     const damage = pokemon.hp;
///     pokemon.faint();
///     return damage;
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // const damage = pokemon.hp;
    let damage = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp
    };

    // pokemon.faint();
    let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon_pokemon.faint();

    // return damage;
    EventResult::Number(damage)
}

