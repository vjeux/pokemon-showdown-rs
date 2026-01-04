use crate::*;
use crate::battle::PriorityItem;

/// Handler info for field event processing
#[derive(Clone)]
struct FieldEventHandler {
    effect_id: ID,
    _effect_type: crate::battle::EffectType,
    holder: Option<(usize, usize)>,
    is_field: bool,
    is_side: bool,
    speed: i32,
    order: Option<i32>,
    priority: i32,
    sub_order: i32,
}

impl Battle {

    /// Determine effect type from effect ID by checking dex
    #[allow(dead_code)]
    fn determine_effect_type(&self, effect_id: &str) -> crate::battle::EffectType {
        // Check abilities
        if self.dex.abilities().get(effect_id).is_some() {
            return crate::battle::EffectType::Ability;
        }
        // Check items
        if self.dex.items().get(effect_id).is_some() {
            return crate::battle::EffectType::Item;
        }
        // Check moves
        if self.dex.moves().get(effect_id).is_some() {
            return crate::battle::EffectType::Move;
        }
        // Check conditions and return their actual effectType
        if let Some(condition_data) = self.dex.conditions().get(effect_id) {
            // Parse effectType string from condition data
            if let Some(effect_type_value) = condition_data.extra.get("effectType") {
                if let Some(effect_type_str) = effect_type_value.as_str() {
                    return match effect_type_str {
                        "Weather" => crate::battle::EffectType::Weather,
                        "FieldCondition" | "Field" => crate::battle::EffectType::FieldCondition,
                        "SideCondition" | "Side" => crate::battle::EffectType::SideCondition,
                        "SlotCondition" | "Slot" => crate::battle::EffectType::SlotCondition,
                        _ => crate::battle::EffectType::Condition,
                    };
                }
            }
            // Default to Condition if no effectType specified
            return crate::battle::EffectType::Condition;
        }
        // Default to Condition (volatiles, status, etc.)
        crate::battle::EffectType::Condition
    }

    /// Helper to create a FieldEventHandler with proper order/priority from dex
    fn create_field_handler(
        &self,
        effect_id: ID,
        effect_type: crate::battle::EffectType,
        holder: Option<(usize, usize)>,
        is_field: bool,
        is_side: bool,
        callback_name: &str,
        event_id: &str,
    ) -> FieldEventHandler {

        use std::sync::atomic::{AtomicUsize, Ordering};
        static CALL_ID: AtomicUsize = AtomicUsize::new(0);
        let call_id = CALL_ID.fetch_add(1, Ordering::SeqCst);

        // For field and side handlers, the callback name needs to be prefixed
        // JS: if ((handler.effectHolder as Side).sideConditions) handlerEventid = `Side${eventid}`;
        // JS: if ((handler.effectHolder as Field).pseudoWeather) handlerEventid = `Field${eventid}`;
        let prefixed_callback = if is_field {
            format!("onField{}", event_id)
        } else if is_side {
            format!("onSide{}", event_id)
        } else {
            callback_name.to_string()
        };

        eprintln!("[CREATE#{} START] effect={}, type={:?}, callback={}, prefixed={}",
            call_id, effect_id.as_str(), effect_type, callback_name, prefixed_callback);

        // Get order and priority from dex using the prefixed callback name
        let order = self.get_callback_order(effect_type, effect_id.as_str(), &prefixed_callback);
        let priority = self.get_callback_priority(effect_type, effect_id.as_str(), &prefixed_callback);

        eprintln!("[CREATE#{} DONE] order={:?}, priority={}", call_id, order, priority);

        // Get sub_order: first try custom value, then fall back to default based on effect type
        let sub_order = self.get_callback_sub_order(effect_type, effect_id.as_str(), callback_name)
            .unwrap_or_else(|| match effect_type {
                crate::battle::EffectType::Ability => 7,
                crate::battle::EffectType::Item => 8,
                crate::battle::EffectType::Condition => 2,
                _ => 0,
            });

        // Get speed from holder Pokemon
        let speed = if let Some((side_idx, poke_idx)) = holder {
            if let Some(side) = self.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    pokemon.stored_stats.spe
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };

        FieldEventHandler {
            effect_id,
            _effect_type: effect_type,
            holder,
            is_field,
            is_side,
            speed,
            order,
            priority,
            sub_order,
        }
    }

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
        // JS: if (eventid === 'Residual') { getKey = 'duration'; }
        let get_key = if event_id == "Residual" {
            Some("duration")
        } else {
            None
        };

        // Collect all handlers
        let mut handlers: Vec<FieldEventHandler> = Vec::new();

        // JS: let handlers = this.findFieldEventHandlers(this.field, `onField${eventid}`, getKey);
        let field_event = format!("onField{}", event_id);
        let field_handlers = self.find_field_event_handlers(&field_event, get_key, None);
        for handler in field_handlers {
            let effect_id = handler.effect_id;
            let holder = handler.effect_holder;
            let effect_type = handler.effect_type;  // Use effect_type from handler, not determine_effect_type
            let handler = self.create_field_handler(
                effect_id,
                effect_type,
                holder,
                true,
                false,
                &callback_name,
                event_id,
            );
            handlers.push(handler);
        }

        // JS: for (const side of this.sides) { ... }
        for side_idx in 0..self.sides.len() {
            // JS: if (side.n < 2 || !side.allySide) {
            //         handlers = handlers.concat(this.findSideEventHandlers(side, `onSide${eventid}`, getKey));
            //     }
            // Rust: side.n == side_idx, side.allySide == ally_index
            if side_idx < 2 || self.sides[side_idx].ally_index.is_none() {
                let side_event = format!("onSide{}", event_id);
                let side_handlers = self.find_side_event_handlers(&side_event, side_idx, get_key, None);
                for handler in side_handlers {
                    let effect_id = handler.effect_id;
                    let holder = handler.effect_holder;
                    let effect_type = handler.effect_type;  // Use effect_type from handler, not determine_effect_type
                    let handler = self.create_field_handler(
                        effect_id,
                        effect_type,
                        holder,
                        false,
                        true,
                        &callback_name,
                        event_id,
                    );
                    handlers.push(handler);
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
                    let any_handlers = self.find_pokemon_event_handlers(&any_event, target_pos, None);
                    for handler in any_handlers {
                        let effect_id = handler.effect_id;
                        let holder = handler.effect_holder;
                        let effect_type = handler.effect_type;
                        let handler = self.create_field_handler(
                            effect_id,
                            effect_type,
                            holder,
                            false,
                            false,
                            &any_event,
                            event_id,
                        );
                        handlers.push(handler);
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
                let pokemon_handlers = self.find_pokemon_event_handlers(&callback_name, target_pos, get_key);
                for handler in pokemon_handlers {
                    let effect_id = handler.effect_id;
                    let holder = handler.effect_holder;
                    let effect_type = handler.effect_type;
                    let handler = self.create_field_handler(
                        effect_id,
                        effect_type,
                        holder,
                        false,
                        false,
                        &callback_name,
                        event_id,
                    );
                    handlers.push(handler);
                }

                // Note: findSideEventHandlers and findFieldEventHandlers with customHolder
                // are not fully implemented in Rust find_*_event_handlers methods
                // This is an architectural simplification
            }
        }

        // JS: this.speedSort(handlers);
        // Sort handlers by Pokemon speed
        eprintln!("[FIELD_EVENT] event='{}', turn={}, handlers.len()={}, handler IDs: {:?}",
            event_id, self.turn, handlers.len(),
            handlers.iter().map(|h| h.effect_id.as_str()).collect::<Vec<_>>());
        for (i, h) in handlers.iter().enumerate() {
            eprintln!("  [{}] id={}, order={:?}, priority={}, speed={}, sub_order={}",
                i, h.effect_id.as_str(), h.order, h.priority, h.speed, h.sub_order);
        }
        self.speed_sort(&mut handlers, |h| {
            PriorityItem {
                order: h.order,
                priority: h.priority,
                speed: h.speed as f64,
                sub_order: h.sub_order,
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

            // JS: if (eventid === 'Residual' && handler.end && handler.state?.duration) {
            //         handler.state.duration--;
            //         if (!handler.state.duration) {
            //             const endCallArgs = handler.endCallArgs || [handler.effectHolder, effect.id];
            //             handler.end.call(...endCallArgs as [any, ...any[]]);
            //             if (this.ended) return;
            //             continue;
            //         }
            //     }

            // Handle field effects (weather, terrain, pseudo-weather)
            if event_id == "Residual" && handler.is_field {
                let should_clear = {
                    // Check weather
                    if self.field.weather == handler.effect_id {
                        if let Some(duration) = self.field.weather_state.duration.as_mut() {
                            *duration -= 1;
                            *duration == 0
                        } else {
                            false
                        }
                    }
                    // Check terrain
                    else if self.field.terrain == handler.effect_id {
                        if let Some(duration) = self.field.terrain_state.duration.as_mut() {
                            *duration -= 1;
                            *duration == 0
                        } else {
                            false
                        }
                    }
                    // Check pseudo-weather
                    else if let Some(pw_state) = self.field.pseudo_weather.get_mut(&handler.effect_id) {
                        if let Some(duration) = pw_state.duration.as_mut() {
                            *duration -= 1;
                            *duration == 0
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                // Clear expired field effects
                if should_clear {
                    // Clear weather
                    if self.field.weather == handler.effect_id {
                        self.field.weather = ID::new("");
                        self.field.weather_state = crate::event_system::EffectState::new(ID::new(""));
                    }
                    // Clear terrain
                    else if self.field.terrain == handler.effect_id {
                        self.field.terrain = ID::new("");
                        self.field.terrain_state = crate::event_system::EffectState::new(ID::new(""));
                    }
                    // Clear pseudo-weather
                    else {
                        self.field.pseudo_weather.remove(&handler.effect_id);
                    }

                    if self.ended {
                        return;
                    }
                    continue;
                }
            }

            // Handle Pokemon volatiles and status effects
            if event_id == "Residual" {
                // Handle volatile/status duration decrements
                if let Some((side_idx, poke_idx)) = handler.holder {
                    let should_remove = {
                        if let Some(pokemon) = self.sides.get_mut(side_idx)
                            .and_then(|s| s.pokemon.get_mut(poke_idx)) {

                            // Check if this is a volatile with duration
                            if let Some(volatile_state) = pokemon.volatiles.get_mut(&handler.effect_id) {
                                if let Some(duration) = volatile_state.duration.as_mut() {
                                    // JavaScript doesn't decrement volatiles on the same turn they're added
                                    // Check if this volatile was created this turn
                                    let skip_decrement = volatile_state.created_turn == Some(self.turn);

                                    if skip_decrement {
                                        eprintln!("[FIELD_EVENT RESIDUAL] turn={}, volatile='{}', pokemon=({},{}), SKIP decrement (created this turn)",
                                            self.turn, handler.effect_id.as_str(), side_idx, poke_idx);
                                        false
                                    } else {
                                        eprintln!("[FIELD_EVENT RESIDUAL] turn={}, volatile='{}', pokemon=({},{}), duration BEFORE decrement={}",
                                            self.turn, handler.effect_id.as_str(), side_idx, poke_idx, *duration);
                                        *duration -= 1;
                                        eprintln!("[FIELD_EVENT RESIDUAL] turn={}, volatile='{}', pokemon=({},{}), duration AFTER decrement={}",
                                            self.turn, handler.effect_id.as_str(), side_idx, poke_idx, *duration);
                                        if *duration == 0 {
                                            eprintln!("[FIELD_EVENT RESIDUAL] turn={}, volatile='{}' EXPIRED, removing and skipping handler",
                                                self.turn, handler.effect_id.as_str());
                                            true
                                        } else {
                                            false
                                        }
                                    }
                                } else {
                                    false
                                }
                            } else {
                                // Might be status, ability, or item state - not yet implemented
                                false
                            }
                        } else {
                            false
                        }
                    };

                    // Remove expired volatile
                    if should_remove {
                        if let Some(pokemon) = self.sides.get_mut(side_idx)
                            .and_then(|s| s.pokemon.get_mut(poke_idx)) {
                            pokemon.volatiles.remove(&handler.effect_id);
                        }
                        // Skip calling the event handler for expired effects
                        if self.ended {
                            return;
                        }
                        continue;
                    }
                }
            }

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
