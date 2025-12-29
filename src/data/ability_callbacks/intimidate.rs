//! Intimidate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     let activated = false;
///     for (const target of pokemon.adjacentFoes()) {
///         if (!activated) {
///             this.add('-ability', pokemon, 'Intimidate', 'boost');
///             activated = true;
///         }
///         if (target.volatiles['substitute']) {
///             this.add('-immune', target);
///         } else {
///             this.boost({ atk: -1 }, target, pokemon, null, true);
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

