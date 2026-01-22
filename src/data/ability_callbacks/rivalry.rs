//! Rivalry Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
pub fn on_base_power(battle: &mut Battle, _base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
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
            battle.chain_modify(1.25);
            return EventResult::Continue;
        } else {
            battle.chain_modify(0.75);
            return EventResult::Continue;
        }
    }

    EventResult::Continue
}

