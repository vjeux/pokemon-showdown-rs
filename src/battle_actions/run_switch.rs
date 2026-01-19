//! BattleActions::runSwitch - Run switch-in effects
//!
//! 1:1 port of runSwitch from battle-actions.ts

use crate::*;
use crate::battle::PriorityItem;

/// Run switch-in effects for a Pokemon
/// runSwitch(pokemon: Pokemon) {
///     const switchersIn = [pokemon];
///     while (this.battle.queue.peek()?.choice === 'runSwitch') {
///         const nextSwitch = this.battle.queue.shift();
///         switchersIn.push(nextSwitch!.pokemon!);
///     }
///     const allActive = this.battle.getAllActive(true);
///     this.battle.speedSort(allActive);
///     this.battle.speedOrder = allActive.map(a => a.side.n * a.battle.sides.length + a.position);
///     this.battle.fieldEvent('SwitchIn', switchersIn);
///
///     for (const poke of switchersIn) {
///         if (!poke.hp) continue;
///         poke.isStarted = true;
///         poke.draggedIn = null;
///     }
///     return true;
/// }
pub fn run_switch(battle: &mut Battle, side_idx: usize, poke_idx: usize) {
    debug_elog!("[RUN_SWITCH DEBUG] Called for side {} pokemon {}", side_idx, poke_idx);
    // Collect all switchers - consume all consecutive runSwitch actions
    let mut switchers_in: Vec<(usize, usize)> = vec![(side_idx, poke_idx)];

    // Collect any additional runSwitch actions from the queue
    while let Some(action) = battle.queue.peek() {
        if action.is_run_switch() {
            if let Some((s, p)) = action.get_switch_target() {
                debug_elog!("[RUN_SWITCH DEBUG] Found additional switcher: side {} pokemon {}", s, p);
                switchers_in.push((s, p));
            }
            battle.queue.shift();
        } else {
            break;
        }
    }

    debug_elog!("[RUN_SWITCH DEBUG] Total switchers: {}", switchers_in.len());

    // JS: const allActive = this.battle.getAllActive(true);
    // JS: this.battle.speedSort(allActive);
    // Collect all active Pokemon with their speeds
    // Note: JS passes includeFainted=true here, so we include all active Pokemon
    // regardless of their fainted status. This is important for proper speedSort
    // shuffling when Pokemon with tied speeds are still technically active
    // even if they've been queued to faint.
    let mut all_active: Vec<(usize, usize, i32)> = Vec::new();
    for (side_idx, side) in battle.sides.iter().enumerate() {
        debug_elog!("[RUN_SWITCH DEBUG] Side {} active slots: {:?}", side_idx, side.active);
        for poke_idx in side.active.iter().flatten() {
            if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                debug_elog!("[RUN_SWITCH DEBUG] Side {} poke {} - fainted={}, faint_queued={}, speed={}",
                    side_idx, poke_idx, pokemon.fainted, pokemon.faint_queued, pokemon.speed);
                // Include all active Pokemon, even if fainted (matches JS getAllActive(true))
                all_active.push((side_idx, *poke_idx, pokemon.speed));
            }
        }
    }

    debug_elog!("[RUN_SWITCH DEBUG] All active count: {}, calling speed_sort...", all_active.len());
    // Sort all active Pokemon using speed_sort (which uses PRNG for ties)
    battle.speed_sort_with_callsite(&mut all_active, |item| PriorityItem {
        order: None,
        priority: 0,
        fractional_priority: 0.0,
        speed: item.2 as f64,
        sub_order: 0,
        effect_order: 0,
        index: 0,
    }, "run_switch:all_active");
    debug_elog!("[RUN_SWITCH DEBUG] speed_sort done");

    // JS: this.battle.speedOrder = allActive.map(a => a.side.n * a.battle.sides.length + a.position);
    battle.speed_order = all_active.iter().map(|(side_idx, poke_idx, _speed)| {
        let pokemon = &battle.sides[*side_idx].pokemon[*poke_idx];
        side_idx * battle.sides.len() + pokemon.position
    }).collect();

    // JS: this.battle.fieldEvent('SwitchIn', switchersIn);
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
