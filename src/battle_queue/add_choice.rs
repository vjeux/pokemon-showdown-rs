use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Add one or more action choices and resolve them
    //
    // 	addChoice(choices: ActionChoice | ActionChoice[]) {
    // 		if (!Array.isArray(choices)) choices = [choices];
    // 		for (const choice of choices) {
    // 			const resolvedChoices = this.resolveAction(choice);
    // 			this.list.push(...resolvedChoices);
    // 			for (const resolvedChoice of resolvedChoices) {
    // 				if (resolvedChoice && resolvedChoice.choice === 'move' && resolvedChoice.move.id !== 'recharge') {
    // 					resolvedChoice.pokemon.side.lastSelectedMove = resolvedChoice.move.id;
    // 				}
    // 			}
    // 		}
    // 	}
    //
    // BattleQueue.prototype.addChoice (lines 302-313 in battle-queue.ts)
    // 	addChoice(choices: ActionChoice | ActionChoice[]) {
    // 		if (!Array.isArray(choices)) choices = [choices];
    // 		for (const choice of choices) {
    // 			const resolvedChoices = this.resolveAction(choice);
    // 			this.list.push(...resolvedChoices);
    // 			for (const resolvedChoice of resolvedChoices) {
    // 				if (resolvedChoice && resolvedChoice.choice === 'move' && resolvedChoice.move.id !== 'recharge') {
    // 					resolvedChoice.pokemon.side.lastSelectedMove = resolvedChoice.move.id;
    // 				}
    // 			}
    // 		}
    // 	}
    pub fn add_choice(&mut self, battle: &mut crate::battle::Battle, side_action: &crate::side::ChosenAction, side_idx: usize) {
        use crate::battle_queue::{Action, MoveAction, MoveActionType, SwitchAction, SwitchActionType, TeamAction, FieldAction, FieldActionType};

        // Convert ChosenAction to a basic Action
        // This action will be passed to resolve_action which will expand it
        let basic_action = match side_action.choice {
            crate::side::ChoiceType::Move => {
                if let Some(ref move_id) = side_action.move_id {
                    let pokemon_idx = battle.sides[side_idx]
                        .active
                        .get(side_action.pokemon_index)
                        .and_then(|opt| *opt)
                        .unwrap_or(0);

                    Action::Move(MoveAction {
                        choice: MoveActionType::Move,
                        sub_order: 0,
                        effect_order: 0,
                        side_index: side_idx,
                        pokemon_index: pokemon_idx,
                        move_id: move_id.clone(),
                        target_loc: side_action.target_loc.unwrap_or(0),
                        original_target: None,
                        mega: side_action.mega,
                        zmove: side_action.zmove.clone(),
                        max_move: side_action.max_move.clone(),
                        source_effect: None,
                        terastallize: side_action.terastallize.clone(),
                        move_priority_modified: None,
                        prankster_boosted: false,
                        priority: 0,
                        fractional_priority: 0.0,
                        speed: 0.0,
                        order: 200,
                    })
                } else {
                    return; // No move specified
                }
            }
            crate::side::ChoiceType::Switch => {
                if let Some(switch_to) = side_action.switch_index {
                    Action::Switch(SwitchAction {
                        choice: SwitchActionType::Switch,
                        sub_order: 0,
                        effect_order: 0,
                        side_index: side_idx,
                        pokemon_index: side_action.pokemon_index,
                        target_index: switch_to,
                        source_effect: None,
                        priority: 0,
                        speed: 0.0,
                        order: 103,
                    })
                } else {
                    return; // No switch target specified
                }
            }
            crate::side::ChoiceType::Team => {
                let priority = -(side_action.switch_index.unwrap_or(0) as i8);
                Action::Team(TeamAction {
                    choice: crate::battle_queue::TeamActionType::Team,
                    priority,
                    sub_order: 0,
                    effect_order: 0,
                    speed: 1.0,
                    pokemon_index: side_action.pokemon_index,
                    side_index: side_idx,
                    index: side_action.pokemon_index,
                })
            }
            crate::side::ChoiceType::Pass => {
                Action::Field(FieldAction {
                    choice: FieldActionType::Pass,
                    sub_order: 0,
                    effect_order: 0,
                    priority: 0,
                })
            }
            crate::side::ChoiceType::InstaSwitch => {
                if let Some(switch_to) = side_action.switch_index {
                    Action::Switch(SwitchAction {
                        choice: SwitchActionType::InstaSwitch,
                        sub_order: 0,
                        effect_order: 0,
                        side_index: side_idx,
                        pokemon_index: side_action.pokemon_index,
                        target_index: switch_to,
                        source_effect: None,
                        priority: 0,
                        speed: 0.0,
                        order: 3,
                    })
                } else {
                    return; // No switch target specified
                }
            }
            crate::side::ChoiceType::RevivalBlessing => {
                if let Some(switch_to) = side_action.switch_index {
                    Action::Switch(SwitchAction {
                        choice: SwitchActionType::RevivalBlessing,
                        sub_order: 0,
                        effect_order: 0,
                        side_index: side_idx,
                        pokemon_index: side_action.pokemon_index,
                        target_index: switch_to,
                        source_effect: None,
                        priority: 0,
                        speed: 0.0,
                        order: 6,
                    })
                } else {
                    return; // No switch target specified
                }
            }
            crate::side::ChoiceType::Shift => {
                return; // Shift is not implemented yet, skip
            }
        };

        // JS: const resolvedChoices = this.resolveAction(choice);
        // Call resolve_action with mid_turn=false (not mid-turn)
        let resolved_actions = BattleQueue::resolve_action(basic_action, battle, false);

        debug_elog!("[ADD_CHOICE] Resolved {} actions for side {}", resolved_actions.len(), side_idx);

        // JS: this.list.push(...resolvedChoices);
        for action in resolved_actions {
            if let Action::Move(ref _m) = action {
                debug_elog!("[ADD_CHOICE] Adding move={} to queue (before: {})", _m.move_id.as_str(), self.list.len());
            }
            self.list.push(action.clone());
            if let Action::Move(ref _m) = action {
                debug_elog!("[ADD_CHOICE] Added move={} to queue (after: {})", _m.move_id.as_str(), self.list.len());
            }

            // JS: if (resolvedChoice && resolvedChoice.choice === 'move' && resolvedChoice.move.id !== 'recharge') {
            // JS:     resolvedChoice.pokemon.side.lastSelectedMove = resolvedChoice.move.id;
            // JS: }
            if let Action::Move(ref move_action) = action {
                if move_action.move_id.as_str() != "recharge" {
                    battle.sides[side_idx].last_selected_move = move_action.move_id.clone();
                }
            }
        }
    }
}
