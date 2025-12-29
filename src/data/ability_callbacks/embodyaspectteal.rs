//! Embody Aspect (Teal) Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.baseSpecies.name === 'Ogerpon-Teal-Tera' && pokemon.terastallized &&
///         !this.effectState.embodied) {
///         this.effectState.embodied = true;
///         this.boost({ spe: 1 }, pokemon);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

