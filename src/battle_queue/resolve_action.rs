use crate::*;
use crate::battle_queue::*;

impl BattleQueue {
    /// Resolve an action into a list of actions to execute
    /// Equivalent to battle-queue.ts resolveAction() (lines 233-342)
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// resolveAction(action: ActionChoice, midTurn = false): Action[]
    /// ```
    ///
    /// This method takes an action choice and converts it into an ordered list of actions.
    /// For example, a move action with mega evolution becomes two actions:
    /// 1. megaEvo action (order 104)
    /// 2. move action (order 200)
    ///
    /// Parameters:
    /// - action: The action to resolve
    /// - battle: Mutable reference to battle (needed for runEvent, getActionSpeed, etc.)
    /// - mid_turn: Whether this is mid-turn resolution (skips beforeTurnMove/mega/etc.)
    ///
    /// Returns: Vec of actions to add to the queue
    pub fn resolve_action(
        action: Action,
        battle: &mut Battle,
        mid_turn: bool,
    ) -> Vec<Action> {
        // if (!action) throw new Error(`Action not passed to resolveAction`);
        // if (action.choice === 'pass') return [];
        match &action {
            Action::Field(field_action) if field_action.choice == FieldActionType::Pass => {
                return vec![];
            }
            _ => {}
        }

        let mut actions = vec![action.clone()];

        // Build list of actions to prepend (in reverse order since we'll insert at position 0)
        let mut actions_to_prepend: Vec<Action> = vec![];

        // Process based on action type
        match &action {
            Action::Move(move_action) => {
                // if (!midTurn) {
                if !mid_turn {
                    // if (action.choice === 'move') {
                    if move_action.choice == MoveActionType::Move {
                        let pokemon_pos = (move_action.side_index, move_action.pokemon_index);

                        // Get move data
                        let (has_before_turn_callback, has_priority_charge_callback, is_max, is_z) = {
                            let move_id_str = move_action.move_id.as_str();
                            (
                                crate::data::move_callbacks::has_before_turn_callback(move_id_str),
                                crate::data::move_callbacks::has_priority_charge_callback(move_id_str),
                                move_action.max_move.is_some(),
                                move_action.zmove.is_some(),
                            )
                        };

                        // if (!action.maxMove && !action.zmove && action.move.priorityChargeCallback) {
                        //     actions.unshift(...this.resolveAction({
                        //         choice: 'priorityChargeMove',
                        //         pokemon: action.pokemon,
                        //         move: action.move,
                        //     }));
                        // }
                        if !is_max && !is_z && has_priority_charge_callback {
                            let priority_charge_action = Action::Move(MoveAction {
                                choice: MoveActionType::PriorityChargeMove,
                                order: 107,
                                priority: move_action.priority,
                                fractional_priority: 0.0,
                                speed: move_action.speed,
                                sub_order: 0,
                                effect_order: 0,
                                pokemon_index: move_action.pokemon_index,
                                side_index: move_action.side_index,
                                target_loc: move_action.target_loc,
                                original_target: move_action.original_target,
                                move_id: move_action.move_id.clone(),
                                mega: false,
                                zmove: None,
                                max_move: None,
                                source_effect: None,
                                terastallize: None,
                                move_priority_modified: None,
                            });
                            actions_to_prepend.push(priority_charge_action);
                        }

                        // if (action.maxMove && !action.pokemon.volatiles['dynamax']) {
                        //     actions.unshift(...this.resolveAction({
                        //         choice: 'runDynamax',
                        //         pokemon: action.pokemon,
                        //     }));
                        // }
                        if is_max {
                            let has_dynamax = {
                                let pokemon = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1);
                                pokemon
                                    .map(|p| p.volatiles.contains_key(&ID::from("dynamax")))
                                    .unwrap_or(false)
                            };
                            if !has_dynamax {
                                let dynamax_action = Action::Pokemon(PokemonAction {
                                    choice: PokemonActionType::RunDynamax,
                                    order: 105,
                                    priority: 0,
                                    speed: move_action.speed,
                                    sub_order: 0,
                                    effect_order: 0,
                                    pokemon_index: move_action.pokemon_index,
                                    side_index: move_action.side_index,
                                    event: None,
                                    dragger: None,
                                });
                                actions_to_prepend.push(dynamax_action);
                            }
                        }

                        // if (action.terastallize && !action.pokemon.terastallized) {
                        //     actions.unshift(...this.resolveAction({
                        //         choice: 'terastallize',
                        //         pokemon: action.pokemon,
                        //     }));
                        // }
                        let is_terastallized = {
                            let pokemon = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1);
                            pokemon.map(|p| p.terastallized.is_some()).unwrap_or(false)
                        };
                        if move_action.terastallize.is_some() && !is_terastallized {
                            let tera_action = Action::Pokemon(PokemonAction {
                                choice: PokemonActionType::Terastallize,
                                order: 106,
                                priority: 0,
                                speed: move_action.speed,
                                sub_order: 0,
                                effect_order: 0,
                                pokemon_index: move_action.pokemon_index,
                                side_index: move_action.side_index,
                                event: None,
                                dragger: None,
                            });
                            actions_to_prepend.push(tera_action);
                        }

                        // if (action.mega && !action.pokemon.isSkyDropped()) {
                        //     actions.unshift(...this.resolveAction({
                        //         choice: 'megaEvo',
                        //         pokemon: action.pokemon,
                        //     }));
                        // }
                        if move_action.mega && !Pokemon::is_sky_dropped(battle, pokemon_pos) {
                            let mega_action = Action::Pokemon(PokemonAction {
                                choice: PokemonActionType::MegaEvo,
                                order: 104,
                                priority: 0,
                                speed: move_action.speed,
                                sub_order: 0,
                                effect_order: 0,
                                pokemon_index: move_action.pokemon_index,
                                side_index: move_action.side_index,
                                event: None,
                                dragger: None,
                            });
                            actions_to_prepend.push(mega_action);
                        }

                        // if (!action.maxMove && !action.zmove && action.move.beforeTurnCallback) {
                        //     actions.unshift(...this.resolveAction({
                        //         choice: 'beforeTurnMove', pokemon: action.pokemon, move: action.move, targetLoc: action.targetLoc,
                        //     }));
                        // }
                        if !is_max && !is_z && has_before_turn_callback {
                            let before_turn_action = Action::Move(MoveAction {
                                choice: MoveActionType::BeforeTurnMove,
                                order: 5, // beforeTurnMove order
                                priority: move_action.priority,
                                fractional_priority: 0.0,
                                speed: move_action.speed,
                                sub_order: 0,
                                effect_order: 0,
                                pokemon_index: move_action.pokemon_index,
                                side_index: move_action.side_index,
                                target_loc: move_action.target_loc,
                                original_target: move_action.original_target,
                                move_id: move_action.move_id.clone(),
                                mega: false,
                                zmove: None,
                                max_move: None,
                                source_effect: None,
                                terastallize: None,
                                move_priority_modified: None,
                            });
                            actions_to_prepend.push(before_turn_action);
                        }

                        // Insert all prepended actions in reverse order (so they appear in correct order)
                        for prepended_action in actions_to_prepend.into_iter().rev() {
                            actions.insert(0, prepended_action);
                        }

                        // Calculate fractional priority for the main move action
                        // action.fractionalPriority = this.battle.runEvent('FractionalPriority', action.pokemon, null, action.move, 0);
                        let fractional_priority = {
                            let move_id = move_action.move_id.clone();
                            let result = battle.run_event_float(
                                "FractionalPriority",
                                Some(pokemon_pos),
                                None,
                                Some(&move_id),
                                Some(0.0),
                            );
                            match result { Some(f) => f, None => 0.0 }
                        };

                        // Update the main action's fractional priority (it's now at the end of the actions list)
                        if let Some(Action::Move(main_action)) = actions.last_mut() {
                            main_action.fractional_priority = fractional_priority;
                        }
                    }
                }

                // Handle target location resolution
                // This operates on the LAST action (the main move action), not the prepended ones
                if let Some(Action::Move(move_action)) = actions.last_mut() {
                    // if (!action.targetLoc) {
                    //     target = this.battle.getRandomTarget(action.pokemon, action.move);
                    //     if (target) action.targetLoc = action.pokemon.getLocOf(target);
                    // }
                    // action.originalTarget = action.pokemon.getAtLoc(action.targetLoc);
                    if move_action.target_loc == 0 {
                        let pokemon_pos = (move_action.side_index, move_action.pokemon_index);

                        // Extract move target before borrowing battle mutably
                        let move_target = battle.dex.moves().get_by_id(&move_action.move_id)
                            .map(|m| m.target.clone());

                        if let Some(target_str) = move_target {
                            // Get random target
                            let target = battle.get_random_target(
                                pokemon_pos.0,
                                pokemon_pos.1,
                                &target_str,
                            );
                            if let Some(target) = target {
                                // Get target location relative to pokemon
                                let target_loc = {
                                    let pokemon = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1);
                                    pokemon.map(|p| p.get_loc_of(target.0, target.1, battle.active_per_half)).unwrap_or(0)
                                };

                                move_action.target_loc = target_loc;
                                move_action.original_target = Some(target);
                            }
                        }
                    }

                    // Set original target based on target_loc
                    if move_action.original_target.is_none() {
                        let pokemon_pos = (move_action.side_index, move_action.pokemon_index);
                        let target_at_loc = {
                            let pokemon = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1);
                            pokemon.and_then(|p| p.get_at_loc(move_action.target_loc, battle.active_per_half))
                        };
                        move_action.original_target = target_at_loc;
                    }
                }
            }
            Action::Switch(switch_action) => {
                // } else if (['switch', 'instaswitch'].includes(action.choice)) {
                //     if (typeof action.pokemon.switchFlag === 'string') {
                //         action.sourceEffect = this.battle.dex.moves.get(action.pokemon.switchFlag as ID) as any;
                //     }
                //     action.pokemon.switchFlag = false;
                // }

                if !mid_turn {
                    let pokemon_pos = (switch_action.side_index, switch_action.pokemon_index);

                    // Get switch flag
                    let switch_flag = {
                        let pokemon = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1);
                        pokemon.and_then(|p| p.switch_flag.clone())
                    };

                    // Set source effect if switchFlag is a non-empty move ID
                    // Need to use actions.last_mut() to get mutable reference
                    if let Some(flag_str) = switch_flag {
                        if !flag_str.is_empty() {
                            if let Some(Action::Switch(switch_action_mut)) = actions.last_mut() {
                                switch_action_mut.source_effect = Some(ID::from(flag_str.as_str()));
                            }
                        }
                    }

                    // Clear switch flag
                    if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                        pokemon.switch_flag = None;
                    }
                }
            }
            _ => {}
        }

        // const deferPriority = this.battle.gen === 7 && action.mega && action.mega !== 'done';
        // if (!deferPriority) this.battle.getActionSpeed(action);
        let defer_priority = match actions.last() {
            Some(Action::Move(move_action)) => battle.gen == 7 && move_action.mega,
            _ => false,
        };

        if !defer_priority {
            // Get speed for each action
            for action in actions.iter_mut() {
                battle.get_action_speed(action);
            }
        }

        actions
    }
}
