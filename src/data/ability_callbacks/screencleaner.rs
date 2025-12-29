//! Screen Cleaner Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     let activated = false;
///     for (const sideCondition of ['reflect', 'lightscreen', 'auroraveil']) {
///         for (const side of [pokemon.side, ...pokemon.side.foeSidesWithConditions()]) {
///             if (side.getSideCondition(sideCondition)) {
///                 if (!activated) {
///                     this.add('-activate', pokemon, 'ability: Screen Cleaner');
///                     activated = true;
///                 }
///                 side.removeSideCondition(sideCondition);
///             }
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

