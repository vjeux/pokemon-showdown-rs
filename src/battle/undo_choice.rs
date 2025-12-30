use crate::*;

impl Battle {

    /// Undo a player's choice
    /// Equivalent to battle.ts undoChoice()
    //
    // 	undoChoice(sideid: SideID) {
    // 		const side = this.getSide(sideid);
    // 		if (!side.requestState) return;
    //
    // 		if (side.choice.cantUndo) {
    // 			side.emitChoiceError(`Can't undo: A trapping/disabling effect would cause undo to leak information`);
    // 			return;
    // 		}
    //
    // 		let updated = false;
    // 		if (side.requestState === 'move') {
    // 			for (const action of side.choice.actions) {
    // 				const pokemon = action.pokemon;
    // 				if (action.choice !== 'move' || !pokemon) continue;
    // 				if (side.updateRequestForPokemon(pokemon, req => side.updateDisabledRequest(pokemon, req))) {
    // 					updated = true;
    // 				}
    // 			}
    // 		}
    //
    // 		side.clearChoice();
    //
    // 		if (updated) side.emitRequest(side.activeRequest!, true);
    // 	}
    //
    // TODO: INCOMPLETE IMPLEMENTATION - Missing Side methods
    // This is a simplified placeholder. To match TypeScript exactly, need to implement:
    // 1. Side.emitChoiceError() - emit error message to player
    // 2. Side.updateRequestForPokemon() - update request for specific Pokemon
    // 3. Side.updateDisabledRequest() - update disabled moves in request
    // 4. Side.emitRequest() - send updated request to player
    // The current implementation only checks cantUndo and clears choice.
    pub fn undo_choice(&mut self, side_id: SideID) -> bool {
        let side_idx = side_id.index();

        if let Some(side) = self.sides.get_mut(side_idx) {
            if side.choice.cant_undo {
                return false;
            }
            side.choice.clear();
            return true;
        }

        false
    }
}
