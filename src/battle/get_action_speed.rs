use crate::*;
use crate::event::EventResult;

impl Battle {

    /// Get action speed for sorting the action queue
    /// Equivalent to battle.ts getActionSpeed() (battle.ts:2590-2627)
    ///
    //
    // 	getActionSpeed(action: AnyObject) {
    // 		if (action.choice === 'move') {
    // 			let move = action.move;
    // 			if (action.zmove) {
    // 				const zMoveName = this.actions.getZMove(action.move, action.pokemon, true);
    // 				if (zMoveName) {
    // 					const zMove = this.dex.getActiveMove(zMoveName);
    // 					if (zMove.exists && zMove.isZ) {
    // 						move = zMove;
    // 					}
    // 				}
    // 			}
    // 			if (action.maxMove) {
    // 				const maxMoveName = this.actions.getMaxMove(action.maxMove, action.pokemon);
    // 				if (maxMoveName) {
    // 					const maxMove = this.actions.getActiveMaxMove(action.move, action.pokemon);
    // 					if (maxMove.exists && maxMove.isMax) {
    // 						move = maxMove;
    // 					}
    // 				}
    // 			}
    // 			// take priority from the base move, so abilities like Prankster only apply once
    // 			// (instead of compounding every time `getActionSpeed` is called)
    // 			let priority = this.dex.moves.get(move.id).priority;
    // 			// Grassy Glide priority
    // 			priority = this.singleEvent('ModifyPriority', move, null, action.pokemon, null, null, priority);
    // 			priority = this.runEvent('ModifyPriority', action.pokemon, null, move, priority);
    // 			action.priority = priority + action.fractionalPriority;
    // 			// In Gen 6, Quick Guard blocks moves with artificially enhanced priority.
    // 			if (this.gen > 5) action.move.priority = priority;
    // 		}
    //
    // 		if (!action.pokemon) {
    // 			action.speed = 1;
    // 		} else {
    // 			action.speed = action.pokemon.getActionSpeed();
    // 		}
    // 	}
    //
    pub fn get_action_speed(&mut self, action: &mut crate::battle_queue::Action) {
        use crate::battle_queue::Action;

        match action {
            Action::Move(ref mut move_action) => {
                // JS: if (action.choice === 'move')

                // Get the move (considering Z-Move/Max Move transformations)
                let mut move_id = move_action.move_id.clone();

                // JS: if (action.zmove) {
                //         const zMoveName = this.actions.getZMove(action.move, action.pokemon, true);
                //         if (zMoveName) {
                //             const zMove = this.dex.getActiveMove(zMoveName);
                //             if (zMove.exists && zMove.isZ) {
                //                 move = zMove;
                //             }
                //         }
                //     }
                if let Some(ref zmove_name) = move_action.zmove {
                    // Z-Move transformation: get the active Z-Move
                    if let Some(z_move) = self.dex.get_active_move(zmove_name) {
                        if z_move.is_z {
                            // Use Z-Move for priority calculation
                            move_id = z_move.id;
                        }
                    }
                }

                // JS: if (action.maxMove) {
                //         const maxMoveName = this.actions.getMaxMove(action.maxMove, action.pokemon);
                //         if (maxMoveName) {
                //             const maxMove = this.actions.getActiveMaxMove(action.move, action.pokemon);
                //             if (maxMove.exists && maxMove.isMax) {
                //                 move = maxMove;
                //             }
                //         }
                //     }
                if let Some(ref max_move_name) = move_action.max_move {
                    // Max Move transformation: get the active Max Move
                    if let Some(max_move) = self.dex.get_active_move(max_move_name) {
                        if max_move.is_max {
                            // Use Max Move for priority calculation
                            move_id = max_move.id;
                        }
                    }
                }

                // JS: let priority = this.dex.moves.get(move.id).priority;
                // Get base priority from move data in Dex (from potentially transformed move)
                let mut priority = if let Some(move_data) = self.dex.moves().get_by_id(&move_id) {
                    move_data.priority
                } else {
                    // Fallback to action priority if move not found in Dex
                    move_action.priority
                };

                // JS: priority = this.singleEvent('ModifyPriority', move, null, action.pokemon, null, null, priority);
                // Allows move-specific priority modification (e.g., Grassy Glide in Grassy Terrain)
                let move_id_ref = &move_action.move_id.clone();
                let pokemon_pos = (move_action.side_index, move_action.pokemon_index);
                let result =
                    self.single_event("ModifyPriority", move_id_ref, Some(pokemon_pos), None, None, None);
                if let Some(new_priority) = result.number() {
                    priority = new_priority as i8;
                }

                // JS: priority = this.runEvent('ModifyPriority', action.pokemon, null, move, priority);
                // Allows ability/item-based priority modification (e.g., Prankster, Quick Claw)
                let effect_id = move_action.move_id.clone();
                let relay_result = self.run_event(
                    "ModifyPriority",
                    Some(pokemon_pos),
                    None,
                    Some(&effect_id),
                    EventResult::Number(priority as i32),
                    false,
                    false,
                );
                if let EventResult::Number(modified_priority) = relay_result {
                    priority = modified_priority as i8;
                }

                // JS: action.priority = priority + action.fractionalPriority;
                // Note: fractionalPriority is already applied when action is created
                // Just ensure priority is set correctly
                move_action.priority = priority;

                // JS: if (this.gen > 5) action.move.priority = priority;
                // In Gen 6+, Quick Guard checks if move priority was artificially enhanced
                // Store the modified priority value for the move itself
                if self.gen > 5 {
                    move_action.move_priority_modified = Some(priority);
                }

                // JS: action.speed = action.pokemon.getActionSpeed();
                let pokemon_speed = self
                    .get_pokemon_action_speed(move_action.side_index, move_action.pokemon_index);
                move_action.speed = pokemon_speed as f64;
            }
            Action::Switch(ref mut switch_action) => {
                // JS: if (!action.pokemon) { action.speed = 1; }
                // For switches, get the pokemon's speed
                let pokemon_speed = self.get_pokemon_action_speed(
                    switch_action.side_index,
                    switch_action.pokemon_index,
                );
                switch_action.speed = pokemon_speed as f64;
            }
            Action::Pokemon(ref mut poke_action) => {
                // Get pokemon speed for pokemon actions
                let pokemon_speed = self
                    .get_pokemon_action_speed(poke_action.side_index, poke_action.pokemon_index);
                poke_action.speed = pokemon_speed as f64;
            }
            Action::Team(ref mut team_action) => {
                // JS: if (!action.pokemon) { action.speed = 1; } else { action.speed = action.pokemon.getActionSpeed(); }
                // Team actions have a pokemon, so they get the pokemon's speed
                let pokemon_speed = self
                    .get_pokemon_action_speed(team_action.side_index, team_action.pokemon_index);
                team_action.speed = pokemon_speed as f64;
            }
            _ => {
                // Only Field actions don't have speed
            }
        }
    }
}
