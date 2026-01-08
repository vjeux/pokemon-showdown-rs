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
pub fn on_fractional_priority(battle: &mut Battle, _priority: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move.category === 'Status')
    let is_status = if let Some(ref active_move) = battle.active_move {
        active_move.category == "Status"
    } else {
        return EventResult::Continue;
    };

    if is_status {
        // return -0.1;
        return EventResult::Number(-1); // Fractional priority is multiplied by 10 internally, so -0.1 = -1
    }

    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.category === 'Status') {
///         move.ignoreAbility = true;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.category === 'Status')
    if let Some(active_move) = active_move {
        if active_move.category == "Status" {
            // move.ignoreAbility = true;
            active_move.ignore_ability = true;
        }
    }

    EventResult::Continue
}

