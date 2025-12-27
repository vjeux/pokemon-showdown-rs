//! Conversion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     const type = this.dex.moves.get(target.moveSlots[0].id).type;
///     if (target.hasType(type) || !target.setType(type)) return false;
///     this.add('-start', target, 'typechange', type);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the type of the first move in the moveset
    let move_type = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if target.move_slots.is_empty() {
            return EventResult::Bool(false);
        }

        let first_move_id = &target.move_slots[0].id;

        match battle.dex.get_move_by_id(first_move_id) {
            Some(move_data) => move_data.move_type.clone(),
            None => return EventResult::Continue,
        }
    };

    // Check if target already has this type
    let already_has_type = if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
        target.has_type(&move_type)
    } else {
        return EventResult::Continue;
    };

    if already_has_type {
        return EventResult::Bool(false);
    }

    // Try to set type
    let success = if let Some(target) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
        target.set_type(&move_type)
    } else {
        return EventResult::Continue;
    };

    if !success {
        return EventResult::Bool(false);
    }

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    use crate::battle::Arg;
    battle.add("-start", &[Arg::Pokemon(target), Arg::Str("typechange"), Arg::String(move_type)]);
    EventResult::Continue
}

