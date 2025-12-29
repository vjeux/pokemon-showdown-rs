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
pub fn on_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryAddVolatile(status, target) {
///     if (status.id === 'yawn') {
///         this.add('-immune', target, '[from] ability: Purifying Salt');
///         return null;
///     }
/// }
pub fn on_try_add_volatile(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
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
pub fn on_source_modify_atk(battle: &mut Battle, move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Ghost" {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onSourceModifySpA(spa, attacker, defender, move) {
///     if (move.type === 'Ghost') {
///         this.debug('Purifying Salt weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_sp_a(battle: &mut Battle, move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Ghost" {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

