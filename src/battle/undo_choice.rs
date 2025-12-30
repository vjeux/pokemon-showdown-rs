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
