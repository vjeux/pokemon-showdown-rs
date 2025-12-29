//! Zero to Hero Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchOut(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Palafin') return;
///     if (pokemon.species.forme !== 'Hero') {
///         pokemon.formeChange('Palafin-Hero', this.effect, true);
///         pokemon.heroMessageDisplayed = false;
///     }
/// }
pub fn on_switch_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSwitchIn(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Palafin') return;
///     if (!pokemon.heroMessageDisplayed && pokemon.species.forme === 'Hero') {
///         this.add('-activate', pokemon, 'ability: Zero to Hero');
///         pokemon.heroMessageDisplayed = true;
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

