//! Dream Eater Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target) {
///     return target.status === 'slp' || target.hasAbility('comatose');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Only works on sleeping targets (or comatose ability - TODO when ability system ready)
    // TODO: Add hasAbility('comatose') check when ability system is ready
    EventResult::Bool(target.status.as_str() == "slp")
}

