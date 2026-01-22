use crate::*;
use crate::battle::BattleRequestState;
use crate::battle_queue::Action;

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
        static CALL_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let _call_id = CALL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // Debug immediately at entry
        debug_elog!("[TURN_LOOP_ENTRY #{}] Queue contents at very start:", _call_id);
        for (_i, action) in self.queue.list.iter().enumerate() {
            match action {
                Action::Move(_m) => debug_elog!("[TURN_LOOP_ENTRY]   [{}] Move: {} from ({}, {})", _i, _m.move_id.as_str(), _m.side_index, _m.pokemon_index),
                Action::Switch(_s) => debug_elog!("[TURN_LOOP_ENTRY]   [{}] Switch: pokemon {}", _i, _s.pokemon_index),
                Action::Field(_f) => debug_elog!("[TURN_LOOP_ENTRY]   [{}] Field: {:?}", _i, _f.choice),
                _ => debug_elog!("[TURN_LOOP_ENTRY]   [{}] Other", _i),
            }
        }

        debug_elog!("[TURN_LOOP #{}] Entry: turn={}, mid_turn={}, request_state={:?}, queue_size={}",
            _call_id, self.turn, self.mid_turn, self.request_state, self.queue.list.len());

        self.add("", &[]);

        // Add timestamp (JS: this.add('t:', Math.floor(Date.now() / 1000)))
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.add("t:", &[timestamp.to_string().into()]);

        if self.request_state != BattleRequestState::None {
            debug_elog!("[TURN_LOOP] Clearing request_state from {:?} to None", self.request_state);
            self.request_state = BattleRequestState::None;
        }

        if !self.mid_turn {
            debug_elog!("[TURN_LOOP] mid_turn is false, adding beforeTurn and residual actions");
            // JS: this.queue.insertChoice({ choice: "beforeTurn" });
            // JS: this.queue.addChoice({ choice: "residual" });
            use crate::battle_queue::{Action, FieldAction, FieldActionType};

            debug_elog!("[TURN_LOOP] Queue BEFORE adding beforeTurn/residual:");
            for (_i, action) in self.queue.list.iter().enumerate() {
                match action {
                    Action::Move(_m) => debug_elog!("  [{}] Move: {}", _i, _m.move_id.as_str()),
                    Action::Switch(_s) => debug_elog!("  [{}] Switch: pokemon {}", _i, _s.pokemon_index),
                    Action::Field(_f) => debug_elog!("  [{}] Field: {:?}", _i, _f.choice),
                    _ => debug_elog!("  [{}] Other", _i),
                }
            }

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

            debug_elog!("[TURN_LOOP] Queue AFTER adding beforeTurn/residual:");
            for (_i, action) in self.queue.list.iter().enumerate() {
                match action {
                    Action::Move(_m) => debug_elog!("  [{}] Move: {}", _i, _m.move_id.as_str()),
                    Action::Switch(_s) => debug_elog!("  [{}] Switch: pokemon {}", _i, _s.pokemon_index),
                    Action::Field(_f) => debug_elog!("  [{}] Field: {:?}", _i, _f.choice),
                    _ => debug_elog!("  [{}] Other", _i),
                }
            }

            self.mid_turn = true;
        } else {
            debug_elog!("[TURN_LOOP] mid_turn is true, NOT adding beforeTurn/residual, continuing with existing queue");
        }

        // Process the action queue
        // JS: while ((action = this.queue.shift())) {
        // JS:     this.runAction(action);
        // JS:     if (this.requestState || this.ended) return;
        // JS: }
        let mut _action_count = 0;
        while let Some(action) = self.queue.shift() {
            _action_count += 1;
            let _action_desc = match &action {
                Action::Move(m) => format!("Move({})", m.move_id.as_str()),
                Action::Switch(s) => format!("Switch(slot {})", s.pokemon_index),
                Action::Field(f) => format!("Field({:?})", f.choice),
                Action::Pokemon(p) => format!("Pokemon({:?})", p.choice),
                Action::Team(_) => "Team".to_string(),
            };
            debug_elog!("[TURN_LOOP] Processing action #{}: {}, queue remaining: {}", _action_count, _action_desc, self.queue.list.len());

            self.run_action(&action);

            if self.ended {
                debug_elog!("[TURN_LOOP] Battle ended, returning early");
                return;
            }

            if self.request_state != BattleRequestState::None {
                debug_elog!("[TURN_LOOP] request_state is now {:?}, returning early WITHOUT calling end_turn()", self.request_state);
                return;
            }
        }

        // Note: Phazing (Roar, Dragon Tail, etc.) now happens inside run_action()
        // after EACH action is processed, not here after all actions.
        // See run_action.rs for the implementation matching battle.ts:2820-2828

        debug_elog!("[TURN_LOOP] All actions processed, calling end_turn()");
        self.end_turn();
        debug_elog!("[TURN_LOOP] After end_turn(), turn is now {}", self.turn);

        // JavaScript ALWAYS sets midTurn=false at the end of turnLoop() (line 35 of JS source)
        // This is NOT conditional - it happens whether we added beforeTurn/residual or not
        debug_elog!("[TURN_LOOP] Setting mid_turn=false (matches JavaScript line 35)");
        self.mid_turn = false;
        self.queue.clear();
    }
}
