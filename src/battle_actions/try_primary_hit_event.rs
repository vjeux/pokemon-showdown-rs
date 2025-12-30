use crate::*;

impl<'a> BattleActions<'a> {

    /// Try primary hit event
    /// Equivalent to tryPrimaryHitEvent in battle-actions.ts
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
    //
    pub fn try_primary_hit_event(
        move_has_primary_effect: bool,
        move_primary_chance: Option<i32>,
        random_value: i32,
    ) -> bool {
        if !move_has_primary_effect {
            return true; // No primary effect to apply
        }

        if let Some(chance) = move_primary_chance {
            random_value % 100 < chance
        } else {
            true // Guaranteed primary effect
        }
    }
}
