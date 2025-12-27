//! Reversal Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon) {
///     const ratio = Math.max(Math.floor(pokemon.hp * 48 / pokemon.maxhp), 1);
///     let bp;
///     if (ratio < 2) {
///         bp = 200;
///     } else if (ratio < 5) {
///         bp = 150;
///     } else if (ratio < 10) {
///         bp = 100;
///     } else if (ratio < 17) {
///         bp = 80;
///     } else if (ratio < 33) {
///         bp = 40;
///     } else {
///         bp = 20;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

