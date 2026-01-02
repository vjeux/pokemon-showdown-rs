// TODO: Implement hitStepBreakProtect from JavaScript
//
// JS Source:
// 	hitStepBreakProtect(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
// 		if (move.breaksProtect) {
// 			for (const target of targets) {
// 				let broke = false;
// 				for (const effectid of [
// 					'banefulbunker', 'burningbulwark', 'kingsshield', 'obstruct', 'protect', 'silktrap', 'spikyshield',
// 				]) {
// 					if (target.removeVolatile(effectid)) broke = true;
// 				}
// 				if (this.battle.gen >= 6 || !target.isAlly(pokemon)) {
// 					for (const effectid of ['craftyshield', 'matblock', 'quickguard', 'wideguard']) {
// 						if (target.side.removeSideCondition(effectid)) broke = true;
// 					}
// 				}
// 				if (broke) {
// 					if (move.id === 'feint') {
// 						this.battle.add('-activate', target, 'move: Feint');
// 					} else {
// 						this.battle.add('-activate', target, `move: ${move.name}`, '[broken]');
// 					}
// 					if (this.battle.gen >= 6) delete target.volatiles['stall'];
// 				}
// 			}
// 		}
// 		return undefined;
// 	}

use crate::*;

impl Battle_actions {
    // TODO: Implement this method
}
