// TODO: Implement hitStepTypeImmunity from JavaScript
//
// JS Source:
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

use crate::*;

impl Battle_actions {
    // TODO: Implement this method
}
