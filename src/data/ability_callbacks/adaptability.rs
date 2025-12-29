//! Adaptability Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySTAB(stab, source, target, move) {
///     if (move.forceSTAB || source.hasType(move.type)) {
///         if (stab === 2) {
///             return 2.25;
///         }
///         return 2;
///     }
/// }
pub fn on_modify_s_t_a_b(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

