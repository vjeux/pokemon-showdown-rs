//! Punk Rock Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (move.flags['sound']) {
///         this.debug('Punk Rock boost');
///         return this.chainModify([5325, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.flags.contains_key("sound") {
            let modified = battle.chain_modify_fraction(5325, 4096);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onSourceModifyDamage(damage, source, target, move) {
///     if (move.flags['sound']) {
///         this.debug('Punk Rock weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.flags.contains_key("sound") {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

