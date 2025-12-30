use crate::*;

impl<'a> BattleActions<'a> {

    /// Hit step try hit event
    /// Equivalent to hitStepTryHitEvent in battle-actions.ts
    // 	hitStepTryHitEvent(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const hitResults = this.battle.runEvent('TryHit', targets, pokemon, move);
    // 		if (!hitResults.includes(true) && hitResults.includes(false)) {
    // 			this.battle.add('-fail', pokemon);
    // 			this.battle.attrLastMove('[still]');
    // 		}
    // 		for (const i of targets.keys()) {
    // 			if (hitResults[i] !== this.battle.NOT_FAIL) hitResults[i] = hitResults[i] || false;
    // 		}
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_try_hit_event(
        target_has_substitute: bool,
        move_ignores_substitute: bool,
        move_is_sound: bool,
    ) -> bool {
        // Substitute blocks most moves
        if target_has_substitute && !move_ignores_substitute && !move_is_sound {
            return false; // Hit substitute instead
        }
        true
    }
}
