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
        eprintln!("[COMMIT_CHOICES DEBUG] Called");
        // JS: this.updateSpeed();
        self.update_speed();

        // JS: const oldQueue = this.queue.list;
        let old_queue = self.queue.list.clone();

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
            for side_action in &side_actions {
                queue.add_choice(self, side_action, side_idx);
            }
        }
        self.queue = queue;

        // JS: this.clearRequest();
        self.clear_request();

        // JS: this.queue.sort();
        // JavaScript's BattleQueue.sort() calls this.battle.speedSort(this.list)
        // speedSort gets action speed first via getActionSpeed, then sorts
        eprintln!("[COMMIT_CHOICES DEBUG] Sorting queue");
        let mut list = std::mem::take(&mut self.queue.list);

        // getActionSpeed is called inside resolveAction (line 270 in battle-queue.ts)
        for action in &mut list {
            self.get_action_speed(action);
        }

        // DEBUG: Log all actions before sorting
        eprintln!("[COMMIT_CHOICES DEBUG] Actions before sorting:");
        for (i, action) in list.iter().enumerate() {
            eprintln!("  Action {}: priority={}, speed={}, order={:?}",
                i, action.priority(), action.speed(), action.order());
        }

        eprintln!("[COMMIT_CHOICES DEBUG] Calling speed_sort on {} actions", list.len());
        self.speed_sort(&mut list, |action| {
            PriorityItem {
                order: Some(action.order()),
                priority: action.priority() as i32,
                speed: action.speed(),
                sub_order: 0,
                effect_order: 0,
                index: 0,
            }
        });
        eprintln!("[COMMIT_CHOICES DEBUG] speed_sort done");

        // DEBUG: Log all actions AFTER sorting
        eprintln!("[COMMIT_CHOICES DEBUG] Actions AFTER sorting:");
        for (i, action) in list.iter().enumerate() {
            eprintln!("  Action {}: priority={}, speed={}, order={:?}",
                i, action.priority(), action.speed(), action.order());
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
        eprintln!("[COMMIT_CHOICES DEBUG] About to call turn_loop()");
        self.turn_loop();
    }
}
