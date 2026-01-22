//! Fur Coat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyDef(def) {
///     return this.chainModify(2);
/// }
///
/// NOTE: In JavaScript, chainModify() returns undefined, so "return this.chainModify(2)"
/// returns undefined. When a callback returns undefined, the relay variable is not changed.
/// Instead, chainModify modifies the internal event.modifier which is applied at the end.
/// So we should call chain_modify() for its side effect and return Continue.
pub fn on_modify_def(battle: &mut Battle, _def: i32, _defender_pos: (usize, usize), _attacker_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    battle.chain_modify(2.0);
    EventResult::Continue
}

