//! Mycelium Might Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFractionalPriority(priority, pokemon, target, move) {
///     if (move.category === 'Status') {
///         return -0.1;
///     }
/// }
pub fn on_fractional_priority(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.category === 'Status') {
///         move.ignoreAbility = true;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

