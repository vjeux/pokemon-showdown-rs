//! Schooling Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 || pokemon.transformed) return;
///     if (pokemon.hp > pokemon.maxhp / 4) {
///         if (pokemon.species.id === 'wishiwashi') {
///             pokemon.formeChange('Wishiwashi-School');
///         }
///     } else {
///         if (pokemon.species.id === 'wishiwashischool') {
///             pokemon.formeChange('Wishiwashi');
///         }
///     }
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (
///         pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 ||
///         pokemon.transformed || !pokemon.hp
///     ) return;
///     if (pokemon.hp > pokemon.maxhp / 4) {
///         if (pokemon.species.id === 'wishiwashi') {
///             pokemon.formeChange('Wishiwashi-School');
///         }
///     } else {
///         if (pokemon.species.id === 'wishiwashischool') {
///             pokemon.formeChange('Wishiwashi');
///         }
///     }
/// }
pub fn on_residual(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

