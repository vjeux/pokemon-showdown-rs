//! Adaptability Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySTAB(stab, source, target, move) {
///     if (move.forceSTAB || source.hasType(move.type)) {
///         if (stab === 2) {
///             return 2.25;
///         }
///         return 2;
///     }
/// }
pub fn on_modify_s_t_a_b(battle: &mut Battle, stab: f64, source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (move.forceSTAB || source.hasType(move.type))
    let src_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    let force_stab = active_move.force_stab;
    let move_type = &active_move.move_type;

    let has_type = {
        let source = match battle.pokemon_at(src_pos.0, src_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.has_type(battle, move_type)
    };

    if force_stab || has_type {
        // if (stab === 2)
        if (stab - 2.0).abs() < 0.001 {
            // return 2.25;
            battle.chain_modify(2.25); return EventResult::Continue;
        }
        // return 2;
        battle.chain_modify(2.0); return EventResult::Continue;
    }

    EventResult::Continue
}

