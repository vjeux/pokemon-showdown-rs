//! Supersweet Syrup Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.syrupTriggered) return;
///     pokemon.syrupTriggered = true;
///     this.add('-ability', pokemon, 'Supersweet Syrup');
///     for (const target of pokemon.adjacentFoes()) {
///         if (target.volatiles['substitute']) {
///             this.add('-immune', target);
///         } else {
///             this.boost({ evasion: -1 }, target, pokemon, null, true);
///         }
///     }
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

