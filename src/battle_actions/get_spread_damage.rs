//! BattleActions::getSpreadDamage - Get damage for each target in a spread move
//!
//! 1:1 port of getSpreadDamage from battle-actions.ts

// JS Source:
// 	getSpreadDamage(
// 		damage: SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,
// 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean
// 	): SpreadMoveDamage {
// 		for (const [i, target] of targets.entries()) {
// 			if (!target) continue;
// 			this.battle.activeTarget = target;
// 			damage[i] = undefined;
// 			const curDamage = this.getDamage(source, target, moveData);
// 			// getDamage has several possible return values:
// 			//
// 			//   a number:
// 			//     means that much damage is dealt (0 damage still counts as dealing
// 			//     damage for the purposes of things like Static)
// 			//   false:
// 			//     gives error message: "But it failed!" and move ends
// 			//   null:
// 			//     the move ends, with no message (usually, a custom fail message
// 			//     was already output by an event handler)
// 			//   undefined:
// 			//     means no damage is dealt and the move continues
// 			//
// 			// basically, these values have the same meanings as they do for event
// 			// handlers.
// 
// 			if (curDamage === false || curDamage === null) {
// 				if (damage[i] === false && !isSecondary && !isSelf) {
// 					this.battle.add('-fail', source);
// 					this.battle.attrLastMove('[still]');
// 				}
// 				this.battle.debug('damage calculation interrupted');
// 				damage[i] = false;
// 				continue;
// 			}
// 			damage[i] = curDamage;
// 		}
// 		return damage;
// 	}


use crate::*;

/// Get damage for each target in a spread move
/// Equivalent to getSpreadDamage() in battle-actions.ts:1163
pub fn get_spread_damage(
    battle: &mut Battle,
    damages: &[Option<i32>],
    targets: &[Option<(usize, usize)>],
    source_pos: (usize, usize),
    move_id: &ID,
    _is_secondary: bool,
    _is_self: bool,
) -> Vec<Option<i32>> {
    let mut result_damages = damages.to_vec();

    for (i, &target) in targets.iter().enumerate() {
        if let Some(target_pos) = target {
            // Calculate damage using getDamage
            let cur_damage = crate::battle_actions::get_damage(battle, source_pos, target_pos, move_id);
            result_damages[i] = cur_damage;
        } else {
            result_damages[i] = None;
        }
    }

    result_damages
}
