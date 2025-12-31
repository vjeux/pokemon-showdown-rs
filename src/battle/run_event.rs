use crate::*;
use crate::battle::EventInfo;

impl Battle {

    /// Run event on all relevant handlers
    /// Equivalent to battle.ts runEvent()
    ///
    /// This is a simplified version that handles common event patterns.
    // TypeScript source:
    // /**
    // 	 * runEvent is the core of Pokemon Showdown's event system.
    // 	 *
    // 	 * Basic usage
    // 	 * ===========
    // 	 *
    // 	 *   this.runEvent('Blah')
    // 	 * will trigger any onBlah global event handlers.
    // 	 *
    // 	 *   this.runEvent('Blah', target)
    // 	 * will additionally trigger any onBlah handlers on the target, onAllyBlah
    // 	 * handlers on any active pokemon on the target's team, and onFoeBlah
    // 	 * handlers on any active pokemon on the target's foe's team
    // 	 *
    // 	 *   this.runEvent('Blah', target, source)
    // 	 * will additionally trigger any onSourceBlah handlers on the source
    // 	 *
    // 	 *   this.runEvent('Blah', target, source, effect)
    // 	 * will additionally pass the effect onto all event handlers triggered
    // 	 *
    // 	 *   this.runEvent('Blah', target, source, effect, relayVar)
    // 	 * will additionally pass the relayVar as the first argument along all event
    // 	 * handlers
    // 	 *
    // 	 * You may leave any of these null. For instance, if you have a relayVar but
    // 	 * no source or effect:
    // 	 *   this.runEvent('Damage', target, null, null, 50)
    // 	 *
    // 	 * Event handlers
    // 	 * ==============
    // 	 *
    // 	 * Items, abilities, statuses, and other effects like SR, confusion, weather,
    // 	 * or Trick Room can have event handlers. Event handlers are functions that
    // 	 * can modify what happens during an event.
    // 	 *
    // 	 * event handlers are passed:
    // 	 *   function (target, source, effect)
    // 	 * although some of these can be blank.
    // 	 *
    // 	 * certain events have a relay variable, in which case they're passed:
    // 	 *   function (relayVar, target, source, effect)
    // 	 *
    // 	 * Relay variables are variables that give additional information about the
    // 	 * event. For instance, the damage event has a relayVar which is the amount
    // 	 * of damage dealt.
    // 	 *
    // 	 * If a relay variable isn't passed to runEvent, there will still be a secret
    // 	 * relayVar defaulting to `true`, but it won't get passed to any event
    // 	 * handlers.
    // 	 *
    // 	 * After an event handler is run, its return value helps determine what
    // 	 * happens next:
    // 	 * 1. If the return value isn't `undefined`, relayVar is set to the return
    // 	 *    value
    // 	 * 2. If relayVar is falsy, no more event handlers are run
    // 	 * 3. Otherwise, if there are more event handlers, the next one is run and
    // 	 *    we go back to step 1.
    // 	 * 4. Once all event handlers are run (or one of them results in a falsy
    // 	 *    relayVar), relayVar is returned by runEvent
    // 	 *
    // 	 * As a shortcut, an event handler that isn't a function will be interpreted
    // 	 * as a function that returns that value.
    // 	 *
    // 	 * You can have return values mean whatever you like, but in general, we
    // 	 * follow the convention that returning `false` or `null` means
    // 	 * stopping or interrupting the event.
    // 	 *
    // 	 * For instance, returning `false` from a TrySetStatus handler means that
    // 	 * the pokemon doesn't get statused.
    // 	 *
    // 	 * If a failed event usually results in a message like "But it failed!"
    // 	 * or "It had no effect!", returning `null` will suppress that message and
    // 	 * returning `false` will display it. Returning `null` is useful if your
    // 	 * event handler already gave its own custom failure message.
    // 	 *
    // 	 * Returning `undefined` means "don't change anything" or "keep going".
    // 	 * A function that does nothing but return `undefined` is the equivalent
    // 	 * of not having an event handler at all.
    // 	 *
    // 	 * Returning a value means that that value is the new `relayVar`. For
    // 	 * instance, if a Damage event handler returns 50, the damage event
    // 	 * will deal 50 damage instead of whatever it was going to deal before.
    // 	 *
    // 	 * Useful values
    // 	 * =============
    // 	 *
    // 	 * In addition to all the methods and attributes of Dex, Battle, and
    // 	 * Scripts, event handlers have some additional values they can access:
    // 	 *
    // 	 * this.effect:
    // 	 *   the Effect having the event handler
    // 	 * this.effectState:
    // 	 *   the data store associated with the above Effect. This is a plain Object
    // 	 *   and you can use it to store data for later event handlers.
    // 	 * this.effectState.target:
    // 	 *   the Pokemon, Side, or Battle that the event handler's effect was
    // 	 *   attached to.
    // 	 * this.event.id:
    // 	 *   the event ID
    // 	 * this.event.target, this.event.source, this.event.effect:
    // 	 *   the target, source, and effect of the event. These are the same
    // 	 *   variables that are passed as arguments to the event handler, but
    // 	 *   they're useful for functions called by the event handler.
    // 	 */
    // 	runEvent(
    // 		eventid: string, target?: Pokemon | Pokemon[] | Side | Battle | null, source?: string | Pokemon | false | null,
    // 		sourceEffect?: Effect | null, relayVar?: any, onEffect?: boolean, fastExit?: boolean
    // 	) {
    // 		// if (Battle.eventCounter) {
    // 		// 	if (!Battle.eventCounter[eventid]) Battle.eventCounter[eventid] = 0;
    // 		// 	Battle.eventCounter[eventid]++;
    // 		// }
    // 		if (this.eventDepth >= 8) {
    // 			// oh fuck
    // 			this.add('message', 'STACK LIMIT EXCEEDED');
    // 			this.add('message', 'PLEASE REPORT IN BUG THREAD');
    // 			this.add('message', 'Event: ' + eventid);
    // 			this.add('message', 'Parent event: ' + this.event.id);
    // 			throw new Error("Stack overflow");
    // 		}
    // 		if (!target) target = this;
    // 		let effectSource = null;
    // 		if (source instanceof Pokemon) effectSource = source;
    // 		const handlers = this.findEventHandlers(target, eventid, effectSource);
    // 		if (onEffect) {
    // 			if (!sourceEffect) throw new Error("onEffect passed without an effect");
    // 			const callback = (sourceEffect as any)[`on${eventid}`];
    // 			if (callback !== undefined) {
    // 				if (Array.isArray(target)) throw new Error("");
    // 				handlers.unshift(this.resolvePriority({
    // 					effect: sourceEffect, callback, state: this.initEffectState({}), end: null, effectHolder: target,
    // 				}, `on${eventid}`));
    // 			}
    // 		}
    //
    // 		if (['Invulnerability', 'TryHit', 'DamagingHit', 'EntryHazard'].includes(eventid)) {
    // 			handlers.sort(Battle.compareLeftToRightOrder);
    // 		} else if (fastExit) {
    // 			handlers.sort(Battle.compareRedirectOrder);
    // 		} else {
    // 			this.speedSort(handlers);
    // 		}
    // 		let hasRelayVar = 1;
    // 		const args = [target, source, sourceEffect];
    // 		// console.log('Event: ' + eventid + ' (depth ' + this.eventDepth + ') t:' + target.id + ' s:' + (!source || source.id) + ' e:' + effect.id);
    // 		if (relayVar === undefined || relayVar === null) {
    // 			relayVar = true;
    // 			hasRelayVar = 0;
    // 		} else {
    // 			args.unshift(relayVar);
    // 		}
    //
    // 		const parentEvent = this.event;
    // 		this.event = { id: eventid, target, source, effect: sourceEffect, modifier: 1 };
    // 		this.eventDepth++;
    //
    // 		let targetRelayVars = [];
    // 		if (Array.isArray(target)) {
    // 			if (Array.isArray(relayVar)) {
    // 				targetRelayVars = relayVar;
    // 			} else {
    // 				for (let i = 0; i < target.length; i++) targetRelayVars[i] = true;
    // 			}
    // 		}
    // 		for (const handler of handlers) {
    // 			if (handler.index !== undefined) {
    // 				// TODO: find a better way to do this
    // 				if (!targetRelayVars[handler.index] && !(targetRelayVars[handler.index] === 0 &&
    // 					eventid === 'DamagingHit')) continue;
    // 				if (handler.target) {
    // 					args[hasRelayVar] = handler.target;
    // 					this.event.target = handler.target;
    // 				}
    // 				if (hasRelayVar) args[0] = targetRelayVars[handler.index];
    // 			}
    // 			const effect = handler.effect;
    // 			const effectHolder = handler.effectHolder;
    // 			// this.debug('match ' + eventid + ': ' + status.id + ' ' + status.effectType);
    // 			if (effect.effectType === 'Status' && (effectHolder as Pokemon).status !== effect.id) {
    // 				// it's changed; call it off
    // 				continue;
    // 			}
    // 			if (effect.effectType === 'Ability' && effect.flags['breakable'] &&
    // 				this.suppressingAbility(effectHolder as Pokemon)) {
    // 				if (effect.flags['breakable']) {
    // 					this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 					continue;
    // 				}
    // 				if (!effect.num) {
    // 					// ignore attacking events for custom abilities
    // 					const AttackingEvents = {
    // 						BeforeMove: 1,
    // 						BasePower: 1,
    // 						Immunity: 1,
    // 						RedirectTarget: 1,
    // 						Heal: 1,
    // 						SetStatus: 1,
    // 						CriticalHit: 1,
    // 						ModifyAtk: 1, ModifyDef: 1, ModifySpA: 1, ModifySpD: 1, ModifySpe: 1, ModifyAccuracy: 1,
    // 						ModifyBoost: 1,
    // 						ModifyDamage: 1,
    // 						ModifySecondaries: 1,
    // 						ModifyWeight: 1,
    // 						TryAddVolatile: 1,
    // 						TryHit: 1,
    // 						TryHitSide: 1,
    // 						TryMove: 1,
    // 						Boost: 1,
    // 						DragOut: 1,
    // 						Effectiveness: 1,
    // 					};
    // 					if (eventid in AttackingEvents) {
    // 						this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 						continue;
    // 					} else if (eventid === 'Damage' && sourceEffect && sourceEffect.effectType === 'Move') {
    // 						this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 						continue;
    // 					}
    // 				}
    // 			}
    // 			if (eventid !== 'Start' && eventid !== 'SwitchIn' && eventid !== 'TakeItem' &&
    // 				effect.effectType === 'Item' && (effectHolder instanceof Pokemon) && effectHolder.ignoringItem()) {
    // 				if (eventid !== 'Update') {
    // 					this.debug(eventid + ' handler suppressed by Embargo, Klutz or Magic Room');
    // 				}
    // 				continue;
    // 			} else if (
    // 				eventid !== 'End' && effect.effectType === 'Ability' &&
    // 				(effectHolder instanceof Pokemon) && effectHolder.ignoringAbility()
    // 			) {
    // 				if (eventid !== 'Update') {
    // 					this.debug(eventid + ' handler suppressed by Gastro Acid or Neutralizing Gas');
    // 				}
    // 				continue;
    // 			}
    // 			if (
    // 				(effect.effectType === 'Weather' || eventid === 'Weather') &&
    // 				eventid !== 'Residual' && eventid !== 'End' && this.field.suppressingWeather()
    // 			) {
    // 				this.debug(eventid + ' handler suppressed by Air Lock');
    // 				continue;
    // 			}
    // 			let returnVal;
    // 			if (typeof handler.callback === 'function') {
    // 				const parentEffect = this.effect;
    // 				const parentEffectState = this.effectState;
    // 				this.effect = handler.effect;
    // 				this.effectState = handler.state || this.initEffectState({});
    // 				this.effectState.target = effectHolder;
    //
    // 				returnVal = handler.callback.apply(this, args);
    //
    // 				this.effect = parentEffect;
    // 				this.effectState = parentEffectState;
    // 			} else {
    // 				returnVal = handler.callback;
    // 			}
    //
    // 			if (returnVal !== undefined) {
    // 				relayVar = returnVal;
    // 				if (!relayVar || fastExit) {
    // 					if (handler.index !== undefined) {
    // 						targetRelayVars[handler.index] = relayVar;
    // 						if (targetRelayVars.every(val => !val)) break;
    // 					} else {
    // 						break;
    // 					}
    // 				}
    // 				if (hasRelayVar) {
    // 					args[0] = relayVar;
    // 				}
    // 			}
    // 		}
    //
    // 		this.eventDepth--;
    // 		if (typeof relayVar === 'number' && relayVar === Math.abs(Math.floor(relayVar))) {
    // 			// this.debug(eventid + ' modifier: 0x' +
    // 			// 	('0000' + (this.event.modifier * 4096).toString(16)).slice(-4).toUpperCase());
    // 			relayVar = this.modify(relayVar, this.event.modifier);
    // 		}
    // 		this.event = parentEvent;
    //
    // 		return Array.isArray(target) ? targetRelayVars : relayVar;
    // 	}
    //
    pub fn run_event(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        relay_var: Option<i32>,
    ) -> Option<i32> {
        use crate::event::EventResult;

        // Check stack depth
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            return None;
        }

        // Save parent event context
        let parent_event = self.current_event.take();
        self.event_depth += 1;

        // Set up current event
        self.current_event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect.cloned(),
            modifier: 4096,
            relay_var,
            relay_var_float: None,
            relay_var_boost: None,
            relay_var_secondaries: None,
            relay_var_type: None,
        });

        // JavaScript: if (relayVar === undefined || relayVar === null) { relayVar = true; hasRelayVar = 0; }
        // If no relay_var is provided, default to Some(1) (truthy value, like JavaScript's true)
        let mut result = relay_var.or(Some(1));

        // Find and run all handlers for this event
        let mut handlers = self.find_event_handlers(event_id, target, source);

        // JavaScript: if (onEffect) { ... handlers.unshift(...) }
        // In JavaScript, sourceEffect's handler is ONLY added when onEffect parameter is true
        // Since Rust doesn't have an onEffect parameter, we should NOT add sourceEffect to handlers
        // The sourceEffect is passed for context (stored in current_event), but NOT for calling its handler
        // This prevents moves from having their onHit called twice:
        //   1. single_event("Hit", move_id, ...) - correctly calls move.onHit
        //   2. run_event("Hit", target, source, move_id, ...) - should NOT call move.onHit again
        // REMOVED: handlers.insert(0, (event_id.to_string(), source_effect_id.clone(), target));


        for (event_variant, effect_id, holder_target) in handlers {
            let event_result =
                self.dispatch_single_event(&event_variant, &effect_id, holder_target, source);

            eprintln!("[RUN_EVENT] Handler returned: {:?}", event_result);

            match event_result {
                EventResult::Boolean(false) => {
                    // JavaScript: if (!relayVar) break;
                    // False is falsy, so set result to 0 and break
                    result = Some(0);
                    break;
                }
                EventResult::Boolean(true) => {
                    // JavaScript: relayVar = returnVal;
                    // True is truthy, so set result to 1 and continue
                    result = Some(1);
                }
                EventResult::NotFail => {
                    // JavaScript: relayVar = this.NOT_FAIL; (NOT_FAIL = '' which is falsy)
                    // NotFail is falsy, so set result to None and break
                    result = None;
                    break;
                }
                EventResult::Stop => {
                    break;
                }
                EventResult::Number(n) => {
                    // JavaScript: if (returnVal !== void 0) { relayVar = returnVal; }
                    // When a handler returns a number, it REPLACES the relayVar, not modifies it
                    result = Some(n);
                }
                EventResult::HitSubstitute => {
                    // JavaScript: this.HIT_SUBSTITUTE = 0
                    // When Substitute blocks damage, return 0
                    result = Some(0);
                }
                _ => {}
            }
        }

        // Run custom event handlers (registered via onEvent in tests)
        if let Some(custom_result) = self.run_custom_event_handlers(event_id) {
            result = Some(custom_result);
        }

        // Apply event modifier if we have a numeric result
        if let (Some(ref mut r), Some(ref event)) = (&mut result, &self.current_event) {
            if event.modifier != 4096 {
                *r = self.modify_internal(*r, event.modifier);
            }
        }

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;

        result
    }
}
