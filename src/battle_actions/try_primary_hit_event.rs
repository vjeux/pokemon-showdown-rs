//! BattleActions::tryPrimaryHitEvent - Fire TryPrimaryHit event for all targets
//!
//! 1:1 port of tryPrimaryHitEvent from battle-actions.ts

// JS Source:
// 	tryPrimaryHitEvent(
// 		damage: SpreadMoveDamage, targets: SpreadMoveTargets, pokemon: Pokemon,
// 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean
// 	): SpreadMoveDamage {
// 		for (const [i, target] of targets.entries()) {
// 			if (!target) continue;
// 			damage[i] = this.battle.runEvent('TryPrimaryHit', target, pokemon, moveData);
// 		}
// 		return damage;
// 	}

use crate::*;
use crate::battle_actions::{SpreadMoveDamage, SpreadMoveDamageValue, SpreadMoveTargets, SpreadMoveTarget};

/// Fire TryPrimaryHit event for all targets
/// Equivalent to tryPrimaryHitEvent() in battle-actions.ts
///
/// JavaScript signature:
/// tryPrimaryHitEvent(damage: SpreadMoveDamage, targets: SpreadMoveTargets, pokemon: Pokemon,
///                    move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean): SpreadMoveDamage
pub fn try_primary_hit_event(
    battle: &mut Battle,
    mut damage: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    pokemon_pos: (usize, usize),
    move_id: &ID,
    _is_secondary: bool,
) -> SpreadMoveDamage {
    // for (const [i, target] of targets.entries()) {
    for (i, target) in targets.iter().enumerate() {
        // if (!target) continue;
        let target_pos = match target {
            SpreadMoveTarget::Target(pos) => *pos,
            _ => continue,
        };

        // damage[i] = this.battle.runEvent('TryPrimaryHit', target, pokemon, moveData);
        let result = battle.run_event(
            "TryPrimaryHit",
            Some(target_pos),
            Some(pokemon_pos),
            Some(move_id),
            None,
        );

        // Convert event result to damage value
        // JavaScript runEvent returns number | boolean | undefined
        // We need to map this to SpreadMoveDamageValue
        // Special case: 0 = Battle.HIT_SUBSTITUTE (substitute blocked the hit)
        if let Some(value) = result {
            if value == Battle::HIT_SUBSTITUTE {
                damage[i] = SpreadMoveDamageValue::HitSubstitute;
            } else {
                damage[i] = SpreadMoveDamageValue::Damage(value);
            }
        }
        // If result is None, keep the existing damage value
    }

    // return damage;
    damage
}
