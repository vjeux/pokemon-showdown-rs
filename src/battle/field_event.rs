use crate::*;
use crate::battle::PriorityItem;

/// Handler info for field event processing
#[derive(Clone)]
struct FieldEventHandler {
    effect_id: ID,
    holder: Option<(usize, usize)>,
    is_field: bool,
    is_side: bool,
    speed: i32,
}

impl Battle {

    /// Run event on field (weather, terrain, pseudo-weather)
    /// Equivalent to battle.ts fieldEvent() (battle.ts:484-568, 85 lines)
    ///
    /// Runs an event with no source on each effect on the field, in Speed order.
    /// Unlike `eachEvent`, this contains a lot of other handling and is only intended for
    /// the 'Residual' and 'SwitchIn' events.
    ///
    /// targets: Optional list of Pokemon positions to filter handler collection
    //
    // 	fieldEvent(eventid: string, targets?: Pokemon[]) {
    // 		const callbackName = `on${eventid}`;
    // 		let getKey: undefined | 'duration';
    // 		if (eventid === 'Residual') {
    // 			getKey = 'duration';
    // 		}
    // 		let handlers = this.findFieldEventHandlers(this.field, `onField${eventid}`, getKey);
    // 		for (const side of this.sides) {
    // 			if (side.n < 2 || !side.allySide) {
    // 				handlers = handlers.concat(this.findSideEventHandlers(side, `onSide${eventid}`, getKey));
    // 			}
    // 			for (const active of side.active) {
    // 				if (!active) continue;
    // 				if (eventid === 'SwitchIn') {
    // 					handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventid}`));
    // 				}
    // 				if (targets && !targets.includes(active)) continue;
    // 				handlers = handlers.concat(this.findPokemonEventHandlers(active, callbackName, getKey));
    // 				handlers = handlers.concat(this.findSideEventHandlers(side, callbackName, undefined, active));
    // 				handlers = handlers.concat(this.findFieldEventHandlers(this.field, callbackName, undefined, active));
    // 				handlers = handlers.concat(this.findBattleEventHandlers(callbackName, getKey, active));
    // 			}
    // 		}
    // 		this.speedSort(handlers);
    // 		while (handlers.length) {
    // 			const handler = handlers[0];
    // 			handlers.shift();
    // 			const effect = handler.effect;
    // 			if ((handler.effectHolder as Pokemon).fainted) {
    // 				if (!(handler.state?.isSlotCondition)) continue;
    // 			}
    // 			if (eventid === 'Residual' && handler.end && handler.state?.duration) {
    // 				handler.state.duration--;
    // 				if (!handler.state.duration) {
    // 					const endCallArgs = handler.endCallArgs || [handler.effectHolder, effect.id];
    // 					handler.end.call(...endCallArgs as [any, ...any[]]);
    // 					if (this.ended) return;
    // 					continue;
    // 				}
    // 			}
    // 			// ... (state validation logic)
    // 			let handlerEventid = eventid;
    // 			if ((handler.effectHolder as Side).sideConditions) handlerEventid = `Side${eventid}`;
    // 			if ((handler.effectHolder as Field).pseudoWeather) handlerEventid = `Field${eventid}`;
    // 			if (handler.callback) {
    // 				this.singleEvent(handlerEventid, effect, handler.state, handler.effectHolder, null, null, undefined, handler.callback);
    // 			}
    // 			this.faintMessages();
    // 			if (this.ended) return;
    // 		}
    // 	}
    //
    pub fn field_event(&mut self, event_id: &str, targets: Option<&[(usize, usize)]>) {
        let callback_name = format!("on{}", event_id);
        let _get_key_is_duration = event_id == "Residual";

        eprintln!("[FIELD_EVENT] Starting {} event", event_id);
        // Debug: Check Pokemon states at start of Residual event
        if event_id == "Residual" {
            for (side_idx, side) in self.sides.iter().enumerate() {
                for (poke_idx, pokemon) in side.pokemon.iter().enumerate() {
                    if !pokemon.fainted {
                        eprintln!("[FIELD_EVENT] INITIAL STATE CHECK: sides[{}].pokemon[{}] '{}' has {} volatiles: {:?}",
                            side_idx, poke_idx, pokemon.name, pokemon.volatiles.len(), pokemon.volatiles.keys().collect::<Vec<_>>());
                    }
                }
            }
        }

        // Collect all handlers
        let mut handlers: Vec<FieldEventHandler> = Vec::new();

        // JS: let handlers = this.findFieldEventHandlers(this.field, `onField${eventid}`, getKey);
        let field_event = format!("onField{}", event_id);
        let field_handlers = self.find_field_event_handlers(&field_event);
        for (effect_id, holder) in field_handlers {
            handlers.push(FieldEventHandler {
                effect_id,
                holder,
                is_field: true,
                is_side: false,
                speed: 0,
            });
        }

        // JS: for (const side of this.sides) { ... }
        for side_idx in 0..self.sides.len() {
            // JS: if (side.n < 2 || !side.allySide) {
            //         handlers = handlers.concat(this.findSideEventHandlers(side, `onSide${eventid}`, getKey));
            //     }
            // Note: allySide tracking not implemented in Rust, so checking side_idx < 2
            if side_idx < 2 {
                let side_event = format!("onSide{}", event_id);
                let side_handlers = self.find_side_event_handlers(&side_event, side_idx);
                for (effect_id, holder) in side_handlers {
                    handlers.push(FieldEventHandler {
                        effect_id,
                        holder,
                        is_field: false,
                        is_side: true,
                        speed: 0,
                    });
                }
            }

            // JS: for (const active of side.active) { ... }
            // Iterate through actual active Pokemon, not all slots
            let active_pokemon: Vec<usize> = if let Some(side) = self.sides.get(side_idx) {
                side.active.iter().flatten().copied().collect()
            } else {
                Vec::new()
            };

            for poke_idx in active_pokemon {
                let target_pos = (side_idx, poke_idx);

                // JS: if (eventid === 'SwitchIn') {
                //         handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventid}`));
                //     }
                if event_id == "SwitchIn" {
                    let any_event = format!("onAny{}", event_id);
                    let any_handlers = self.find_pokemon_event_handlers(&any_event, target_pos);
                    for (effect_id, holder) in any_handlers {
                        let speed = self.sides.get(side_idx)
                            .and_then(|s| s.pokemon.get(poke_idx))
                            .map(|p| p.speed)
                            .unwrap_or(0);
                        handlers.push(FieldEventHandler {
                            effect_id,
                            holder,
                            is_field: false,
                            is_side: false,
                            speed,
                        });
                    }
                }

                // JS: if (targets && !targets.includes(active)) continue;
                // If targets is provided and this Pokemon is not in the targets list, skip remaining handlers
                if let Some(target_list) = targets {
                    if !target_list.contains(&target_pos) {
                        continue;
                    }
                }

                // JS: handlers = handlers.concat(this.findPokemonEventHandlers(active, callbackName, getKey));
                let pokemon_handlers = self.find_pokemon_event_handlers(&callback_name, target_pos);
                for (effect_id, holder) in pokemon_handlers {
                    let speed = self.sides.get(side_idx)
                        .and_then(|s| s.pokemon.get(poke_idx))
                        .map(|p| p.speed)
                        .unwrap_or(0);
                    handlers.push(FieldEventHandler {
                        effect_id,
                        holder,
                        is_field: false,
                        is_side: false,
                        speed,
                    });
                }

                // Note: findSideEventHandlers and findFieldEventHandlers with customHolder
                // are not fully implemented in Rust find_*_event_handlers methods
                // This is an architectural simplification
            }
        }

        // JS: this.speedSort(handlers);
        // Sort handlers by Pokemon speed
        eprintln!("[FIELD_EVENT] Sorting {} handlers before processing", handlers.len());
        for (i, h) in handlers.iter().enumerate() {
            eprintln!("[FIELD_EVENT] Handler {}: effect={}, speed={}, is_field={}, is_side={}",
                i, h.effect_id.as_str(), h.speed, h.is_field, h.is_side);
        }
        self.speed_sort(&mut handlers, |h| {
            PriorityItem {
                order: None,
                priority: 0,
                speed: h.speed,
                sub_order: 0,
                effect_order: 0,
                index: 0,
            }
        });

        // JS: while (handlers.length) { ... }
        while !handlers.is_empty() {
            let handler = handlers.remove(0);

            // JS: if ((handler.effectHolder as Pokemon).fainted) {
            //         if (!(handler.state?.isSlotCondition)) continue;
            //     }
            if let Some((side_idx, poke_idx)) = handler.holder {
                if let Some(pokemon) = self.sides.get(side_idx)
                    .and_then(|s| s.pokemon.get(poke_idx)) {
                    if pokemon.fainted {
                        // Skip fainted Pokemon (slot conditions not implemented)
                        continue;
                    }
                }
            }

            // TODO: Handle Residual event duration decrements
            // This requires accessing handler.state.duration which is not available
            // in the simplified handler structure

            // JS: let handlerEventid = eventid;
            //     if ((handler.effectHolder as Side).sideConditions) handlerEventid = `Side${eventid}`;
            //     if ((handler.effectHolder as Field).pseudoWeather) handlerEventid = `Field${eventid}`;
            let handler_event_id = if handler.is_side {
                format!("Side{}", event_id)
            } else if handler.is_field {
                format!("Field{}", event_id)
            } else {
                event_id.to_string()
            };

            // JS: if (handler.callback) {
            //         this.singleEvent(handlerEventid, effect, handler.state, handler.effectHolder, null, null, undefined, handler.callback);
            //     }
            self.single_event(&handler_event_id, &handler.effect_id, handler.holder, None, None);

            // JS: this.faintMessages();
            self.faint_messages(false, false, true);

            // JS: if (this.ended) return;
            if self.ended {
                return;
            }
        }
    }
}
