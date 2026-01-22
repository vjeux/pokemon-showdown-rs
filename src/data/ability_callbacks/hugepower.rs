//! Huge Power Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyAtk(atk) {
///     return this.chainModify(2);
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    battle.chain_modify(2.0);
    EventResult::Continue
}

