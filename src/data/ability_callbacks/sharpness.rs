//! Sharpness Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (move.flags['slicing']) {
///         this.debug('Sharpness boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.flags.contains_key("slicing") {
            battle.chain_modify(1.5);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

