//! Emergency Exit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onEmergencyExit(target) {
///     if (!this.canSwitch(target.side) || target.forceSwitchFlag || target.switchFlag) return;
///     for (const side of this.sides) {
///         for (const active of side.active) {
///             active.switchFlag = false;
///         }
///     }
///     target.switchFlag = true;
///     this.add('-activate', target, 'ability: Emergency Exit');
/// }
pub fn on_emergency_exit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::battle::Arg;

    // Get target position
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!this.canSwitch(target.side) || target.forceSwitchFlag || target.switchFlag) return;
    let (force_switch_flag, switch_flag) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target.force_switch_flag, target.switch_flag.is_some())
    };

    // Check if can switch
    if battle.can_switch(target_pos.0) == 0 || force_switch_flag || switch_flag {
        return EventResult::Continue;
    }

    // for (const side of this.sides) {
    //     for (const active of side.active) {
        //         active.switchFlag = false;
    //     }
    // }
    // Clear all switchFlags for all active Pokemon
    for side_idx in 0..battle.sides.len() {
        let active_count = battle.sides[side_idx].active.len();
        for active_slot in 0..active_count {
            if let Some(pokemon_index) = battle.sides[side_idx].active[active_slot] {
                if pokemon_index < battle.sides[side_idx].pokemon.len() {
                    battle.sides[side_idx].pokemon[pokemon_index].switch_flag = None;
                }
            }
        }
    }

    // target.switchFlag = true;
    let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    target_mut.switch_flag = Some(String::new()); // Generic switch (ability-triggered)

    // this.add('-activate', target, 'ability: Emergency Exit');
    let target_slot = target_mut.get_slot();
    battle.add("-activate", &[
        Arg::String(target_slot),
        Arg::Str("ability: Emergency Exit"),
    ]);

    EventResult::Continue
}

