use crate::*;
use crate::battle::BattleRequestState;
use crate::battle::PriorityItem;

impl Battle {

    // commitChoices() {
    // 	this.updateSpeed();
    // 	const oldQueue = this.queue.list;
    // 	this.queue.clear();
    // 	if (!this.allChoicesDone()) throw new Error("Not all choices done");
    // 	for (const side of this.sides) {
    // 		const choice = side.getChoice();
    // 		if (choice) this.inputLog.push(`>${side.id} ${choice}`);
    // 	}
    // 	for (const side of this.sides) {
    // 		this.queue.addChoice(side.choice.actions);
    // 	}
    // 	this.clearRequest();
    // 	this.queue.sort();
    // 	this.queue.list.push(...oldQueue);
    // 	this.requestState = '';
    // 	for (const side of this.sides) {
    // 		side.activeRequest = null;
    // 	}
    // 	this.turnLoop();
    // }
    pub fn commit_choices(&mut self) {
        // JS: this.updateSpeed();
        self.update_speed();

        // JS: const oldQueue = this.queue.list;
        let old_queue = self.queue.list.clone();
        debug_elog!("[COMMIT_CHOICES] Old queue has {} actions before clearing", old_queue.len());
        for (_i, action) in old_queue.iter().enumerate() {
            if let crate::battle_queue::Action::Move(_move_action) = action {
                debug_elog!("[COMMIT_CHOICES] old_queue[{}]: Move {} from ({}, {})",
                    i, move_action.move_id.as_str(), move_action.side_index, move_action.pokemon_index);
            }
        }

        // JS: this.queue.clear();
        self.queue.clear();

        // JS: if (!this.allChoicesDone()) throw new Error("Not all choices done");
        if !self.all_choices_done() {
            panic!("Not all choices done");
        }

        // JS: for (const side of this.sides) {
        // JS:     const choice = side.getChoice();
        // JS:     if (choice) this.inputLog.push(`>${side.id} ${choice}`);
        // JS: }
        for side in &self.sides {
            let choice_str = side.get_choice();
            if !choice_str.is_empty() {
                self.input_log.push(format!(">{} {}", side.id_str(), choice_str));
            }
        }

        // JS: for (const side of this.sides) {
        // JS:     this.queue.addChoice(side.choice.actions);
        // JS: }
        // Temporarily take queue to avoid double mutable borrow
        let mut queue = std::mem::take(&mut self.queue);
        for side_idx in 0..self.sides.len() {
            let side_actions = self.sides[side_idx].choice.actions.clone();
            debug_elog!("[COMMIT_CHOICES] Side {} has {} actions", side_idx, side_actions.len());
            for (_action_idx, side_action) in side_actions.iter().enumerate() {
                if let crate::side::ChoiceType::Move = side_action.choice {
                    debug_elog!("[COMMIT_CHOICES] Side {} action[{}]: Move {} targeting {:?}",
                        side_idx, action_idx,
                        side_action.move_id.as_ref().map(|m| m.as_str()).unwrap_or("none"),
                        side_action.target_loc);
                }
                queue.add_choice(self, side_action, side_idx);
            }
        }
        self.queue = queue;

        debug_elog!("[COMMIT_CHOICES] Queue after adding all choices: {} actions", self.queue.list.len());
        for (_i, action) in self.queue.list.iter().enumerate() {
            if let crate::battle_queue::Action::Move(_move_action) = action {
                debug_elog!("[COMMIT_CHOICES] queue[{}]: Move {} from ({}, {})",
                    i, move_action.move_id.as_str(), move_action.side_index, move_action.pokemon_index);
            }
        }

        // JS: this.clearRequest();
        self.clear_request();

        // JS: this.queue.sort();
        // JavaScript's BattleQueue.sort() calls this.battle.speedSort(this.list)
        // Note: getActionSpeed is already called inside resolveAction (line 270 in battle-queue.ts)
        // during add_choice, so we don't need to call it again here - speeds are already set
        let mut list = std::mem::take(&mut self.queue.list);

        self.speed_sort_with_callsite(&mut list, |action| {
            PriorityItem {
                order: Some(action.order()),
                priority: action.priority() as i32,
                fractional_priority: action.fractional_priority(),
                speed: action.speed(),
                sub_order: 0,
                effect_order: 0,
                index: 0,
            }
        }, "commit_choices:queue");

        debug_elog!("[COMMIT_CHOICES] Queue after sorting: list has {} actions, self.queue.list has {} actions", list.len(), self.queue.list.len());
        for (_i, action) in list.iter().enumerate() {
            if let crate::battle_queue::Action::Move(_move_action) = action {
                debug_elog!("[COMMIT_CHOICES] sorted list[{}]: Move {} from ({}, {})",
                    i, move_action.move_id.as_str(), move_action.side_index, move_action.pokemon_index);
            }
        }
        for (_i, action) in self.queue.list.iter().enumerate() {
            if let crate::battle_queue::Action::Move(_move_action) = action {
                debug_elog!("[COMMIT_CHOICES] ORPHAN self.queue.list[{}]: Move {} from ({}, {})",
                    i, move_action.move_id.as_str(), move_action.side_index, move_action.pokemon_index);
            }
        }

        self.queue.list = list;

        // JS: this.queue.list.push(...oldQueue);
        self.queue.list.extend(old_queue);

        // JS: this.requestState = '';
        self.request_state = BattleRequestState::None;

        // JS: for (const side of this.sides) {
        // JS:     side.activeRequest = null;
        // JS: }
        for side in &mut self.sides {
            side.active_request = None;
        }

        // JS: this.turnLoop();
        debug_elog!("[COMMIT_CHOICES] Right before turn_loop, queue has {} actions", self.queue.list.len());
        for (_i, action) in self.queue.list.iter().enumerate() {
            if let crate::battle_queue::Action::Move(_move_action) = action {
                debug_elog!("[COMMIT_CHOICES] final queue[{}]: Move {} from ({}, {})",
                    i, move_action.move_id.as_str(), move_action.side_index, move_action.pokemon_index);
            }
        }
        self.turn_loop();
    }
}
