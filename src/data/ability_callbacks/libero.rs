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
pub fn on_prepare_hit(battle: &mut Battle, source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    use serde_json::Value;

    // if (this.effectState.libero) return;
    if let Some(Value::Bool(true)) = battle.effect_state.data.get("libero") {
        return EventResult::Continue;
    }

    // Get source position
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
    let should_return = if let Some(ref active_move) = battle.active_move {
        active_move.has_bounced
            || active_move.flags.future_move
            || active_move.source_effect.as_ref().map(|e| e.as_str()) == Some("snatch")
            || active_move.calls_move.is_some()
    } else {
        return EventResult::Continue;
    };

    if should_return {
        return EventResult::Continue;
    }

    // const type = move.type;
    // if (type && type !== '???' && source.getTypes().join() !== type)
    let move_type = if let Some(ref active_move) = battle.active_move {
        if active_move.move_type.is_empty() || active_move.move_type == "???" {
            return EventResult::Continue;
        }
        active_move.move_type.clone()
    } else {
        return EventResult::Continue;
    };

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
    battle.effect_state.data.insert("libero".to_string(), Value::Bool(true));

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

