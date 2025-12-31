use crate::*;
use crate::battle::EffectType;
use crate::battle::EventListener;
use crate::dex_data::StatID;

impl Battle {

    /// Get callback order from dex data
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`] || false;
    /// Returns None if no order is specified (equivalent to false in JS)
    pub(crate) fn get_callback_order(effect_type: EffectType, effect_id: &str, callback_name: &str) -> Option<i32> {
        // Extract event name from callback (e.g., "onResidual" -> "Residual")
        let event = if callback_name.starts_with("on") {
            &callback_name[2..]
        } else {
            callback_name
        };

        match (effect_type, event) {
            // Ability onResidualOrder values (from data/abilities.ts)
            (EffectType::Ability, "Residual") => match effect_id {
                "slowstart" => Some(28),
                _ => None,
            },
            // Item onResidualOrder values (from data/items.ts)
            (EffectType::Item, "Residual") => match effect_id {
                "leftovers" => Some(5),
                "blacksludge" => Some(5),
                _ => None,
            },
            // Status onResidualOrder values (from data/conditions.ts)
            (EffectType::Condition, "Residual") => match effect_id {
                "brn" => Some(10),
                "psn" => Some(9),
                "tox" => Some(9),
                _ => None,
            },
            _ => None,
        }
    }

    /// Get callback priority from dex data
    /// JavaScript: handler.priority = (handler.effect as any)[`${callbackName}Priority`] || 0;
    /// Returns 0 if no priority is specified (default in JS)
    pub(crate) fn get_callback_priority(effect_type: EffectType, effect_id: &str, callback_name: &str) -> i32 {
        // Most callbacks don't have custom priorities, return 0 by default
        // Can be expanded as needed
        let _ = (effect_type, effect_id, callback_name);
        0
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
    // 			// TODO: In-game, other events are also sorted this way, but that's an implementation for another refactor
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
        handler.order = Self::get_callback_order(handler.effect_type, handler.effect_id.as_str(), callback_name);
        handler.priority = Self::get_callback_priority(handler.effect_type, handler.effect_id.as_str(), callback_name);

        // If subOrder is not already set, calculate it based on effectType
        if handler.sub_order == 0 {
            // https://www.smogon.com/forums/threads/sword-shield-battle-mechanics-research.3655528/page-59#post-8685465
            handler.sub_order = match handler.effect_type {
                EffectType::ZMove => 1,
                EffectType::Condition => 2,
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
                    let ability_name = handler.effect_id.as_str();
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

        // JS: if (handler.effectHolder && (handler.effectHolder as Pokemon).getStat)
        if let Some(effect_holder) = handler.effect_holder {
            // Check if we need special handling for Magic Bounce
            let needs_magic_bounce_speed = handler.effect_type == EffectType::Ability
                && handler.effect_id.as_str() == "magicbounce"
                && callback_name == "onAllyTryHitSide";

            // Get Pokemon speed
            if let Some(side) = self.sides.get(effect_holder.0) {
                if let Some(pokemon) = side.pokemon.get(effect_holder.1) {
                    handler.speed = Some(pokemon.stored_stats.spe);

                    // JS: if (handler.effect.effectType === 'Ability' && handler.effect.name === 'Magic Bounce' && callbackName === 'onAllyTryHitSide')
                    if needs_magic_bounce_speed {
                        // JS: handler.speed = pokemon.getStat('spe', true, true);
                        // Get unmodified speed stat (unboosted=true, unmodified=true)
                        // Need to call get_pokemon_stat
                        let speed_stat = self.get_pokemon_stat(effect_holder, StatID::Spe, true, true);
                        handler.speed = Some(speed_stat);
                    }

                    // JS: if (callbackName.endsWith('SwitchIn'))
                    if callback_name.ends_with("SwitchIn") {
                        // JS: const fieldPositionValue = pokemon.side.n * this.sides.length + pokemon.position;
                        // JS: handler.speed -= this.speedOrder.indexOf(fieldPositionValue) / (this.activePerHalf * 2);
                        //
                        // This adjusts speed for switch-in ordering by subtracting a fractional value
                        // based on field position in the pre-sorted speed order
                        //
                        // TODO: Requires this.speedOrder array which stores pre-sorted field positions
                        // For now, skip this adjustment
                    }
                }
            }
        }
    }
}
