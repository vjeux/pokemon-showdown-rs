//! Eject Button Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterMoveSecondary(target, source, move) {
///     if (source && source !== target && target.hp && move && move.category !== 'Status' && !move.flags['futuremove']) {
///         if (!this.canSwitch(target.side) || target.forceSwitchFlag || target.beingCalledBack || target.isSkyDropped()) return;
///         if (target.volatiles['commanding'] || target.volatiles['commanded']) return;
///         for (const pokemon of this.getAllActive()) {
///             if (pokemon.switchFlag === true) return;
///         }
///         target.switchFlag = true;
///         if (target.useItem()) {
///             source.switchFlag = false;
///         } else {
///             target.switchFlag = false;
///         }
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (source && source !== target && target.hp && move && move.category !== 'Status' && !move.flags['futuremove']) {
    //     if (!this.canSwitch(target.side) || target.forceSwitchFlag || target.beingCalledBack || target.isSkyDropped()) return;
    //     if (target.volatiles['commanding'] || target.volatiles['commanded']) return;
    //     for (const pokemon of this.getAllActive()) {
    //         if (pokemon.switchFlag === true) return;
    //     }
    //     target.switchFlag = true;
    //     if (target.useItem()) {
    //         source.switchFlag = false;
    //     } else {
    //         target.switchFlag = false;
    //     }
    // }

    use crate::dex_data::ID;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source !== target
    if source_pos == target_pos {
        return EventResult::Continue;
    }

    // Check if move && move.category !== 'Status' && !move.flags['futuremove']
    let (is_status_move, is_future_move) = match &battle.active_move {
        Some(active_move) => (
            active_move.category == "Status",
            active_move.flags.future_move,
        ),
        None => return EventResult::Continue,
    };

    if is_status_move || is_future_move {
        return EventResult::Continue;
    }

    // Extract data from target (immutable phase)
    let (target_hp, target_force_switch_flag, target_being_called_back, target_side) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            target.hp,
            target.force_switch_flag,
            target.being_called_back,
            target_pos.0,
        )
    };
    let target_is_sky_dropped = Pokemon::is_sky_dropped(battle, target_pos);

    // target.hp
    if target_hp == 0 {
        return EventResult::Continue;
    }

    // if (!this.canSwitch(target.side) || target.forceSwitchFlag || target.beingCalledBack || target.isSkyDropped())
    if battle.can_switch(target_side) == 0 || target_force_switch_flag || target_being_called_back || target_is_sky_dropped {
        return EventResult::Continue;
    }

    // if (target.volatiles['commanding'] || target.volatiles['commanded'])
    let has_commanding_volatile = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.volatiles.contains_key(&ID::from("commanding")) ||
        target.volatiles.contains_key(&ID::from("commanded"))
    };

    if has_commanding_volatile {
        return EventResult::Continue;
    }

    // for (const pokemon of this.getAllActive()) {
    //     if (pokemon.switchFlag === true) return;
    // }
    let any_switch_flag = {
        let all_active = battle.get_all_active(false);
        all_active.iter().any(|&pos| {
            battle.pokemon_at(pos.0, pos.1)
                .map(|p| p.switch_flag)
                .unwrap_or(false)
        })
    };

    if any_switch_flag {
        return EventResult::Continue;
    }

    // target.switchFlag = true;
    {
        let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_mut.switch_flag = true;
    }

    // if (target.useItem())
    let item_used = {
        let _target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, target_pos, None, None)
    };

    if item_used.is_some() {
        // source.switchFlag = false;
        if let Some(source_mut) = battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            source_mut.switch_flag = false;
        }
    } else {
        // target.switchFlag = false;
        if let Some(target_mut) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            target_mut.switch_flag = false;
        }
    }

    EventResult::Continue
}
