//! Final Gambit Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// damageCallback(pokemon) {
///     const damage = pokemon.hp;
///     pokemon.faint();
///     return damage;
/// }
pub fn damage_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
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
    Pokemon::faint(battle, pokemon, None, None);

    // return damage;
    EventResult::Number(damage)
}
