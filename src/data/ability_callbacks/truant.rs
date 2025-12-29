//! Truant Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     pokemon.removeVolatile('truant');
///     if (pokemon.activeTurns && (pokemon.moveThisTurnResult !== undefined || !this.queue.willMove(pokemon))) {
///         pokemon.addVolatile('truant');
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onBeforeMove(pokemon) {
///     if (pokemon.removeVolatile('truant')) {
///         this.add('cant', pokemon, 'ability: Truant');
///         return false;
///     }
///     pokemon.addVolatile('truant');
/// }
pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

