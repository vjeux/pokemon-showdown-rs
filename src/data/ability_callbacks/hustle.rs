//! Hustle Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAtk(atk) {
///     return this.modify(atk, 1.5);
/// }
pub fn on_modify_atk(battle: &mut Battle, atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let _move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    let _modified = battle.modify_f(atk, 1.5);
    EventResult::Continue
}

/// onSourceModifyAccuracy(accuracy, target, source, move) {
///     if (move.category === 'Physical' && typeof accuracy === 'number') {
///         return this.chainModify([3277, 4096]);
///     }
/// }
pub fn on_source_modify_accuracy(battle: &mut Battle, _accuracy: i32, _target_pos: (usize, usize), _source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.category == "Physical" {
            battle.chain_modify_fraction(3277, 4096);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

