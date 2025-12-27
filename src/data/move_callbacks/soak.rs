//! Soak Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// onHit(target) {
///     if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
///         // Soak should animate even when it fails.
///         // Returning false would suppress the animation.
///         this.add('-fail', target);
///         return null;
///     }
///     this.add('-start', target, 'typechange', 'Water');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if target is already pure Water type
    let current_types = if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
        target.get_types()
    } else {
        return EventResult::Continue;
    };

    let is_already_water = current_types.len() == 1 && current_types[0] == "Water";

    if is_already_water {
        // Already Water type - fail but still animate
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.add("-fail", &[Arg::Pokemon(target)]);
        return EventResult::Null;
    }

    // Try to set type to Water
    let success = if let Some(target) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
        target.set_type("Water")
    } else {
        return EventResult::Continue;
    };

    if !success {
        // Failed to set type - fail but still animate
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.add("-fail", &[Arg::Pokemon(target)]);
        return EventResult::Null;
    }

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    battle.add("-start", &[Arg::Pokemon(target), Arg::Str("typechange"), Arg::Str("Water")]);
    EventResult::Continue
}

