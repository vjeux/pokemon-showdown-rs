//! Serene Grace Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.secondaries) {
///         this.debug('doubling secondary chance');
///         for (const secondary of move.secondaries) {
///             if (secondary.chance) secondary.chance *= 2;
///         }
///     }
///     if (move.self?.chance) move.self.chance *= 2;
/// }
pub fn on_modify_move(_battle: &mut Battle, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

