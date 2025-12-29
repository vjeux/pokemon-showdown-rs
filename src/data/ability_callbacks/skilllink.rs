//! Skill Link Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.multihit && Array.isArray(move.multihit) && move.multihit.length) {
///         move.multihit = move.multihit[1];
///     }
///     if (move.multiaccuracy) {
///         delete move.multiaccuracy;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

