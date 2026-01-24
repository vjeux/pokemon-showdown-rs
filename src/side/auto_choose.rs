use crate::side::*;
use crate::Battle;
use crate::ID;

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
                    // Get the current choice index to find which active Pokemon needs a switch choice
                    let index = self.get_choice_index(false);

                    // Check if this active Pokemon has a revival blessing slot condition
                    // JS: if (this.slotConditions[pokemon.position]['revivalblessing']) {
                    //     slot = 0;
                    //     while (!this.pokemon[slot].fainted) slot++;
                    // } else {
                    //     slot = this.active.length;
                    //     while (this.choice.switchIns.has(slot) || this.pokemon[slot].fainted) slot++;
                    // }
                    let pokemon_position = if let Some(&Some(poke_idx)) = self.active.get(index) {
                        if let Some(pokemon) = self.pokemon.get(poke_idx) {
                            pokemon.position
                        } else {
                            index
                        }
                    } else {
                        index
                    };

                    let revivalblessing_id = ID::new("revivalblessing");
                    let has_revivalblessing = self.slot_conditions
                        .get(pokemon_position)
                        .map(|conditions| conditions.contains_key(&revivalblessing_id))
                        .unwrap_or(false);

                    if has_revivalblessing {
                        // RevivalBlessing case: find first fainted Pokemon in POSITION order
                        // JS: slot = 0; while (!this.pokemon[slot].fainted) slot++;
                        // JavaScript physically reorders the pokemon array by position, so we need
                        // to iterate in position order to match.
                        // IMPORTANT: Skip Pokemon at active positions (position < active.len) since
                        // Revival Blessing revives a BENCH Pokemon, not an active one that just fainted.
                        let mut indices: Vec<usize> = (0..self.pokemon.len()).collect();
                        indices.sort_by_key(|&i| self.pokemon[i].position);

                        let mut found = false;
                        for &i in indices.iter() {
                            if let Some(pokemon) = self.pokemon.get(i) {
                                // Skip Pokemon at active positions - they cannot be Revival Blessing targets
                                if pokemon.position < self.active.len() {
                                    continue;
                                }
                                if pokemon.is_fainted() {
                                    let _ = self.choose_switch(i);
                                    found = true;
                                    break;
                                }
                            }
                        }
                        if !found {
                            break; // No fainted Pokemon found for revival
                        }
                    } else {
                        // Normal switch case: find first non-fainted Pokemon starting at active.length
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
                                        .and_then(|v| v.borrow().target_loc)
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
