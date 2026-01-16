use crate::*;
use crate::battle::EffectType;
use crate::battle::EventListener;
use crate::dex_data::StatID;

impl Battle {

    /// Get callback sub-order from dex data
    /// JavaScript: handler.subOrder = (handler.effect as any)[`${callbackName}SubOrder`] || 0;
    /// Returns None if no sub-order is specified (will use default based on effect type)
    pub(crate) fn get_callback_sub_order(&self, effect_type: EffectType, effect_id: &str, callback_name: &str) -> Option<i32> {
        // Extract event name from callback (e.g., "onResidual" -> "Residual")
        let event = if callback_name.starts_with("on") {
            &callback_name[2..]
        } else {
            callback_name
        };

        // JavaScript: handler.subOrder = (handler.effect as any)[`${callbackName}SubOrder`] || 0;
        // Construct the property name: e.g., "onResidualSubOrder"
        let property_name = format!("on{}SubOrder", event);

        match effect_type {
            EffectType::Ability => {
                if let Some(ability_data) = self.dex.abilities().get(effect_id) {
                    if let Some(value) = ability_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                None
            }
            EffectType::Item => {
                if let Some(item_data) = self.dex.items().get(effect_id) {
                    if let Some(value) = item_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                None
            }
            // Weather, Condition, Status, and other effect types stored in conditions dex
            EffectType::Condition | EffectType::Status | EffectType::Weather | EffectType::FieldCondition |
            EffectType::SideCondition | EffectType::SlotCondition => {
                // First check conditions dex
                if let Some(condition_data) = self.dex.conditions().get(effect_id) {
                    if let Some(value) = condition_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                // JavaScript fallback: check if it's a move/ability/item with embedded condition
                // (see dex-conditions.ts lines 687-690)
                if let Some(move_data) = self.dex.moves().get(effect_id) {
                    if let Some(condition) = &move_data.condition {
                        if let Some(value) = condition.extra.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                if let Some(ability_data) = self.dex.abilities().get(effect_id) {
                    // Ability condition is stored in extra["condition"] as JSON
                    if let Some(condition) = ability_data.extra.get("condition") {
                        if let Some(value) = condition.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                if let Some(item_data) = self.dex.items().get(effect_id) {
                    // Item condition is stored in extra["condition"] as JSON
                    if let Some(condition) = item_data.extra.get("condition") {
                        if let Some(value) = condition.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                None
            }
            // Terrain effects are stored in moves dex as embedded conditions
            EffectType::Terrain => {
                // First check conditions dex (in case terrain was added there)
                if let Some(condition_data) = self.dex.conditions().get(effect_id) {
                    if let Some(value) = condition_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                // Fall back to move embedded condition data
                if let Some(move_data) = self.dex.moves().get(effect_id) {
                    if let Some(condition) = &move_data.condition {
                        if let Some(value) = condition.extra.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Get callback order from dex data
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`] || false;
    /// Returns None if no order is specified (equivalent to false in JS)
    pub(crate) fn get_callback_order(&self, effect_type: EffectType, effect_id: &str, callback_name: &str) -> Option<i32> {
        // Extract event name from callback (e.g., "onResidual" -> "Residual")
        let event = if callback_name.starts_with("on") {
            &callback_name[2..]
        } else {
            callback_name
        };

        // JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`] || false;
        // Construct the property name: e.g., "onResidualOrder"
        let property_name = format!("on{}Order", event);

        let result = match effect_type {
            EffectType::Ability => {
                if let Some(ability_data) = self.dex.abilities().get(effect_id) {
                    if let Some(value) = ability_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                None
            }
            EffectType::Item => {
                if let Some(item_data) = self.dex.items().get(effect_id) {
                    if let Some(value) = item_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                None
            }
            // Weather, Condition, Status, and other effect types stored in conditions dex
            EffectType::Condition | EffectType::Status | EffectType::Weather | EffectType::FieldCondition |
            EffectType::SideCondition | EffectType::SlotCondition => {
                // First check conditions dex
                if let Some(condition_data) = self.dex.conditions().get(effect_id) {
                    if let Some(value) = condition_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                // JavaScript fallback: check if it's a move/ability/item with embedded condition
                // (see dex-conditions.ts lines 687-690)
                if let Some(move_data) = self.dex.moves().get(effect_id) {
                    if let Some(condition) = &move_data.condition {
                        if let Some(value) = condition.extra.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                if let Some(ability_data) = self.dex.abilities().get(effect_id) {
                    // Ability condition is stored in extra["condition"] as JSON
                    if let Some(condition) = ability_data.extra.get("condition") {
                        if let Some(value) = condition.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                if let Some(item_data) = self.dex.items().get(effect_id) {
                    // Item condition is stored in extra["condition"] as JSON
                    if let Some(condition) = item_data.extra.get("condition") {
                        if let Some(value) = condition.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                None
            }
            // Terrain effects are stored in moves dex as embedded conditions
            EffectType::Terrain => {
                // First check conditions dex (in case terrain was added there)
                if let Some(condition_data) = self.dex.conditions().get(effect_id) {
                    if let Some(value) = condition_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32);
                    }
                }
                // Fall back to move embedded condition data
                if let Some(move_data) = self.dex.moves().get(effect_id) {
                    if let Some(condition) = &move_data.condition {
                        if let Some(value) = condition.extra.get(&property_name) {
                            return value.as_i64().map(|v| v as i32);
                        }
                    }
                }
                None
            }
            _ => None,
        };
        result
    }

    /// Get callback priority from dex data
    /// JavaScript: handler.priority = (handler.effect as any)[`${callbackName}Priority`] || 0;
    /// Returns 0 if no priority is specified (default in JS)
    pub(crate) fn get_callback_priority(&self, effect_type: EffectType, effect_id: &str, callback_name: &str) -> i32 {
        // Extract event name from callback (e.g., "onBeforeMove" -> "BeforeMove")
        let event = if callback_name.starts_with("on") {
            &callback_name[2..]
        } else {
            callback_name
        };

        // JavaScript: handler.priority = (handler.effect as any)[`${callbackName}Priority`] || 0;
        // Construct the property name: e.g., "onBeforeMovePriority"
        let property_name = format!("on{}Priority", event);

        match effect_type {
            EffectType::Ability => {
                if let Some(ability_data) = self.dex.abilities().get(effect_id) {
                    if let Some(value) = ability_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32).unwrap_or(0);
                    }
                }
                0
            }
            EffectType::Item => {
                if let Some(item_data) = self.dex.items().get(effect_id) {
                    if let Some(value) = item_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32).unwrap_or(0);
                    }
                }
                0
            }
            // Weather, Condition, Status, and other effect types stored in conditions dex
            EffectType::Condition | EffectType::Status | EffectType::Weather | EffectType::FieldCondition |
            EffectType::SideCondition | EffectType::SlotCondition => {
                // First check conditions dex
                if let Some(condition_data) = self.dex.conditions().get(effect_id) {
                    if let Some(value) = condition_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32).unwrap_or(0);
                    }
                }
                // JavaScript fallback: check if it's a move/ability/item with embedded condition
                // (see dex-conditions.ts lines 687-690)
                if let Some(move_data) = self.dex.moves().get(effect_id) {
                    if let Some(condition) = &move_data.condition {
                        if let Some(value) = condition.extra.get(&property_name) {
                            return value.as_i64().map(|v| v as i32).unwrap_or(0);
                        }
                    }
                }
                if let Some(ability_data) = self.dex.abilities().get(effect_id) {
                    // Ability condition is stored in extra["condition"] as JSON
                    if let Some(condition) = ability_data.extra.get("condition") {
                        if let Some(value) = condition.get(&property_name) {
                            return value.as_i64().map(|v| v as i32).unwrap_or(0);
                        }
                    }
                }
                if let Some(item_data) = self.dex.items().get(effect_id) {
                    // Item condition is stored in extra["condition"] as JSON
                    if let Some(condition) = item_data.extra.get("condition") {
                        debug_elog!("[GET_CALLBACK_PRIORITY] effect_id={}, property_name={}, condition={:?}", effect_id, property_name, condition);
                        if let Some(value) = condition.get(&property_name) {
                            let priority = value.as_i64().map(|v| v as i32).unwrap_or(0);
                            debug_elog!("[GET_CALLBACK_PRIORITY] Found priority={} for effect_id={}", priority, effect_id);
                            return priority;
                        }
                    }
                }
                0
            }
            // Terrain effects are stored in moves dex as embedded conditions
            EffectType::Terrain => {
                // First check conditions dex (in case terrain was added there)
                if let Some(condition_data) = self.dex.conditions().get(effect_id) {
                    if let Some(value) = condition_data.extra.get(&property_name) {
                        return value.as_i64().map(|v| v as i32).unwrap_or(0);
                    }
                }
                // Fall back to move embedded condition data
                if let Some(move_data) = self.dex.moves().get(effect_id) {
                    if let Some(condition) = &move_data.condition {
                        if let Some(value) = condition.extra.get(&property_name) {
                            return value.as_i64().map(|v| v as i32).unwrap_or(0);
                        }
                    }
                }
                0
            }
            _ => 0,
        }
    }

    /// Resolve event handler priority
    /// Equivalent to battle.ts resolvePriority()
    ///
    /// JavaScript Source (battle.ts:950-1017):
    /// Takes an EventListenerWithoutPriority and enriches it with priority/order/subOrder
    /// based on effect callback properties and effectType ordering
    //
    // 	resolvePriority(h: EventListenerWithoutPriority, callbackName: string) {
    // 		const handler = h as EventListener;
    // 		handler.order = (handler.effect as any)[`${callbackName}Order`] || false;
    // 		handler.priority = (handler.effect as any)[`${callbackName}Priority`] || 0;
    // 		handler.subOrder = (handler.effect as any)[`${callbackName}SubOrder`] || 0;
    // 		if (!handler.subOrder) {
    // 			// https://www.smogon.com/forums/threads/sword-shield-battle-mechanics-research.3655528/page-59#post-8685465
    // 			const effectTypeOrder: { [k in EffectType]?: number } = {
    // 				// Z-Move: 1,
    // 				Condition: 2,
    // 				// Slot Condition: 3,
    // 				// Side Condition: 4,
    // 				// Field Condition: 5, (includes weather but also terrains and pseudoweathers)
    // 				Weather: 5,
    // 				Format: 5,
    // 				Rule: 5,
    // 				Ruleset: 5,
    // 				// Poison Touch: 6, (also includes Perish Body)
    // 				Ability: 7,
    // 				Item: 8,
    // 				// Stall: 9,
    // 			};
    // 			handler.subOrder = effectTypeOrder[handler.effect.effectType] || 0;
    // 			if (handler.effect.effectType === 'Condition') {
    // 				if (handler.state?.target instanceof Side) {
    // 					if (handler.state.isSlotCondition) {
    // 						// slot condition
    // 						handler.subOrder = 3;
    // 					} else {
    // 						// side condition
    // 						handler.subOrder = 4;
    // 					}
    // 				} else if (handler.state?.target instanceof Field) {
    // 					// field condition
    // 					handler.subOrder = 5;
    // 				}
    // 			} else if (handler.effect.effectType === 'Ability') {
    // 				if (handler.effect.name === 'Poison Touch' || handler.effect.name === 'Perish Body') {
    // 					handler.subOrder = 6;
    // 				} else if (handler.effect.name === 'Stall') {
    // 					handler.subOrder = 9;
    // 				}
    // 			}
    // 		}
    // 		if (callbackName.endsWith('SwitchIn') || callbackName.endsWith('RedirectTarget')) {
    // 			// If multiple hazards are present on one side, their event handlers all perfectly tie in speed, priority,
    // 			// and subOrder. They should activate in the order they were created, which is where effectOrder comes in.
    // 			// This also applies to speed ties for which ability like Lightning Rod redirects moves.
    // 			// Note: In-game, other events are also sorted this way, but that's an implementation for another refactor
    // 			handler.effectOrder = handler.state?.effectOrder;
    // 		}
    // 		if (handler.effectHolder && (handler.effectHolder as Pokemon).getStat) {
    // 			const pokemon = handler.effectHolder as Pokemon;
    // 			handler.speed = pokemon.speed;
    // 			if (handler.effect.effectType === 'Ability' && handler.effect.name === 'Magic Bounce' &&
    // 				callbackName === 'onAllyTryHitSide') {
    // 				handler.speed = pokemon.getStat('spe', true, true);
    // 			}
    // 			if (callbackName.endsWith('SwitchIn')) {
    // 				// Pokemon speeds including ties are resolved before all onSwitchIn handlers and aren't re-sorted in-between
    // 				// so we subtract a fractional speed from each Pokemon's respective event handlers by using the index of their
    // 				// unique field position in a pre-sorted-by-speed array
    // 				const fieldPositionValue = pokemon.side.n * this.sides.length + pokemon.position;
    // 				handler.speed -= this.speedOrder.indexOf(fieldPositionValue) / (this.activePerHalf * 2);
    // 			}
    // 		}
    // 		return handler;
    // 	}
    //
    pub fn resolve_priority(&mut self, handler: &mut EventListener, callback_name: &str) {
        // JS: handler.order = (handler.effect as any)[`${callbackName}Order`] || false;
        // JS: handler.priority = (handler.effect as any)[`${callbackName}Priority`] || 0;
        // JS: handler.subOrder = (handler.effect as any)[`${callbackName}SubOrder`] || 0;
        //
        // Look up order and priority from dex data based on effect type and ID
        handler.order = self.get_callback_order(handler.effect.effect_type, handler.effect.id.as_str(), callback_name);
        handler.priority = self.get_callback_priority(handler.effect.effect_type, handler.effect.id.as_str(), callback_name);

        // Check for custom sub_order from dex data first
        if let Some(custom_sub_order) = self.get_callback_sub_order(handler.effect.effect_type, handler.effect.id.as_str(), callback_name) {
            handler.sub_order = custom_sub_order;
        }

        // If subOrder is not set (neither from input nor from dex), calculate it based on effectType
        // JS: if (!handler.subOrder) { ... }
        if handler.sub_order == 0 {
            // https://www.smogon.com/forums/threads/sword-shield-battle-mechanics-research.3655528/page-59#post-8685465
            handler.sub_order = match handler.effect.effect_type {
                EffectType::ZMove => 1,
                EffectType::Condition => {
                    // JS: if (handler.effect.effectType === 'Condition' && handler.state?.target instanceof Field)
                    // If the condition's target is a Field (pseudo-weather), use sub_order 5
                    // This happens after the first event has executed on the field condition
                    if handler.state.as_ref().is_some_and(|s| s.target_is_field) {
                        5
                    } else {
                        2
                    }
                }
                EffectType::Status => 2,
                EffectType::SlotCondition => 3,
                EffectType::SideCondition => 4,
                EffectType::FieldCondition => 5,
                EffectType::Weather => 5,
                EffectType::Format => 5,
                EffectType::Rule => 5,
                EffectType::Ruleset => 5,
                EffectType::Ability => {
                    // JS: if (handler.effect.name === 'Poison Touch' || handler.effect.name === 'Perish Body') { handler.subOrder = 6; }
                    // JS: else if (handler.effect.name === 'Stall') { handler.subOrder = 9; }
                    let ability_name = handler.effect.id.as_str();
                    if ability_name == "poisontouch" || ability_name == "perishbody" {
                        6
                    } else if ability_name == "stall" {
                        9
                    } else {
                        7
                    }
                }
                EffectType::Item => 8,
                _ => 0,
            };

            // JS: if (handler.effect.effectType === 'Condition' && handler.state?.target instanceof Side)
            // Check if this is a slot/side/field condition based on effect_type already set above
            // (This was already handled in match statement above)
        }

        // JS: if (callbackName.endsWith('SwitchIn') || callbackName.endsWith('RedirectTarget'))
        if callback_name.ends_with("SwitchIn") || callback_name.ends_with("RedirectTarget") {
            // JS: handler.effectOrder = handler.state?.effectOrder;
            if let Some(state) = &handler.state {
                handler.effect_order = Some(state.effect_order);
            }
        }

        // NOTE: JavaScript does NOT set handler.effectOrder for field conditions
        // (weather, terrain, pseudo-weather). All handlers tie on effectOrder and
        // get shuffled. This is intentional for PRNG synchronization.
        // Do NOT add a workaround here - let the shuffle happen as in JavaScript.

        // JS: if (handler.effectHolder && (handler.effectHolder as Pokemon).getStat)
        if let Some(effect_holder) = handler.effect_holder {
            // Check if we need special handling for Magic Bounce
            let needs_magic_bounce_speed = handler.effect.effect_type == EffectType::Ability
                && handler.effect.id.as_str() == "magicbounce"
                && callback_name == "onAllyTryHitSide";

            // Get Magic Bounce speed if needed (requires mutable borrow)
            let magic_bounce_speed = if needs_magic_bounce_speed {
                Some(self.get_pokemon_stat(effect_holder, StatID::Spe, true, true) as f64)
            } else {
                None
            };

            // Get Pokemon speed
            if let Some(side) = self.sides.get(effect_holder.0) {
                if let Some(pokemon) = side.pokemon.get(effect_holder.1) {
                    // JS: handler.speed = pokemon.speed;
                    // Use pokemon.speed (the action speed) not stored_stats.spe (the base stat)
                    handler.speed = Some(pokemon.speed as f64);

                    // JS: if (handler.effect.effectType === 'Ability' && handler.effect.name === 'Magic Bounce' && callbackName === 'onAllyTryHitSide')
                    if let Some(mb_speed) = magic_bounce_speed {
                        handler.speed = Some(mb_speed);
                    }

                    // JS: if (callbackName.endsWith('SwitchIn'))
                    if callback_name.ends_with("SwitchIn") {
                        // JS: const fieldPositionValue = pokemon.side.n * this.sides.length + pokemon.position;
                        // JS: handler.speed -= this.speedOrder.indexOf(fieldPositionValue) / (this.activePerHalf * 2);
                        //
                        // This adjusts speed for switch-in ordering by subtracting a fractional value
                        // based on field position in the pre-sorted speed order
                        let field_position_value = effect_holder.0 * self.sides.len() + pokemon.position;

                        // Find index in speed_order array
                        if let Some(index) = self.speed_order.iter().position(|&pos| pos == field_position_value) {
                            let adjustment = index as f64 / (self.active_per_half * 2) as f64;
                            if let Some(speed) = handler.speed {
                                handler.speed = Some(speed - adjustment);
                            }
                        }
                    }
                }
            }
        }
    }
}
