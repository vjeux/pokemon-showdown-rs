use crate::side::*;

impl Side {

    /// Auto-choose remaining actions
    // TypeScript source:
    // /** Automatically finish a choice if not currently complete. */
    // 	autoChoose() {
    // 		if (this.requestState === 'teampreview') {
    // 			if (!this.isChoiceDone()) this.chooseTeam();
    // 		} else if (this.requestState === 'switch') {
    // 			let i = 0;
    // 			while (!this.isChoiceDone()) {
    // 				if (!this.chooseSwitch()) throw new Error(`autoChoose switch crashed: ${this.choice.error}`);
    // 				i++;
    // 				if (i > 10) throw new Error(`autoChoose failed: infinite looping`);
    // 			}
    // 		} else if (this.requestState === 'move') {
    // 			let i = 0;
    // 			while (!this.isChoiceDone()) {
    // 				if (!this.chooseMove()) throw new Error(`autoChoose crashed: ${this.choice.error}`);
    // 				i++;
    // 				if (i > 10) throw new Error(`autoChoose failed: infinite looping`);
    // 			}
    // 		}
    // 		return true;
    // 	}
    //
    pub fn auto_choose(&mut self) -> bool {

        match self.request_state {
            RequestState::TeamPreview => {
                if !self.is_choice_done(None) {
                    let positions: Vec<usize> = (0..self.pokemon.len()).collect();
                    let _ = self.choose_team(positions);
                }
            }
            RequestState::Switch => {
                let mut iterations = 0;
                while !self.is_choice_done(None) && iterations < 10 {
                    // Find first available switch target
                    for i in self.active.len()..self.pokemon.len() {
                        if !self.choice.switch_ins.contains(&i) {
                            if let Some(pokemon) = self.pokemon.get(i) {
                                if !pokemon.is_fainted() {
                                    let _ = self.choose_switch(i);
                                    break;
                                }
                            }
                        }
                    }
                    iterations += 1;
                }
            }
            RequestState::Move => {
                let mut iterations = 0;
                while !self.is_choice_done(None) && iterations < 10 {
                    let index = self.get_choice_index(false);
                    if let Some(Some(pokemon_idx)) = self.active.get(index) {
                        let pokemon = &self.pokemon[*pokemon_idx];
                        if pokemon.is_fainted() {
                            self.choose_pass();
                        } else {
                            // Find first non-disabled move
                            // JavaScript: if (autoChoose) {
                            // JavaScript:     for (const [i, move2] of request.moves.entries()) {
                            // JavaScript:         if (move2.disabled) continue;
                            // JavaScript:         if (i < moves.length && move2.id === moves[i].id && moves[i].disabled) continue;
                            // JavaScript:         moveid = move2.id;
                            // JavaScript:         targetType = move2.target;
                            // JavaScript:         break;
                            // JavaScript:     }
                            // JavaScript: }
                            let mut found_move = false;
                            for move_slot in &pokemon.move_slots {
                                // Skip if disabled
                                if move_slot.disabled {
                                    continue;
                                }
                                // Skip if PP is 0
                                // JavaScript: if ((moveSlot.pp <= 0 && !this.volatiles['partialtrappinglock']) || ...) { disabled = true; }
                                if move_slot.pp == 0 {
                                    continue;
                                }
                                // Skip Z-moves if already used
                                if move_slot.is_z && self.z_move_used {
                                    continue;
                                }

                                let move_id = move_slot.id.clone();
                                // Pass zmove name if this is a Z-move and it hasn't been used
                                let zmove = if move_slot.is_z && !self.z_move_used {
                                    Some(move_slot.move_name.clone())
                                } else {
                                    None
                                };
                                let _ = self.choose_move(move_id, None, false, zmove, None, None);
                                found_move = true;
                                break;
                            }
                            if !found_move {
                                // All moves disabled, use Struggle
                                let _ = self.choose_move(
                                    ID::new("struggle"),
                                    None,
                                    false,
                                    None,
                                    None,
                                    None,
                                );
                            }
                        }
                    }
                    iterations += 1;
                }
            }
            RequestState::None => {}
        }
        true
    }
}
