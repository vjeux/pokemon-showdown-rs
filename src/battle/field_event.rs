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
    side_idx: Option<usize>, // For side conditions, which side this handler is for
    speed: i32,
    order: Option<i32>,
    priority: i32,
    sub_order: i32,
    effect_order: i32, // JavaScript: effectOrder (creation order for tie-breaking)
    has_callback: bool, // JavaScript: handler.callback !== undefined
    callback_name: String, // JavaScript: the callback name without "on" prefix (e.g., "AnySwitchIn" for onAnySwitchIn)
}

impl Battle {

    /// Helper to create a FieldEventHandler with proper order/priority from dex
    fn create_field_handler(
        &self,
        effect_id: ID,
        effect_type: crate::battle::EffectType,
        holder: Option<(usize, usize)>,
        is_field: bool,
        is_side: bool,
        side_idx: Option<usize>, // For side conditions, which side this handler is for
        callback_name: &str,
        event_id: &str,
        effect_order: i32, // JavaScript: effectOrder from handler state
        has_callback: bool, // JavaScript: handler.callback !== undefined
    ) -> FieldEventHandler {

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

        // Get order and priority from dex using the prefixed callback name
        let order = self.get_callback_order(effect_type, effect_id.as_str(), &prefixed_callback);
        let priority = self.get_callback_priority(effect_type, effect_id.as_str(), &prefixed_callback);

        // Get sub_order: first try custom value, then fall back to default based on effect type
        // This matches JavaScript's resolvePriority effectTypeOrder
        // IMPORTANT: Use prefixed_callback (e.g., "onSideResidual") not callback_name (e.g., "onResidual")
        // because side conditions have properties like onSideResidualSubOrder, not onResidualSubOrder
        let sub_order = self.get_callback_sub_order(effect_type, effect_id.as_str(), &prefixed_callback)
            .unwrap_or_else(|| match effect_type {
                crate::battle::EffectType::ZMove => 1,
                crate::battle::EffectType::Condition => 2,
                crate::battle::EffectType::SlotCondition => 3,
                crate::battle::EffectType::SideCondition => 4,
                crate::battle::EffectType::FieldCondition => 5,
                crate::battle::EffectType::Weather => 5,
                crate::battle::EffectType::Format => 5,
                crate::battle::EffectType::Rule => 5,
                crate::battle::EffectType::Ruleset => 5,
                crate::battle::EffectType::Ability => 7,
                crate::battle::EffectType::Item => 8,
                // Status and other types default to 0 (like JavaScript)
                _ => 0,
            });

        // Get speed from holder Pokemon
        // JS: handler.speed = pokemon.speed;
        // Use pokemon.speed (the action speed) not stored_stats.spe (the base stat)
        let speed = if let Some((side_idx, poke_idx)) = holder {
            if let Some(side) = self.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    pokemon.speed
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
            side_idx,
            speed,
            order,
            priority,
            sub_order,
            effect_order, // JavaScript: effectOrder for tie-breaking
            has_callback,
            // Store callback name without "on" prefix for dispatch
            // e.g., "onAnySwitchIn" -> "AnySwitchIn", "onSwitchIn" -> "SwitchIn"
            callback_name: if is_field {
                format!("Field{}", event_id)
            } else if is_side {
                format!("Side{}", event_id)
            } else {
                callback_name.strip_prefix("on").unwrap_or(callback_name).to_string()
            },
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
            let effect_id = handler.effect.id;
            let holder = handler.effect_holder;
            let effect_type = handler.effect.effect_type;  // Use effect_type from handler, not determine_effect_type
            // JavaScript sets effectOrder for SwitchIn/RedirectTarget events, undefined for others
            let effect_order = if event_id == "SwitchIn" || event_id == "RedirectTarget" {
                handler.state.as_ref().map(|s| s.effect_order).unwrap_or(0)
            } else {
                0
            };
            // JavaScript: handler.callback is set from getCallback() result
            // If getCallback returns undefined, callback is undefined even if handler exists
            let handler_has_callback = self.has_callback_for_effect_type(&effect_id, &field_event, &effect_type);
            let handler = self.create_field_handler(
                effect_id,
                effect_type,
                holder,
                true,
                false,
                None, // Field handlers are not side-specific
                &callback_name,
                event_id,
                effect_order,
                handler_has_callback,
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
                    let effect_id = handler.effect.id;
                    let holder = handler.effect_holder;
                    let effect_type = handler.effect.effect_type;  // Use effect_type from handler, not determine_effect_type
                    // JavaScript fieldEvent() in resolvePriority only sets effectOrder for 'SwitchIn' and 'RedirectTarget' events.
                    // For these events, effectOrder comes from handler.state.effectOrder to ensure hazards activate
                    // in the order they were created (e.g., Spikes before Stealth Rock if Spikes was set first).
                    // For other events like 'Residual', effectOrder is undefined, causing handlers with same
                    // priority/speed/subOrder to be tied and shuffled.
                    let effect_order = if event_id == "SwitchIn" || event_id == "RedirectTarget" {
                        handler.state.as_ref().map(|s| s.effect_order).unwrap_or(0)
                    } else {
                        0
                    };
                    // JavaScript: handler.callback is set from getCallback() result
                    // For onSideResidual handlers, check if the effect has this callback
                    let handler_has_callback = self.has_callback_for_effect_type(&effect_id, &side_event, &effect_type);
                    let handler = self.create_field_handler(
                        effect_id,
                        effect_type,
                        holder,
                        false,
                        true,
                        Some(side_idx), // Side handlers are for this specific side
                        &callback_name,
                        event_id,
                        effect_order,
                        handler_has_callback,
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
                        let effect_id = handler.effect.id;
                        let holder = handler.effect_holder;
                        let effect_type = handler.effect.effect_type;
                        // JavaScript sets effectOrder for SwitchIn and RedirectTarget events from handler.state.effectOrder
                        // onAnySwitchIn ends with "SwitchIn" so it should also get effectOrder
                        let effect_order = handler.state.as_ref().map(|s| s.effect_order).unwrap_or(0);
                        // For any event handlers, check if the effect has this callback
                        let handler_has_callback = self.has_callback_for_effect_type(&effect_id, &any_event, &effect_type);
                        let handler = self.create_field_handler(
                            effect_id,
                            effect_type,
                            holder,
                            false,
                            false,
                            None, // Any event handlers are not side-specific
                            &any_event,
                            event_id,
                            effect_order,
                            handler_has_callback,
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
                    let effect_id = handler.effect.id;
                    let holder = handler.effect_holder;
                    let effect_type = handler.effect.effect_type;
                    // JavaScript fieldEvent() in resolvePriority only sets effectOrder for 'SwitchIn' and 'RedirectTarget' events.
                    // For other events like 'Residual', effectOrder is undefined (0), causing ties and shuffles.
                    let effect_order = if event_id == "SwitchIn" || event_id == "RedirectTarget" {
                        handler.state.as_ref().map(|s| s.effect_order).unwrap_or(0)
                    } else {
                        0
                    };
                    // For Pokemon handlers, check if the effect has this callback
                    let handler_has_callback = self.has_callback_for_effect_type(&effect_id, &callback_name, &effect_type);
                    let handler = self.create_field_handler(
                        effect_id,
                        effect_type,
                        holder,
                        false,
                        false,
                        None, // Pokemon handlers are not side-specific
                        &callback_name,
                        event_id,
                        effect_order,
                        handler_has_callback,
                    );
                    handlers.push(handler);
                }

                // JS: handlers = handlers.concat(this.findSideEventHandlers(side, callbackName, undefined, active));
                // This finds side condition handlers (like gmaxcannonade's onResidual) that target each Pokemon
                let side_handlers_for_pokemon = self.find_side_event_handlers(&callback_name, side_idx, None, Some(target_pos));
                for handler in side_handlers_for_pokemon {
                    let effect_id = handler.effect.id;
                    let effect_type = handler.effect.effect_type;
                    // JavaScript fieldEvent() in resolvePriority only sets effectOrder for 'SwitchIn' and 'RedirectTarget' events.
                    // For 'Residual' events, effectOrder is undefined (0), causing ties and shuffles.
                    // For 'SwitchIn' events, use the stored effect_order from state to prevent unnecessary shuffles.
                    let effect_order = if event_id == "SwitchIn" || event_id == "RedirectTarget" {
                        handler.state.as_ref().map(|s| s.effect_order).unwrap_or(0)
                    } else {
                        0
                    };
                    // For side condition handlers targeting Pokemon, check if the effect has this callback
                    let handler_has_callback = self.has_callback_for_effect_type(&effect_id, &callback_name, &effect_type);
                    let handler = self.create_field_handler(
                        effect_id,
                        effect_type,
                        Some(target_pos), // Handler's holder is the target Pokemon
                        false,
                        false,
                        Some(side_idx), // Side condition handlers are side-specific
                        &callback_name,
                        event_id,
                        effect_order,
                        handler_has_callback,
                    );
                    handlers.push(handler);
                }

                // JS: handlers = handlers.concat(this.findFieldEventHandlers(this.field, callbackName, undefined, active));
                // This finds field handlers (like sandstorm's onResidual) that target each Pokemon
                // Note: This is different from onFieldResidual - this is onResidual with customHolder = active Pokemon
                let field_handlers_for_pokemon = self.find_field_event_handlers(&callback_name, None, Some(target_pos));
                for handler in field_handlers_for_pokemon {
                    let effect_id = handler.effect.id;
                    let effect_type = handler.effect.effect_type;
                    // JavaScript sets effectOrder for SwitchIn/RedirectTarget events, undefined for others
                    let effect_order = if event_id == "SwitchIn" || event_id == "RedirectTarget" {
                        handler.state.as_ref().map(|s| s.effect_order).unwrap_or(0)
                    } else {
                        0
                    };
                    // For field handlers targeting Pokemon, check if the effect has this callback
                    let handler_has_callback = self.has_callback_for_effect_type(&effect_id, &callback_name, &effect_type);
                    let handler = self.create_field_handler(
                        effect_id,
                        effect_type,
                        Some(target_pos), // Handler's holder is the target Pokemon
                        false, // is_field = false because effectHolder is the Pokemon, not Field
                        false,
                        None,
                        &callback_name,
                        event_id,
                        effect_order,
                        handler_has_callback,
                    );
                    handlers.push(handler);
                }

                // JS: handlers = handlers.concat(this.findBattleEventHandlers(callbackName, getKey, active));
                // This finds battle-level handlers that target each Pokemon
                let battle_handlers_for_pokemon = self.find_battle_event_handlers(&callback_name, get_key, Some(target_pos));
                for handler in battle_handlers_for_pokemon {
                    let effect_id = handler.effect.id;
                    let effect_type = handler.effect.effect_type;
                    // JavaScript sets effectOrder for SwitchIn/RedirectTarget events, undefined for others
                    let effect_order = if event_id == "SwitchIn" || event_id == "RedirectTarget" {
                        handler.state.as_ref().map(|s| s.effect_order).unwrap_or(0)
                    } else {
                        0
                    };
                    // For battle handlers targeting Pokemon, check if the effect has this callback
                    let handler_has_callback = self.has_callback_for_effect_type(&effect_id, &callback_name, &effect_type);
                    let handler = self.create_field_handler(
                        effect_id,
                        effect_type,
                        Some(target_pos), // Handler's holder is the target Pokemon
                        false,
                        false,
                        None,
                        &callback_name,
                        event_id,
                        effect_order,
                        handler_has_callback,
                    );
                    handlers.push(handler);
                }
            }
        }

        // JS: this.speedSort(handlers);
        // Sort handlers by Pokemon speed
        eprintln!("[FIELD_EVENT] event='{}', turn={}, BEFORE speed_sort, handlers.len()={}, handler IDs: {:?}",
            event_id, self.turn, handlers.len(),
            handlers.iter().map(|h| h.effect_id.as_str()).collect::<Vec<_>>());
        for (i, h) in handlers.iter().enumerate() {
            eprintln!("  [{}] id={}, order={:?}, priority={}, speed={}, sub_order={}, effect_order={}",
                i, h.effect_id.as_str(), h.order, h.priority, h.speed, h.sub_order, h.effect_order);
        }
        self.speed_sort(&mut handlers, |h| {
            PriorityItem {
                order: h.order,
                priority: h.priority,
                fractional_priority: 0.0,
                speed: h.speed as f64,
                sub_order: h.sub_order,
                effect_order: h.effect_order, // JavaScript: effectOrder for tie-breaking
                index: 0,
            }
        });

        eprintln!("[FIELD_EVENT] event='{}', turn={}, AFTER speed_sort, handler IDs: {:?}",
            event_id, self.turn,
            handlers.iter().map(|h| h.effect_id.as_str()).collect::<Vec<_>>());

        // JS: while (handlers.length) { ... }
        while !handlers.is_empty() {
            let handler = handlers.remove(0);

            // JS: if ((handler.effectHolder as Pokemon).fainted) {
            //         if (!(handler.state?.isSlotCondition)) continue;
            //     }
            // Skip fainted Pokemon UNLESS it's a slot condition
            // Slot conditions persist even when the Pokemon faints/switches out
            // IMPORTANT: This check should ONLY apply when effectHolder is a Pokemon.
            // For side conditions (is_side=true), effectHolder is the Side, not a Pokemon.
            // For field conditions (is_field=true), effectHolder is the Field, not a Pokemon.
            // JavaScript's (handler.effectHolder as Pokemon).fainted returns undefined (falsy)
            // for non-Pokemon effectHolders, so the check doesn't skip those handlers.
            if !handler.is_side && !handler.is_field {
                if handler._effect_type != crate::battle::EffectType::SlotCondition {
                    if let Some((side_idx, poke_idx)) = handler.holder {
                        if let Some(pokemon) = self.sides.get(side_idx)
                            .and_then(|s| s.pokemon.get(poke_idx)) {
                            if pokemon.fainted {
                                continue;
                            }
                        }
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
            // JavaScript (battle.ts:515-522):
            // if (eventid === 'Residual' && handler.end && handler.state?.duration) {
            //     handler.state.duration--;
            //     if (!handler.state.duration) {
            //         const endCallArgs = handler.endCallArgs || [handler.effectHolder, effect.id];
            //         handler.end.call(...endCallArgs as [any, ...any[]]);
            //         if (this.ended) return;
            //         continue;
            //     }
            // }
            // Note: handler.end is the end callback (clearWeather, clearTerrain, removePseudoWeather)
            // This is different from handler.callback (the onFieldResidual callback).
            // Duration is decremented and effects are cleared based on handler.end, NOT handler.callback.
            eprintln!("[FIELD_EVENT HANDLER] event={}, effect_id={}, is_field={}, is_side={}",
                event_id, handler.effect_id.as_str(), handler.is_field, handler.is_side);
            if event_id == "Residual" && handler.is_field {
                eprintln!("[FIELD_EVENT RESIDUAL FIELD] effect_id={}, weather={}, terrain={}",
                    handler.effect_id.as_str(), self.field.weather.as_str(), self.field.terrain.as_str());

                let should_clear = {
                    // Check weather
                    if self.field.weather == handler.effect_id {
                        eprintln!("[WEATHER DURATION] Checking sandstorm duration, current={:?}", self.field.weather_state.duration);
                        if let Some(duration) = self.field.weather_state.duration.as_mut() {
                            eprintln!("[WEATHER DURATION] BEFORE decrement: duration={}", *duration);
                            *duration -= 1;
                            eprintln!("[WEATHER DURATION] AFTER decrement: duration={}", *duration);
                            *duration == 0
                        } else {
                            eprintln!("[WEATHER DURATION] No duration set (permanent weather)");
                            false
                        }
                    }
                    // Check terrain
                    else if self.field.terrain == handler.effect_id {
                        eprintln!("[TERRAIN DURATION] Checking terrain duration, current={:?}", self.field.terrain_state.duration);
                        if let Some(duration) = self.field.terrain_state.duration.as_mut() {
                            eprintln!("[TERRAIN DURATION] BEFORE decrement: duration={}", *duration);
                            *duration -= 1;
                            eprintln!("[TERRAIN DURATION] AFTER decrement: duration={}", *duration);
                            *duration == 0
                        } else {
                            eprintln!("[TERRAIN DURATION] No duration set (permanent terrain)");
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

                // Clear expired field effects and call FieldEnd
                if should_clear {
                    // Call FieldEnd event before clearing
                    self.single_event(
                        "FieldEnd",
                        &crate::battle::Effect::new(handler.effect_id.clone(), handler._effect_type),
                        None,
                        None,
                        None,
                        None,
                        None,
                    );

                    // Clear weather
                    // JavaScript (field.ts clearWeather):
                    //   this.weather = '';
                    //   this.battle.clearEffectState(this.weatherState);
                    //   this.battle.eachEvent('WeatherChange');
                    if self.field.weather == handler.effect_id {
                        self.field.weather = ID::new("");
                        self.field.weather_state = crate::event_system::EffectState::new(ID::new(""));
                        // JavaScript calls eachEvent('WeatherChange') after clearing weather
                        self.each_event("WeatherChange", None, None);
                    }
                    // Clear terrain
                    // JavaScript (field.ts clearTerrain):
                    //   this.terrain = '';
                    //   this.battle.clearEffectState(this.terrainState);
                    //   this.battle.eachEvent('TerrainChange');
                    else if self.field.terrain == handler.effect_id {
                        self.field.terrain = ID::new("");
                        self.field.terrain_state = crate::event_system::EffectState::new(ID::new(""));
                        // JavaScript calls eachEvent('TerrainChange') after clearing terrain
                        self.each_event("TerrainChange", None, None);
                    }
                    // Clear pseudo-weather
                    else {
                        self.field.pseudo_weather.shift_remove(&handler.effect_id);
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
                                    // JavaScript: Always decrement duration during Residual events
                                    // JavaScript code (battle.ts:515-522):
                                    // if (eventid === 'Residual' && handler.end && handler.state?.duration) {
                                    //     handler.state.duration--;
                                    //     if (!handler.state.duration) { /* remove volatile */ }
                                    // }
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
                        // Call End event before removing (this is what faints Pokemon for Perish Song, etc.)
                        // JS: handler.end.call(this, target, pokemon.volatiles[handler.effectId]);
                        eprintln!("[FIELD_EVENT RESIDUAL] turn={}, volatile='{}' calling End event before removal",
                            self.turn, handler.effect_id.as_str());

                        // Get the volatile's state to pass to the End callback
                        // This is critical for twoturnmove which stores move_id in effectState
                        let volatile_state = self.sides.get(side_idx)
                            .and_then(|s| s.pokemon.get(poke_idx))
                            .and_then(|p| p.volatiles.get(&handler.effect_id))
                            .cloned();

                        self.single_event("End", &crate::battle::Effect::condition(handler.effect_id.clone()),
                            volatile_state.as_ref(), Some((side_idx, poke_idx)), None, None, None);

                        // Actually remove the volatile
                        if let Some(pokemon) = self.sides.get_mut(side_idx)
                            .and_then(|s| s.pokemon.get_mut(poke_idx)) {
                            pokemon.volatiles.remove(&handler.effect_id);
                            eprintln!("[FIELD_EVENT RESIDUAL] turn={}, volatile='{}' REMOVED from pokemon ({}, {})",
                                self.turn, handler.effect_id.as_str(), side_idx, poke_idx);
                        }
                        // Skip calling the residual handler for expired effects
                        if self.ended {
                            return;
                        }
                        continue;
                    }
                }
            }

            // Handle side condition duration decrements
            if event_id == "Residual" && handler.is_side {
                if let Some(side_idx) = handler.side_idx {
                    let should_remove = {
                        if let Some(side) = self.sides.get_mut(side_idx) {
                            if let Some(condition_state) = side.side_conditions.get_mut(&handler.effect_id) {
                                if let Some(duration) = condition_state.duration.as_mut() {
                                    // JavaScript: handler.state.duration--; if (!handler.state.duration) { handler.end.call(...); }
                                    eprintln!("[FIELD_EVENT RESIDUAL] turn={}, side_condition='{}', side={}, duration BEFORE decrement={}",
                                        self.turn, handler.effect_id.as_str(), side_idx, *duration);
                                    *duration -= 1;
                                    eprintln!("[FIELD_EVENT RESIDUAL] turn={}, side_condition='{}', side={}, duration AFTER decrement={}",
                                        self.turn, handler.effect_id.as_str(), side_idx, *duration);
                                    if *duration == 0 {
                                        eprintln!("[FIELD_EVENT RESIDUAL] turn={}, side_condition='{}' EXPIRED on side {}, calling SideEnd",
                                            self.turn, handler.effect_id.as_str(), side_idx);
                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    };

                    // Remove expired side condition
                    if should_remove {
                        // Call SideEnd event before removing
                        let end_event = format!("SideEnd");
                        self.single_event(&end_event, &crate::battle::Effect::side_condition(handler.effect_id.clone()), None, None, None, None, None);

                        // Actually remove the side condition
                        if let Some(side) = self.sides.get_mut(side_idx) {
                            side.side_conditions.remove(&handler.effect_id);
                            eprintln!("[FIELD_EVENT RESIDUAL] turn={}, side_condition='{}' REMOVED from side {}",
                                self.turn, handler.effect_id.as_str(), side_idx);
                        }

                        // Skip calling the residual handler for expired effects
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
            // Use the callback_name stored in the handler - this correctly handles "AnySwitchIn" for onAnySwitchIn handlers
            let handler_event_id = handler.callback_name.clone();

            // JS: if (handler.callback) {
            //         this.singleEvent(handlerEventid, effect, handler.state, handler.effectHolder, null, null, undefined, handler.callback);
            //     }
            // Only call the callback if one exists - JavaScript handlers can be added for duration without a callback
            if handler.has_callback {
                // Special handling for side condition Residual events
                // These callbacks take a Pokemon as target and should be called for each active Pokemon on the side
                if handler.is_side && event_id == "Residual" {
                    // Only process the specific side this handler is for (not all sides with the condition)
                    if let Some(side_idx) = handler.side_idx {
                        if self.sides[side_idx].side_conditions.contains_key(&handler.effect_id) {
                            // Get active Pokemon positions on this side
                            let active_positions: Vec<usize> = self.sides[side_idx].active
                                .iter()
                                .flatten()
                                .copied()
                                .collect();

                            // Call the callback for each active Pokemon on this side
                            for poke_idx in active_positions {
                                let target_pos = (side_idx, poke_idx);
                                eprintln!("[FIELD_EVENT] Calling side condition {} Residual for Pokemon {:?}",
                                    handler.effect_id.as_str(), target_pos);
                                self.single_event(&handler_event_id, &crate::battle::Effect::side_condition(handler.effect_id.clone()), None, Some(target_pos), None, None, None);

                                // JS: this.faintMessages();
                                self.faint_messages(false, false, true);

                                // JS: if (this.ended) return;
                                if self.ended {
                                    return;
                                }
                            }
                        }
                    }
                } else {
                    // Normal handling for non-side-Residual events
                    eprintln!("[FIELD_EVENT] Calling single_event for event='{}', effect='{}', turn={}",
                        handler_event_id, handler.effect_id.as_str(), self.turn);

                    // For slot conditions, we need to pass the actual state so callbacks can access it
                    // Clone the state to avoid borrow checker issues with self.single_event
                    let state_owned = if handler._effect_type == crate::battle::EffectType::SlotCondition {
                        if let Some((side_idx, poke_idx)) = handler.holder {
                            // IMPORTANT: handler.holder is (side_idx, poke_idx) which is the party index
                            // But slot_conditions are indexed by pokemon.position (active slot: 0, 1, 2...)
                            // We need to convert poke_idx to position first
                            let position = self.sides.get(side_idx)
                                .and_then(|side| side.pokemon.get(poke_idx))
                                .map(|pokemon| pokemon.position);

                            if let Some(pos) = position {
                                self.sides.get(side_idx)
                                    .and_then(|side| side.slot_conditions.get(pos))
                                    .and_then(|slot_conds| slot_conds.get(&handler.effect_id))
                                    .cloned()
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else if handler._effect_type == crate::battle::EffectType::SideCondition {
                        if let Some(side_idx) = handler.side_idx {
                            self.sides.get(side_idx)
                                .and_then(|side| side.side_conditions.get(&handler.effect_id))
                                .cloned()
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    self.single_event(&handler_event_id, &crate::battle::Effect::new(handler.effect_id.clone(), handler._effect_type), state_owned.as_ref(), handler.holder, None, None, None);

                    // JS: this.faintMessages();
                    self.faint_messages(false, false, true);

                    // JS: if (this.ended) return;
                    if self.ended {
                        return;
                    }
                }
            }
        }
    }
}
