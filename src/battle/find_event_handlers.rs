use crate::*;
use crate::battle::EventListener;

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
    /// - Currently simplified: only handles Pokemon targets (no Side/Battle distinction)
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
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
    ) -> Vec<EventListener> {
        let mut handlers = Vec::new();

        // JavaScript: const prefixedHandlers = !['BeforeTurn', 'Update', 'Weather', 'WeatherChange', 'TerrainChange'].includes(eventName);
        let event_name = event_id.trim_start_matches("on");
        let prefixed_handlers = !matches!(
            event_name,
            "BeforeTurn" | "Update" | "Weather" | "WeatherChange" | "TerrainChange"
        );

        // JavaScript: if (target instanceof Pokemon && (target.isActive || source?.isActive))
        // Rust: We only handle Pokemon targets currently
        if let Some(target_pos) = target {
            // JavaScript: handlers = this.findPokemonEventHandlers(target, `on${eventName}`);
            let prefixed_event = format!("on{}", event_name);
            let mut pokemon_handlers = self.find_pokemon_event_handlers(&prefixed_event, target_pos, None);
            // Add event name to each handler and resolve priority
            for handler in &mut pokemon_handlers {
                handler.event_name = event_name.to_string();
            }
            handlers.extend(pokemon_handlers);

            if prefixed_handlers {
                let (target_side, _target_idx) = target_pos;

                // Add prefixed handlers (onAlly, onFoe, onAny)
                // JavaScript:
                // for (const allyActive of target.alliesAndSelf()) {
                //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAlly${eventName}`));
                //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAny${eventName}`));
                // }

                // Get all active Pokemon on target's side (allies and self)
                if let Some(side) = self.sides.get(target_side) {
                    for poke_idx in side.active.iter().flatten() {
                        let ally_pos = (target_side, *poke_idx);
                        // onAlly handlers
                        let ally_event = format!("onAlly{}", event_name);
                        let mut ally_handlers =
                            self.find_pokemon_event_handlers(&ally_event, ally_pos, None);
                        for handler in &mut ally_handlers {
                            handler.event_name = format!("Ally{}", event_name);
                        }
                        handlers.extend(ally_handlers);

                        // onAny handlers
                        let any_event = format!("onAny{}", event_name);
                        let mut any_handlers =
                            self.find_pokemon_event_handlers(&any_event, ally_pos, None);
                        for handler in &mut any_handlers {
                            handler.event_name = format!("Any{}", event_name);
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
                for (side_idx, side) in self.sides.iter().enumerate() {
                    if side_idx != target_side {
                        eprintln!("[FIND_EVENT_HANDLERS] Looking for onFoe{} handlers, target_side={}, checking opposing side={}",
                            event_name, target_side, side_idx);
                        eprintln!("[FIND_EVENT_HANDLERS] Opposing side.active = {:?}", side.active);
                        for poke_idx in side.active.iter().flatten() {
                            let foe_pos = (side_idx, *poke_idx);
                            eprintln!("[FIND_EVENT_HANDLERS] Checking foe at position {:?} for onFoe{}", foe_pos, event_name);
                            // onFoe handlers
                            let foe_event = format!("onFoe{}", event_name);
                            let mut foe_handlers =
                                self.find_pokemon_event_handlers(&foe_event, foe_pos, None);
                            eprintln!("[FIND_EVENT_HANDLERS] Found {} onFoe{} handlers from position {:?}",
                                foe_handlers.len(), event_name, foe_pos);
                            for handler in &mut foe_handlers {
                                handler.event_name = format!("Foe{}", event_name);
                            }
                            handlers.extend(foe_handlers);

                            // onAny handlers
                            let any_event = format!("onAny{}", event_name);
                            let mut any_handlers =
                                self.find_pokemon_event_handlers(&any_event, foe_pos, None);
                            for handler in &mut any_handlers {
                                handler.event_name = format!("Any{}", event_name);
                            }
                            handlers.extend(any_handlers);
                        }
                    }
                }
            }
        }

        // JavaScript: if (source && prefixedHandlers) {
        //     handlers.push(...this.findPokemonEventHandlers(source, `onSource${eventName}`));
        // }
        if let Some(source_pos) = source {
            if prefixed_handlers {
                let source_event = format!("onSource{}", event_name);
                let mut source_handlers =
                    self.find_pokemon_event_handlers(&source_event, source_pos, None);
                for handler in &mut source_handlers {
                    handler.event_name = format!("Source{}", event_name);
                }
                handlers.extend(source_handlers);
            }
        }

        // JavaScript (lines 56-77):
        // After Pokemon handlers, target becomes target.side
        // Then for each side in battle, find side event handlers
        // if (target instanceof Side) {
        //     for (const side of this.sides) {
        //         // ... shouldBubbleDown logic (not implemented yet) ...
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

        // When target is a Pokemon, convert to its side for side event handlers
        let target_side_idx = target.map(|(side, _)| side);

        eprintln!("[FIND_EVENT_HANDLERS] About to check side handlers, target_side_idx={:?}, event_name={}", target_side_idx, event_name);

        // Loop through all sides to find side event handlers
        for side_idx in 0..self.sides.len() {
            // JavaScript: if (side.n < 2 || !side.allySide)
            // In Rust, we don't have allySide yet, so we just check all sides

            if let Some(target_side) = target_side_idx {
                eprintln!("[FIND_EVENT_HANDLERS] Checking side {} (target_side={})", side_idx, target_side);
                // JavaScript: if (side === target || side === target.allySide)
                if side_idx == target_side {
                    // Find handlers for on${eventName}
                    let prefixed_event = format!("on{}", event_name);
                    eprintln!("[FIND_EVENT_HANDLERS] Calling find_side_event_handlers for side {}, callback={}", side_idx, prefixed_event);
                    let mut side_handlers = self.find_side_event_handlers(&prefixed_event, side_idx, None, None);
                    eprintln!("[FIND_EVENT_HANDLERS] Found {} side handlers for {}", side_handlers.len(), prefixed_event);
                    for handler in &mut side_handlers {
                        handler.event_name = event_name.to_string();
                    }
                    handlers.extend(side_handlers);
                } else if prefixed_handlers {
                    // JavaScript: else if (prefixedHandlers)
                    //     handlers.push(...this.findSideEventHandlers(side, `onFoe${eventName}`));
                    let foe_event = format!("onFoe{}", event_name);
                    let mut foe_side_handlers = self.find_side_event_handlers(&foe_event, side_idx, None, None);
                    for handler in &mut foe_side_handlers {
                        handler.event_name = format!("Foe{}", event_name);
                    }
                    handlers.extend(foe_side_handlers);
                }

                // JavaScript: if (prefixedHandlers) handlers.push(...this.findSideEventHandlers(side, `onAny${eventName}`));
                if prefixed_handlers {
                    let any_event = format!("onAny{}", event_name);
                    let mut any_side_handlers = self.find_side_event_handlers(&any_event, side_idx, None, None);
                    for handler in &mut any_side_handlers {
                        handler.event_name = format!("Any{}", event_name);
                    }
                    handlers.extend(any_side_handlers);
                }
            }
        }

        // JavaScript: handlers.push(...this.findFieldEventHandlers(this.field, `on${eventName}`));
        // In JavaScript, findFieldEventHandlers is called with the target Pokemon as customHolder
        // so that weather callbacks (like sandstorm's onModifySpD) know which Pokemon to check
        let prefixed_event = format!("on{}", event_name);
        let mut field_handlers = self.find_field_event_handlers(&prefixed_event, None, target);
        for handler in &mut field_handlers {
            handler.event_name = event_name.to_string();
        }
        handlers.extend(field_handlers);

        // JavaScript: handlers.push(...this.findBattleEventHandlers(`on${eventName}`));
        let mut battle_handlers = self.find_battle_event_handlers(&prefixed_event, None, None);
        for handler in &mut battle_handlers {
            handler.event_name = event_name.to_string();
        }
        handlers.extend(battle_handlers);

        // JavaScript: In findPokemonEventHandlers, each handler is passed through resolvePriority before being added:
        // handlers.push(this.resolvePriority({ effect, callback, state, end, effectHolder }, callbackName));
        // In Rust, we collect all handlers first, then call resolve_priority on each one
        for handler in &mut handlers {
            // Use the full callback name (e.g., "onSourceModifySpA" not just "ModifySpA")
            let full_callback_name = format!("on{}", handler.event_name);
            self.resolve_priority(handler, &full_callback_name);
        }

        handlers
    }
}
