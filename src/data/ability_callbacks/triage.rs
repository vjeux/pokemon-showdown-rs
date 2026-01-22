//! Triage Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyPriority(priority, pokemon, target, move) {
///     if (move?.flags['heal']) return priority + 3;
/// }
pub fn on_modify_priority(battle: &mut Battle, priority: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(active_move) = battle.dex.get_active_move(move_id) {
        if active_move.flags.heal {
            return EventResult::Number(priority + 3);
        }
    }
    EventResult::Continue
}

