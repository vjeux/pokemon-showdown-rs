//! Pickpocket Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onAfterMoveSecondary(target, source, move) {
///     if (source && source !== target && move?.flags['contact']) {
///         if (target.item || target.switchFlag || target.forceSwitchFlag || source.switchFlag === true) {
///             return;
///         }
///         const yourItem = source.takeItem(target);
///         if (!yourItem) {
///             return;
///         }
///         if (!target.setItem(yourItem)) {
///             source.item = yourItem.id;
///             return;
///         }
///         this.add('-enditem', source, yourItem, '[silent]', '[from] ability: Pickpocket', `[of] ${source}`);
///         this.add('-item', target, yourItem, '[from] ability: Pickpocket', `[of] ${source}`);
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), _move_id: &str) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (source && source !== target && move?.flags['contact'])
    if source_pos == target_pos {
        return EventResult::Continue;
    }

    // Check if move has contact flag
    let move_id = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.id.clone()
    };

    let has_contact = battle.check_move_makes_contact(&move_id, source_pos, target_pos, false);
    if !has_contact {
        return EventResult::Continue;
    }

    // if (target.item || target.switchFlag || target.forceSwitchFlag || source.switchFlag === true)
    let (target_has_item, target_switch_flag, target_force_switch_flag) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (!target.item.is_empty(), target.switch_flag.is_some(), target.force_switch_flag)
    };

    if target_has_item || target_switch_flag || target_force_switch_flag {
        return EventResult::Continue;
    }

    let source_switch_flag = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.switch_flag.is_some()
    };

    if source_switch_flag {
        return EventResult::Continue;
    }

    // const yourItem = source.takeItem(target);
    let your_item: ID = match Pokemon::take_item(battle, source_pos, Some(target_pos)) {
        Some(item) => item,
        None => return EventResult::Continue,
    };

    // if (!target.setItem(yourItem))
    let set_success = Pokemon::set_item(battle, target_pos, your_item.clone(), Some(source_pos), None);
    if !set_success {
        // source.item = yourItem.id;
        let source_mut = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_mut.item = your_item.clone();
        return EventResult::Continue;
    }

    // this.add('-enditem', source, yourItem, '[silent]', '[from] ability: Pickpocket', `[of] ${source}`);
    let (source_slot, target_slot) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source.get_slot(), target.get_slot())
    };

    battle.add("-enditem", &[
        Arg::String(source_slot.clone()),
        Arg::String(your_item.to_string()),
        Arg::Str("[silent]"),
        Arg::Str("[from] ability: Pickpocket"),
        Arg::String(format!("[of] {}", source_slot)),
    ]);

    // this.add('-item', target, yourItem, '[from] ability: Pickpocket', `[of] ${source}`);
    battle.add("-item", &[
        Arg::String(target_slot),
        Arg::String(your_item.to_string()),
        Arg::Str("[from] ability: Pickpocket"),
        Arg::String(format!("[of] {}", source_slot)),
    ]);

    EventResult::Continue
}

