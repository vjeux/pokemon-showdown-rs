//! Prankster Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyPriority(priority, pokemon, target, move) {
///     if (move?.category === 'Status') {
///         move.pranksterBoosted = true;
///         return priority + 1;
///     }
/// }
pub fn on_modify_priority(battle: &mut Battle, priority: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (move?.category === 'Status') {
    //     move.pranksterBoosted = true;
    //     return priority + 1;
    // }

    if let Some(ref mut active_move) = battle.active_move {
        if active_move.category == "Status" {
            // move.pranksterBoosted = true;
            active_move.prankster_boosted = true;
            // return priority + 1;
            return EventResult::Number(priority + 1);
        }
    }

    EventResult::Continue
}

