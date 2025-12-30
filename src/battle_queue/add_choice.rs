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

        // Convert ChosenAction to Action (inline logic from deleted Battle.resolve_action)
        let mut actions = Vec::new();

        match side_action.choice {
            crate::side::ChoiceType::Move => {
                if let Some(ref move_id) = side_action.move_id {
                    let pokemon_idx = battle.sides[side_idx]
                        .active
                        .get(side_action.pokemon_index)
                        .and_then(|opt| *opt)
                        .unwrap_or(0);

                    actions.push(Action::Move(MoveAction {
                        choice: MoveActionType::Move,
                        side_index: side_idx,
                        pokemon_index: pokemon_idx,
                        move_id: move_id.clone(),
                        target_loc: side_action.target_loc.unwrap_or(0),
                        mega: side_action.mega,
                        zmove: side_action.zmove.clone(),
                        max_move: side_action.max_move.clone(),
                        terastallize: side_action.terastallize.clone(),
                        move_priority_modified: None,
                        priority: 0,
                        fractional_priority: 0.0,
                        speed: 0,
                        order: 200,
                    }));
                }
            }
            crate::side::ChoiceType::Switch => {
                if let Some(switch_to) = side_action.switch_index {
                    actions.push(Action::Switch(SwitchAction {
                        choice: SwitchActionType::Switch,
                        side_index: side_idx,
                        pokemon_index: side_action.pokemon_index,
                        target_index: switch_to,
                        source_effect: None,
                        priority: 0,
                        speed: 0,
                        order: 103,
                    }));
                }
            }
            crate::side::ChoiceType::Team => {
                // JavaScript: priority: -index (line 1027 in side.ts)
                // The index is stored in switch_index by choose_team
                let priority = -(side_action.switch_index.unwrap_or(0) as i8);
                actions.push(Action::Team(TeamAction {
                    priority,
                    speed: 1,  // Will be set by get_action_speed
                    pokemon_index: side_action.pokemon_index,
                    side_index: side_idx,
                    index: side_action.pokemon_index,
                }));
            }
            crate::side::ChoiceType::Pass => {
                actions.push(Action::Field(FieldAction {
                    choice: FieldActionType::Pass,
                    priority: 0,
                }));
            }
            crate::side::ChoiceType::InstaSwitch => {
                if let Some(switch_to) = side_action.switch_index {
                    actions.push(Action::Switch(SwitchAction {
                        choice: SwitchActionType::InstaSwitch,
                        side_index: side_idx,
                        pokemon_index: side_action.pokemon_index,
                        target_index: switch_to,
                        source_effect: None,
                        priority: 0,
                        speed: 0,
                        order: 3,
                    }));
                }
            }
            crate::side::ChoiceType::RevivalBlessing => {
                if let Some(switch_to) = side_action.switch_index {
                    actions.push(Action::Switch(SwitchAction {
                        choice: SwitchActionType::RevivalBlessing,
                        side_index: side_idx,
                        pokemon_index: side_action.pokemon_index,
                        target_index: switch_to,
                        source_effect: None,
                        priority: 0,
                        speed: 0,
                        order: 6,
                    }));
                }
            }
            crate::side::ChoiceType::Shift => {
                // Shift is not implemented yet, skip
            }
        }

        // JS: this.list.push(...resolvedChoices);
        for action in actions {
            self.list.push(action.clone());

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
