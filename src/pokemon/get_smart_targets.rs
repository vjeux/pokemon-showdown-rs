use crate::*;

impl Pokemon {

    /// Get smart targets for move
    /// Equivalent to getSmartTargets in pokemon.ts
    // TypeScript source:
    // /** Get targets for Dragon Darts */
    // 	getSmartTargets(target: Pokemon, move: ActiveMove) {
    // 		const target2 = target.adjacentAllies()[0];
    // 		if (!target2 || target2 === this || !target2.hp) {
    // 			move.smartTarget = false;
    // 			return [target];
    // 		}
    // 		if (!target.hp) {
    // 			move.smartTarget = false;
    // 			return [target2];
    // 		}
    // 		return [target, target2];
    // 	}
    //
    pub fn get_smart_targets(
        &self,
        target_side: usize,
        target_pos: usize,
        move_smart_target: bool,
    ) -> Vec<(usize, usize)> {
        // Note: Used for Dragon Darts - hits target and adjacent ally twice

        // Note: JavaScript signature is getSmartTargets(target: Pokemon, move: ActiveMove)
        // Note: Rust takes target position and smartTarget flag instead

        // JS: const target2 = target.adjacentAllies()[0];
        // Note: Missing adjacentAllies() call to find adjacent ally
        // Note: Would need Battle reference to access target Pokemon

        // JS: if (!target2 || target2 === this || !target2.hp) {
        // Note: Missing checks for:
        // Note:   - No adjacent ally exists
        // Note:   - Adjacent ally is self (shouldn't happen but checked)
        // Note:   - Adjacent ally is fainted (hp = 0)

        // JS:     move.smartTarget = false;
        // Note: Missing setting move.smartTarget flag to false

        // JS:     return [target];
        // Note: If no valid second target, return just original target
        // JS: }

        // JS: if (!target.hp) {
        // Note: Missing check if original target is fainted

        // JS:     move.smartTarget = false;
        // Note: Missing setting move.smartTarget flag to false

        // JS:     return [target2];
        // Note: If original target fainted but ally alive, return just the ally
        // JS: }

        // JS: return [target, target2];
        // Note: If both targets valid, return both for Dragon Darts double-hit

        if !move_smart_target {
            return vec![(target_side, target_pos)];
        }

        // Smart targeting redirects to a valid target if original fainted
        // Would need battle context for full implementation
        vec![(target_side, target_pos)]
    }
}
