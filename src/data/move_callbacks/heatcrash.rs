//! Heat Crash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     const targetWeight = target.getWeight();
///     const pokemonWeight = pokemon.getWeight();
///     let bp;
///     if (pokemonWeight >= targetWeight * 5) {
///         bp = 120;
///     } else if (pokemonWeight >= targetWeight * 4) {
///         bp = 100;
///     } else if (pokemonWeight >= targetWeight * 3) {
///         bp = 80;
///     } else if (pokemonWeight >= targetWeight * 2) {
///         bp = 60;
///     } else {
///         bp = 40;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(_battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryHit(target, pokemon, move) {
///     if (target.volatiles['dynamax']) {
///         this.add('-fail', pokemon, 'Dynamax');
///         this.attrLastMove('[still]');
///         return null;
///     }
/// }
pub fn on_try_hit(_battle: &mut Battle, _source_pos: (usize, usize), _target_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

