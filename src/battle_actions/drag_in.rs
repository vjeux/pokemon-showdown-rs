//! BattleActions::dragIn - Drag in a random Pokemon
//!
//! 1:1 port of dragIn from battle-actions.ts

use crate::*;
use crate::battle::SwitchResult;

/// Drag in a random Pokemon (for moves like Whirlwind/Roar)
/// Equivalent to battle-actions.ts dragIn()
/// 1:1 port of dragIn from battle-actions.ts
pub fn drag_in(battle: &mut Battle, side_idx: usize, slot: usize) -> bool {
    // Get a random switchable Pokemon
    let switch_target = match battle.get_random_switchable(side_idx) {
        Some(idx) => idx,
        None => return false,
    };

    // Check if there's an active Pokemon in that slot
    let old_active = battle.sides[side_idx].active.get(slot).copied().flatten();
    if old_active.is_none() {
        return false;
    }

    // Check if the old Pokemon can be dragged out (not fainted)
    let old_poke_idx = old_active.unwrap();
    if battle.sides[side_idx].pokemon[old_poke_idx].hp == 0 {
        return false;
    }

    // Run DragOut event (abilities/items that prevent being dragged out)
    if !battle.run_event_bool("DragOut", Some((side_idx, old_poke_idx)), None, None) {
        return false;
    }

    // Call switchIn with is_drag = true
    matches!(
        crate::battle_actions::switch_in(battle, side_idx, slot, switch_target, None, true),
        SwitchResult::Success
    )
}
