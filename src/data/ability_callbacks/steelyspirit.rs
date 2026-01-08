//! Steely Spirit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAllyBasePower(basePower, attacker, defender, move) {
///     if (move.type === 'Steel') {
///         this.debug('Steely Spirit boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_ally_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: Option<(usize, usize)>, _defender_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Steel" {
            let modified = battle.chain_modify(1.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

