//! Triage Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyPriority(priority, pokemon, target, move) {
///     if (move?.flags['heal']) return priority + 3;
/// }
pub fn on_modify_priority(battle: &mut Battle, priority: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

