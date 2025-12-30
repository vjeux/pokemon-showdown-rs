//! Steelworker Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Steel') {
///         this.debug('Steelworker boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Steel" {
            let modified = battle.chain_modify(1.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Steel') {
///         this.debug('Steelworker boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Steel" {
            let modified = battle.chain_modify(1.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

