// TODO: Implement deductPP from JavaScript
//
// JS Source:
// 
// 	deductPP(move: string | Move, amount?: number | null, target?: Pokemon | null | false) {
// 		const gen = this.battle.gen;
// 		move = this.battle.dex.moves.get(move);
// 		const ppData = this.getMoveData(move);
// 		if (!ppData) return 0;
// 		ppData.used = true;
// 		if (!ppData.pp && gen > 1) return 0;
// 
// 		if (!amount) amount = 1;
// 		ppData.pp -= amount;
// 		if (ppData.pp < 0 && gen > 1) {
// 			amount += ppData.pp;
// 			ppData.pp = 0;
// 		}
// 		return amount;
// 	}

use crate::*;

impl Pokemon {
    // TODO: Implement this method
}
