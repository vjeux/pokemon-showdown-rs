//! BattleActions::dragIn - Drag in a random Pokemon
//!
//! 1:1 port of dragIn from battle-actions.ts

// JS Source:
// 	dragIn(side: Side, pos: number) {
// 		const pokemon = this.battle.getRandomSwitchable(side);
// 		if (!pokemon || pokemon.isActive) return false;
// 		const oldActive = side.active[pos];
// 		if (!oldActive) throw new Error(`nothing to drag out`);
// 		if (!oldActive.hp) return false;
// 
// 		if (!this.battle.runEvent('DragOut', oldActive)) {
// 			return false;
// 		}
// 		if (!this.switchIn(pokemon, pos, null, true)) return false;
// 		return true;
// 	}


use crate::*;
use crate::battle::SwitchResult;

/// Drag in a random Pokemon (for moves like Whirlwind/Roar)
/// Equivalent to battle-actions.ts dragIn()
/// 1:1 port of dragIn from battle-actions.ts
///
/// JavaScript (battle-actions.ts):
///   dragIn(side: Side, pos: number) {
///     const pokemon = this.battle.getRandomSwitchable(side);
///     if (!pokemon || pokemon.isActive) return false;
///     const oldActive = side.active[pos];
///     if (!oldActive) throw new Error(`nothing to drag out`);
///     if (!oldActive.hp) return false;
///
///     if (!this.battle.runEvent('DragOut', oldActive)) {
///       return false;
///     }
///     if (!this.switchIn(pokemon, pos, null, true)) return false;
///     return true;
///   }
pub fn drag_in(battle: &mut Battle, side_idx: usize, slot: usize) -> bool {
    debug_elog!("[DRAG_IN] Entry: side={}, slot={}", side_idx, slot);
    // const pokemon = this.battle.getRandomSwitchable(side);
    // if (!pokemon || pokemon.isActive) return false;
    let switch_target = match battle.get_random_switchable(side_idx) {
        Some(idx) => {
            debug_elog!("[DRAG_IN] Found random switchable: pokemon index {}", idx);
            idx
        }
        None => {
            debug_elog!("[DRAG_IN] No random switchable found, returning false");
            return false;
        }
    };

    // Check if the pokemon is already active
    let is_active = battle.sides[side_idx]
        .active
        .iter()
        .any(|&slot_idx| slot_idx == Some(switch_target));
    debug_elog!("[DRAG_IN] is_active check: {}", is_active);
    if is_active {
        debug_elog!("[DRAG_IN] Pokemon is already active, returning false");
        return false;
    }

    // const oldActive = side.active[pos];
    // if (!oldActive) throw new Error(`nothing to drag out`);
    let old_active = battle.sides[side_idx].active.get(slot).copied().flatten();
    debug_elog!("[DRAG_IN] old_active: {:?}", old_active);
    if old_active.is_none() {
        // JavaScript throws error, but we'll just return false
        debug_elog!("[DRAG_IN] No old active pokemon, returning false");
        return false;
    }

    // if (!oldActive.hp) return false;
    let old_poke_idx = old_active.unwrap();
    let old_poke_hp = battle.sides[side_idx].pokemon[old_poke_idx].hp;
    debug_elog!("[DRAG_IN] old_poke hp: {}", old_poke_hp);
    if old_poke_hp == 0 {
        debug_elog!("[DRAG_IN] Old pokemon has 0 HP, returning false");
        return false;
    }

    // if (!this.battle.runEvent('DragOut', oldActive)) {
    //   return false;
    // }
    debug_elog!("[DRAG_IN] Calling runEvent DragOut for old pokemon");
    if !battle.run_event(
                "DragOut",
                Some(crate::event::EventTarget::Pokemon((side_idx, old_poke_idx))),
        None,
        None,
        crate::event::EventResult::Number(1),
        false,
        false,
    ).is_truthy() {
        debug_elog!("[DRAG_IN] DragOut event returned false, returning false");
        return false;
    }

    // if (!this.switchIn(pokemon, pos, null, true)) return false;
    // return true;
    debug_elog!("[DRAG_IN] Calling switchIn for pokemon {} to slot {}", switch_target, slot);
    let result = crate::battle_actions::switch_in(battle, side_idx, slot, switch_target, None, true);
    debug_elog!("[DRAG_IN] switchIn result: {:?}", result);
    matches!(
        result,
        SwitchResult::Success
    )
}
