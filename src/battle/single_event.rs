use crate::*;
use crate::battle::EventInfo;
use crate::event::EventResult;

impl Battle {

    // =========================================================================
    // EVENT SYSTEM (ported from battle.ts)
    // =========================================================================

    /// Single event - runs a single callback on an effect
    /// Equivalent to battle.ts singleEvent() (lines 571-652)
    ///
    /// This fires a single event handler with full suppression logic.
    // TypeScript source:
    // /** The entire event system revolves around this function and runEvent. */
    // 	singleEvent(
    // 		eventid: string, effect: Effect, state: EffectState | Record<string, never> | null,
    // 		target: string | Pokemon | Side | Field | Battle | null, source?: string | Pokemon | Effect | false | null,
    // 		sourceEffect?: Effect | string | null, relayVar?: any, customCallback?: unknown
    // 	) {
    // 		if (this.eventDepth >= 8) {
    // 			// oh fuck
    // 			this.add('message', 'STACK LIMIT EXCEEDED');
    // 			this.add('message', 'PLEASE REPORT IN BUG THREAD');
    // 			this.add('message', 'Event: ' + eventid);
    // 			this.add('message', 'Parent event: ' + this.event.id);
    // 			throw new Error("Stack overflow");
    // 		}
    // 		if (this.log.length - this.sentLogPos > 1000) {
    // 			this.add('message', 'LINE LIMIT EXCEEDED');
    // 			this.add('message', 'PLEASE REPORT IN BUG THREAD');
    // 			this.add('message', 'Event: ' + eventid);
    // 			this.add('message', 'Parent event: ' + this.event.id);
    // 			throw new Error("Infinite loop");
    // 		}
    // 		// this.add('Event: ' + eventid + ' (depth ' + this.eventDepth + ')');
    // 		let hasRelayVar = true;
    // 		if (relayVar === undefined) {
    // 			relayVar = true;
    // 			hasRelayVar = false;
    // 		}
    //
    // 		if (effect.effectType === 'Status' && (target instanceof Pokemon) && target.status !== effect.id) {
    // 			// it's changed; call it off
    // 			return relayVar;
    // 		}
    // 		if (eventid === 'SwitchIn' && effect.effectType === 'Ability' && effect.flags['breakable'] &&
    // 			this.suppressingAbility(target as Pokemon)) {
    // 			this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 			return relayVar;
    // 		}
    // 		if (eventid !== 'Start' && eventid !== 'TakeItem' && effect.effectType === 'Item' &&
    // 			(target instanceof Pokemon) && target.ignoringItem()) {
    // 			this.debug(eventid + ' handler suppressed by Embargo, Klutz or Magic Room');
    // 			return relayVar;
    // 		}
    // 		if (eventid !== 'End' && effect.effectType === 'Ability' && (target instanceof Pokemon) && target.ignoringAbility()) {
    // 			this.debug(eventid + ' handler suppressed by Gastro Acid or Neutralizing Gas');
    // 			return relayVar;
    // 		}
    // 		if (
    // 			effect.effectType === 'Weather' && eventid !== 'FieldStart' && eventid !== 'FieldResidual' &&
    // 			eventid !== 'FieldEnd' && this.field.suppressingWeather()
    // 		) {
    // 			this.debug(eventid + ' handler suppressed by Air Lock');
    // 			return relayVar;
    // 		}
    //
    // 		const callback = customCallback || (effect as any)[`on${eventid}`];
    // 		if (callback === undefined) return relayVar;
    //
    // 		const parentEffect = this.effect;
    // 		const parentEffectState = this.effectState;
    // 		const parentEvent = this.event;
    //
    // 		this.effect = effect;
    // 		this.effectState = state as EffectState || this.initEffectState({});
    // 		this.event = { id: eventid, target, source, effect: sourceEffect };
    // 		this.eventDepth++;
    //
    // 		const args = [target, source, sourceEffect];
    // 		if (hasRelayVar) args.unshift(relayVar);
    //
    // 		let returnVal;
    // 		if (typeof callback === 'function') {
    // 			returnVal = callback.apply(this, args);
    // 		} else {
    // 			returnVal = callback;
    // 		}
    //
    // 		this.eventDepth--;
    // 		this.effect = parentEffect;
    // 		this.effectState = parentEffectState;
    // 		this.event = parentEvent;
    //
    // 		return returnVal === undefined ? relayVar : returnVal;
    // 	}
    //
    pub fn single_event(
        &mut self,
        event_id: &str,
        effect_id: &ID,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        relay_var: Option<EventResult>,
    ) -> EventResult {
        eprintln!("[SINGLE_EVENT] event_id={}, effect_id={}, target={:?}, depth={}",
            event_id, effect_id.as_str(), target, self.event_depth);

        // JavaScript: if (this.eventDepth >= 8) throw Error
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.current_event {
                self.add("message", &[format!("Parent event: {}", evt.id).into()]);
            }
            return EventResult::Boolean(false);
        }

        // JavaScript: if (this.log.length - this.sentLogPos > 1000) throw Error
        if self.log.len() - self.sent_log_pos > 1000 {
            self.add("message", &["LINE LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.current_event {
                self.add("message", &[format!("Parent event: {}", evt.id).into()]);
            }
            return EventResult::Boolean(false);
        }

        // Determine effect type for suppression checks
        // IMPORTANT: Check if effect_id is a volatile on the target Pokemon FIRST
        // This is needed because some volatiles share names with abilities (e.g., "stall")
        // If we use get_effect_type(), it finds the Stall ability before the stall volatile
        // which causes with_effect_state() to look in ability_state instead of volatiles
        let effect_type = if let Some((side_idx, poke_idx)) = target {
            if let Some(pokemon) = self.pokemon_at(side_idx, poke_idx) {
                if pokemon.volatiles.contains_key(effect_id) {
                    "Condition".to_string()
                } else if !pokemon.status.is_empty() && pokemon.status == *effect_id {
                    "Status".to_string()
                } else {
                    self.get_effect_type(effect_id).to_string()
                }
            } else {
                self.get_effect_type(effect_id).to_string()
            }
        } else {
            self.get_effect_type(effect_id).to_string()
        };

        // SUPPRESSION CHECKS (from JavaScript battle.ts:598-622)

        // JavaScript: if (effect.effectType === 'Status' && target.status !== effect.id) return relayVar
        if effect_type == "Status" {
            if let Some((side_idx, poke_idx)) = target {
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.status != *effect_id {
                            return EventResult::Continue;
                        }
                    }
                }
            }
        }

        // JavaScript: if (eventid === 'SwitchIn' && effect.effectType === 'Ability' && effect.flags['breakable'] && this.suppressingAbility(target))
        if event_id == "SwitchIn" && effect_type == "Ability" && self.suppressing_ability(target) {
            self.debug(&format!("{} handler suppressed by Mold Breaker", event_id));
            return EventResult::Continue;
        }

        // JavaScript: if (eventid !== 'Start' && eventid !== 'TakeItem' && effect.effectType === 'Item' && target.ignoringItem())
        if event_id != "Start" && event_id != "TakeItem" && effect_type == "Item" {
            if let Some((side_idx, poke_idx)) = target {
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.ignoring_item(self, false) {
                            self.debug(&format!(
                                "{} handler suppressed by Embargo, Klutz or Magic Room",
                                event_id
                            ));
                            return EventResult::Continue;
                        }
                    }
                }
            }
        }

        // JavaScript: if (eventid !== 'End' && effect.effectType === 'Ability' && target.ignoringAbility())
        if event_id != "End" && effect_type == "Ability" {
            if let Some((side_idx, poke_idx)) = target {
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.ignoring_ability(self) {
                            self.debug(&format!(
                                "{} handler suppressed by Gastro Acid or Neutralizing Gas",
                                event_id
                            ));
                            return EventResult::Continue;
                        }
                    }
                }
            }
        }

        // JavaScript: if (effect.effectType === 'Weather' && eventid !== 'FieldStart' && eventid !== 'FieldResidual' && eventid !== 'FieldEnd' && this.field.suppressingWeather())
        if effect_type == "Weather"
            && event_id != "FieldStart"
            && event_id != "FieldResidual"
            && event_id != "FieldEnd"
            && self.suppressing_weather()
        {
            self.debug(&format!("{} handler suppressed by Air Lock", event_id));
            return EventResult::Continue;
        }

        // Save parent event context
        let parent_event = self.current_event.take();
        let parent_context = self.current_effect_context.take();

        // Determine EffectType from string
        let effect_type_enum = match effect_type.as_str() {
            "Ability" => crate::battle::EffectType::Ability,
            "Item" => crate::battle::EffectType::Item,
            "Move" => crate::battle::EffectType::Move,
            "Status" => crate::battle::EffectType::Status,
            "Weather" => crate::battle::EffectType::Weather,
            "Terrain" => crate::battle::EffectType::Terrain,
            "SideCondition" => crate::battle::EffectType::SideCondition,
            "SlotCondition" => crate::battle::EffectType::SlotCondition,
            "FieldCondition" => crate::battle::EffectType::FieldCondition,
            _ => crate::battle::EffectType::Condition,
        };

        // Set up current effect context
        self.current_effect_context = Some(crate::EffectContext {
            effect_id: effect_id.clone(),
            effect_type: effect_type_enum,
            effect_holder: target,
            side_index: target.map(|(side, _)| side),
            prankster_boosted: false,
        });

        // Set up current event
        // JavaScript: this.event = { id: eventid, target, source, effect: sourceEffect };
        // JavaScript: if (hasRelayVar) args.unshift(relayVar); // relayVar becomes first argument to handler
        self.current_event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect.cloned(),
            modifier: 4096,
            relay_var: relay_var.clone(),
            type_param: None,
        });
        self.event_depth += 1;

        // Dispatch based on effect type
        let target_event = target.map(crate::event::EventTarget::Pokemon);
        eprintln!("[SINGLE_EVENT] BEFORE dispatch_single_event: event_id={}, effect_id={}", event_id, effect_id.as_str());
        let result = self.dispatch_single_event(event_id, effect_id, target_event.as_ref(), source);
        eprintln!("[SINGLE_EVENT] AFTER dispatch_single_event: event_id={}, effect_id={}, result={:?}", event_id, effect_id.as_str(), result);

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;
        self.current_effect_context = parent_context;

        // JavaScript: return returnVal === undefined ? relayVar : returnVal;
        // If handler returns Continue (undefined equivalent) and we have relay_var, return it
        // Otherwise return the handler's result
        match result {
            EventResult::Continue => {
                relay_var.unwrap_or(EventResult::Continue)
            }
            _ => result,
        }
    }
}
