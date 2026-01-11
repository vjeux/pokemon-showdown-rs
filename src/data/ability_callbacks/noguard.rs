//! No Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyInvulnerability(target, source, move) {
///     if (move && (source === this.effectState.target || target === this.effectState.target)) return 0;
/// }
pub fn on_any_invulnerability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move && (source === this.effectState.target || target === this.effectState.target)) return 0;
    let noguard_user = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if source or target is the No Guard user
    let is_involved = source_pos == Some(noguard_user) || target_pos == Some(noguard_user);

    if is_involved {
        // return 0; - makes the Pokemon always hittable (ignores invulnerability)
        return EventResult::Number(0);
    }

    EventResult::Continue
}

/// onAnyAccuracy(accuracy, target, source, move) {
///     if (move && (source === this.effectState.target || target === this.effectState.target)) {
///         return true;
///     }
///     return accuracy;
/// }
pub fn on_any_accuracy(battle: &mut Battle, accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move && (source === this.effectState.target || target === this.effectState.target)) return true;

    // JavaScript checks "if (move && ...)" - move must be truthy
    if active_move.is_none() {
        eprintln!("[NOGUARD_ACCURACY] active_move is None, returning Continue");
        return EventResult::Continue;
    }

    let noguard_user = match battle.effect_state.target {
        Some(pos) => pos,
        None => {
            eprintln!("[NOGUARD_ACCURACY] effect_state.target is None, returning Continue");
            return EventResult::Continue;
        }
    };

    eprintln!("[NOGUARD_ACCURACY] noguard_user={:?}, target_pos={:?}, source_pos={:?}", noguard_user, target_pos, source_pos);

    // Check if source or target is the No Guard user
    let is_involved = source_pos == Some(noguard_user) || target_pos == Some(noguard_user);

    if is_involved {
        eprintln!("[NOGUARD_ACCURACY] is_involved=true, returning Boolean(true)");
        // return true; - makes moves always hit
        return EventResult::Boolean(true);
    }

    eprintln!("[NOGUARD_ACCURACY] is_involved=false, returning Number({})", accuracy);
    // return accuracy;
    EventResult::Number(accuracy)
}

