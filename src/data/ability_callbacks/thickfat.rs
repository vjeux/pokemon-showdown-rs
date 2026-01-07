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
pub fn on_source_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Ice" || move_data.move_type == "Fire" {
            battle.chain_modify(0.5);
            return EventResult::Continue; // JavaScript chainModify returns void, so we return Continue
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
    eprintln!("[THICK FAT] onSourceModifySpA called! move_id={}", move_id);
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        eprintln!("[THICK FAT] move_type={}", move_data.move_type);
        if move_data.move_type == "Ice" || move_data.move_type == "Fire" {
            eprintln!("[THICK FAT] Halving SpA! (Fire/Ice move detected)");
            battle.chain_modify(0.5);
            eprintln!("[THICK FAT] chain_modify(0.5) called, returning Continue");
            return EventResult::Continue; // JavaScript chainModify returns void, so we return Continue
        } else {
            eprintln!("[THICK FAT] Not a Fire/Ice move, skipping");
        }
    } else {
        eprintln!("[THICK FAT] No move data found for move_id={}", move_id);
    }
    EventResult::Continue
}

