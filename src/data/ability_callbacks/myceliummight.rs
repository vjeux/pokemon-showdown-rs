//! Mycelium Might Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onFractionalPriority(priority, pokemon, target, move) {
///     if (move.category === 'Status') {
///         return -0.1;
///     }
/// }
pub fn on_fractional_priority(battle: &mut Battle, _priority: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move.category === 'Status')
    // The move is passed as a parameter, not read from battle.active_move
    // During action resolution (FractionalPriority), active_move is not yet set
    // JavaScript passes move as 4th arg: runEvent('FractionalPriority', pokemon, null, move, 0)
    let is_status = if let Some(mv) = active_move {
        mv.category == "Status"
    } else if let Some(ref mv) = battle.active_move {
        // Fallback to battle.active_move if available
        mv.category == "Status"
    } else if let Some(ref event) = battle.event {
        // Try to get move category from event's source_effect
        if let Some(ref effect) = event.effect {
            if let Some(move_data) = battle.dex.moves().get(effect.id.as_str()) {
                move_data.category == "Status"
            } else {
                return EventResult::Continue;
            }
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    if is_status {
        // return -0.1;
        return EventResult::Float(-0.1);
    }

    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.category === 'Status') {
///         move.ignoreAbility = true;
///     }
/// }
pub fn on_modify_move(_battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.category === 'Status')
    if let Some(active_move) = active_move {
        if active_move.category == "Status" {
            // move.ignoreAbility = true;
            active_move.ignore_ability = true;
        }
    }

    EventResult::Continue
}

