//! Water Shuriken Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (pokemon.species_id.as_str() === 'Greninja-Ash' && pokemon.hasAbility('battlebond') &&
///         !pokemon.transformed) {
///         return move.basePower + 5;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(_battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

