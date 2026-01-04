use crate::*;
use crate::battle::{EventInfo, EventListener, PriorityItem};
use crate::event::EventResult;

impl Battle {

    /// Run event on all relevant handlers
    /// Equivalent to battle.ts runEvent()
    ///
    // /**
    //   * runEvent is the core of Pokemon Showdown's event system.
    //   *
    //   * Basic usage
    //   * ===========
    //   *
    //   *   this.runEvent('Blah')
    //   * will trigger any onBlah global event handlers.
    //   *
    //   *   this.runEvent('Blah', target)
    //   * will additionally trigger any onBlah handlers on the target, onAllyBlah
    //   * handlers on any active pokemon on the target's team, and onFoeBlah
    //   * handlers on any active pokemon on the target's foe's team
    //   *
    //   *   this.runEvent('Blah', target, source)
    //   * will additionally trigger any onSourceBlah handlers on the source
    //   *
    //   *   this.runEvent('Blah', target, source, effect)
    //   * will additionally pass the effect onto all event handlers triggered
    //   *
    //   *   this.runEvent('Blah', target, source, effect, relayVar)
    //   * will additionally pass the relayVar as the first argument along all event
    //   * handlers
    //   *
    //   * You may leave any of these null. For instance, if you have a relayVar but
    //   * no source or effect:
    //   *   this.runEvent('Damage', target, null, null, 50)
    //   *
    //   * Event handlers
    //   * ==============
    //   *
    //   * Items, abilities, statuses, and other effects like SR, confusion, weather,
    //   * or Trick Room can have event handlers. Event handlers are functions that
    //   * can modify what happens during an event.
    //   *
    //   * event handlers are passed:
    //   *   function (target, source, effect)
    //   * although some of these can be blank.
    //   *
    //   * certain events have a relay variable, in which case they're passed:
    //   *   function (relayVar, target, source, effect)
    //   *
    //   * Relay variables are variables that give additional information about the
    //   * event. For instance, the damage event has a relayVar which is the amount
    //   * of damage dealt.
    //   *
    //   * If a relay variable isn't passed to runEvent, there will still be a secret
    //   * relayVar defaulting to `true`, but it won't get passed to any event
    //   * handlers.
    //   *
    //   * After an event handler is run, its return value helps determine what
    //   * happens next:
    //   * 1. If the return value isn't `undefined`, relayVar is set to the return
    //   *    value
    //   * 2. If relayVar is falsy, no more event handlers are run
    //   * 3. Otherwise, if there are more event handlers, the next one is run and
    //   *    we go back to step 1.
    //   * 4. Once all event handlers are run (or one of them results in a falsy
    //   *    relayVar), relayVar is returned by runEvent
    //   *
    //   * As a shortcut, an event handler that isn't a function will be interpreted
    //   * as a function that returns that value.
    //   *
    //   * You can have return values mean whatever you like, but in general, we
    //   * follow the convention that returning `false` or `null` means
    //   * stopping or interrupting the event.
    //   *
    //   * For instance, returning `false` from a TrySetStatus handler means that
    //   * the pokemon doesn't get statused.
    //   *
    //   * If a failed event usually results in a message like "But it failed!"
    //   * or "It had no effect!", returning `null` will suppress that message and
    //   * returning `false` will display it. Returning `null` is useful if your
    //   * event handler already gave its own custom failure message.
    //   *
    //   * Returning `undefined` means "don't change anything" or "keep going".
    //   * A function that does nothing but return `undefined` is the equivalent
    //   * of not having an event handler at all.
    //   *
    //   * Returning a value means that that value is the new `relayVar`. For
    //   * instance, if a Damage event handler returns 50, the damage event
    //   * will deal 50 damage instead of whatever it was going to deal before.
    //   *
    //   * Useful values
    //   * =============
    //   *
    //   * In addition to all the methods and attributes of Dex, Battle, and
    //   * Scripts, event handlers have some additional values they can access:
    //   *
    //   * this.effect:
    //   *   the Effect having the event handler
    //   * this.effectState:
    //   *   the data store associated with the above Effect. This is a plain Object
    //   *   and you can use it to store data for later event handlers.
    //   * this.effectState.target:
    //   *   the Pokemon, Side, or Battle that the event handler's effect was
    //   *   attached to.
    //   * this.event.id:
    //   *   the event ID
    //   * this.event.target, this.event.source, this.event.effect:
    //   *   the target, source, and effect of the event. These are the same
    //   *   variables that are passed as arguments to the event handler, but
    //   *   they're useful for functions called by the event handler.
    //   */
    //  runEvent(
    //      eventid: string, target?: Pokemon | Pokemon[] | Side | Battle | null, source?: string | Pokemon | false | null,
    //      sourceEffect?: Effect | null, relayVar?: any, onEffect?: boolean, fastExit?: boolean
    //  ) {
    //      // if (Battle.eventCounter) {
    //      //  if (!Battle.eventCounter[eventid]) Battle.eventCounter[eventid] = 0;
    //      //  Battle.eventCounter[eventid]++;
    //      // }
    //      if (this.eventDepth >= 8) {
    //          // oh fuck
    //          this.add('message', 'STACK LIMIT EXCEEDED');
    //          this.add('message', 'PLEASE REPORT IN BUG THREAD');
    //          this.add('message', 'Event: ' + eventid);
    //          this.add('message', 'Parent event: ' + this.event.id);
    //          throw new Error("Stack overflow");
    //      }
    //      if (!target) target = this;
    //      let effectSource = null;
    //      if (source instanceof Pokemon) effectSource = source;
    //      const handlers = this.findEventHandlers(target, eventid, effectSource);
    //      if (onEffect) {
    //          if (!sourceEffect) throw new Error("onEffect passed without an effect");
    //          const callback = (sourceEffect as any)[`on${eventid}`];
    //          if (callback !== undefined) {
    //              if (Array.isArray(target)) throw new Error("");
    //              handlers.unshift(this.resolvePriority({
    //                  effect: sourceEffect, callback, state: this.initEffectState({}), end: null, effectHolder: target,
    //              }, `on${eventid}`));
    //          }
    //      }
    //
    //      if (['Invulnerability', 'TryHit', 'DamagingHit', 'EntryHazard'].includes(eventid)) {
    //          handlers.sort(Battle.compareLeftToRightOrder);
    //      } else if (fastExit) {
    //          handlers.sort(Battle.compareRedirectOrder);
    //      } else {
    //          this.speedSort(handlers);
    //      }
    //      let hasRelayVar = 1;
    //      const args = [target, source, sourceEffect];
    //      // console.log('Event: ' + eventid + ' (depth ' + this.eventDepth + ') t:' + target.id + ' s:' + (!source || source.id) + ' e:' + effect.id);
    //      if (relayVar === undefined || relayVar === null) {
    //          relayVar = true;
    //          hasRelayVar = 0;
    //      } else {
    //          args.unshift(relayVar);
    //      }
    //
    //      const parentEvent = this.event;
    //      this.event = { id: eventid, target, source, effect: sourceEffect, modifier: 1 };
    //      this.eventDepth++;
    //
    //      let targetRelayVars = [];
    //      if (Array.isArray(target)) {
    //          if (Array.isArray(relayVar)) {
    //              targetRelayVars = relayVar;
    //          } else {
    //              for (let i = 0; i < target.length; i++) targetRelayVars[i] = true;
    //          }
    //      }
    //      for (const handler of handlers) {
    //          if (handler.index !== undefined) {
    //              // TODO: find a better way to do this
    //              if (!targetRelayVars[handler.index] && !(targetRelayVars[handler.index] === 0 &&
    //                  eventid === 'DamagingHit')) continue;
    //              if (handler.target) {
    //                  args[hasRelayVar] = handler.target;
    //                  this.event.target = handler.target;
    //              }
    //              if (hasRelayVar) args[0] = targetRelayVars[handler.index];
    //          }
    //          const effect = handler.effect;
    //          const effectHolder = handler.effectHolder;
    //          // this.debug('match ' + eventid + ': ' + status.id + ' ' + status.effectType);
    //          if (effect.effectType === 'Status' && (effectHolder as Pokemon).status !== effect.id) {
    //              // it's changed; call it off
    //              continue;
    //          }
    //          if (effect.effectType === 'Ability' && effect.flags['breakable'] &&
    //              this.suppressingAbility(effectHolder as Pokemon)) {
    //              if (effect.flags['breakable']) {
    //                  this.debug(eventid + ' handler suppressed by Mold Breaker');
    //                  continue;
    //              }
    //              if (!effect.num) {
    //                  // ignore attacking events for custom abilities
    //                  const AttackingEvents = {
    //                      BeforeMove: 1,
    //                      BasePower: 1,
    //                      Immunity: 1,
    //                      RedirectTarget: 1,
    //                      Heal: 1,
    //                      SetStatus: 1,
    //                      CriticalHit: 1,
    //                      ModifyAtk: 1, ModifyDef: 1, ModifySpA: 1, ModifySpD: 1, ModifySpe: 1, ModifyAccuracy: 1,
    //                      ModifyBoost: 1,
    //                      ModifyDamage: 1,
    //                      ModifySecondaries: 1,
    //                      ModifyWeight: 1,
    //                      TryAddVolatile: 1,
    //                      TryHit: 1,
    //                      TryHitSide: 1,
    //                      TryMove: 1,
    //                      Boost: 1,
    //                      DragOut: 1,
    //                      Effectiveness: 1,
    //                  };
    //                  if (eventid in AttackingEvents) {
    //                      this.debug(eventid + ' handler suppressed by Mold Breaker');
    //                      continue;
    //                  } else if (eventid === 'Damage' && sourceEffect && sourceEffect.effectType === 'Move') {
    //                      this.debug(eventid + ' handler suppressed by Mold Breaker');
    //                      continue;
    //                  }
    //              }
    //          }
    //          if (eventid !== 'Start' && eventid !== 'SwitchIn' && eventid !== 'TakeItem' &&
    //              effect.effectType === 'Item' && (effectHolder instanceof Pokemon) && effectHolder.ignoringItem()) {
    //              if (eventid !== 'Update') {
    //                  this.debug(eventid + ' handler suppressed by Embargo, Klutz or Magic Room');
    //              }
    //              continue;
    //          } else if (
    //              eventid !== 'End' && effect.effectType === 'Ability' &&
    //              (effectHolder instanceof Pokemon) && effectHolder.ignoringAbility()
    //          ) {
    //              if (eventid !== 'Update') {
    //                  this.debug(eventid + ' handler suppressed by Gastro Acid or Neutralizing Gas');
    //              }
    //              continue;
    //          }
    //          if (
    //              (effect.effectType === 'Weather' || eventid === 'Weather') &&
    //              eventid !== 'Residual' && eventid !== 'End' && this.field.suppressingWeather()
    //          ) {
    //              this.debug(eventid + ' handler suppressed by Air Lock');
    //              continue;
    //          }
    //          let returnVal;
    //          if (typeof handler.callback === 'function') {
    //              const parentEffect = this.effect;
    //              const parentEffectState = this.effectState;
    //              this.effect = handler.effect;
    //              this.effectState = handler.state || this.initEffectState({});
    //              this.effectState.target = effectHolder;
    //
    //              returnVal = handler.callback.apply(this, args);
    //
    //              this.effect = parentEffect;
    //              this.effectState = parentEffectState;
    //          } else {
    //              returnVal = handler.callback;
    //          }
    //
    //          if (returnVal !== undefined) {
    //              relayVar = returnVal;
    //              if (!relayVar || fastExit) {
    //                  if (handler.index !== undefined) {
    //                      targetRelayVars[handler.index] = relayVar;
    //                      if (targetRelayVars.every(val => !val)) break;
    //                  } else {
    //                      break;
    //                  }
    //              }
    //              if (hasRelayVar) {
    //                  args[0] = relayVar;
    //              }
    //          }
    //      }
    //
    //      this.eventDepth--;
    //      if (typeof relayVar === 'number' && relayVar === Math.abs(Math.floor(relayVar))) {
    //          // this.debug(eventid + ' modifier: 0x' +
    //          //  ('0000' + (this.event.modifier * 4096).toString(16)).slice(-4).toUpperCase());
    //          relayVar = this.modify(relayVar, this.event.modifier);
    //      }
    //      this.event = parentEvent;
    //
    //      return Array.isArray(target) ? targetRelayVars : relayVar;
    //  }
    //
    pub fn run_event(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        mut relay_var: EventResult,
        on_effect: bool,
        fast_exit: bool,
    ) -> EventResult {
        // JavaScript: if (this.eventDepth >= 8) { ... }
        // Stack overflow protection
        if self.event_depth >= 8 {
            self.add("message", &[Arg::Str("STACK LIMIT EXCEEDED")]);
            self.add("message", &[Arg::Str("PLEASE REPORT IN BUG THREAD")]);
            self.add("message", &[Arg::String(format!("Event: {}", event_id))]);
            if let Some(ref event) = self.event {
                self.add("message", &[Arg::String(format!("Parent event: {}", event.id))]);
            }
            panic!("Stack overflow");
        }

        // JavaScript: const handlers = this.findEventHandlers(target, eventid, effectSource);
        // Find all event handlers for this event
        let mut handlers = self.find_event_handlers(event_id, target, source);

        // TODO: Implement onEffect handler insertion (JavaScript lines 134-143)
        // This requires more infrastructure for handling effect callbacks
        if on_effect && source_effect.is_some() {
            // For now, skip onEffect handling as it's not commonly used
            // Will need to implement when encounter a case that needs it
        }

        // JavaScript: Sort handlers based on event type (lines 145-151)
        if matches!(event_id, "Invulnerability" | "TryHit" | "DamagingHit" | "EntryHazard") {
            // Left-to-right order
            handlers.sort_by(|a, b| {
                let a_item = Self::event_listener_to_priority_item(a);
                let b_item = Self::event_listener_to_priority_item(b);
                Self::compare_left_to_right_order(&a_item, &b_item)
            });
        } else if fast_exit {
            // Redirect order
            handlers.sort_by(|a, b| {
                let a_item = Self::event_listener_to_priority_item(a);
                let b_item = Self::event_listener_to_priority_item(b);
                Self::compare_redirect_order(&a_item, &b_item)
            });
        } else {
            // Speed sort (default)
            self.speed_sort(&mut handlers, |listener| {
                Self::event_listener_to_priority_item(listener)
            });
        }

        // JavaScript: let hasRelayVar = 1; (lines 152-160)
        // Track if relay variable was explicitly provided
        // Note: In Rust we don't need this for argument handling, but keep for future reference
        let mut _has_relay_var = true;
        match relay_var {
            EventResult::Continue => {
                // JavaScript: if (relayVar === undefined || relayVar === null) { relayVar = true; hasRelayVar = 0; }
                relay_var = EventResult::Boolean(true);
                _has_relay_var = false;
            }
            _ => {}
        }

        // JavaScript: const parentEvent = this.event; (lines 162-164)
        // Save parent event
        let parent_event = self.event.take();

        // JavaScript: this.event = { id: eventid, target, source, effect: sourceEffect, modifier: 1 };
        // Create new event context
        self.event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect.cloned(),
            modifier: 4096, // 4096 = 1.0x in JavaScript
            relay_var: None,
            relay_var_float: None,
            relay_var_boost: None,
            relay_var_secondaries: None,
            relay_var_type: None,
        });

        // JavaScript: this.eventDepth++;
        self.event_depth += 1;

        // JavaScript: Loop through handlers (lines 174-282)
        // Execute each handler and accumulate results
        for handler in handlers {
            let effect_id = handler.effect_id.clone();
            let handler_target = handler.effect_holder;
            let event_variant = handler.event_name.clone();

            // JavaScript: const effect = handler.effect;
            // JavaScript: const effectHolder = handler.effectHolder;

            // JavaScript: if (effect.effectType === 'Status' && (effectHolder as Pokemon).status !== effect.id) continue;
            // Check if status has changed
            if let Some(handler_pos) = handler_target {
                if let Some(pokemon) = self.pokemon_at(handler_pos.0, handler_pos.1) {
                    // Check if this is a status effect and it's no longer active
                    if let Some(_status_data) = self.dex.conditions().get_by_id(&effect_id) {
                        if pokemon.status.as_str() != effect_id.as_str() && !pokemon.volatiles.contains_key(&effect_id) {
                            // Status has changed, skip this handler
                            continue;
                        }
                    }
                }
            }

            // JavaScript: Ability suppression check (lines 192-229)
            // Check if ability is suppressed by Mold Breaker, etc.
            if let Some(ability_data) = self.dex.abilities().get(effect_id.as_str()) {
                if let Some(handler_pos) = handler_target {
                    // Check if ability is suppressed by Mold Breaker
                    if self.suppressing_ability(Some(handler_pos)) {
                        // JavaScript: if (effect.flags['breakable']) { continue; }
                        let is_breakable = ability_data.flags.get("breakable").copied().unwrap_or(0) != 0;

                        if is_breakable {
                            self.debug(&format!("{} handler suppressed by Mold Breaker", event_id));
                            continue;
                        }

                        // JavaScript: if (!effect.num) { ... check AttackingEvents ... }
                        // Custom abilities (no num) have their attacking events suppressed
                        if ability_data.num == 0 {
                            // List of attacking events that get suppressed for custom abilities
                            let attacking_events = [
                                "BeforeMove", "BasePower", "Immunity", "RedirectTarget", "Heal", "SetStatus",
                                "CriticalHit", "ModifyAtk", "ModifyDef", "ModifySpA", "ModifySpD", "ModifySpe",
                                "ModifyAccuracy", "ModifyBoost", "ModifyDamage", "ModifySecondaries",
                                "ModifyWeight", "TryAddVolatile", "TryHit", "TryHitSide", "TryMove",
                                "Boost", "DragOut", "Effectiveness",
                            ];

                            if attacking_events.contains(&event_id) {
                                self.debug(&format!("{} handler suppressed by Mold Breaker", event_id));
                                continue;
                            }

                            // JavaScript: else if (eventid === 'Damage' && sourceEffect && sourceEffect.effectType === 'Move')
                            if event_id == "Damage" {
                                if let Some(source_eff) = source_effect {
                                    if self.get_effect_type(source_eff) == "Move" {
                                        self.debug(&format!("{} handler suppressed by Mold Breaker", event_id));
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // JavaScript: Item suppression check (lines 230-235)
            // Check if item is being ignored
            if event_id != "Start" && event_id != "SwitchIn" && event_id != "TakeItem" {
                if let Some(_item_data) = self.dex.items().get(effect_id.as_str()) {
                    if let Some(handler_pos) = handler_target {
                        if let Some(pokemon) = self.pokemon_at(handler_pos.0, handler_pos.1) {
                            if pokemon.ignoring_item(self, false) {
                                if event_id != "Update" {
                                    self.debug(&format!("{} handler suppressed by Embargo, Klutz or Magic Room", event_id));
                                }
                                continue;
                            }
                        }
                    }
                }
            }

            // JavaScript: Ability ignoring check (lines 236-244)
            // Check if ability is being ignored
            if event_id != "End" {
                if let Some(_ability_data) = self.dex.abilities().get(effect_id.as_str()) {
                    if let Some(handler_pos) = handler_target {
                        if let Some(pokemon) = self.pokemon_at(handler_pos.0, handler_pos.1) {
                            if pokemon.ignoring_ability(self) {
                                if event_id != "Update" {
                                    self.debug(&format!("{} handler suppressed by Gastro Acid or Neutralizing Gas", event_id));
                                }
                                continue;
                            }
                        }
                    }
                }
            }

            // JavaScript: Weather suppression check (lines 245-251)
            // Check if weather is being suppressed
            if event_id != "Residual" && event_id != "End" {
                // Check if this is a weather effect
                if !self.field.weather.is_empty() && self.field.weather.as_str() == effect_id.as_str() {
                    if self.field.suppressing_weather() {
                        self.debug(&format!("{} handler suppressed by Air Lock", event_id));
                        continue;
                    }
                }
            }

            // JavaScript: Execute handler (lines 252-266)
            // Dispatch to the appropriate handler
            let return_val = self.dispatch_single_event(&event_variant, &effect_id, handler_target, source);

            // JavaScript: if (returnVal !== undefined) { ... } (lines 268-281)
            // Update relay variable if handler returned a value
            match return_val {
                EventResult::Continue => {
                    // Handler returned undefined/Continue, keep going
                }
                _ => {
                    // Handler returned a value, update relay_var
                    relay_var = return_val.clone();

                    // JavaScript: if (!relayVar || fastExit) { ... break; }
                    // Check if we should stop processing
                    let should_stop = match &relay_var {
                        EventResult::Boolean(false) => true,
                        EventResult::Null => true,
                        EventResult::Stop => true,
                        EventResult::NotFail => true,
                        _ => false,
                    };

                    if should_stop || fast_exit {
                        break;
                    }
                }
            }
        }

        // JavaScript: this.eventDepth--;
        self.event_depth -= 1;

        // JavaScript: if (typeof relayVar === 'number' && relayVar === Math.abs(Math.floor(relayVar))) {
        //     relayVar = this.modify(relayVar, this.event.modifier);
        // }
        // Apply modifier to numeric relay variables
        if let EventResult::Number(num) = relay_var {
            if let Some(ref event) = self.event {
                let modified = self.modify_internal(num, event.modifier);
                relay_var = EventResult::Number(modified);
            }
        }

        // JavaScript: this.event = parentEvent;
        // Restore parent event
        self.event = parent_event;

        relay_var
    }

    // ===========================================================================
    // Helper methods
    // ===========================================================================

    /// Convert EventListener to PriorityItem for sorting
    /// Not in JavaScript - Rust helper for sorting handlers
    fn event_listener_to_priority_item(listener: &EventListener) -> PriorityItem {
        PriorityItem {
            order: listener.order,
            priority: listener.priority,
            speed: listener.speed.unwrap_or(0.0),
            sub_order: listener.sub_order,
            effect_order: listener.effect_order.unwrap_or(0),
            index: listener.index.unwrap_or(0),
        }
    }
}
