//! Snore Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     return source.status === 'slp' || source.hasAbility('comatose');
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Only works if user is asleep or has comatose ability
    let is_asleep = source.has_status("slp");
    let has_comatose = source.has_ability("comatose");

    EventResult::Bool(is_asleep || has_comatose)
}

