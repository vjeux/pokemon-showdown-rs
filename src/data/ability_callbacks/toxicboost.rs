//! Toxic Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if ((attacker.status === 'psn' || attacker.status === 'tox') && move.category === 'Physical') {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.category (the active move's category, not the dex category)
    let is_physical = active_move.map(|m| m.category == "Physical").unwrap_or(false);

    if is_physical {
        if let Some(attacker) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            if attacker.status == "psn".into() || attacker.status == "tox".into() {
                battle.chain_modify(1.5);
            }
        }
    }
    EventResult::Continue
}
