//! Bulletproof Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(pokemon, target, move) {
///     if (move.flags['bullet']) {
///         this.add('-immune', pokemon, '[from] ability: Bulletproof');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

