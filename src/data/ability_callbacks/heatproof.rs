//! Heatproof Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         this.debug('Heatproof Atk weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
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
///         this.debug('Heatproof SpA weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Fire" {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onDamage(damage, target, source, effect) {
///     if (effect && effect.id === 'brn') {
///         return damage / 2;
///     }
/// }
pub fn on_damage(_battle: &mut Battle, damage: i32, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    if let Some(effect) = effect_id {
        if effect == "brn" {
            return EventResult::Number(damage / 2);
        }
    }
    EventResult::Continue
}

