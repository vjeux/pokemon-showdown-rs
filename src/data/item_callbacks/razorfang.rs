//! Razor Fang Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.category !== "Status") {
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
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.category !== "Status") {
    //     if (!move.secondaries) move.secondaries = [];
    //     for (const secondary of move.secondaries) {
    //         if (secondary.volatileStatus === 'flinch') return;
    //     }
    //     move.secondaries.push({
    //         chance: 10,
    //         volatileStatus: 'flinch',
    //     });
    // }
    // TODO: Need move.category, move.secondaries infrastructure to add flinch secondary effect
    // Should add 10% flinch chance to non-status moves if not already present
    EventResult::Continue
}
