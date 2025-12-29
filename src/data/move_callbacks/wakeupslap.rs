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
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // Get the target
    let target = match target_pos {
        Some(pos) => match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (target.status === 'slp' || target.hasAbility('comatose'))
    if target.status == ID::from("slp") || target.has_ability(&["comatose"]) {
        let bp = active_move.base_power * 2;
        // this.debug('BP doubled on sleeping target');
        battle.debug("BP doubled on sleeping target");
        return EventResult::Number(bp);
    }

    EventResult::Number(active_move.base_power)
}

/// onHit(target) {
///     if (target.status === 'slp') target.cureStatus();
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // Get the target position
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if target is asleep before getting mutable reference
    let is_asleep = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.status == ID::from("slp")
    };

    // if (target.status === 'slp') target.cureStatus();
    if is_asleep {
        let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.cure_status();
    }

    EventResult::Continue
}
