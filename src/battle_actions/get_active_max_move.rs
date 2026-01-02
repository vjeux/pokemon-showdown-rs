// TODO: Implement getActiveMaxMove from JavaScript
//
// JS Source:
// 
// 	getActiveMaxMove(move: Move, pokemon: Pokemon) {
// 		if (typeof move === 'string') move = this.dex.getActiveMove(move);
// 		if (move.name === 'Struggle') return this.dex.getActiveMove(move);
// 		let maxMove = this.dex.getActiveMove(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
// 		if (move.category !== 'Status') {
// 			if (pokemon.gigantamax && pokemon.canGigantamax) {
// 				const gMaxMove = this.dex.getActiveMove(pokemon.canGigantamax);
// 				if (gMaxMove.exists && gMaxMove.type === move.type) maxMove = gMaxMove;
// 			}
// 			if (!move.maxMove?.basePower) throw new Error(`${move.name} doesn't have a maxMove basePower`);
// 			if (!['gmaxdrumsolo', 'gmaxfireball', 'gmaxhydrosnipe'].includes(maxMove.id)) {
// 				maxMove.basePower = move.maxMove.basePower;
// 			}
// 			maxMove.category = move.category;
// 		}
// 		maxMove.baseMove = move.id;
// 		// copy the priority for Psychic Terrain, Quick Guard
// 		maxMove.priority = move.priority;
// 		maxMove.isZOrMaxPowered = true;
// 		return maxMove;
// 	}

use crate::*;

impl Battle_actions {
    // TODO: Implement this method
}
