//! Veevee Volley Move
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
    // Get the pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // const bp = Math.floor((pokemon.happiness * 10) / 25) || 1;
    let bp = ((pokemon.happiness as i32 * 10) / 25).max(1);

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    EventResult::Number(bp)
}
