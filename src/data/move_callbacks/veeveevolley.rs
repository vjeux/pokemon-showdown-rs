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
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
