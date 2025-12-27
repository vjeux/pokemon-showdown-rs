//! Wake-Up Slap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (target.status === 'slp' || target.hasAbility('comatose')) {
///         this.debug('BP doubled on sleeping target');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let move_id = match &battle.active_move {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    let move_data = match battle.dex.get_move_by_id(move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if target is asleep or has comatose ability
    let is_asleep = target.has_status("slp");
    let has_comatose = target.has_ability("comatose");

    if is_asleep || has_comatose {
        battle.debug("BP doubled on sleeping target");
        EventResult::Number(move_data.base_power * 2)
    } else {
        EventResult::Number(move_data.base_power)
    }
}

/// onHit(target) {
///     if (target.status === 'slp') target.cureStatus();
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if target is asleep, then cure it
    let is_asleep = if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
        target.has_status("slp")
    } else {
        return EventResult::Continue;
    };

    if is_asleep {
        if let Some(target) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            target.cure_status();
        }
    }

    EventResult::Continue
}

