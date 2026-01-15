use crate::*;
use crate::battle::Effect;
use crate::event::EventResult;

impl Pokemon {
    /// Add a volatile condition to a Pokemon
    // addVolatile(
    //     status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null,
    //     linkedStatus: string | Condition | null = null
    // ): boolean | any {
    //     let result;
    //     status = this.battle.dex.conditions.get(status);
    //     if (!this.hp && !status.affectsFainted) return false;
    //     if (linkedStatus && source && !source.hp) return false;
    //     if (this.battle.event) {
    //         if (!source) source = this.battle.event.source;
    //         if (!sourceEffect) sourceEffect = this.battle.effect;
    //     }
    //     if (!source) source = this;

    //     if (this.volatiles[status.id]) {
    //         if (!status.onRestart) return false;
    //         return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
    //     }
    //     if (!this.runStatusImmunity(status.id)) {
    //         this.battle.debug('immune to volatile status');
    //         if ((sourceEffect as Move)?.status) {
    //             this.battle.add('-immune', this);
    //         }
    //         return false;
    //     }
    //     result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
    //     if (!result) {
    //         this.battle.debug('add volatile [' + status.id + '] interrupted');
    //         return result;
    //     }
    //     this.volatiles[status.id] = this.battle.initEffectState({ id: status.id, name: status.name, target: this });
    //     if (source) {
    //         this.volatiles[status.id].source = source;
    //         this.volatiles[status.id].sourceSlot = source.getSlot();
    //     }
    //     if (sourceEffect) this.volatiles[status.id].sourceEffect = sourceEffect;
    //     if (status.duration) this.volatiles[status.id].duration = status.duration;
    //     if (status.durationCallback) {
    //         this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
    //     }
    //     result = this.battle.singleEvent('Start', status, this.volatiles[status.id], this, source, sourceEffect);
    //     if (!result) {
    //         // cancel
    //         delete this.volatiles[status.id];
    //         return result;
    //     }
    //     if (linkedStatus && source) {
    //         if (!source.volatiles[linkedStatus.toString()]) {
    //             source.addVolatile(linkedStatus, this, sourceEffect);
    //             source.volatiles[linkedStatus.toString()].linkedPokemon = [this];
    //             source.volatiles[linkedStatus.toString()].linkedStatus = status;
    //         } else {
    //             source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
    //         }
    //         this.volatiles[status.toString()].linkedPokemon = [source];
    //         this.volatiles[status.toString()].linkedStatus = linkedStatus;
    //     }
    //     return true;
    // }
    ///
    /// In Rust, this is an associated function (not a method) because it needs
    /// mutable access to Battle while operating on a Pokemon within that Battle.
    /// Call as: Pokemon::add_volatile(battle, target_pos, volatile_id, source_pos, source_effect, linked_status)
    ///
    /// Returns:
    /// - Some(true): volatile was added successfully
    /// - Some(false): volatile was blocked/prevented (immunity, TryAddVolatile, Start failed)
    /// - None: volatile already existed and onRestart was called (neither success nor failure)
    ///         This matches JavaScript's undefined return for the restart case
    pub fn add_volatile(
        battle: &mut Battle,
        target_pos: (usize, usize),
        volatile_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&Effect>,
        linked_status: Option<ID>,
        embedded_condition: Option<&crate::dex::ConditionData>,
    ) -> Option<bool> {
        // Get Pokemon name for logging
        let pokemon_name = if let Some(pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
            pokemon.name.clone()
        } else {
            "Unknown".to_string()
        };

        if battle.turn == 17 && volatile_id.as_str() == "skydrop" {
            eprintln!("[ADD_VOLATILE_DEBUG] turn=17, adding skydrop to {}, source_effect={:?}",
                pokemon_name, source_effect.map(|s| s.id.as_str()));
        }

        crate::trace_volatile!("turn={}, ADD volatile='{}' to {}, source_effect={:?}, embedded_condition={}",
            battle.turn, volatile_id.as_str(), pokemon_name, source_effect.map(|s| s.id.as_str()), embedded_condition.is_some());

        // JS: status = this.battle.dex.conditions.get(status);
        // JS: if (!this.hp && !status.affectsFainted) return false;
        // ✅ NOW IMPLEMENTED (Session 24 Part 37): HP check with affectsFainted flag
        // ✅ NOW IMPLEMENTED: Check embedded condition if not in dex
        let (target_hp, affects_fainted) = {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return Some(false),
            };

            // Try dex.conditions first, then fall back to embedded condition
            let affects_fainted = battle.dex.conditions().get_by_id(&volatile_id)
                .map(|cond| cond.affects_fainted)
                .or_else(|| {
                    embedded_condition
                        .map(|cond| cond.affects_fainted)
                })
                .unwrap_or(false);
            (pokemon.hp, affects_fainted)
        };

        if target_hp == 0 && !affects_fainted {
            return Some(false);
        }

        // JS: if (linkedStatus && source && !source.hp) return false;
        // ✅ NOW IMPLEMENTED: linkedStatus parameter (Session 24 Part 12)
        // ✅ NOW IMPLEMENTED: source HP check (Session 24 Part 15)
        if let (Some(_linked_status_id), Some(src_pos)) = (linked_status.as_ref(), source_pos) {
            let source_pokemon = match battle.pokemon_at(src_pos.0, src_pos.1) {
                Some(p) => p,
                None => return Some(false),
            };
            if source_pokemon.hp == 0 {
                return Some(false);
            }
        }

        // JS: if (this.battle.event) {
        // JS:     if (!source) source = this.battle.event.source;
        // JS:     if (!sourceEffect) sourceEffect = this.battle.effect;
        // JS: }
        // Note: Missing battle.event source/sourceEffect defaulting - requires event system

        // JS: if (!source) source = this;
        // ✅ NOW IMPLEMENTED (Session 24 Part 37): Default source to target if not provided
        let source_pos = source_pos.or(Some(target_pos));

        // Check if pokemon already has this volatile
        {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return Some(false),
            };

            // JS: if (this.volatiles[status.id]) {
            // JS:     if (!status.onRestart) return false;
            // JS:     return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
            // JS: }
            if pokemon.volatiles.contains_key(&volatile_id) {
                // JavaScript checks if onRestart callback EXISTS before calling singleEvent
                // If no onRestart callback, return false immediately (no durationCallback called)
                if !crate::data::condition_callbacks::has_on_restart_callback(volatile_id.as_str()) {
                    return Some(false);
                }

                // Call onRestart callback if the volatile already exists
                // IMPORTANT: Call through single_event (not dispatch_on_restart directly)
                // to ensure current_effect_state is set up correctly
                // IMPORTANT: Effect must have effect_holder set so with_effect_state can find the volatile state
                let volatile_effect = crate::battle::Effect {
                    id: volatile_id.clone(),
                    name: volatile_id.to_string(),
                    effect_type: crate::battle::EffectType::Condition,
                    effect_holder: Some(target_pos),
                    side_index: Some(target_pos.0),
                    prankster_boosted: false,
                };
                let restart_result = battle.single_event(
                    "Restart",
                    &volatile_effect,
                    None,
                    Some(target_pos),
                    source_pos,
                    source_effect,
                    None,
                );

                // JavaScript behavior for restart case:
                // - singleEvent('Restart') returns the callback's return value
                // - If callback returns undefined (EventResult::Continue), singleEvent returns undefined
                // - undefined is truthy-ish in combineResults but not a success/failure
                // - This is why we return None for restart case: neither success nor explicit failure
                use crate::event::EventResult;
                match restart_result {
                    // Callback explicitly returned false - treat as failure
                    EventResult::Boolean(false) => return Some(false),
                    // Callback returned true - treat as success
                    EventResult::Boolean(true) => return Some(true),
                    // Callback returned null - in JavaScript, null has lower priority than boolean
                    // in combineResults, so combineResults(bool, null) returns the bool.
                    // Map to Some(false) so that combine_results(Success, Failed) returns Success
                    // and combine_results(Failed, Failed) returns Failed.
                    EventResult::Null => return Some(false),
                    // Callback returned undefined/Continue - restart happened but neither success nor failure
                    // This matches JavaScript returning undefined from singleEvent when callback returns undefined
                    EventResult::Continue => return None,
                    // Other truthy values (Number, String, etc.) - treat as success
                    _ => return Some(true),
                }
            }
        }

        // JS: if (!this.runStatusImmunity(status.id)) {
        // JS:     this.battle.debug('immune to volatile status');
        // JS:     if ((sourceEffect as Move)?.status) {
        // JS:         this.battle.add('-immune', this);
        // JS:     }
        // JS:     return false;
        // JS: }
        // ✅ NOW IMPLEMENTED: runStatusImmunity check for volatile
        let can_be_volatile = Pokemon::run_status_immunity(battle, target_pos, volatile_id.as_str(), false);

        if !can_be_volatile {
            eprintln!("[ADD_VOLATILE_FAIL] {} is immune to volatile '{}'", pokemon_name, volatile_id.as_str());
            // ✅ NOW IMPLEMENTED (Session 24 Part 37): sourceEffect.status check for -immune message
            if let Some(src_effect) = source_effect {
                // Check if sourceEffect is a Move with a status property
                if let Some(move_data) = battle.dex.moves().get_by_id(&src_effect.id) {
                    if move_data.secondary.is_some() || move_data.status.is_some() {
                        // Add -immune message
                        let target_arg = {
                            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                                Some(p) => p,
                                None => return Some(false),
                            };
                            pokemon.get_slot()
                        };
                        battle.add("-immune", &[target_arg.into()]);
                    }
                }
            }
            return Some(false);
        }

        // JS: result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
        // JS: if (!result) {
        // JS:     this.battle.debug('add volatile [' + status.id + '] interrupted');
        // JS:     return result;
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 81): runEvent('TryAddVolatile')
        // JavaScript: result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
        // The status object is passed as 5th parameter (relayVar)
        // Rust: Use run_event to pass volatile_id as EventResult::String
        let try_add_result = battle.run_event(
                "TryAddVolatile",
                Some(crate::event::EventTarget::Pokemon(target_pos)),
            source_pos,
            source_effect,
            crate::event::EventResult::String(volatile_id.as_str().to_string()),
            false,
            false,
        );
        // runEvent returns EventResult, check if it's falsy (Boolean(false), Number(0), Null)
        if !try_add_result.is_truthy() {
            eprintln!("[ADD_VOLATILE_FAIL] TryAddVolatile blocked for '{}' on {}", volatile_id.as_str(), pokemon_name);
            return Some(false);
        }

        // Get default duration from dex.conditions, embedded condition, or item's condition block
        // JS: if (status.duration) this.volatiles[status.id].duration = status.duration;
        // JavaScript dex.conditions.get() also checks items for their condition blocks
        let default_duration = battle.dex.conditions().get_by_id(&volatile_id)
            .and_then(|cond| cond.duration)
            .or_else(|| embedded_condition.and_then(|cond| cond.duration))
            .or_else(|| {
                // Check if this volatile comes from an item's condition block
                // JavaScript: dex.conditions.get() checks items[id].condition if not found in conditions
                battle.dex.items().get_by_id(&volatile_id)
                    .and_then(|item| item.extra.get("condition"))
                    .and_then(|cond| cond.as_object())
                    .and_then(|cond| cond.get("duration"))
                    .and_then(|d| d.as_i64())
                    .map(|d| d as i32)
            });

        if battle.turn == 17 && volatile_id.as_str() == "skydrop" {
            let from_dex = battle.dex.conditions().get_by_id(&volatile_id)
                .and_then(|cond| cond.duration);
            let from_embedded = embedded_condition.and_then(|cond| cond.duration);
            eprintln!("[ADD_VOLATILE_DURATION] turn=17, volatile='skydrop', from_dex={:?}, from_embedded={:?}, final={:?}",
                from_dex, from_embedded, default_duration);
        }

        // Check if condition has a durationCallback
        // JS: if (status.durationCallback) {
        //     this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
        // }
        let callback_duration = {
            let result = crate::data::condition_callbacks::dispatch_duration_callback(
                battle,
                volatile_id.as_str(),
                target_pos,
                source_pos,
                source_effect.map(|eff| eff.id.as_str()),
            );
            match result {
                crate::event::EventResult::Number(n) => Some(n),
                _ => None,
            }
        };

        // durationCallback overrides default duration
        let final_duration = callback_duration.or(default_duration);

        // Capture current turn before mutable borrow
        let current_turn = battle.turn;

        // JS: this.volatiles[status.id] = this.battle.initEffectState({ id: status.id, name: status.name, target: this });
        // JS: if (source) {
        // JS:     this.volatiles[status.id].source = source;
        // JS:     this.volatiles[status.id].sourceSlot = source.getSlot();
        // JS: }
        // JS: if (sourceEffect) this.volatiles[status.id].sourceEffect = sourceEffect;
        // Create effect state with duration
        let mut state = crate::event_system::EffectState::new(volatile_id.clone());
        state.duration = final_duration;
        state.created_turn = Some(current_turn);
        // ✅ NOW IMPLEMENTED: target assignment (Session 24 Part 21)
        state.target = Some(target_pos);
        // ✅ NOW IMPLEMENTED: source, sourceSlot assignments (Session 24 Part 20)
        // source_slot should be the active slot position (pokemon.position), not party index
        // JavaScript: this.volatiles[status.id].sourceSlot = source.getSlot();
        // getSlot() uses this.position (the active slot index: 0, 1, 2...)
        if let Some(src_pos) = source_pos {
            state.source = Some(src_pos);
            // Get the source pokemon's active slot position
            let source_position = if let Some(source_pokemon) = battle.pokemon_at(src_pos.0, src_pos.1) {
                source_pokemon.position
            } else {
                0
            };
            state.source_slot = Some(source_position);
        }
        // ✅ NOW IMPLEMENTED: sourceEffect assignment (Session 24 Part 27)
        if let Some(src_effect) = source_effect {
            state.source_effect = Some(src_effect.clone());
        }

        // JavaScript: this.volatiles[status.id] = this.battle.initEffectState(this.volatiles[status.id]);
        // Initialize effect_order (increments battle.effect_order counter)
        let state = battle.init_effect_state(state, None);

        // Add the volatile to pokemon
        let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return Some(false),
        };
        pokemon_mut.volatiles.insert(volatile_id.clone(), state);
        eprintln!("[ADD_VOLATILE_SUCCESS] Added volatile '{}' to {}", volatile_id.as_str(), pokemon_name);

        // JavaScript: result = this.battle.singleEvent("Start", status, this.volatiles[status.id], this, source, sourceEffect);
        // Call the Start event for the newly added volatile
        // IMPORTANT: Effect must have effect_holder set so with_effect_state can find the volatile state
        let volatile_effect = crate::battle::Effect {
            id: volatile_id.clone(),
            name: volatile_id.to_string(),
            effect_type: crate::battle::EffectType::Condition,
            effect_holder: Some(target_pos),
            side_index: Some(target_pos.0),
            prankster_boosted: false,
        };
        let start_result = battle.single_event(
            "Start",
            &volatile_effect,
            None,
            Some(target_pos),
            source_pos,
            source_effect,
            None,
        );

        // JavaScript: if (!result) { delete this.volatiles[status.id]; return result; }
        // If Start event returns false/failure, remove the volatile
        match start_result {
            EventResult::Boolean(false) => {
                // Start event failed, remove the volatile
                // NOTE: Direct removal is correct here (not Pokemon::remove_volatile) because:
                // 1. JavaScript does direct `delete this.volatiles[status.id]` (not removeVolatile)
                // 2. This is rollback before linkedPokemon setup, so no cleanup needed
                // 3. We shouldn't call singleEvent('End') for a volatile that failed Start
                let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return Some(false),
                };
                pokemon_mut.volatiles.remove(&volatile_id);
                return Some(false);
            }
            _ => {
                // Start event succeeded or returned non-Boolean
                // JS: if (linkedStatus && source) {
                // JS:     if (!source.volatiles[linkedStatus.toString()]) {
                // JS:         source.addVolatile(linkedStatus, this, sourceEffect);
                // JS:         source.volatiles[linkedStatus.toString()].linkedPokemon = [this];
                // JS:         source.volatiles[linkedStatus.toString()].linkedStatus = status;
                // JS:     } else {
                // JS:         source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
                // JS:     }
                // JS:     this.volatiles[status.toString()].linkedPokemon = [source];
                // JS:     this.volatiles[status.toString()].linkedStatus = linkedStatus;
                // JS: }

                // ✅ NOW IMPLEMENTED: linkedStatus bidirectional linking (Leech Seed, Powder moves, etc.)
                if let (Some(linked_status_id), Some(src_pos)) = (linked_status, source_pos) {
                    // Check if source already has the linked volatile
                    let source_has_volatile = {
                        let source_pokemon = match battle.pokemon_at(src_pos.0, src_pos.1) {
                            Some(p) => p,
                            None => return Some(true), // Target volatile was added successfully
                        };
                        source_pokemon.volatiles.contains_key(&linked_status_id)
                    };

                    if !source_has_volatile {
                        // Add the linked volatile to source (recursive call)
                        // JS: source.addVolatile(linkedStatus, this, sourceEffect);
                        Pokemon::add_volatile(battle, src_pos, linked_status_id.clone(), Some(target_pos), source_effect, None, None);

                        // Initialize linkedPokemon array for source
                        // JS: source.volatiles[linkedStatus.toString()].linkedPokemon = [this];
                        if let Some(source_pokemon) = battle.pokemon_at_mut(src_pos.0, src_pos.1) {
                            if let Some(state) = source_pokemon.volatiles.get_mut(&linked_status_id) {
                                state.linked_pokemon = Some(vec![target_pos]);
                                // JS: source.volatiles[linkedStatus.toString()].linkedStatus = status;
                                state.linked_status = Some(volatile_id.as_str().to_string());
                            }
                        }
                    } else {
                        // Source already has the linked volatile, append to linkedPokemon array
                        // JS: source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
                        if let Some(source_pokemon) = battle.pokemon_at_mut(src_pos.0, src_pos.1) {
                            if let Some(state) = source_pokemon.volatiles.get_mut(&linked_status_id) {
                                if let Some(ref mut linked_vec) = state.linked_pokemon {
                                    linked_vec.push(target_pos);
                                } else {
                                    // Initialize if missing
                                    state.linked_pokemon = Some(vec![target_pos]);
                                }
                            }
                        }
                    }

                    // Set linkedPokemon and linkedStatus on the target's volatile
                    // JS: this.volatiles[status.toString()].linkedPokemon = [source];
                    // JS: this.volatiles[status.toString()].linkedStatus = linkedStatus;
                    if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                        if let Some(state) = target_pokemon.volatiles.get_mut(&volatile_id) {
                            state.linked_pokemon = Some(vec![src_pos]);
                            state.linked_status = Some(linked_status_id.as_str().to_string());
                        }
                    }
                }

                return Some(true);
            }
        }
    }
}
