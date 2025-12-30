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
pub fn on_modify_atk(battle: &mut Battle, atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    let modified = battle.modify_f(atk, 1.5);
    EventResult::Number(modified)
}

/// onSourceModifyAccuracy(accuracy, target, source, move) {
///     if (move.category === 'Physical' && typeof accuracy === 'number') {
///         return this.chainModify([3277, 4096]);
///     }
/// }
pub fn on_source_modify_accuracy(battle: &mut Battle, _accuracy: i32, _target_pos: (usize, usize), _source_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.category == "Physical" {
            let modified = battle.chain_modify_fraction(3277, 4096);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

