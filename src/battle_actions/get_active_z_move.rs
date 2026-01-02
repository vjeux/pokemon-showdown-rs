// TODO: Implement getActiveZMove from JavaScript
//
// JS Source:
// 
// 	getActiveZMove(move: Move, pokemon: Pokemon): ActiveMove {
// 		if (pokemon) {
// 			const item = pokemon.getItem();
// 			if (move.name === item.zMoveFrom) {
// 				const zMove = this.dex.getActiveMove(item.zMove as string);
// 				zMove.isZOrMaxPowered = true;
// 				return zMove;
// 			}
// 		}
// 
// 		if (move.category === 'Status') {
// 			const zMove = this.dex.getActiveMove(move);
// 			zMove.isZ = true;
// 			zMove.isZOrMaxPowered = true;
// 			return zMove;
// 		}
// 		const zMove = this.dex.getActiveMove(this.Z_MOVES[move.type]);
// 		zMove.basePower = move.zMove!.basePower!;
// 		zMove.category = move.category;
// 		// copy the priority for Quick Guard
// 		zMove.priority = move.priority;
// 		zMove.isZOrMaxPowered = true;
// 		return zMove;
// 	}

use crate::*;

impl Battle_actions {
    // TODO: Implement this method
}
