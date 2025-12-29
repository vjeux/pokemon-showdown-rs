//! Toxic Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if ((attacker.status === 'psn' || attacker.status === 'tox') && move.category === 'Physical') {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.category == "Physical" {
            if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                if attacker.status == "psn".into() || attacker.status == "tox".into() {
                    let modified = battle.chain_modify(1.5);
                    return EventResult::Number(modified);
                }
            }
        }
    }
    EventResult::Continue
}

