//! Tough Claws Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (move.flags['contact']) {
///         return this.chainModify([5325, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Check the active move's flags, NOT the static dex data
    // Some moves like Shell Side Arm dynamically add the contact flag
    if let Some(active_move) = active_move {
        if active_move.flags.contains_key("contact") {
            battle.chain_modify_fraction(5325, 4096);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

