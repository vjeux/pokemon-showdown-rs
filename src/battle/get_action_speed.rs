use crate::*;
use crate::event::EventResult;
use crate::battle::Effect;

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
        use crate::battle_queue::MoveActionType;

        match action {
            Action::Move(ref mut move_action) => {
                // JS: if (action.choice === 'move')
                // IMPORTANT: Only calculate priority for 'move' choice type, NOT for
                // 'beforeTurnMove' or 'priorityChargeMove'. JavaScript explicitly checks
                // `action.choice === 'move'` before the priority calculation block.
                // priorityChargeMove and beforeTurnMove keep their original priority (0).
                let is_main_move = move_action.choice == MoveActionType::Move;

                if is_main_move {
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
                        if z_move.is_z.is_some() {
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
                        if max_move.is_max.is_some() {
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
                // Pass priority as relay_var so callbacks can modify it (e.g., Grassy Glide adds +1)
                let result =
                    self.single_event("ModifyPriority", &crate::battle::Effect::move_(move_id_ref.clone()), None, Some(pokemon_pos), None, None, Some(EventResult::Number(priority as i32)));
                if let Some(new_priority) = result.number() {
                    priority = new_priority as i8;
                }

                // JS: priority = this.runEvent('ModifyPriority', action.pokemon, null, move, priority);
                // Allows ability/item-based priority modification (e.g., Prankster, Quick Claw)
                // IMPORTANT: We need to temporarily set battle.active_move for callbacks like
                // Prankster that check move.category and set move.pranksterBoosted
                let effect_id = move_action.move_id.clone();

                // Create a temporary ActiveMove for the ModifyPriority event
                // This is needed because ability callbacks like Prankster check battle.active_move
                let temp_active_move = self.dex.get_active_move(move_id.as_str());
                let previous_active_move = self.active_move.take();
                self.active_move = temp_active_move;

                let relay_result = self.run_event(
                "ModifyPriority",
                Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
                    None,
                    Some(&Effect::move_(effect_id)),
                    EventResult::Number(priority as i32),
                    false,
                    false,
                );

                // Check if prankster_boosted was set
                let prankster_boosted = self.active_move.as_ref()
                    .map(|m| m.prankster_boosted)
                    .unwrap_or(false);
                if prankster_boosted {
                    move_action.prankster_boosted = true;
                }

                // Restore previous active_move
                self.active_move = previous_active_move;

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
                } // End of if is_main_move

                // JS: action.speed = action.pokemon.getActionSpeed();
                // Speed is calculated for ALL move action types (move, beforeTurnMove, priorityChargeMove)
                let pokemon_speed = self
                    .get_pokemon_action_speed(move_action.side_index, move_action.pokemon_index);
                move_action.speed = pokemon_speed as f64;
            }
            Action::Switch(ref mut switch_action) => {
                // JS: action.speed = action.pokemon.getActionSpeed()
                // In JS, action.pokemon is a reference to the Pokemon stored at choice time.
                // In Rust, pokemon_index is now the team index (resolved in add_choice.rs),
                // so we can use it directly.
                let pokemon_speed = self
                    .get_pokemon_action_speed(switch_action.side_index, switch_action.pokemon_index);
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
