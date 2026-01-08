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
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Bug" {
            if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                if attacker.hp <= attacker.maxhp / 3 {
                    battle.chain_modify(1.5);
                    return EventResult::Continue;
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
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Bug" {
            if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                if attacker.hp <= attacker.maxhp / 3 {
                    battle.chain_modify(1.5);
                    return EventResult::Continue;
                }
            }
        }
    }
    EventResult::Continue
}

