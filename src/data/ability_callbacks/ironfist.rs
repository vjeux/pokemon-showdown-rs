//! Iron Fist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (move.flags['punch']) {
///         this.debug('Iron Fist boost');
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.flags.contains_key("punch") {
            let modified = battle.chain_modify_fraction(4915, 4096);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

