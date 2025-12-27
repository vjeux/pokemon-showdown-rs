//! Clear Smog Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// onHit(target) {
///     target.clearBoosts();
///     this.add('-clearboost', target);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Clear all stat boosts from target
    if let Some(target) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
        target.clear_boosts();
    }

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    battle.add("-clearboost", &[Arg::Pokemon(target)]);

    EventResult::Continue
}

