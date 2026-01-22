//! Prankster Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyPriority(priority, pokemon, target, move) {
///     if (move?.category === 'Status') {
///         move.pranksterBoosted = true;
///         return priority + 1;
///     }
/// }
pub fn on_modify_priority(battle: &mut Battle, priority: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move?.category === 'Status') {
    //     move.pranksterBoosted = true;
    //     return priority + 1;
    // }

    if let Some(ref mut active_move) = battle.active_move {
        if active_move.category == "Status" {
            // move.pranksterBoosted = true;
            active_move.prankster_boosted = true;
            // Also update the move's priority so it's available for later checks (like Dazzling)
            active_move.priority = (priority + 1) as i8;
            // return priority + 1;
            return EventResult::Number(priority + 1);
        }
    }

    EventResult::Continue
}

