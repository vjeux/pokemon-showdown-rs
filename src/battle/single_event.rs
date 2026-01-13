use crate::*;
use crate::battle::{EventInfo, EffectType, Effect};
use crate::event::EventResult;
use crate::event_system::EffectState;

impl Battle {

    // =========================================================================
    // EVENT SYSTEM (ported from battle.ts)
    // =========================================================================

    /// Single event - runs a single callback on an effect
    /// Equivalent to battle.ts singleEvent() (lines 571-652)
    ///
    /// This fires a single event handler with full suppression logic.
    ///
    /// JavaScript signature: singleEvent(eventid, effect, state, target, source, sourceEffect, relayVar)
    /// In JavaScript, `effect` is an object with `id` and `effectType` properties.
    /// In Rust, we pass an `Effect` struct that contains both.
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
        effect: &Effect,
        state: Option<&EffectState>,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&Effect>,
        relay_var: Option<EventResult>,
    ) -> EventResult {
        let effect_id = &effect.id;
        let effect_type = effect.effect_type;

        // JavaScript: if (this.eventDepth >= 8) throw Error
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.event {
                self.add("message", &[format!("Parent event: {}", evt.id).into()]);
            }
            return EventResult::Boolean(false);
        }

        // JavaScript: if (this.log.length - this.sentLogPos > 1000) throw Error
        let log_len = self.log.len();
        if log_len > self.sent_log_pos && log_len - self.sent_log_pos > 1000 {
            self.add("message", &["LINE LIMIT EXCEEDED".into()]);
            self.add("message", &["PLEASE REPORT IN BUG THREAD".into()]);
            self.add("message", &[format!("Event: {}", event_id).into()]);
            if let Some(ref evt) = self.event {
                self.add("message", &[format!("Parent event: {}", evt.id).into()]);
            }
            return EventResult::Boolean(false);
        }

        // SUPPRESSION CHECKS (from JavaScript battle.ts:598-622)

        // JavaScript: if (effect.effectType === 'Status' && target.status !== effect.id) return relayVar
        if effect_type == EffectType::Status {
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
        if event_id == "SwitchIn" && effect_type == EffectType::Ability && self.suppressing_ability(target) {
            self.debug(&format!("{} handler suppressed by Mold Breaker", event_id));
            return EventResult::Continue;
        }

        // JavaScript: if (eventid !== 'Start' && eventid !== 'TakeItem' && effect.effectType === 'Item' && target.ignoringItem())
        if event_id != "Start" && event_id != "TakeItem" && effect_type == EffectType::Item {
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
        if event_id != "End" && effect_type == EffectType::Ability {
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
        if effect_type == EffectType::Weather
            && event_id != "FieldStart"
            && event_id != "FieldResidual"
            && event_id != "FieldEnd"
            && self.suppressing_weather()
        {
            self.debug(&format!("{} handler suppressed by Air Lock", event_id));
            return EventResult::Continue;
        }

        // Save parent event context
        // JavaScript: const parentEffect = this.effect;
        // JavaScript: const parentEffectState = this.effectState;
        // JavaScript: const parentEvent = this.event;
        let parent_event = self.event.take();
        let parent_effect = self.effect.take();
        let parent_effect_state = std::mem::take(&mut self.effect_state);

        // Extract type_param from parent event to preserve it for Effectiveness events
        let preserved_type_param = parent_event.as_ref().and_then(|e| e.type_param.clone());

        // Set up current effect state
        // JavaScript: this.effectState = state as EffectState || this.initEffectState({});
        self.effect_state = match state {
            Some(s) => s.clone(),
            None => EffectState::new(effect_id.clone()),
        };

        // Look up proper name based on effect type
        let effect_name = match effect_type {
            EffectType::Ability => self.dex.abilities().get(effect_id.as_str())
                .map(|a| a.name.clone())
                .unwrap_or_else(|| effect_id.to_string()),
            EffectType::Item => self.dex.items().get(effect_id.as_str())
                .map(|i| i.name.clone())
                .unwrap_or_else(|| effect_id.to_string()),
            EffectType::Move | EffectType::MoveSelf => self.dex.moves().get(effect_id.as_str())
                .map(|m| m.name.clone())
                .unwrap_or_else(|| effect_id.to_string()),
            _ => self.dex.conditions().get_by_id(effect_id)
                .and_then(|c| c.name.clone())
                .or_else(|| self.dex.moves().get(effect_id.as_str()).map(|m| m.name.clone()))
                .unwrap_or_else(|| effect_id.to_string()),
        };

        // Set up current effect context
        // For all effect types, effect_holder uses target (party index for Pokemon)
        // For slot conditions, with_effect_state_ref converts party index to slot position internally
        self.effect = Some(crate::Effect {
            id: effect_id.clone(),
            name: effect_name,
            effect_type,
            effect_holder: target,
            side_index: target.map(|(side, _)| side),
            prankster_boosted: false,
        });

        // Set up current event
        // JavaScript: this.event = { id: eventid, target, source, effect: sourceEffect };
        // JavaScript: if (hasRelayVar) args.unshift(relayVar); // relayVar becomes first argument to handler

        // Use source_effect directly (it's already an Effect)
        let source_effect_obj = source_effect.cloned();

        self.event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect_obj,
            modifier: 4096,
            relay_var: relay_var.clone(),
            type_param: preserved_type_param,
        });
        self.event_depth += 1;

        // Dispatch based on effect type
        let target_event = target.map(crate::event::EventTarget::Pokemon);
        let result = self.dispatch_single_event(event_id, effect_id, target_event.as_ref(), source);

        // Restore parent context
        // JavaScript: this.eventDepth--;
        // JavaScript: this.effect = parentEffect;
        // JavaScript: this.effectState = parentEffectState;
        // JavaScript: this.event = parentEvent;
        self.event_depth -= 1;
        self.event = parent_event;
        self.effect = parent_effect;
        self.effect_state = parent_effect_state;

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
