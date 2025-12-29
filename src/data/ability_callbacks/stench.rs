//! Stench Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.category !== "Status") {
///         this.debug('Adding Stench flinch');
///         if (!move.secondaries) move.secondaries = [];
///         for (const secondary of move.secondaries) {
///             if (secondary.volatileStatus === 'flinch') return;
///         }
///         move.secondaries.push({
///             chance: 10,
///             volatileStatus: 'flinch',
///         });
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

