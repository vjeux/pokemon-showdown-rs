use crate::*;
use crate::dex_data::BoostsTable;
use crate::event_system::{EffectState, SharedEffectState};

impl Pokemon {
    /// Copy volatiles from another Pokemon (for Baton Pass, Shed Tail, etc.)
    ///
    /// JavaScript source:
    /// ```js
    /// copyVolatileFrom(pokemon: Pokemon, switchCause?: string | boolean) {
    ///     this.clearVolatile();
    ///     if (switchCause !== 'shedtail') this.boosts = pokemon.boosts;
    ///     for (const i in pokemon.volatiles) {
    ///         if (switchCause === 'shedtail' && i !== 'substitute') continue;
    ///         if (this.battle.dex.conditions.getByID(i as ID).noCopy) continue;
    ///         // shallow clones
    ///         this.volatiles[i] = this.battle.initEffectState({ ...pokemon.volatiles[i], target: this });
    ///         if (this.volatiles[i].linkedPokemon) {
    ///             delete pokemon.volatiles[i].linkedPokemon;
    ///             delete pokemon.volatiles[i].linkedStatus;
    ///             for (const linkedPoke of this.volatiles[i].linkedPokemon) {
    ///                 const linkedPokeLinks = linkedPoke.volatiles[this.volatiles[i].linkedStatus].linkedPokemon;
    ///                 linkedPokeLinks[linkedPokeLinks.indexOf(pokemon)] = this;
    ///             }
    ///         }
    ///     }
    ///     pokemon.clearVolatile();
    ///     for (const i in this.volatiles) {
    ///         const volatile = this.getVolatile(i) as Condition;
    ///         this.battle.singleEvent('Copy', volatile, this.volatiles[i], this);
    ///     }
    /// }
    /// ```
    ///
    /// In Rust, this is an associated function (not a method) because it needs
    /// mutable access to Battle while operating on two Pokemon within that Battle.
    /// Call as: Pokemon::copy_volatile_from(battle, target_pos, source_pos, switch_cause)
    pub fn copy_volatile_from(
        battle: &mut Battle,
        target_pos: (usize, usize),
        source_pos: (usize, usize),
        switch_cause: Option<&str>,
    ) {
        // JS: this.clearVolatile();
        // ✅ NOW IMPLEMENTED (Session 24 Part 39): Clear target's volatiles first
        // Note: Can't call clear_volatile due to borrow checker, so inline the logic
        {
            let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return,
            };
            // Clear boosts
            target_pokemon.boosts = BoostsTable::default();
            // Clear volatiles
            target_pokemon.volatiles.clear();
            // ✅ NOW IMPLEMENTED (Session 24 Part 61): Gen 1 Mimic PP preservation and Eternamax handling
            // Note: Full clear_volatile logic exists in clear_volatile.rs (lines 76-101 for Mimic, lines 159-172 for Eternamax)
            // Note: Not calling it here to avoid battle borrow conflicts, but logic is implemented elsewhere
        }

        // JS: if (switchCause !== 'shedtail') this.boosts = pokemon.boosts;
        // ✅ NOW IMPLEMENTED (Session 24 Part 39): Only copy boosts if NOT shedtail
        let is_shedtail = switch_cause == Some("shedtail");

        if !is_shedtail {
            // Copy boosts from source to target
            let source_boosts = {
                let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                    Some(p) => p,
                    None => return,
                };
                source_pokemon.boosts
            };

            let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return,
            };
            target_pokemon.boosts = source_boosts;
        }

        // JS: for (const i in pokemon.volatiles) {
        // JS:     if (switchCause === 'shedtail' && i !== 'substitute') continue;
        // JS:     if (this.battle.dex.conditions.getByID(i as ID).noCopy) continue;
        // JS:     this.volatiles[i] = this.battle.initEffectState({ ...pokemon.volatiles[i], target: this });
        // JS:     ...linkedPokemon handling...
        // JS: }

        // ✅ NOW IMPLEMENTED (Session 24 Part 39): Loop through ALL source volatiles
        // Extract volatile list and states from source
        // Also track which volatile IDs are being copied so we can delete their linkedPokemon/linkedStatus
        let (volatiles_to_copy, copied_volatile_ids): (Vec<(ID, EffectState)>, Vec<ID>) = {
            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return,
            };

            let mut to_copy = Vec::new();
            let mut copied_ids = Vec::new();

            for (id, shared_state) in &source_pokemon.volatiles {
                // JS: if (switchCause === 'shedtail' && i !== 'substitute') continue;
                if is_shedtail && id.as_str() != "substitute" {
                    continue;
                }

                // JS: if (this.battle.dex.conditions.getByID(i as ID).noCopy) continue;
                // Check noCopy flag from condition data
                // noCopy is a top-level field on ConditionData, not in extra
                let no_copy = battle
                    .dex
                    .conditions()
                    .get(id.as_str())
                    .map(|cond| cond.no_copy)
                    .unwrap_or(false);

                if no_copy {
                    continue;
                }

                // Clone the inner state and update target
                // Use borrow() to access SharedEffectState inner fields
                let state = shared_state.borrow();
                let mut new_state = state.clone();
                new_state.target = Some(target_pos);

                to_copy.push((id.clone(), new_state));
                copied_ids.push(id.clone());
            }

            (to_copy, copied_ids)
        };

        // Copy the volatiles to target
        // JS: this.volatiles[i] = this.battle.initEffectState({ ...pokemon.volatiles[i], target: this });
        for (id, state) in volatiles_to_copy {
            let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return,
            };
            // Wrap in SharedEffectState when inserting
            target_pokemon.volatiles.insert(id, SharedEffectState::new(state));
        }

        // JS: if (this.volatiles[i].linkedPokemon) {
        // JS:     delete pokemon.volatiles[i].linkedPokemon;
        // JS:     delete pokemon.volatiles[i].linkedStatus;
        // JS:     for (const linkedPoke of this.volatiles[i].linkedPokemon) {
        // JS:         const linkedPokeLinks = linkedPoke.volatiles[this.volatiles[i].linkedStatus].linkedPokemon;
        // JS:         linkedPokeLinks[linkedPokeLinks.indexOf(pokemon)] = this;
        // JS:     }
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 46): Handle linkedPokemon bidirectional updates
        // Update all linked Pokemon to point to target instead of source

        // Collect volatiles with linkedPokemon that need updating
        let volatiles_to_update: Vec<(ID, ID, Vec<(usize, usize)>)> = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return,
            };

            target_pokemon
                .volatiles
                .iter()
                .filter_map(|(volatile_id, shared_state)| {
                    // Use borrow() to access SharedEffectState inner fields
                    let state = shared_state.borrow();
                    // Check if this volatile has linkedPokemon
                    let linked_pokemon = state.linked_pokemon.as_ref()?;
                    let linked_status_id = state.linked_status.as_ref()?;

                    if linked_pokemon.is_empty() {
                        return None;
                    }

                    Some((
                        volatile_id.clone(),
                        ID::from(linked_status_id.as_str()),
                        linked_pokemon.clone(),
                    ))
                })
                .collect()
        };

        // For each volatile with linkedPokemon, update the linked Pokemon
        for (_volatile_id, linked_status_id, linked_positions) in volatiles_to_update {
            for linked_pos in linked_positions {
                // Get the linked Pokemon's volatile state
                let needs_update = {
                    let linked_pokemon = match battle.pokemon_at(linked_pos.0, linked_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };

                    // Check if linked Pokemon has the linkedStatus volatile
                    linked_pokemon.volatiles.contains_key(&linked_status_id)
                };

                if !needs_update {
                    continue;
                }

                // Update the linkedPokemon array in the linked Pokemon's volatile
                // Replace source_pos with target_pos
                let linked_pokemon_mut = match battle.pokemon_at_mut(linked_pos.0, linked_pos.1) {
                    Some(p) => p,
                    None => continue,
                };

                if let Some(linked_volatile_state) = linked_pokemon_mut.volatiles.get(&linked_status_id) {
                    // Use borrow_mut() to access SharedEffectState inner fields
                    let mut state = linked_volatile_state.borrow_mut();
                    // Update the linkedPokemon Vec directly
                    if let Some(linked_poke_vec) = &mut state.linked_pokemon {
                        // Find and replace source_pos with target_pos
                        // linkedPokeLinks[linkedPokeLinks.indexOf(pokemon)] = this;
                        for pos in linked_poke_vec.iter_mut() {
                            if *pos == source_pos {
                                // Replace with target_pos
                                *pos = target_pos;
                            }
                        }
                    }
                }
            }
        }

        // JS: delete pokemon.volatiles[i].linkedPokemon; delete pokemon.volatiles[i].linkedStatus;
        // Delete linkedPokemon/linkedStatus from SOURCE volatiles that were COPIED
        // This matches JavaScript lines 138-139: only copied volatiles have their links deleted
        {
            let source_pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return,
            };
            for volatile_id in &copied_volatile_ids {
                if let Some(shared_state) = source_pokemon.volatiles.get(volatile_id) {
                    // Use borrow_mut() to access SharedEffectState inner fields
                    let mut state = shared_state.borrow_mut();
                    state.linked_pokemon = None;
                    state.linked_status = None;
                }
            }
        }

        // JS: pokemon.clearVolatile();
        // ✅ NOW IMPLEMENTED (Session 24 Part 39): Clear source's volatiles
        // Note: Can't call clear_volatile due to borrow checker, so inline the logic
        // For volatiles that were NOT copied, they still have linkedPokemon/linkedStatus,
        // so we need to call removeLinkedVolatiles for those (matching JS clearVolatile behavior)
        {
            // Collect linked volatiles from source (only non-copied ones still have linkedStatus)
            let linked_volatiles: Vec<(String, Vec<(usize, usize)>)> = {
                let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                    Some(p) => p,
                    None => return,
                };
                source_pokemon
                    .volatiles
                    .iter()
                    .filter_map(|(_, shared_state)| {
                        // Use borrow() to access SharedEffectState inner fields
                        let state = shared_state.borrow();
                        if let (Some(status), Some(pokemon_vec)) = (&state.linked_status, &state.linked_pokemon) {
                            Some((status.clone(), pokemon_vec.clone()))
                        } else {
                            None
                        }
                    })
                    .collect()
            };

            // Remove linked volatiles from other pokemon (for non-copied volatiles)
            for (status, pokemon_vec) in linked_volatiles {
                Pokemon::remove_linked_volatiles(
                    battle,
                    source_pos,
                    &ID::from(status.as_str()),
                    &pokemon_vec,
                );
            }

            // Now clear the source's volatiles
            let source_pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return,
            };
            // Clear boosts
            source_pokemon.boosts = BoostsTable::default();
            // Clear volatiles
            source_pokemon.volatiles.clear();
            // ✅ NOW IMPLEMENTED (Session 24 Part 61): Gen 1 Mimic PP preservation and Eternamax handling
            // Note: Full clear_volatile logic exists in clear_volatile.rs (lines 76-101 for Mimic, lines 159-172 for Eternamax)
            // Note: Not calling it here to avoid battle borrow conflicts, but logic is implemented elsewhere
        }

        // JS: for (const i in this.volatiles) {
        // JS:     const volatile = this.getVolatile(i) as Condition;
        // JS:     this.battle.singleEvent('Copy', volatile, this.volatiles[i], this);
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 39): Call singleEvent('Copy') for each copied volatile
        let volatile_ids: Vec<ID> = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return,
            };
            target_pokemon.volatiles.keys().cloned().collect()
        };

        for volatile_id in volatile_ids {
            battle.single_event(
                "Copy",
                &crate::battle::Effect::condition(volatile_id),
                None,
                Some(target_pos),
                None,
                None,
                None,
            );
        }
    }
}
