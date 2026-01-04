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
        eprintln!("[TURN_LOOP] Entry: turn={}, mid_turn={}, request_state={:?}, queue_size={}",
            self.turn, self.mid_turn, self.request_state, self.queue.list.len());

        self.add("", &[]);

        // Add timestamp (JS: this.add('t:', Math.floor(Date.now() / 1000)))
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.add("t:", &[timestamp.to_string().into()]);

        if self.request_state != BattleRequestState::None {
            eprintln!("[TURN_LOOP] Clearing request_state from {:?} to None", self.request_state);
            self.request_state = BattleRequestState::None;
        }

        if !self.mid_turn {
            eprintln!("[TURN_LOOP] mid_turn is false, adding beforeTurn and residual actions");
            // JS: this.queue.insertChoice({ choice: "beforeTurn" });
            // JS: this.queue.addChoice({ choice: "residual" });
            use crate::battle_queue::{Action, FieldAction, FieldActionType};

            // JS: this.queue.insertChoice({ choice: 'beforeTurn' });
            let before_turn_action = Action::Field(FieldAction {
                choice: FieldActionType::BeforeTurn,
                priority: 0,
                sub_order: 0,
                effect_order: 0,
            });
            self.queue_insert_choice(before_turn_action);

            // JS: this.queue.addChoice({ choice: 'residual' });
            let residual_action = Action::Field(FieldAction {
                choice: FieldActionType::Residual,
                priority: 0,
                sub_order: 0,
                effect_order: 0,
            });
            self.queue.list.push(residual_action);

            self.mid_turn = true;
        } else {
            eprintln!("[TURN_LOOP] mid_turn is true, NOT adding beforeTurn/residual, continuing with existing queue");
        }

        // Process the action queue
        // JS: while ((action = this.queue.shift())) {
        // JS:     this.runAction(action);
        // JS:     if (this.requestState || this.ended) return;
        // JS: }
        let mut action_count = 0;
        while let Some(action) = self.queue.shift() {
            action_count += 1;
            eprintln!("[TURN_LOOP] Processing action #{}, queue remaining: {}", action_count, self.queue.list.len());
            self.run_action(&action);

            if self.ended {
                eprintln!("[TURN_LOOP] Battle ended, returning early");
                return;
            }

            if self.request_state != BattleRequestState::None {
                eprintln!("[TURN_LOOP] request_state is now {:?}, returning early WITHOUT calling end_turn()", self.request_state);
                return;
            }
        }

        eprintln!("[TURN_LOOP] Queue empty, calling end_turn()");
        self.end_turn();
        eprintln!("[TURN_LOOP] After end_turn(), turn is now {}", self.turn);

        // JS: this.midTurn = false;
        eprintln!("[TURN_LOOP] Setting mid_turn=false");
        self.mid_turn = false;
        self.queue.clear();
    }
}
