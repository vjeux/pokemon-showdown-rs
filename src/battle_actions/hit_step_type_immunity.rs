use crate::*;
use crate::data::typechart::get_effectiveness_multi;

impl<'a> BattleActions<'a> {

    /// Check type immunity for targets
    /// Equivalent to hitStepTypeImmunity in battle-actions.ts
    // 	hitStepTypeImmunity(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		if (move.ignoreImmunity === undefined) {
    // 			move.ignoreImmunity = (move.category === 'Status');
    // 		}
    //
    // 		const hitResults = [];
    // 		for (const i of targets.keys()) {
    // 			hitResults[i] = targets[i].runImmunity(move, !move.smartTarget);
    // 		}
    //
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_type_immunity(
        defender_types: &[String],
        move_type: &str,
        ignore_immunity: bool,
    ) -> bool {
        if ignore_immunity {
            return true; // Bypass immunity check
        }

        // Check type chart for immunity
        let effectiveness = get_effectiveness_multi(move_type, defender_types);
        effectiveness > 0.0
    }
}
