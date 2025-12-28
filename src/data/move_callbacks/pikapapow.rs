//! Pika Papow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon) {
///     const bp = Math.floor((pokemon.happiness * 10) / 25) || 1;
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // const bp = Math.floor((pokemon.happiness * 10) / 25) || 1;
    let bp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let happiness = pokemon_pokemon.happiness;
        let calculated_bp = (happiness * 10) / 25;
        if calculated_bp == 0 {
            1
        } else {
            calculated_bp
        }
    };

    // this.debug(`BP: ${bp}`);
    // (debug is typically not needed in Rust implementation)

    // return bp;
    EventResult::Number(bp as i32)
}
