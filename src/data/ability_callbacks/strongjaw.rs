//! Strong Jaw Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (move.flags['bite']) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.flags.contains_key("bite") {
            let modified = battle.chain_modify(1.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

