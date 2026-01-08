//! Fur Coat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDef(def) {
///     return this.chainModify(2);
/// }
pub fn on_modify_def(battle: &mut Battle, _def: i32, _defender_pos: (usize, usize), _attacker_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    let modified = battle.chain_modify(2.0);
    EventResult::Number(modified)
}

