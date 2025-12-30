use crate::*;
use super::use_move_inner::use_move_inner;


// =========================================================================
// MOVE EXECUTION - useMove and useMoveInner
// These are standalone functions that take &mut Battle as parameter
// Equivalent to battle-actions.ts useMove() and useMoveInner()
// =========================================================================

/// Use a move - wrapper for use_move_inner
/// Equivalent to battle-actions.ts useMove() (lines 368-379)
// TypeScript source:
// /**
// 	 * useMove is the "inside" move caller. It handles effects of the
// 	 * move itself, but not the idea of using the move.
// 	 *
// 	 * Most caller effects, like Sleep Talk, Nature Power, Magic Bounce,
// 	 * etc use useMove.
// 	 *
// 	 * The only ones that use runMove are Instruct, Pursuit, and
// 	 * Dancer.
// 	 */
// 	useMove(
// 		move: Move | string, pokemon: Pokemon, options?: {
// 			target?: Pokemon | null, sourceEffect?: Effect | null,
// 			zMove?: string, maxMove?: string,
// 		}
// 	) {
// 		pokemon.moveThisTurnResult = undefined;
// 		const oldMoveResult: boolean | null | undefined = pokemon.moveThisTurnResult;
// 		const moveResult = this.useMoveInner(move, pokemon, options);
// 		if (oldMoveResult === pokemon.moveThisTurnResult) pokemon.moveThisTurnResult = moveResult;
// 		return moveResult;
// 	}
//
pub fn use_move(
    battle: &mut crate::battle::Battle,
    move_id: &ID,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    source_effect: Option<&ID>,
    z_move: Option<&str>,
    max_move: Option<&str>,
) -> bool {
    // pokemon.moveThisTurnResult = undefined;
    // Note: moveThisTurnResult tracking would go here

    // const oldMoveResult: boolean | null | undefined = pokemon.moveThisTurnResult;
    // const moveResult = this.useMoveInner(move, pokemon, options);

    // if (oldMoveResult === pokemon.moveThisTurnResult) pokemon.moveThisTurnResult = moveResult;
    // Note: moveThisTurnResult syncing would go here

    // return moveResult;
    use_move_inner(
        battle,
        move_id,
        pokemon_pos,
        target_pos,
        source_effect,
        z_move,
        max_move,
    )
}

