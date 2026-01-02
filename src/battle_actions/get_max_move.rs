// TODO: Implement getMaxMove from JavaScript
//
// JS Source:
// 
// 	getMaxMove(move: Move, pokemon: Pokemon) {
// 		if (typeof move === 'string') move = this.dex.moves.get(move);
// 		if (move.name === 'Struggle') return move;
// 		if (pokemon.gigantamax && pokemon.canGigantamax && move.category !== 'Status') {
// 			const gMaxMove = this.dex.moves.get(pokemon.canGigantamax);
// 			if (gMaxMove.exists && gMaxMove.type === move.type) return gMaxMove;
// 		}
// 		const maxMove = this.dex.moves.get(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
// 		if (maxMove.exists) return maxMove;
// 	}

use crate::*;

impl Battle_actions {
    // TODO: Implement this method
}
