//! Stakeout Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAtk(atk, attacker, defender) {
///     if (!defender.activeTurns) {
///         this.debug('Stakeout boost');
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // Double Attack if defender just switched in (activeTurns == 0)
    let defender_active_turns = {
        let defender = match battle.pokemon_at(defender_pos.0, defender_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        defender.active_turns
    };

    if defender_active_turns == 0 {
        battle.chain_modify(2.0);
    }

    EventResult::Continue
}

/// onModifySpA(atk, attacker, defender) {
///     if (!defender.activeTurns) {
///         this.debug('Stakeout boost');
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // Double Special Attack if defender just switched in (activeTurns == 0)
    let defender_active_turns = {
        let defender = match battle.pokemon_at(defender_pos.0, defender_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        defender.active_turns
    };

    if defender_active_turns == 0 {
        battle.chain_modify(2.0);
    }

    EventResult::Continue
}

