//! Torrent Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Water' && attacker.hp <= attacker.maxhp / 3) {
///         this.debug('Torrent boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    if active_move.map(|m| m.move_type == "Water").unwrap_or(false) {
        if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            if attacker.hp <= attacker.maxhp / 3 {
                battle.chain_modify(1.5);
            }
        }
    }
    EventResult::Continue
}

/// onModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Water' && attacker.hp <= attacker.maxhp / 3) {
///         this.debug('Torrent boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    if active_move.map(|m| m.move_type == "Water").unwrap_or(false) {
        if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            if attacker.hp <= attacker.maxhp / 3 {
                battle.chain_modify(1.5);
            }
        }
    }
    EventResult::Continue
}
