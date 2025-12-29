//! Thick Fat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Ice' || move.type === 'Fire') {
///         this.debug('Thick Fat weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_atk(battle: &mut Battle, move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Ice" || move_data.move_type == "Fire" {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onSourceModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Ice' || move.type === 'Fire') {
///         this.debug('Thick Fat weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(battle: &mut Battle, move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Ice" || move_data.move_type == "Fire" {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

