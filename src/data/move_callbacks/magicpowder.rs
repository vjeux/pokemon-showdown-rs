//! Magic Powder Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// onHit(target) {
///     if (target.getTypes().join() === 'Psychic' || !target.setType('Psychic')) return false;
///     this.add('-start', target, 'typechange', 'Psychic');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if target is already pure Psychic type
    let current_types = if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
        target.get_types()
    } else {
        return EventResult::Continue;
    };

    let is_already_psychic = current_types.len() == 1 && current_types[0] == "Psychic";

    if is_already_psychic {
        // Already Psychic type - fail
        return EventResult::Bool(false);
    }

    // Try to set type to Psychic
    let success = if let Some(target) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
        target.set_type("Psychic")
    } else {
        return EventResult::Continue;
    };

    if !success {
        // Failed to set type
        return EventResult::Bool(false);
    }

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    battle.add("-start", &[Arg::Pokemon(target), Arg::Str("typechange"), Arg::Str("Psychic")]);
    EventResult::Continue
}

