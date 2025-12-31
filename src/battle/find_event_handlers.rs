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
        &self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
    ) -> Vec<(ID, Option<(usize, usize)>)> {
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
            let mut pokemon_handlers = self.find_pokemon_event_handlers(&prefixed_event, target_pos);
            handlers.append(&mut pokemon_handlers);

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
                            self.find_pokemon_event_handlers(&ally_event, ally_pos);
                        handlers.append(&mut ally_handlers);

                        // onAny handlers
                        let any_event = format!("onAny{}", event_name);
                        let mut any_handlers =
                            self.find_pokemon_event_handlers(&any_event, ally_pos);
                        handlers.append(&mut any_handlers);
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
                            let foe_event = format!("onFoe{}", event_name);
                            let mut foe_handlers =
                                self.find_pokemon_event_handlers(&foe_event, foe_pos);
                            handlers.append(&mut foe_handlers);

                            // onAny handlers
                            let any_event = format!("onAny{}", event_name);
                            let mut any_handlers =
                                self.find_pokemon_event_handlers(&any_event, foe_pos);
                            handlers.append(&mut any_handlers);
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
                    self.find_pokemon_event_handlers(&source_event, source_pos);
                handlers.append(&mut source_handlers);
            }
        }

        // JavaScript: handlers.push(...this.findFieldEventHandlers(this.field, `on${eventName}`));
        let prefixed_event = format!("on{}", event_name);
        let mut field_handlers = self.find_field_event_handlers(&prefixed_event);
        handlers.append(&mut field_handlers);

        // JavaScript: handlers.push(...this.findBattleEventHandlers(`on${eventName}`));
        let battle_handler_ids = self.find_battle_event_handlers(&prefixed_event);
        for id in battle_handler_ids {
            handlers.push((id, None));
        }

        handlers
    }
}
