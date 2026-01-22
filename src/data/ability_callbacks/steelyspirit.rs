//! Steely Spirit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onAllyBasePower(basePower, attacker, defender, move) {
///     if (move.type === 'Steel') {
///         this.debug('Steely Spirit boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_ally_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: Option<(usize, usize)>, _defender_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    if active_move.map(|m| m.move_type == "Steel").unwrap_or(false) {
        battle.chain_modify(1.5);
    }
    EventResult::Continue
}
