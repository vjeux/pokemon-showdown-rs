//! BattleActions::runSwitch - Run switch-in effects
//!
//! 1:1 port of runSwitch from battle-actions.ts

use crate::*;
use crate::battle::PriorityItem;

/// Run switch-in effects for a Pokemon
/// Equivalent to battle-actions.ts runSwitch()
/// 1:1 port of battle-actions.ts runSwitch()
pub fn run_switch(battle: &mut Battle, side_idx: usize, poke_idx: usize) {
    eprintln!("[RUN_SWITCH DEBUG] Called for side {} pokemon {}", side_idx, poke_idx);
    // Collect all switchers - consume all consecutive runSwitch actions
    let mut switchers_in: Vec<(usize, usize)> = vec![(side_idx, poke_idx)];

    // Collect any additional runSwitch actions from the queue
    while let Some(action) = battle.queue.peek() {
        if action.is_run_switch() {
            if let Some((s, p)) = action.get_switch_target() {
                eprintln!("[RUN_SWITCH DEBUG] Found additional switcher: side {} pokemon {}", s, p);
                switchers_in.push((s, p));
            }
            battle.queue.shift();
        } else {
            break;
        }
    }

    eprintln!("[RUN_SWITCH DEBUG] Total switchers: {}", switchers_in.len());

    // JS: const allActive = this.battle.getAllActive(true);
    // JS: this.battle.speedSort(allActive);
    // Collect all active Pokemon with their speeds
    let mut all_active: Vec<(usize, usize, i32)> = Vec::new();
    for (side_idx, side) in battle.sides.iter().enumerate() {
        for poke_idx in side.active.iter().flatten() {
            if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                if !pokemon.fainted {
                    all_active.push((side_idx, *poke_idx, pokemon.speed));
                }
            }
        }
    }

    eprintln!("[RUN_SWITCH DEBUG] All active count: {}, calling speed_sort...", all_active.len());
    // Sort all active Pokemon using speed_sort (which uses PRNG for ties)
    battle.speed_sort(&mut all_active, |item| PriorityItem {
        order: None,
        priority: 0,
        speed: item.2,
        sub_order: 0,
        effect_order: 0,
        index: 0,
    });
    eprintln!("[RUN_SWITCH DEBUG] speed_sort done");

    // Speed sort all active Pokemon
    battle.update_speed();

    // Run SwitchIn field event for all switchers
    battle.field_event_switch_in(&switchers_in);

    // Mark all switchers as started and clear draggedIn
    for (s_idx, p_idx) in &switchers_in {
        if battle.sides[*s_idx].pokemon[*p_idx].hp == 0 {
            continue;
        }
        battle.sides[*s_idx].pokemon[*p_idx].is_started = true;
        battle.sides[*s_idx].pokemon[*p_idx].dragged_in = None;
    }
}
