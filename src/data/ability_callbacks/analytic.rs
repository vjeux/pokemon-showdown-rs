//! Analytic Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon) {
///     let boosted = true;
///     for (const target of this.getAllActive()) {
///         if (target === pokemon) continue;
///         if (this.queue.willMove(target)) {
///             boosted = false;
///             break;
///         }
///     }
///     if (boosted) {
///         this.debug('Analytic boost');
///         return this.chainModify([5325, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // let boosted = true;
    let mut boosted = true;

    // for (const target of this.getAllActive()) {
    let active_pokemon: Vec<(usize, usize)> = battle.get_all_active(false);

    for target_pos in active_pokemon {
        // if (target === pokemon) continue;
        if target_pos == attacker_pos {
            continue;
        }

        // if (this.queue.willMove(target)) {
        if battle.queue.will_move(target_pos.0, target_pos.1).is_some() {
            // boosted = false;
            boosted = false;
            // break;
            break;
        }
    }

    // if (boosted) {
    if boosted {
        // this.debug('Analytic boost');
        battle.debug("Analytic boost");
        // return this.chainModify([5325, 4096]);
        battle.chain_modify_fraction(5325, 4096); return EventResult::Continue;
    }

    EventResult::Continue
}

