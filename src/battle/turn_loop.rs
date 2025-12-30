use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    /// Main turn loop
    /// Equivalent to battle.ts turnLoop()
    // TypeScript source:
    // /**
    // 	 * Generally called at the beginning of a turn, to go through the
    // 	 * turn one action at a time.
    // 	 *
    // 	 * If there is a mid-turn decision (like U-Turn), this will return
    // 	 * and be called again later to resume the turn.
    // 	 */
    // 	turnLoop() {
    // 		this.add('');
    // 		this.add('t:', Math.floor(Date.now() / 1000));
    // 		if (this.requestState) this.requestState = '';
    //
    // 		if (!this.midTurn) {
    // 			this.queue.insertChoice({ choice: 'beforeTurn' });
    // 			this.queue.addChoice({ choice: 'residual' });
    // 			this.midTurn = true;
    // 		}
    //
    // 		let action;
    // 		while ((action = this.queue.shift())) {
    // 			this.runAction(action);
    // 			if (this.requestState || this.ended) return;
    // 		}
    //
    // 		this.endTurn();
    // 		this.midTurn = false;
    // 		this.queue.clear();
    // 	}
    //
    pub fn turn_loop(&mut self) {
        eprintln!("=== TURN {} START, QUEUE LENGTH: {} ===", self.turn + 1, self.queue.list.len());
        self.add("", &[]);

        // Add timestamp (JS: this.add('t:', Math.floor(Date.now() / 1000)))
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.add("t:", &[timestamp.to_string().into()]);

        if self.request_state != BattleRequestState::None {
            self.request_state = BattleRequestState::None;
        }

        if !self.mid_turn {
            // JS: this.queue.insertChoice({ choice: "beforeTurn" });
            // JS: this.queue.addChoice({ choice: "residual" });
            use crate::battle_queue::{Action, FieldAction, FieldActionType};

            // Insert beforeTurn action with priority insertion (uses PRNG for tie-breaking)
            let before_turn_action = Action::Field(FieldAction {
                choice: FieldActionType::BeforeTurn,
                priority: 0,
            });
            self.insert_field_action(before_turn_action);

            // Add residual action to end of queue
            let residual_action = Action::Field(FieldAction {
                choice: FieldActionType::Residual,
                priority: 0,
            });
            self.queue.add_choice_raw(residual_action);

            self.mid_turn = true;
        }

        // Process the action queue
        while let Some(action) = self.queue.shift() {
            eprintln!("DEBUG [turn_loop]: Processing action, {} items remain in queue", self.queue.list.len());
            self.run_action(&action);
            if self.request_state != BattleRequestState::None || self.ended {
                return;
            }
        }

        self.end_turn();
        self.mid_turn = false;
        self.queue.clear();
    }
}
