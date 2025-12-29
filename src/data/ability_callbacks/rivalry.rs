//! Rivalry Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (attacker.gender && defender.gender) {
///         if (attacker.gender === defender.gender) {
///             this.debug('Rivalry boost');
///             return this.chainModify(1.25);
///         } else {
///             this.debug('Rivalry weaken');
///             return this.chainModify(0.75);
///         }
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str) -> EventResult {
    use crate::dex_data::Gender;

    let (attacker_gender, defender_gender) = {
        let attacker = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let defender = match battle.pokemon_at(defender_pos.0, defender_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (attacker.gender, defender.gender)
    };

    if attacker_gender != Gender::None && defender_gender != Gender::None {
        if attacker_gender == defender_gender {
            let modified = battle.chain_modify(1.25);
            return EventResult::Number(modified);
        } else {
            let modified = battle.chain_modify(0.75);
            return EventResult::Number(modified);
        }
    }

    EventResult::Continue
}

