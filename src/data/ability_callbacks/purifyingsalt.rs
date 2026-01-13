//! Purifying Salt Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSetStatus(status, target, source, effect) {
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Purifying Salt');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, _status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // Check if effect is a move with status
    if let Some(eff_id) = effect_id {
        if let Some(move_data) = battle.dex.moves().get(eff_id) {
            if move_data.status.is_some() {
                let target_ident = {
                    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Boolean(false),
                    };
                    target.get_slot()
                };

                battle.add(
                    "-immune",
                    &[
                        target_ident.as_str().into(),
                        "[from] ability: Purifying Salt".into(),
                    ],
                );
            }
        }
    }

    EventResult::Boolean(false)
}

/// onTryAddVolatile(status, target) {
///     if (status.id === 'yawn') {
///         this.add('-immune', target, '[from] ability: Purifying Salt');
///         return null;
///     }
/// }
pub fn on_try_add_volatile(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    if status_id == "yawn" {
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            target.get_slot()
        };

        battle.add(
            "-immune",
            &[
                target_ident.as_str().into(),
                "[from] ability: Purifying Salt".into(),
            ],
        );
        return EventResult::Null;
    }
    EventResult::Continue
}

/// onSourceModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Ghost') {
///         this.debug('Purifying Salt weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_atk(_battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type which is the active move's current type (may be modified by abilities)
    let move_type = match active_move {
        Some(m) => m.move_type.as_str(),
        None => return EventResult::Continue,
    };

    if move_type == "Ghost" {
        _battle.chain_modify(0.5);
    }
    EventResult::Continue
}

/// onSourceModifySpA(spa, attacker, defender, move) {
///     if (move.type === 'Ghost') {
///         this.debug('Purifying Salt weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(_battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type which is the active move's current type (may be modified by abilities)
    let move_type = match active_move {
        Some(m) => m.move_type.as_str(),
        None => return EventResult::Continue,
    };

    if move_type == "Ghost" {
        _battle.chain_modify(0.5);
    }
    EventResult::Continue
}

