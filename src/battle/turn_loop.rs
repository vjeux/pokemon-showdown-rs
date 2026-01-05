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
        let call_id = CALL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        eprintln!("[TURN_LOOP #{}] Entry: turn={}, mid_turn={}, request_state={:?}, queue_size={}",
            call_id, self.turn, self.mid_turn, self.request_state, self.queue.list.len());

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

            eprintln!("[TURN_LOOP] Queue BEFORE adding beforeTurn/residual:");
            for (i, action) in self.queue.list.iter().enumerate() {
                match action {
                    Action::Move(m) => eprintln!("  [{}] Move: {}", i, m.move_id.as_str()),
                    Action::Switch(s) => eprintln!("  [{}] Switch: pokemon {}", i, s.pokemon_index),
                    Action::Field(f) => eprintln!("  [{}] Field: {:?}", i, f.choice),
                    _ => eprintln!("  [{}] Other", i),
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

            eprintln!("[TURN_LOOP] Queue AFTER adding beforeTurn/residual:");
            for (i, action) in self.queue.list.iter().enumerate() {
                match action {
                    Action::Move(m) => eprintln!("  [{}] Move: {}", i, m.move_id.as_str()),
                    Action::Switch(s) => eprintln!("  [{}] Switch: pokemon {}", i, s.pokemon_index),
                    Action::Field(f) => eprintln!("  [{}] Field: {:?}", i, f.choice),
                    _ => eprintln!("  [{}] Other", i),
                }
            }

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
            let action_desc = match &action {
                Action::Move(m) => format!("Move({})", m.move_id.as_str()),
                Action::Switch(s) => format!("Switch(slot {})", s.pokemon_index),
                Action::Field(f) => format!("Field({:?})", f.choice),
                Action::Pokemon(p) => format!("Pokemon({:?})", p.choice),
                Action::Team(_) => "Team".to_string(),
            };
            eprintln!("[TURN_LOOP] Processing action #{}: {}, queue remaining: {}", action_count, action_desc, self.queue.list.len());

            // IMPORTANT: Process forced switches (phazing) BEFORE Residual action
            // This ensures Pokemon switched in by moves like Dragon Tail take residual damage
            if matches!(action, Action::Field(ref f) if matches!(f.choice, crate::battle_queue::FieldActionType::Residual)) {
                // JavaScript: phazing (Roar, etc) - battle.ts:2821-2833
                // for (const side of this.sides) {
                //     for (const pokemon of side.active) {
                //         if (pokemon.forceSwitchFlag) {
                //             if (pokemon.hp) this.actions.dragIn(pokemon.side, pokemon.position);
                //             pokemon.forceSwitchFlag = false;
                //         }
                //     }
                // }
                for side_idx in 0..self.sides.len() {
                    let active_count = self.sides[side_idx].active.len();
                    for slot in 0..active_count {
                        // Get the pokemon index from the active array
                        let pokemon_idx = match self.sides[side_idx].active.get(slot).and_then(|&opt| opt) {
                            Some(idx) => idx,
                            None => continue,
                        };

                        let (should_drag_in, has_hp) = {
                            let pokemon = &self.sides[side_idx].pokemon[pokemon_idx];
                            (pokemon.force_switch_flag, pokemon.hp > 0)
                        };

                        if should_drag_in {
                            if has_hp {
                                crate::battle_actions::drag_in(self, side_idx, slot);
                            }
                            // Clear the flag
                            self.sides[side_idx].pokemon[pokemon_idx].force_switch_flag = false;
                        }
                    }
                }
            }

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

        // Phazing is now processed BEFORE Residual action (see lines 126-159 above)
        // This comment preserves the original JavaScript reference for documentation
        // JavaScript: phazing (Roar, etc) - battle.ts:2821-2833
        // for (const side of this.sides) {
        //     for (const pokemon of side.active) {
        //         if (pokemon.forceSwitchFlag) {
        //             if (pokemon.hp) this.actions.dragIn(pokemon.side, pokemon.position);
        //             pokemon.forceSwitchFlag = false;
        //         }
        //     }
        // }

        // JavaScript: this.clearActiveMove();
        self.clear_active_move(false);

        eprintln!("[TURN_LOOP] After phazing, calling end_turn()");
        self.end_turn();
        eprintln!("[TURN_LOOP] After end_turn(), turn is now {}", self.turn);

        // JavaScript ALWAYS sets midTurn=false at the end of turnLoop() (line 35 of JS source)
        // This is NOT conditional - it happens whether we added beforeTurn/residual or not
        eprintln!("[TURN_LOOP] Setting mid_turn=false (matches JavaScript line 35)");
        self.mid_turn = false;
        self.queue.clear();
    }
}
