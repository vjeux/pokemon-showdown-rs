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
pub fn on_source_modify_atk(_battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type which is the active move's current type (may be modified by abilities like Refrigerate)
    let move_type = match active_move {
        Some(m) => m.move_type.as_str(),
        None => return EventResult::Continue,
    };

    if move_type == "Fire" {
        _battle.chain_modify(0.5);
    }
    EventResult::Continue
}

/// onSourceModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         this.debug('Heatproof SpA weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(_battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type which is the active move's current type (may be modified by abilities like Refrigerate)
    let move_type = match active_move {
        Some(m) => m.move_type.as_str(),
        None => return EventResult::Continue,
    };

    if move_type == "Fire" {
        _battle.chain_modify(0.5);
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

