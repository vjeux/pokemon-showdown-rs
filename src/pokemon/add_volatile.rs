use crate::*;
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
    pub fn add_volatile(
        battle: &mut Battle,
        target_pos: (usize, usize),
        volatile_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        linked_status: Option<ID>,
        embedded_condition: Option<&crate::dex::ConditionData>,
    ) -> bool {
        // Get Pokemon name for logging
        let pokemon_name = if let Some(pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
            pokemon.name.clone()
        } else {
            "Unknown".to_string()
        };

        crate::trace_volatile!("turn={}, ADD volatile='{}' to {}, source_effect={:?}, embedded_condition={}",
            battle.turn, volatile_id.as_str(), pokemon_name, source_effect.map(|s| s.as_str()), embedded_condition.is_some());

        // JS: status = this.battle.dex.conditions.get(status);
        // JS: if (!this.hp && !status.affectsFainted) return false;
        // ✅ NOW IMPLEMENTED (Session 24 Part 37): HP check with affectsFainted flag
        // ✅ NOW IMPLEMENTED: Check embedded condition if not in dex
        let (target_hp, affects_fainted) = {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return false,
            };

            // Try dex.conditions first, then fall back to embedded condition
            let affects_fainted = battle.dex.conditions().get_by_id(&volatile_id)
                .and_then(|cond| cond.extra.get("affectsFainted"))
                .and_then(|v| v.as_bool())
                .or_else(|| {
                    embedded_condition
                        .and_then(|cond| cond.extra.get("affectsFainted"))
                        .and_then(|v| v.as_bool())
                })
                .unwrap_or(false);
            (pokemon.hp, affects_fainted)
        };

        if target_hp == 0 && !affects_fainted {
            return false;
        }

        // JS: if (linkedStatus && source && !source.hp) return false;
        // ✅ NOW IMPLEMENTED: linkedStatus parameter (Session 24 Part 12)
        // ✅ NOW IMPLEMENTED: source HP check (Session 24 Part 15)
        if let (Some(_linked_status_id), Some(src_pos)) = (linked_status.as_ref(), source_pos) {
            let source_pokemon = match battle.pokemon_at(src_pos.0, src_pos.1) {
                Some(p) => p,
                None => return false,
            };
            if source_pokemon.hp == 0 {
                return false;
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
                None => return false,
            };

            // JS: if (this.volatiles[status.id]) {
            // JS:     if (!status.onRestart) return false;
            // JS:     return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
            // JS: }
            if pokemon.volatiles.contains_key(&volatile_id) {
                // Call onRestart callback if the volatile already exists
                let restart_result = crate::data::condition_callbacks::dispatch_on_restart(
                    battle,
                    volatile_id.as_str(),
                    target_pos,
                );

                // If onRestart returns false or Continue, return false (volatile not re-added)
                use crate::event::EventResult;
                match restart_result {
                    EventResult::Boolean(false) | EventResult::Continue => return false,
                    _ => return true,
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
            // ✅ NOW IMPLEMENTED (Session 24 Part 37): sourceEffect.status check for -immune message
            if let Some(src_effect) = source_effect {
                // Check if sourceEffect is a Move with a status property
                if let Some(move_data) = battle.dex.moves().get_by_id(src_effect) {
                    if move_data.secondary.is_some() || move_data.status.is_some() {
                        // Add -immune message
                        let target_arg = {
                            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                                Some(p) => p,
                                None => return false,
                            };
                            pokemon.get_slot()
                        };
                        battle.add("-immune", &[target_arg.into()]);
                    }
                }
            }
            return false;
        }

        // JS: result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
        // JS: if (!result) {
        // JS:     this.battle.debug('add volatile [' + status.id + '] interrupted');
        // JS:     return result;
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 81): runEvent('TryAddVolatile')
        // Note: JavaScript passes status as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check the volatile_id being added
        let try_add_result = battle.run_event("TryAddVolatile", Some(target_pos), source_pos, source_effect, EventResult::Continue, false, false);
        // runEvent returns Option<i32>, None or Some(0) means failure
        if matches!(try_add_result, EventResult::Number(0)) || matches!(try_add_result, EventResult::Null) {
            return false;
        }

        // Get default duration from dex.conditions or embedded condition
        // JS: if (status.duration) this.volatiles[status.id].duration = status.duration;
        let default_duration = battle.dex.conditions().get_by_id(&volatile_id)
            .and_then(|cond| cond.duration)
            .or_else(|| embedded_condition.and_then(|cond| cond.duration));

        // Check if condition has a durationCallback
        // JS: if (status.durationCallback) {
        //     this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
        // }
        let callback_duration = {
            let result = crate::data::condition_callbacks::dispatch_duration_callback(
                battle,
                volatile_id.as_str(),
                target_pos,
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

        // Add the volatile
        let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return false,
        };

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
        if let Some(src_pos) = source_pos {
            state.source = Some(src_pos);
            state.source_slot = Some(src_pos.1); // slot = position
        }
        // ✅ NOW IMPLEMENTED: sourceEffect assignment (Session 24 Part 27)
        if let Some(src_effect) = source_effect {
            state.source_effect = Some(src_effect.clone());
        }

        pokemon_mut.volatiles.insert(volatile_id.clone(), state);

        // JavaScript: result = this.battle.singleEvent("Start", status, this.volatiles[status.id], this, source, sourceEffect);
        // Call the Start event for the newly added volatile
        let start_result = battle.single_event(
            "Start",
            &volatile_id,
            Some(target_pos),
            source_pos,
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
                    None => return false,
                };
                pokemon_mut.volatiles.remove(&volatile_id);
                return false;
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
                            None => return true, // Target volatile was added successfully
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
                                state.data.insert(
                                    "linkedPokemon".to_string(),
                                    serde_json::json!([[target_pos.0, target_pos.1]]),
                                );
                                // JS: source.volatiles[linkedStatus.toString()].linkedStatus = status;
                                state.data.insert(
                                    "linkedStatus".to_string(),
                                    serde_json::json!(volatile_id.as_str()),
                                );
                            }
                        }
                    } else {
                        // Source already has the linked volatile, append to linkedPokemon array
                        // JS: source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
                        if let Some(source_pokemon) = battle.pokemon_at_mut(src_pos.0, src_pos.1) {
                            if let Some(state) = source_pokemon.volatiles.get_mut(&linked_status_id) {
                                if let Some(linked_pokemon) = state.data.get_mut("linkedPokemon") {
                                    if let Some(array) = linked_pokemon.as_array_mut() {
                                        array.push(serde_json::json!([target_pos.0, target_pos.1]));
                                    }
                                } else {
                                    // Initialize if missing
                                    state.data.insert(
                                        "linkedPokemon".to_string(),
                                        serde_json::json!([[target_pos.0, target_pos.1]]),
                                    );
                                }
                            }
                        }
                    }

                    // Set linkedPokemon and linkedStatus on the target's volatile
                    // JS: this.volatiles[status.toString()].linkedPokemon = [source];
                    // JS: this.volatiles[status.toString()].linkedStatus = linkedStatus;
                    if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                        if let Some(state) = target_pokemon.volatiles.get_mut(&volatile_id) {
                            state.data.insert(
                                "linkedPokemon".to_string(),
                                serde_json::json!([[src_pos.0, src_pos.1]]),
                            );
                            state.data.insert(
                                "linkedStatus".to_string(),
                                serde_json::json!(linked_status_id.as_str()),
                            );
                        }
                    }
                }

                return true;
            }
        }
    }
}
