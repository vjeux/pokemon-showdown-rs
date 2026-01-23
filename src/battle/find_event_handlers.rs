use crate::*;
use crate::battle::EventListener;
use crate::event::EventTarget;

impl Battle {

    /// Find all event handlers for an event
    /// Equivalent to battle.ts findEventHandlers()
    ///
    /// JavaScript (battle.ts:1036-1096):
    /// - Handles array targets recursively
    /// - Distinguishes Pokemon/Side/Battle types
    /// - Implements bubble down for Side events
    /// - Adds prefixed handlers (onAlly, onFoe, onAny, onSource)
    ///
    /// Rust Implementation Notes:
    /// - Handles Pokemon and Side targets
    /// - No array recursion (not used in current Rust codebase)
    /// - Calls specialized find*EventHandlers methods
    /// - Returns full EventListener structs for proper sorting
    //
    // 	findEventHandlers(target: Pokemon | Pokemon[] | Side | Battle, eventName: string, source?: Pokemon | null) {
    // 		let handlers: EventListener[] = [];
    // 		if (Array.isArray(target)) {
    // 			for (const [i, pokemon] of target.entries()) {
    // 				// console.log(`Event: ${eventName}, Target: ${pokemon}, ${i}`);
    // 				const curHandlers = this.findEventHandlers(pokemon, eventName, source);
    // 				for (const handler of curHandlers) {
    // 					handler.target = pokemon; // Original "effectHolder"
    // 					handler.index = i;
    // 				}
    // 				handlers = handlers.concat(curHandlers);
    // 			}
    // 			return handlers;
    // 		}
    // 		// events that target a Pokemon normally bubble up to the Side
    // 		const shouldBubbleDown = target instanceof Side;
    // 		// events usually run through EachEvent should never have any handlers besides `on${eventName}` so don't check for them
    // 		const prefixedHandlers = !['BeforeTurn', 'Update', 'Weather', 'WeatherChange', 'TerrainChange'].includes(eventName);
    // 		if (target instanceof Pokemon && (target.isActive || source?.isActive)) {
    // 			handlers = this.findPokemonEventHandlers(target, `on${eventName}`);
    // 			if (prefixedHandlers) {
    // 				for (const allyActive of target.alliesAndSelf()) {
    // 					handlers.push(...this.findPokemonEventHandlers(allyActive, `onAlly${eventName}`));
    // 					handlers.push(...this.findPokemonEventHandlers(allyActive, `onAny${eventName}`));
    // 				}
    // 				for (const foeActive of target.foes()) {
    // 					handlers.push(...this.findPokemonEventHandlers(foeActive, `onFoe${eventName}`));
    // 					handlers.push(...this.findPokemonEventHandlers(foeActive, `onAny${eventName}`));
    // 				}
    // 			}
    // 			target = target.side;
    // 		}
    // 		if (source && prefixedHandlers) {
    // 			handlers.push(...this.findPokemonEventHandlers(source, `onSource${eventName}`));
    // 		}
    // 		if (target instanceof Side) {
    // 			for (const side of this.sides) {
    // 				if (shouldBubbleDown) {
    // 					for (const active of side.active) {
    // 						if (side === target || side === target.allySide) {
    // 							handlers = handlers.concat(this.findPokemonEventHandlers(active, `on${eventName}`));
    // 						} else if (prefixedHandlers) {
    // 							handlers = handlers.concat(this.findPokemonEventHandlers(active, `onFoe${eventName}`));
    // 						}
    // 						if (prefixedHandlers) handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventName}`));
    // 					}
    // 				}
    // 				if (side.n < 2 || !side.allySide) {
    // 					if (side === target || side === target.allySide) {
    // 						handlers.push(...this.findSideEventHandlers(side, `on${eventName}`));
    // 					} else if (prefixedHandlers) {
    // 						handlers.push(...this.findSideEventHandlers(side, `onFoe${eventName}`));
    // 					}
    // 					if (prefixedHandlers) handlers.push(...this.findSideEventHandlers(side, `onAny${eventName}`));
    // 				}
    // 			}
    // 		}
    // 		handlers.push(...this.findFieldEventHandlers(this.field, `on${eventName}`));
    // 		handlers.push(...this.findBattleEventHandlers(`on${eventName}`));
    // 		return handlers;
    // 	}
    //
    pub fn find_event_handlers(
        &mut self,
        event_id: &str,
        target: Option<&EventTarget>,
        source: Option<(usize, usize)>,
    ) -> Vec<EventListener> {
        let mut handlers = Vec::new();

        // JavaScript: const prefixedHandlers = !['BeforeTurn', 'Update', 'Weather', 'WeatherChange', 'TerrainChange'].includes(eventName);
        let event_name = event_id.trim_start_matches("on");
        let prefixed_handlers = !matches!(
            event_name,
            "BeforeTurn" | "Update" | "Weather" | "WeatherChange" | "TerrainChange"
        );

        // JavaScript: const shouldBubbleDown = target instanceof Side;
        let should_bubble_down = matches!(target, Some(EventTarget::Side(_)));

        // Get target as Pokemon position if applicable
        let target_pos = target.and_then(|t| t.as_pokemon());

        // JavaScript: if (target instanceof Pokemon && (target.isActive || source?.isActive))
        // Check if target Pokemon is active OR source Pokemon is active
        // If neither is active, we skip the Pokemon handlers AND side handlers
        // (because in JS, target = target.side is inside this block, so if we skip it,
        // target stays as Pokemon and the "if (target instanceof Side)" check fails)
        let target_is_active = target_pos
            .and_then(|pos| self.pokemon_at(pos.0, pos.1))
            .map(|p| p.is_active)
            .unwrap_or(false);
        let source_is_active = source
            .and_then(|pos| self.pokemon_at(pos.0, pos.1))
            .map(|p| p.is_active)
            .unwrap_or(false);
        let should_find_pokemon_handlers = target_is_active || source_is_active;

        // Get target as Side index if applicable
        // IMPORTANT: For Pokemon targets, we only set this if the active check passes
        // This mimics JavaScript's "target = target.side" which is inside the isActive block
        let target_side_from_target = match target {
            Some(EventTarget::Side(idx)) => Some(*idx),
            Some(EventTarget::Pokemon((side, _))) if should_find_pokemon_handlers => Some(*side),
            _ => None,
        };

        // JavaScript: if (target instanceof Pokemon && (target.isActive || source?.isActive))
        if let Some(target_pokemon_pos) = target_pos {
            // Skip if neither target nor source is active
            if !should_find_pokemon_handlers {
                // In JavaScript, when this check fails, the entire block is skipped
                // including "target = target.side", so side handlers are also not found
                // We achieve this by having target_side_from_target be None above
            } else {
            // JavaScript: handlers = this.findPokemonEventHandlers(target, `on${eventName}`);
            let prefixed_event = format!("on{}", event_name);
            let mut pokemon_handlers = self.find_pokemon_event_handlers(&prefixed_event, target_pokemon_pos, None);
            // Add event name to each handler and resolve priority
            for handler in &mut pokemon_handlers {
                handler.callback_name = event_name.to_string();
            }
            handlers.extend(pokemon_handlers);

            if prefixed_handlers {
                let (target_side, _target_idx) = target_pokemon_pos;

                // Add prefixed handlers (onAlly, onFoe, onAny)
                // JavaScript:
                // for (const allyActive of target.alliesAndSelf()) {
                //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAlly${eventName}`));
                //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAny${eventName}`));
                // }

                // Get all active Pokemon on target's side (allies and self)
                // JavaScript: alliesAndSelf() calls side.allies() which filters out fainted Pokemon (!!ally.hp)
                if let Some(side) = self.sides.get(target_side) {
                    for poke_idx in side.active.iter().flatten() {
                        let ally_pos = (target_side, *poke_idx);

                        // JavaScript: allies() method filters out fainted Pokemon
                        // allies(all?: boolean) { ... if (!all) allies = allies.filter(ally => !!ally.hp); ... }
                        // Note: JavaScript uses !!ally.hp which is false when hp === 0
                        let ally_has_no_hp = self.pokemon_at(ally_pos.0, ally_pos.1)
                            .map(|p| p.hp == 0)
                            .unwrap_or(true); // If not found, treat as fainted
                        if ally_has_no_hp {
                            continue;
                        }

                        // onAlly handlers
                        let ally_event = format!("onAlly{}", event_name);
                        let mut ally_handlers =
                            self.find_pokemon_event_handlers(&ally_event, ally_pos, None);
                        for handler in &mut ally_handlers {
                            handler.callback_name = format!("Ally{}", event_name);
                        }
                        handlers.extend(ally_handlers);

                        // onAny handlers
                        let any_event = format!("onAny{}", event_name);
                        let mut any_handlers =
                            self.find_pokemon_event_handlers(&any_event, ally_pos, None);
                        for handler in &mut any_handlers {
                            handler.callback_name = format!("Any{}", event_name);
                        }
                        handlers.extend(any_handlers);
                    }
                }

                // JavaScript:
                // for (const foeActive of target.foes()) {
                //     handlers.push(...this.findPokemonEventHandlers(foeActive, `onFoe${eventName}`));
                //     handlers.push(...this.findPokemonEventHandlers(foeActive, `onAny${eventName}`));
                // }

                // Get all active Pokemon on opposing side(s) (foes)
                // JavaScript: target.foes() filters out fainted Pokemon (!!ally.hp)
                for (side_idx, side) in self.sides.iter().enumerate() {
                    if side_idx != target_side {
                        for poke_idx in side.active.iter().flatten() {
                            let foe_pos = (side_idx, *poke_idx);

                            // JavaScript: foes() method filters out fainted Pokemon
                            // allies(all?: boolean) { ... if (!all) allies = allies.filter(ally => !!ally.hp); ... }
                            // Note: JavaScript uses !!ally.hp which is false when hp === 0
                            // We need to check if the foe Pokemon has 0 HP and skip it
                            let foe_has_no_hp = self.pokemon_at(foe_pos.0, foe_pos.1)
                                .map(|p| p.hp == 0)
                                .unwrap_or(true); // If not found, treat as fainted
                            if foe_has_no_hp {
                                continue;
                            }

                            // onFoe handlers
                            let foe_event = format!("onFoe{}", event_name);
                            let mut foe_handlers =
                                self.find_pokemon_event_handlers(&foe_event, foe_pos, None);
                            for handler in &mut foe_handlers {
                                handler.callback_name = format!("Foe{}", event_name);
                            }
                            handlers.extend(foe_handlers);

                            // onAny handlers
                            let any_event = format!("onAny{}", event_name);
                            let mut any_handlers =
                                self.find_pokemon_event_handlers(&any_event, foe_pos, None);
                            for handler in &mut any_handlers {
                                handler.callback_name = format!("Any{}", event_name);
                            }
                            handlers.extend(any_handlers);
                        }
                    }
                }
            }
            } // end of else for should_find_pokemon_handlers
        }

        // JavaScript: if (source && prefixedHandlers) {
        //     handlers.push(...this.findPokemonEventHandlers(source, `onSource${eventName}`));
        // }
        if let Some(source_pos) = source {
            if prefixed_handlers {
                let source_event = format!("onSource{}", event_name);
                debug_elog!("[FIND_EVENT_HANDLERS] Looking for source event '{}' on source {:?}", source_event, source_pos);
                let mut source_handlers =
                    self.find_pokemon_event_handlers(&source_event, source_pos, None);
                debug_elog!("[FIND_EVENT_HANDLERS] Found {} source handlers for {}", source_handlers.len(), source_event);
                for handler in &mut source_handlers {
                    debug_elog!("[FIND_EVENT_HANDLERS] Source handler: {:?}", handler.effect.id);
                    handler.callback_name = format!("Source{}", event_name);
                }
                handlers.extend(source_handlers);
            }
        }

        // JavaScript (lines 57-78):
        // if (target instanceof Side) {
        //     for (const side of this.sides) {
        //         if (shouldBubbleDown) {
        //             for (const active of side.active) {
        //                 if (side === target || side === target.allySide) {
        //                     handlers = handlers.concat(this.findPokemonEventHandlers(active, `on${eventName}`));
        //                 } else if (prefixedHandlers) {
        //                     handlers = handlers.concat(this.findPokemonEventHandlers(active, `onFoe${eventName}`));
        //                 }
        //                 if (prefixedHandlers) handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventName}`));
        //             }
        //         }
        //         if (side.n < 2 || !side.allySide) {
        //             if (side === target || side === target.allySide) {
        //                 handlers.push(...this.findSideEventHandlers(side, `on${eventName}`));
        //             } else if (prefixedHandlers) {
        //                 handlers.push(...this.findSideEventHandlers(side, `onFoe${eventName}`));
        //             }
        //             if (prefixedHandlers) handlers.push(...this.findSideEventHandlers(side, `onAny${eventName}`));
        //         }
        //     }
        // }

        // Use target_side_from_target which handles both Pokemon and Side targets
        let target_side_idx = target_side_from_target;

        // Loop through all sides to find side event handlers
        for side_idx in 0..self.sides.len() {
            // JavaScript: if (shouldBubbleDown)
            // When target is a Side directly, we need to bubble DOWN to find Pokemon handlers
            if should_bubble_down {
                // Get active Pokemon on this side
                let active_pokemon: Vec<(usize, usize)> = self.sides.get(side_idx)
                    .map(|side| side.active.iter().flatten().map(|idx| (side_idx, *idx)).collect())
                    .unwrap_or_default();

                for poke_pos in active_pokemon {
                    // JavaScript: if (side === target || side === target.allySide)
                    if let Some(target_side) = target_side_idx {
                        if side_idx == target_side {
                            // Find handlers for on${eventName}
                            let prefixed_event = format!("on{}", event_name);
                            let mut pokemon_handlers = self.find_pokemon_event_handlers(&prefixed_event, poke_pos, None);
                            for handler in &mut pokemon_handlers {
                                handler.callback_name = event_name.to_string();
                            }
                            handlers.extend(pokemon_handlers);
                        } else if prefixed_handlers {
                            // JavaScript: else if (prefixedHandlers)
                            //     handlers = handlers.concat(this.findPokemonEventHandlers(active, `onFoe${eventName}`));
                            let foe_event = format!("onFoe{}", event_name);
                            let mut foe_handlers = self.find_pokemon_event_handlers(&foe_event, poke_pos, None);
                            for handler in &mut foe_handlers {
                                handler.callback_name = format!("Foe{}", event_name);
                            }
                            handlers.extend(foe_handlers);
                        }

                        // JavaScript: if (prefixedHandlers) handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventName}`));
                        if prefixed_handlers {
                            let any_event = format!("onAny{}", event_name);
                            let mut any_handlers = self.find_pokemon_event_handlers(&any_event, poke_pos, None);
                            for handler in &mut any_handlers {
                                handler.callback_name = format!("Any{}", event_name);
                            }
                            handlers.extend(any_handlers);
                        }
                    }
                }
            }

            // JavaScript: if (side.n < 2 || !side.allySide)
            // In Rust, we don't have allySide yet, so we just check all sides
            if let Some(target_side) = target_side_idx {
                // JavaScript: if (side === target || side === target.allySide)
                if side_idx == target_side {
                    // Find handlers for on${eventName}
                    let prefixed_event = format!("on{}", event_name);
                    let mut side_handlers = self.find_side_event_handlers(&prefixed_event, side_idx, None, None);
                    for handler in &mut side_handlers {
                        handler.callback_name = event_name.to_string();
                    }
                    handlers.extend(side_handlers);
                } else if prefixed_handlers {
                    // JavaScript: else if (prefixedHandlers)
                    //     handlers.push(...this.findSideEventHandlers(side, `onFoe${eventName}`));
                    let foe_event = format!("onFoe{}", event_name);
                    let mut foe_side_handlers = self.find_side_event_handlers(&foe_event, side_idx, None, None);
                    for handler in &mut foe_side_handlers {
                        handler.callback_name = format!("Foe{}", event_name);
                    }
                    handlers.extend(foe_side_handlers);
                }

                // JavaScript: if (prefixedHandlers) handlers.push(...this.findSideEventHandlers(side, `onAny${eventName}`));
                if prefixed_handlers {
                    let any_event = format!("onAny{}", event_name);
                    let mut any_side_handlers = self.find_side_event_handlers(&any_event, side_idx, None, None);
                    for handler in &mut any_side_handlers {
                        handler.callback_name = format!("Any{}", event_name);
                    }
                    handlers.extend(any_side_handlers);
                }
            }
        }

        // JavaScript: handlers.push(...this.findFieldEventHandlers(this.field, `on${eventName}`));
        // Note: JavaScript does NOT pass customHolder here - that's only done in fieldEvent()
        // We must NOT pass target here or the field handler will incorrectly inherit the Pokemon's speed
        let prefixed_event = format!("on{}", event_name);
        let mut field_handlers = self.find_field_event_handlers(&prefixed_event, None, None);
        for handler in &mut field_handlers {
            handler.callback_name = event_name.to_string();
        }
        handlers.extend(field_handlers);

        // JavaScript: handlers.push(...this.findBattleEventHandlers(`on${eventName}`));
        let mut battle_handlers = self.find_battle_event_handlers(&prefixed_event, None, None);
        for handler in &mut battle_handlers {
            handler.callback_name = event_name.to_string();
        }
        handlers.extend(battle_handlers);

        // JavaScript: In findPokemonEventHandlers, each handler is passed through resolvePriority before being added:
        // handlers.push(this.resolvePriority({ effect, callback, state, end, effectHolder }, callbackName));
        // In Rust, we collect all handlers first, then call resolve_priority on each one
        for handler in &mut handlers {
            // Use the full callback name (e.g., "onSourceModifySpA" not just "ModifySpA")
            let full_callback_name = format!("on{}", handler.callback_name);
            self.resolve_priority(handler, &full_callback_name);
        }

        handlers
    }
}
