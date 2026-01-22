//! Defeatist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyAtk(atk, pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 2) {
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    if let Some(pokemon) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
        if pokemon.hp <= pokemon.maxhp / 2 {
            battle.chain_modify(0.5);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

/// onModifySpA(atk, pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 2) {
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    if let Some(pokemon) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
        if pokemon.hp <= pokemon.maxhp / 2 {
            battle.chain_modify(0.5);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

