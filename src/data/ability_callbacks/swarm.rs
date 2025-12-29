//! Swarm Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAtk(atk, attacker, defender, move) {
///     if (move.type === 'Bug' && attacker.hp <= attacker.maxhp / 3) {
///         this.debug('Swarm boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, atk: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Bug" {
            if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                if attacker.hp <= attacker.maxhp / 3 {
                    let modified = battle.chain_modify(1.5);
                    return EventResult::Number(modified);
                }
            }
        }
    }
    EventResult::Continue
}

/// onModifySpA(atk, attacker, defender, move) {
///     if (move.type === 'Bug' && attacker.hp <= attacker.maxhp / 3) {
///         this.debug('Swarm boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, spa: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.move_type == "Bug" {
            if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                if attacker.hp <= attacker.maxhp / 3 {
                    let modified = battle.chain_modify(1.5);
                    return EventResult::Number(modified);
                }
            }
        }
    }
    EventResult::Continue
}

