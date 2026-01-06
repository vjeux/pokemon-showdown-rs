//! Power Spot Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAllyBasePower(basePower, attacker, defender, move) {
///     if (attacker !== this.effectState.target) {
///         this.debug('Power Spot boost');
///         return this.chainModify([5325, 4096]);
///     }
/// }
pub fn on_ally_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: Option<(usize, usize)>, _defender_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // Get the ability holder (Power Spot user)
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the attacker from current event
    let attacker_pos = match &battle.current_event {
        Some(event) => event.source,
        None => return EventResult::Continue,
    };

    let attacker_pos = match attacker_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (attacker !== this.effectState.target)
    if attacker_pos == ability_holder {
        return EventResult::Continue;
    }

    // this.debug('Power Spot boost');
    // return this.chainModify([5325, 4096]);
    EventResult::Number(battle.chain_modify_fraction(5325, 4096))
}

