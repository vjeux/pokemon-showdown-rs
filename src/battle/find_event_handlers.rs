use crate::*;

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
    /// - TODO: Add prefixed handler support when needed
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
    ) -> Vec<(String, ID, Option<(usize, usize)>)> {  // Now returns (event_variant, effect_id, target)
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
            let pokemon_handlers = self.find_pokemon_event_handlers(&prefixed_event, target_pos, None);
            // Add event variant name to each handler
            for handler in pokemon_handlers {
                handlers.push((event_name.to_string(), handler.effect_id, handler.effect_holder));
            }

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
                        let ally_variant = format!("Ally{}", event_name);
                        let ally_event = format!("onAlly{}", event_name);
                        let ally_handlers =
                            self.find_pokemon_event_handlers(&ally_event, ally_pos, None);
                        for handler in ally_handlers {
                            handlers.push((ally_variant.clone(), handler.effect_id, handler.effect_holder));
                        }

                        // onAny handlers
                        let any_variant = format!("Any{}", event_name);
                        let any_event = format!("onAny{}", event_name);
                        let any_handlers =
                            self.find_pokemon_event_handlers(&any_event, ally_pos, None);
                        for handler in any_handlers {
                            handlers.push((any_variant.clone(), handler.effect_id, handler.effect_holder));
                        }
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
                        for poke_idx in side.active.iter().flatten() {
                            let foe_pos = (side_idx, *poke_idx);
                            // onFoe handlers
                            let foe_variant = format!("Foe{}", event_name);
                            let foe_event = format!("onFoe{}", event_name);
                            let foe_handlers =
                                self.find_pokemon_event_handlers(&foe_event, foe_pos, None);
                            for handler in foe_handlers {
                                handlers.push((foe_variant.clone(), handler.effect_id, handler.effect_holder));
                            }

                            // onAny handlers
                            let any_variant = format!("Any{}", event_name);
                            let any_event = format!("onAny{}", event_name);
                            let any_handlers =
                                self.find_pokemon_event_handlers(&any_event, foe_pos, None);
                            for handler in any_handlers {
                                handlers.push((any_variant.clone(), handler.effect_id, handler.effect_holder));
                            }
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
                let source_variant = format!("Source{}", event_name);
                let source_event = format!("onSource{}", event_name);
                let source_handlers =
                    self.find_pokemon_event_handlers(&source_event, source_pos, None);
                for handler in source_handlers {
                    handlers.push((source_variant.clone(), handler.effect_id, handler.effect_holder));
                }
            }
        }

        // JavaScript: handlers.push(...this.findFieldEventHandlers(this.field, `on${eventName}`));
        let prefixed_event = format!("on{}", event_name);
        let field_handlers = self.find_field_event_handlers(&prefixed_event, None, None);
        for handler in field_handlers {
            // For Weather/Update/BeforeTurn events, field effects (like sandstorm) should use the target Pokemon
            // that was passed to this function, not None. The event is being run ON that Pokemon.
            // JavaScript doesn't have this issue because it passes target directly to the callback.
            let handler_target = if !prefixed_handlers && target.is_some() {
                target
            } else {
                handler.effect_holder
            };
            handlers.push((event_name.to_string(), handler.effect_id, handler_target));
        }

        // JavaScript: handlers.push(...this.findBattleEventHandlers(`on${eventName}`));
        let battle_handlers = self.find_battle_event_handlers(&prefixed_event, None, None);
        for handler in battle_handlers {
            handlers.push((event_name.to_string(), handler.effect_id, handler.effect_holder));
        }

        handlers
    }
}
