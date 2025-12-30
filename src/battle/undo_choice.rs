use crate::*;

impl Battle {

    /// Undo a player's choice
    /// Equivalent to battle.ts undoChoice() (battle.ts:3020-3036, 17 lines)
    ///
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
    pub fn undo_choice(&mut self, side_id: SideID) {
        // JS: const side = this.getSide(sideid);
        let side_idx = side_id.index();

        // Extract necessary data immutably first
        let (cant_undo, is_move_request, pokemon_indices) = {
            let side = match self.sides.get(side_idx) {
                Some(s) => s,
                None => return,
            };

            // JS: if (!side.requestState) return;
            if side.request_state == crate::side::RequestState::None {
                return;
            }

            // JS: if (side.choice.cantUndo)
            let cant_undo = side.choice.cant_undo;

            // JS: if (side.requestState === 'move')
            let is_move_request = side.request_state == crate::side::RequestState::Move;

            // Extract pokemon indices from choice actions for move requests
            let pokemon_indices: Vec<usize> = if is_move_request {
                side.choice.actions.iter()
                    .filter_map(|action| {
                        if matches!(action.choice, crate::side::ChoiceType::Move) {
                            Some(action.pokemon_index)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            };

            (cant_undo, is_move_request, pokemon_indices)
        };

        // JS: if (side.choice.cantUndo) { side.emitChoiceError(...); return; }
        if cant_undo {
            if let Some(side) = self.sides.get(side_idx) {
                side.emit_choice_error("Can't undo: A trapping/disabling effect would cause undo to leak information");
            }
            return;
        }

        // JS: let updated = false;
        // JS: if (side.requestState === 'move') { ... }
        let updated = if is_move_request {
            let mut any_updated = false;

            // JS: for (const action of side.choice.actions) { ... }
            for pokemon_idx in pokemon_indices {
                // JS: if (side.updateRequestForPokemon(pokemon, req => side.updateDisabledRequest(pokemon, req)))
                if let Some(side) = self.sides.get_mut(side_idx) {
                    // Update disabled request for this Pokemon
                    side.update_disabled_request(pokemon_idx);
                    side.update_request_for_pokemon(pokemon_idx);
                    any_updated = true;
                }
            }

            any_updated
        } else {
            false
        };

        // JS: side.clearChoice();
        if let Some(side) = self.sides.get_mut(side_idx) {
            side.choice.clear();
        }

        // JS: if (updated) side.emitRequest(side.activeRequest!, true);
        // TODO: emit_request expects &serde_json::Value but active_request is Option<BattleRequest>
        // Need to either:
        // 1. Store activeRequest as JSON instead of BattleRequest struct, OR
        // 2. Add Serialize derive to BattleRequest and convert here
        // For now, skip emitting since emit_request is a stub anyway
        if updated {
            let _ = updated; // Silence unused warning
            // if let Some(side) = self.sides.get(side_idx) {
            //     if let Some(ref active_request) = side.active_request {
            //         // Need to convert BattleRequest to JSON here
            //         side.emit_request(active_request);
            //     }
            // }
        }
    }
}
