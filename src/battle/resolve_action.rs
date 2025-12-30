use crate::*;

impl Battle {

    /// Commit choices and run the turn
    //
    // 	commitChoices() {
    // 		this.updateSpeed();
    //
    // 		// Sometimes you need to make switch choices mid-turn (e.g. U-turn,
    // 		// fainting). When this happens, the rest of the turn is saved (and not
    // 		// re-sorted), but the new switch choices are sorted and inserted before
    // 		// the rest of the turn.
    // 		const oldQueue = this.queue.list;
    // 		this.queue.clear();
    // 		if (!this.allChoicesDone()) throw new Error("Not all choices done");
    //
    // 		for (const side of this.sides) {
    // 			const choice = side.getChoice();
    // 			if (choice) this.inputLog.push(`>${side.id} ${choice}`);
    // 		}
    // 		for (const side of this.sides) {
    // 			this.queue.addChoice(side.choice.actions);
    // 		}
    // 		this.clearRequest();
    //
    // 		this.queue.sort();
    // 		this.queue.list.push(...oldQueue);
    //
    // 		this.requestState = '';
    // 		for (const side of this.sides) {
    // 			side.activeRequest = null;
    // 		}
    //
    // 		this.turnLoop();
    //
    // 		// workaround for tests
    // 		if (this.log.length - this.sentLogPos > 500) this.sendUpdates();
    // 	}
    //
    // BattleQueue.prototype.resolveAction (lines 166-272 in battle-queue.ts)
    // Converts a choice into one or more queue actions
    pub fn resolve_action(&self, side_action: &crate::side::ChosenAction, side_idx: usize) -> Vec<crate::battle_queue::Action> {
        use crate::battle_queue::{Action, MoveAction, MoveActionType, SwitchAction, SwitchActionType, TeamAction, FieldAction, FieldActionType};

        let mut actions = Vec::new();

        match side_action.choice {
            crate::side::ChoiceType::Move => {
                // JS: if (!action.order) { action.order = 200; }
                if let Some(ref move_id) = side_action.move_id {
                    let pokemon_idx = self.sides[side_idx]
                        .active
                        .get(side_action.pokemon_index)
                        .and_then(|opt| *opt)
                        .unwrap_or(0);

                    // JS: action.order = orders[action.choice] || 200
                    // For moves: default order is 200 (line 199 in battle-queue.ts)
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
                        order: 200,  // JS: default order for moves
                    }));
                }
            }
            crate::side::ChoiceType::Switch => {
                // JS: action.order = 103 (line 194 in battle-queue.ts)
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
                // JS: action.order = 1 (line 175 in battle-queue.ts)
                actions.push(Action::Team(TeamAction {
                    priority: 1,
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
                        order: 3,  // JS: instaswitch order = 3
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
                        order: 6,  // JS: revivalblessing order = 6
                    }));
                }
            }
            crate::side::ChoiceType::Shift => {
                // JS: shift order = 200
                // Shift is not implemented yet, skip
            }
        }

        actions
    }
}
