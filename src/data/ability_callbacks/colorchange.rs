//! Color Change Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onAfterMoveSecondary(target, source, move) {
///     if (!target.hp) return;
///     const type = move.type;
///     if (
///         target.isActive && move.effectType === 'Move' && move.category !== 'Status' &&
///         type !== '???' && !target.hasType(type)
///     ) {
///         if (!target.setType(type)) return false;
///         this.add('-start', target, 'typechange', type, '[from] ability: Color Change');
///
///         if (target.side.active.length === 2 && target.position === 1) {
///             // Curse Glitch
///             const action = this.queue.willMove(target);
///             if (action && action.move.id === 'curse') {
///                 action.targetLoc = -1;
///             }
///         }
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: (usize, usize), _source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (!target.hp) return;
    let has_hp = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.hp > 0
    };

    if !has_hp {
        return EventResult::Continue;
    }

    // const type = move.type;
    let active_move = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check conditions
    if active_move.category == "Status" || active_move.move_type.is_empty() || active_move.move_type == "???" {
        return EventResult::Continue;
    }

    let move_type = active_move.move_type.clone();

    // if (target.isActive && move.effectType === 'Move' && move.category !== 'Status' && type !== '???' && !target.hasType(type))
    let (is_active, should_change_type) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Check if target already has this type
        let has_type = target.has_type(battle, &move_type);

        (target.is_active, !has_type)
    };

    if !is_active || !should_change_type {
        return EventResult::Continue;
    }

    // if (!target.setType(type)) return false;
    let success = Pokemon::set_type(battle, target_pos, vec![move_type.clone()], false);
    if !success {
        return EventResult::Boolean(false);
    }

    // Get target slot for message
    let target_slot = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    // this.add('-start', target, 'typechange', type, '[from] ability: Color Change');
    battle.add("-start", &[
        target_slot.as_str().into(),
        "typechange".into(),
        move_type.clone().into(),
        "[from] ability: Color Change".into(),
    ]);

    // NOTE: Curse Glitch handling skipped - requires queue.willMove() infrastructure
    // if (target.side.active.length === 2 && target.position === 1) {
    //     const action = this.queue.willMove(target);
    //     if (action && action.move.id === 'curse') {
    //         action.targetLoc = -1;
    //     }
    // }

    EventResult::Continue
}

