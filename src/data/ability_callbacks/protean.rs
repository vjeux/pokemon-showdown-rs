//! Protean Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(source, target, move) {
///     if (this.effectState.protean) return;
///     if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
///     const type = move.type;
///     if (type && type !== '???' && source.getTypes().join() !== type) {
///         if (!source.setType(type)) return;
///         this.effectState.protean = true;
///         this.add('-start', source, 'typechange', type, '[from] ability: Protean');
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    eprintln!("[PROTEAN] on_prepare_hit called: source_pos={:?}, move_id={}", source_pos, move_id);

    // Get source position
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get move data to check type
    let move_data = match battle.dex.moves().get(move_id) {
        Some(m) => m.clone(),
        None => return EventResult::Continue,
    };

    let move_type = &move_data.move_type;

    // JS: if (type && type !== '???' && source.getTypes().join() !== type)
    if move_type.is_empty() || move_type == "???" {
        return EventResult::Continue;
    }

    // Check if source already has this type (get types first to avoid borrow issues)
    let source_types = if let Some(side) = battle.sides.get(source_pos.0) {
        if let Some(pokemon) = side.pokemon.get(source_pos.1) {
            pokemon.types.clone()
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    // If Pokemon already is this type (single type), don't change
    if source_types.len() == 1 && source_types[0] == *move_type {
        return EventResult::Continue;
    }

    // If Pokemon has this type in dual typing, still change to single type
    // JavaScript: source.getTypes().join() !== type checks if types string doesn't match
    // For single type Dark, join() = "Dark". For dual Grass/Dark, join() = "Grass,Dark"
    // So we only skip if it's already a single type matching move_type

    // Set type (this will replace all types with just this one)
    // JS: if (!source.setType(type)) return;
    let set_success = if let Some(side) = battle.sides.get_mut(source_pos.0) {
        if let Some(pokemon) = side.pokemon.get_mut(source_pos.1) {
            pokemon.types = vec![move_type.clone()];
            true
        } else {
            false
        }
    } else {
        false
    };

    if !set_success {
        return EventResult::Continue;
    }

    // Get source slot for message
    let source_slot = if let Some(side) = battle.sides.get(source_pos.0) {
        if let Some(pokemon) = side.pokemon.get(source_pos.1) {
            pokemon.get_slot()
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    // JS: this.add('-start', source, 'typechange', type, '[from] ability: Protean');
    battle.add("-start", &[
        source_slot.as_str().into(),
        "typechange".into(),
        move_type.clone().into(),
        "[from] ability: Protean".into(),
    ]);

    EventResult::Continue
}

