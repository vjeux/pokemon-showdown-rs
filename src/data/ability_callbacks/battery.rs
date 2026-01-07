//! Battery Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAllyBasePower(basePower, attacker, defender, move) {
///     if (attacker !== this.effectState.target && move.category === 'Special') {
///         this.debug('Battery boost');
///         return this.chainModify([5325, 4096]);
///     }
/// }
pub fn on_ally_base_power(battle: &mut Battle, _base_power: i32, attacker_pos: Option<(usize, usize)>, _defender_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (attacker !== this.effectState.target && move.category === 'Special')
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
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

    // Check if move.category === 'Special'
    let is_special = if let Some(active_move) = &battle.active_move {
        active_move.category == "Special"
    } else {
        false
    };

    if is_special {
        // this.debug('Battery boost');
        // return this.chainModify([5325, 4096]);
        battle.chain_modify_fraction(5325, 4096); return EventResult::Continue;
    }

    EventResult::Continue
}

