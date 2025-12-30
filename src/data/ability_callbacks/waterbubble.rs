//! Water Bubble Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_atk(battle: &mut Battle, move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Fire" {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onSourceModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(battle: &mut Battle, move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Fire" {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Water') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Water" {
            let modified = battle.chain_modify(2.0);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Water') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Water" {
            let modified = battle.chain_modify(2.0);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (pokemon.status === 'brn') {
///         this.add('-activate', pokemon, 'ability: Water Bubble');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if (status.id !== 'brn') return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Water Bubble');
///     }
///     return false;
/// }
pub fn on_set_status(_battle: &mut Battle, _status_id: &str, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

