//! Teraform Zero Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterTerastallization(pokemon) {
///     if (pokemon.baseSpecies.name !== 'Terapagos-Stellar') return;
///     if (this.field.weather || this.field.terrain) {
///         this.add('-ability', pokemon, 'Teraform Zero');
///         this.field.clearWeather();
///         this.field.clearTerrain();
///     }
/// }
pub fn on_after_terastallization(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

