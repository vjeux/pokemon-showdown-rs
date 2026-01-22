//! Red Card Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterMoveSecondary(target, source, move) {
///     if (source && source !== target && source.hp && target.hp && move && move.category !== 'Status') {
///         if (!source.isActive || !this.canSwitch(source.side) || source.forceSwitchFlag || target.forceSwitchFlag) {
///             return;
///         }
///         // The item is used up even against a pokemon with Ingrain or that otherwise can't be forced out
///         if (target.useItem(source)) {
///             if (this.runEvent('DragOut', source, target, move)) {
///                 source.forceSwitchFlag = true;
///             }
///         }
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (source && source !== target && source.hp && target.hp && move && move.category !== 'Status') {
    //     if (!source.isActive || !this.canSwitch(source.side) || source.forceSwitchFlag || target.forceSwitchFlag) {
    //         return;
    //     }
    //     if (target.useItem(source)) {
    //         if (this.runEvent('DragOut', source, target, move)) {
    //             source.forceSwitchFlag = true;
    //         }
    //     }
    // }

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // source !== target
    if source_pos == target_pos {
        return EventResult::Continue;
    }

    // Check if move.category !== 'Status'
    if active_move_ref.category == "Status" {
        return EventResult::Continue;
    }

    // Extract data we need from source and target (immutable phase)
    let (source_hp, source_is_active, source_force_switch_flag, source_side) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source.hp, source.is_active, source.force_switch_flag, source_pos.0)
    };

    let (target_hp, target_force_switch_flag) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target.hp, target.force_switch_flag)
    };

    // source.hp && target.hp
    if source_hp == 0 || target_hp == 0 {
        return EventResult::Continue;
    }

    // if (!source.isActive || !this.canSwitch(source.side) || source.forceSwitchFlag || target.forceSwitchFlag)
    if !source_is_active || battle.can_switch(source_side) == 0 || source_force_switch_flag || target_force_switch_flag {
        return EventResult::Continue;
    }

    // if (target.useItem(source))
    let item_used = {
        let _target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, target_pos, None, None)
    };

    if item_used.is_some() {
        // if (this.runEvent('DragOut', source, target, move))
        if battle.run_event(
            "DragOut",
            Some(crate::event::EventTarget::Pokemon(source_pos)),
            Some(target_pos),
            None,
            crate::event::EventResult::Number(1),
            false,
            false,
        ).is_truthy() {
            // source.forceSwitchFlag = true;
            if let Some(source_mut) = battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                source_mut.force_switch_flag = true;
            }
        }
    }

    EventResult::Continue
}
