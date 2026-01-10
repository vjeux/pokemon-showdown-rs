//! Libero Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onPrepareHit(source, target, move) {
///     if (this.effectState.libero) return;
///     if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
///     const type = move.type;
///     if (type && type !== '???' && source.getTypes().join() !== type) {
///         if (!source.setType(type)) return;
///         this.effectState.libero = true;
///         this.add('-start', source, 'typechange', type, '[from] ability: Libero');
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (this.effectState.libero) return;
    // Use with_effect_state_ref to check persistent state
    let libero_used = battle.with_effect_state_ref(|state| {
        state.libero
    }).flatten().unwrap_or(false);
    if libero_used {
        return EventResult::Continue;
    }

    // Get source position
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get active move
    let active_move = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
    let should_return = active_move.has_bounced
        || active_move.flags.future_move
        || active_move.source_effect.as_ref().map(|e| e.as_str()) == Some("snatch")
        || active_move.calls_move;

    if should_return {
        return EventResult::Continue;
    }

    // const type = move.type;
    // if (type && type !== '???' && source.getTypes().join() !== type)
    if active_move.move_type.is_empty() || active_move.move_type == "???" {
        return EventResult::Continue;
    }
    let move_type = active_move.move_type.clone();

    // Check if source types already match move type
    let types_match = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let current_types = source.get_types(battle, false);
        current_types.join("") == move_type
    };

    if types_match {
        return EventResult::Continue;
    }

    // if (!source.setType(type)) return;
    let success = Pokemon::set_type(battle, source_pos, vec![move_type.clone()], false);
    if !success {
        return EventResult::Continue;
    }

    // this.effectState.libero = true;
    // Use with_effect_state to persist in the ability's effect state
    battle.with_effect_state(|state| {
        state.libero = Some(true);
    });

    // Get source slot for message
    let source_slot = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.get_slot()
    };

    // this.add('-start', source, 'typechange', type, '[from] ability: Libero');
    battle.add("-start", &[
        source_slot.as_str().into(),
        "typechange".into(),
        move_type.clone().into(),
        "[from] ability: Libero".into(),
    ]);

    EventResult::Continue
}

