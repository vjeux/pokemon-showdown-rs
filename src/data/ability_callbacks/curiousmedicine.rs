//! Curious Medicine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     for (const ally of pokemon.adjacentAllies()) {
///         ally.clearBoosts();
///         this.add('-clearboost', ally, '[from] ability: Curious Medicine', `[of] ${pokemon}`);
///     }
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

