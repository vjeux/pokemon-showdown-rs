//! Technician Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     const basePowerAfterMultiplier = this.modify(basePower, this.event.modifier);
///     this.debug(`Base Power: ${basePowerAfterMultiplier}`);
///     if (basePowerAfterMultiplier <= 60) {
///         this.debug('Technician boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // const basePowerAfterMultiplier = this.modify(basePower, this.event.modifier);
    let modifier = battle.get_event_modifier();
    let base_power_after_multiplier = battle.modify_f(base_power, modifier as f64 / 4096.0);

    // this.debug(`Base Power: ${basePowerAfterMultiplier}`);
    eprintln!("Base Power: {}", base_power_after_multiplier);

    // if (basePowerAfterMultiplier <= 60) {
    //     this.debug('Technician boost');
    //     return this.chainModify(1.5);
    // }
    if base_power_after_multiplier <= 60 {
        eprintln!("Technician boost");
        battle.chain_modify(1.5); return EventResult::Continue;
    }

    EventResult::Continue
}

