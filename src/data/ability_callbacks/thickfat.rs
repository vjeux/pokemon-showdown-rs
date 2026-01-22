//! Thick Fat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onSourceModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Ice' || move.type === 'Fire') {
///         this.debug('Thick Fat weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type which is the active move's current type (may be modified by abilities like Refrigerate)
    let move_type = match active_move {
        Some(m) => m.move_type.as_str(),
        None => return EventResult::Continue,
    };

    if move_type == "Ice" || move_type == "Fire" {
        battle.chain_modify(0.5);
    }
    EventResult::Continue
}

/// onSourceModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Ice' || move.type === 'Fire') {
///         this.debug('Thick Fat weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type which is the active move's current type (may be modified by abilities like Refrigerate)
    let move_type = match active_move {
        Some(m) => m.move_type.as_str(),
        None => return EventResult::Continue,
    };

    if move_type == "Ice" || move_type == "Fire" {
        battle.chain_modify(0.5);
    }
    EventResult::Continue
}

