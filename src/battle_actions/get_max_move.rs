use crate::*;
use super::get_max_move_name;

impl<'a> BattleActions<'a> {

    // =========================================================================
    // MAX MOVE METHODS
    // =========================================================================

    /// Get Max Move for a move
    /// Equivalent to getMaxMove in battle-actions.ts
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
    //
    pub fn get_max_move(move_name: &str, move_type: &str, move_category: &str) -> Option<String> {
        if move_name == "Struggle" {
            return Some("Struggle".to_string());
        }

        let max_type = if move_category == "Status" {
            "Status"
        } else {
            move_type
        };

        Some(get_max_move_name(max_type).to_string())
    }
}
