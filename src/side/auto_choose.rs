use crate::side::*;
use crate::Battle;

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
    pub fn auto_choose(&mut self, battle: &mut Battle) -> bool {

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
                    // JavaScript iterates through pokemon array starting at active.length.
                    // Since JavaScript physically reorders the array when switching,
                    // we need to iterate in position order to match.

                    // Collect Pokemon indices, sorted by position
                    let mut indices: Vec<usize> = (0..self.pokemon.len()).collect();
                    indices.sort_by_key(|&i| self.pokemon[i].position);

                    // Find first available switch target starting from active.length
                    let mut found = false;
                    for &i in indices.iter().skip(self.active.len()) {
                        if !self.choice.switch_ins.contains(&i) {
                            if let Some(pokemon) = self.pokemon.get(i) {
                                if !pokemon.is_fainted() {
                                    let _ = self.choose_switch(i);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found {
                        break; // No valid switch found
                    }

                    iterations += 1;
                }
            }
            RequestState::Move => {
                let mut iterations = 0;
                while !self.is_choice_done(None) && iterations < 10 {
                    let index = self.get_choice_index(false);
                    if let Some(Some(pokemon_idx)) = self.active.get(index) {
                        let pokemon_pos = (self.n, *pokemon_idx);
                        let pokemon = &self.pokemon[*pokemon_idx];
                        if pokemon.is_fainted() {
                            self.choose_pass();
                        } else {
                            // JavaScript: Check for locked move FIRST before choosing freely
                            // JavaScript: const lockedMove = pokemon.getLockedMove();
                            // JavaScript: if (lockedMove) { ... use locked move ... }
                            let locked_move = Pokemon::get_locked_move(battle, pokemon_pos);

                            if let Some(move_id) = locked_move {
                                // Pokemon is locked to a specific move (e.g., second turn of two-turn move)
                                // JavaScript: this.choice.actions.push({
                                // JavaScript:     choice: 'move',
                                // JavaScript:     pokemon,
                                // JavaScript:     targetLoc: lockedMoveTargetLoc,
                                // JavaScript:     moveid: lockedMoveID
                                // JavaScript: });

                                // Get target location from volatile data if available
                                let target_loc = {
                                    let pokemon = &self.pokemon[*pokemon_idx];
                                    pokemon.volatiles.get(&move_id)
                                        .and_then(|v| v.data.get("targetLoc"))
                                        .and_then(|t| t.as_i64())
                                        .map(|t| t as i8)
                                        .or_else(|| pokemon.last_move_target_loc)
                                };

                                let _ = self.choose_move(battle, move_id, target_loc, false, None, None, None);
                            } else {
                                // No locked move, choose normally

                                // JavaScript auto_choose uses request.moves, not pokemon.move_slots
                                // JavaScript: if (autoChoose) {
                                // JavaScript:     for (const [i, move2] of request.moves.entries()) {
                                // JavaScript:         if (move2.disabled) continue;
                                // JavaScript:         if (i < moves.length && move2.id === moves[i].id && moves[i].disabled) continue;
                                // JavaScript:         moveid = move2.id;
                                // JavaScript:         targetType = move2.target;
                                // JavaScript:         break;
                                // JavaScript:     }
                                // JavaScript: }

                                // NOTE: We need to reconstruct the Battle to get request data
                                // This is a workaround because auto_choose only has &mut self
                                // We iterate move_slots to match JavaScript which iterates request.moves
                                // Z-moves that are in the moveset ARE included, matching JavaScript

                                let mut found_move = false;
                                for move_slot in &pokemon.move_slots {
                                    // Skip if disabled
                                    if move_slot.disabled {
                                        continue;
                                    }
                                    // Skip if PP is 0
                                    if move_slot.pp == 0 {
                                        continue;
                                    }
                                    // NOTE: Z-moves that are in the Pokemon's base moveset (like "tectonicrage")
                                    // SHOULD be selectable during auto_choose, matching JavaScript behavior.
                                    // The canZMove field is for converting regular moves to Z-moves,
                                    // not for Z-moves already in the moveset!

                                    let move_id = move_slot.id.clone();
                                    let _ = self.choose_move(battle, move_id, None, false, None, None, None);
                                    found_move = true;
                                    break;
                                }
                                if !found_move {
                                    // All moves disabled, use Struggle
                                    let _ = self.choose_move(
                                        battle,
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
                    }
                    iterations += 1;
                }
            }
            RequestState::None => {}
        }
        true
    }
}
