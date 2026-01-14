use crate::*;
use crate::battle::Effect;
use crate::battle_queue::*;

impl BattleQueue {
    /// Resolve an action into a list of actions to execute
    /// Equivalent to battle-queue.ts resolveAction() (lines 166-272)
    ///
    // /**
    //  * Takes an ActionChoice, and fills it out into a full Action object.
    //  *
    //  * Returns an array of Actions because some ActionChoices (like mega moves)
    //  * resolve to two Actions (mega evolution + use move)
    //  */
    // resolveAction(action: ActionChoice, midTurn = false): Action[] {
    //     if (!action) throw new Error(`Action not passed to resolveAction`);
    //     if (action.choice === 'pass') return [];
    //     const actions = [action];
    //
    //     if (!action.side && action.pokemon) action.side = action.pokemon.side;
    //     if (!action.move && action.moveid) action.move = this.battle.dex.getActiveMove(action.moveid);
    //     if (!action.order) {
    //         const orders: { [choice: string]: number } = {
    //             team: 1,
    //             start: 2,
    //             instaswitch: 3,
    //             beforeTurn: 4,
    //             beforeTurnMove: 5,
    //             revivalblessing: 6,
    //
    //             runSwitch: 101,
    //             switch: 103,
    //             megaEvo: 104,
    //             megaEvoX: 104,
    //             megaEvoY: 104,
    //             runDynamax: 105,
    //             terastallize: 106,
    //             priorityChargeMove: 107,
    //
    //             shift: 200,
    //             // default is 200 (for moves)
    //
    //             residual: 300,
    //         };
    //         if (action.choice in orders) {
    //             action.order = orders[action.choice];
    //         } else {
    //             action.order = 200;
    //             if (!['move', 'event'].includes(action.choice)) {
    //                 throw new Error(`Unexpected orderless action ${action.choice}`);
    //             }
    //         }
    //     }
    //     if (!midTurn) {
    //         if (action.choice === 'move') {
    //             if (!action.maxMove && !action.zmove && action.move.beforeTurnCallback) {
    //                 actions.unshift(...this.resolveAction({
    //                     choice: 'beforeTurnMove', pokemon: action.pokemon, move: action.move, targetLoc: action.targetLoc,
    //                 }));
    //             }
    //             if (action.mega && !action.pokemon.isSkyDropped()) {
    //                 actions.unshift(...this.resolveAction({
    //                     choice: 'megaEvo',
    //                     pokemon: action.pokemon,
    //                 }));
    //             }
    //             if (action.megax && !action.pokemon.isSkyDropped()) {
    //                 actions.unshift(...this.resolveAction({
    //                     choice: 'megaEvoX',
    //                     pokemon: action.pokemon,
    //                 }));
    //             }
    //             if (action.megay && !action.pokemon.isSkyDropped()) {
    //                 actions.unshift(...this.resolveAction({
    //                     choice: 'megaEvoY',
    //                     pokemon: action.pokemon,
    //                 }));
    //             }
    //             if (action.terastallize && !action.pokemon.terastallized) {
    //                 actions.unshift(...this.resolveAction({
    //                     choice: 'terastallize',
    //                     pokemon: action.pokemon,
    //                 }));
    //             }
    //             if (action.maxMove && !action.pokemon.volatiles['dynamax']) {
    //                 actions.unshift(...this.resolveAction({
    //                     choice: 'runDynamax',
    //                     pokemon: action.pokemon,
    //                 }));
    //             }
    //             if (!action.maxMove && !action.zmove && action.move.priorityChargeCallback) {
    //                 actions.unshift(...this.resolveAction({
    //                     choice: 'priorityChargeMove',
    //                     pokemon: action.pokemon,
    //                     move: action.move,
    //                 }));
    //             }
    //             action.fractionalPriority = this.battle.runEvent('FractionalPriority', action.pokemon, null, action.move, 0);
    //         } else if (['switch', 'instaswitch'].includes(action.choice)) {
    //             if (typeof action.pokemon.switchFlag === 'string') {
    //                 action.sourceEffect = this.battle.dex.moves.get(action.pokemon.switchFlag as ID) as any;
    //             }
    //             action.pokemon.switchFlag = false;
    //         }
    //     }
    //
    //     const deferPriority = this.battle.gen === 7 && action.mega && action.mega !== 'done';
    //     if (action.move) {
    //         let target = null;
    //         action.move = this.battle.dex.getActiveMove(action.move);
    //
    //         if (!action.targetLoc) {
    //             target = this.battle.getRandomTarget(action.pokemon, action.move);
    //             // TODO: what actually happens here?
    //             if (target) action.targetLoc = action.pokemon.getLocOf(target);
    //         }
    //         action.originalTarget = action.pokemon.getAtLoc(action.targetLoc);
    //     }
    //     if (!deferPriority) this.battle.getActionSpeed(action);
    //     return actions as any;
    // }
    ///
    /// Returns: Vec of actions to add to the queue
    pub fn resolve_action(
        mut action: Action,
        battle: &mut Battle,
        mid_turn: bool,
    ) -> Vec<Action> {
        // Debug: trace what action is being resolved
        if let Action::Move(ref m) = action {
            eprintln!("[RESOLVE_ACTION] Entry: move={}, choice={:?}, mid_turn={}", m.move_id.as_str(), m.choice, mid_turn);
        }

        // JS: if (action.choice === 'pass') return [];
        if let Action::Field(ref field_action) = action {
            if field_action.choice == FieldActionType::Pass {
                return vec![];
            }
        }

        // JS: const actions = [action];
        // We'll build the actions list and unshift additional actions at the end
        let mut prefix_actions: Vec<Action> = Vec::new();

        // JS: if (!action.order) { ... }
        // Set the order based on choice type if not already set
        match &mut action {
            Action::Move(ref mut move_action) => {
                // JS: default is 200 for moves
                if move_action.order == 0 {
                    match move_action.choice {
                        MoveActionType::Move => move_action.order = 200,
                        MoveActionType::BeforeTurnMove => move_action.order = 5,
                        MoveActionType::PriorityChargeMove => move_action.order = 107,
                    }
                }
            }
            Action::Switch(ref mut switch_action) => {
                if switch_action.order == 0 {
                    match switch_action.choice {
                        SwitchActionType::Switch => switch_action.order = 103,
                        SwitchActionType::InstaSwitch => switch_action.order = 3,
                        SwitchActionType::RevivalBlessing => switch_action.order = 6,
                    }
                }
            }
            Action::Team(_) => {
                // Order is always 1 for team actions (set via order() method)
            }
            Action::Field(ref field_action) => {
                // Order is set via order() method based on choice type
                // start: 2, beforeTurn: 4, pass: 200, residual: 300
                match field_action.choice {
                    FieldActionType::Start => {} // order 2
                    FieldActionType::BeforeTurn => {} // order 4
                    FieldActionType::Pass => {} // order 200
                    FieldActionType::Residual => {} // order 300
                }
            }
            Action::Pokemon(ref mut poke_action) => {
                if poke_action.order == 0 {
                    match poke_action.choice {
                        PokemonActionType::Start => poke_action.order = 2,
                        PokemonActionType::BeforeTurn => poke_action.order = 4,
                        PokemonActionType::RunSwitch => poke_action.order = 101,
                        PokemonActionType::MegaEvo
                        | PokemonActionType::MegaEvoX
                        | PokemonActionType::MegaEvoY => poke_action.order = 104,
                        PokemonActionType::RunDynamax => poke_action.order = 105,
                        PokemonActionType::Terastallize => poke_action.order = 106,
                        PokemonActionType::Shift => poke_action.order = 200,
                        PokemonActionType::Event => poke_action.order = 200,
                        PokemonActionType::Residual => poke_action.order = 300,
                    }
                }
            }
        }

        // JS: if (!midTurn) { ... }
        if !mid_turn {
            if let Action::Move(ref mut move_action) = action {
                let pokemon_pos = (move_action.side_index, move_action.pokemon_index);
                let move_id = move_action.move_id.clone();

                // Get the move data for checking callbacks
                let active_move = battle.dex.get_active_move(move_id.as_str());

                // IMPORTANT: Only add sub-actions (beforeTurnMove, priorityChargeMove, mega, etc.)
                // when the choice is specifically MoveActionType::Move. Otherwise we'd create
                // infinite recursion when resolving beforeTurnMove/priorityChargeMove actions.
                // JavaScript checks: if (action.choice === 'move')
                let is_main_move_action = move_action.choice == MoveActionType::Move;

                // JS: if (!action.maxMove && !action.zmove && action.move.beforeTurnCallback) {
                //         actions.unshift(...this.resolveAction({
                //             choice: 'beforeTurnMove', pokemon: action.pokemon, move: action.move, targetLoc: action.targetLoc,
                //         }));
                //     }
                if is_main_move_action
                    && move_action.max_move.is_none()
                    && move_action.zmove.is_none()
                    && crate::data::move_callbacks::has_before_turn_callback(active_move.as_ref())
                {
                    let before_turn_action = Action::Move(MoveAction {
                        choice: MoveActionType::BeforeTurnMove,
                        order: 5,
                        priority: 0,
                        fractional_priority: 0.0,
                        speed: 0.0,
                        sub_order: 0,
                        effect_order: 0,
                        side_index: move_action.side_index,
                        pokemon_index: move_action.pokemon_index,
                        target_loc: move_action.target_loc,
                        original_target: None,
                        move_id: move_action.move_id.clone(),
                        mega: false,
                        zmove: None,
                        max_move: None,
                        source_effect: None,
                        terastallize: None,
                        move_priority_modified: None,
                        prankster_boosted: false,
                    });
                    let resolved = BattleQueue::resolve_action(before_turn_action, battle, mid_turn);
                    prefix_actions.extend(resolved);
                }

                // JS: if (action.mega && !action.pokemon.isSkyDropped()) {
                //         actions.unshift(...this.resolveAction({
                //             choice: 'megaEvo',
                //             pokemon: action.pokemon,
                //         }));
                //     }
                if is_main_move_action && move_action.mega && !Pokemon::is_sky_dropped(battle, pokemon_pos) {
                    let mega_action = Action::Pokemon(PokemonAction {
                        choice: PokemonActionType::MegaEvo,
                        order: 104,
                        priority: 0,
                        speed: 0.0,
                        sub_order: 0,
                        effect_order: 0,
                        side_index: move_action.side_index,
                        pokemon_index: move_action.pokemon_index,
                        event: None,
                        dragger: None,
                    });
                    let resolved = BattleQueue::resolve_action(mega_action, battle, mid_turn);
                    prefix_actions.extend(resolved);
                }

                // JS: if (action.terastallize && !action.pokemon.terastallized) {
                //         actions.unshift(...this.resolveAction({
                //             choice: 'terastallize',
                //             pokemon: action.pokemon,
                //         }));
                //     }
                if is_main_move_action && move_action.terastallize.is_some() {
                    // Check if the pokemon is already terastallized
                    let is_terastallized = battle
                        .pokemon_at(pokemon_pos.0, pokemon_pos.1)
                        .map(|p| p.terastallized.is_some())
                        .unwrap_or(false);

                    if !is_terastallized {
                        let tera_action = Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::Terastallize,
                            order: 106,
                            priority: 0,
                            speed: 0.0,
                            sub_order: 0,
                            effect_order: 0,
                            side_index: move_action.side_index,
                            pokemon_index: move_action.pokemon_index,
                            event: None,
                            dragger: None,
                        });
                        let resolved = BattleQueue::resolve_action(tera_action, battle, mid_turn);
                        prefix_actions.extend(resolved);
                    }
                }

                // JS: if (action.maxMove && !action.pokemon.volatiles['dynamax']) {
                //         actions.unshift(...this.resolveAction({
                //             choice: 'runDynamax',
                //             pokemon: action.pokemon,
                //         }));
                //     }
                if is_main_move_action && move_action.max_move.is_some() {
                    // Check if the pokemon already has dynamax volatile
                    let has_dynamax = battle
                        .pokemon_at(pokemon_pos.0, pokemon_pos.1)
                        .map(|p| p.has_volatile(&ID::new("dynamax")))
                        .unwrap_or(false);

                    if !has_dynamax {
                        let dynamax_action = Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::RunDynamax,
                            order: 105,
                            priority: 0,
                            speed: 0.0,
                            sub_order: 0,
                            effect_order: 0,
                            side_index: move_action.side_index,
                            pokemon_index: move_action.pokemon_index,
                            event: None,
                            dragger: None,
                        });
                        let resolved = BattleQueue::resolve_action(dynamax_action, battle, mid_turn);
                        prefix_actions.extend(resolved);
                    }
                }

                // JS: if (!action.maxMove && !action.zmove && action.move.priorityChargeCallback) {
                //         actions.unshift(...this.resolveAction({
                //             choice: 'priorityChargeMove',
                //             pokemon: action.pokemon,
                //             move: action.move,
                //         }));
                //     }
                if is_main_move_action
                    && move_action.max_move.is_none()
                    && move_action.zmove.is_none()
                    && crate::data::move_callbacks::has_priority_charge_callback(active_move.as_ref())
                {
                    let priority_charge_action = Action::Move(MoveAction {
                        choice: MoveActionType::PriorityChargeMove,
                        order: 107,
                        priority: 0,
                        fractional_priority: 0.0,
                        speed: 0.0,
                        sub_order: 0,
                        effect_order: 0,
                        side_index: move_action.side_index,
                        pokemon_index: move_action.pokemon_index,
                        target_loc: 0,
                        original_target: None,
                        move_id: move_action.move_id.clone(),
                        mega: false,
                        zmove: None,
                        max_move: None,
                        source_effect: None,
                        terastallize: None,
                        move_priority_modified: None,
                        prankster_boosted: false,
                    });
                    let resolved =
                        BattleQueue::resolve_action(priority_charge_action, battle, mid_turn);
                    prefix_actions.extend(resolved);
                }

                // JS: action.fractionalPriority = this.battle.runEvent('FractionalPriority', action.pokemon, null, action.move, 0);
                // Run FractionalPriority event to get fractional priority modifier
                // IMPORTANT: Only run for main move actions (choice === 'move' in JS),
                // not for beforeTurnMove, priorityChargeMove, etc.
                if is_main_move_action {
                    let effect = Effect::move_(move_id.clone());
                    let frac_result = battle.run_event(
                        "FractionalPriority",
                        Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
                        None,
                        Some(&effect),
                        crate::event::EventResult::Float(0.0),
                        false,
                        false,
                    );
                    // Handle both Float and Number results (fullincense returns Float(-0.1))
                    let frac_priority = match frac_result {
                        crate::event::EventResult::Float(f) => f,
                        crate::event::EventResult::Number(n) => n as f64,
                        _ => 0.0,
                    };
                    if let Action::Move(ref mut m) = action {
                        m.fractional_priority = frac_priority;
                    }
                }
            } else if let Action::Switch(ref mut switch_action) = action {
                // JS: else if (['switch', 'instaswitch'].includes(action.choice)) {
                //         if (typeof action.pokemon.switchFlag === 'string') {
                //             action.sourceEffect = this.battle.dex.moves.get(action.pokemon.switchFlag as ID) as any;
                //         }
                //         action.pokemon.switchFlag = false;
                //     }
                if switch_action.choice == SwitchActionType::Switch
                    || switch_action.choice == SwitchActionType::InstaSwitch
                {
                    // Note: switch_action.pokemon_index is the active slot index (0 for active[0]),
                    // not the pokemon array index. We need to convert it.
                    let side_idx = switch_action.side_index;
                    let active_slot = switch_action.pokemon_index;

                    // Get the actual pokemon array index from the active slot
                    let actual_poke_idx = battle.sides.get(side_idx)
                        .and_then(|side| side.active.get(active_slot))
                        .and_then(|&opt_idx| opt_idx);

                    if let Some(poke_idx) = actual_poke_idx {
                        // Check if switchFlag is a string (move ID)
                        // JS: if (typeof action.pokemon.switchFlag === 'string')
                        let switch_flag_move = {
                            if let Some(pokemon) = battle.pokemon_at(side_idx, poke_idx) {
                                // switch_flag is Option<String> - Some(move_id) means a move forced the switch
                                pokemon.switch_flag.clone()
                            } else {
                                None
                            }
                        };

                        // JS: action.sourceEffect = this.battle.dex.moves.get(action.pokemon.switchFlag as ID) as any;
                        if let Some(move_id) = switch_flag_move {
                            if !move_id.is_empty() {
                                switch_action.source_effect = Some(Effect::move_(ID::new(&move_id)));
                            }
                        }

                        // JS: action.pokemon.switchFlag = false;
                        if let Some(pokemon) = battle.pokemon_at_mut(side_idx, poke_idx) {
                            pokemon.switch_flag = None;
                        }
                    }
                }
            }
        }

        // JS: const deferPriority = this.battle.gen === 7 && action.mega && action.mega !== 'done';
        let defer_priority = if let Action::Move(ref move_action) = action {
            battle.gen == 7 && move_action.mega
            // Note: In JS, mega can be 'done' after mega evolution completes
            // In Rust, we use a simple bool, so we just check if it's true
            // The 'done' check would need to be handled differently if needed
        } else {
            false
        };

        // JS: if (action.move) { ... }
        // Handle move target resolution
        if let Action::Move(ref mut move_action) = action {
            let pokemon_pos = (move_action.side_index, move_action.pokemon_index);

            // JS: if (!action.targetLoc) {
            //         target = this.battle.getRandomTarget(action.pokemon, action.move);
            //         if (target) action.targetLoc = action.pokemon.getLocOf(target);
            //     }
            if move_action.target_loc == 0 {
                // Get move target type from dex
                let move_target = battle
                    .dex
                    .moves()
                    .get(move_action.move_id.as_str())
                    .map(|m| m.target.clone())
                    .unwrap_or_else(|| "normal".to_string());

                if let Some((target_side, target_idx)) =
                    battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &move_target)
                {
                    // Get the location of the target relative to the user
                    if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        move_action.target_loc =
                            pokemon.get_loc_of(target_side, target_idx, battle.active_per_half);
                    }
                }
            }

            // JS: action.originalTarget = action.pokemon.getAtLoc(action.targetLoc);
            if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                move_action.original_target =
                    pokemon.get_at_loc(move_action.target_loc, battle.active_per_half);
            }
        }

        // JS: if (!deferPriority) this.battle.getActionSpeed(action);
        if !defer_priority {
            battle.get_action_speed(&mut action);
        }

        // JS: return actions as any;
        // Prepend prefix_actions to the main action
        prefix_actions.push(action.clone());

        // Debug: trace what actions are being returned
        if let Action::Move(ref m) = action {
            eprintln!("[RESOLVE_ACTION] Exit: move={}, returning {} actions", m.move_id.as_str(), prefix_actions.len());
            for (i, act) in prefix_actions.iter().enumerate() {
                if let Action::Move(ref ma) = act {
                    eprintln!("[RESOLVE_ACTION]   [{}] move={}, choice={:?}", i, ma.move_id.as_str(), ma.choice);
                }
            }
        }

        prefix_actions
    }
}
