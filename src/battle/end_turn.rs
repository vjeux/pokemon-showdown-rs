use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    // =========================================================================
    // Turn Management Methods (ported from battle.ts)
    // =========================================================================

    /// End the current turn
    /// Equivalent to battle.ts endTurn() (battle.ts:1577-1754)
    ///
    //
    // 	endTurn() {
    // 		this.turn++;
    // 		this.lastSuccessfulMoveThisTurn = null;
    //
    // 		const dynamaxEnding: Pokemon[] = [];
    // 		for (const pokemon of this.getAllActive()) {
    // 			if (pokemon.volatiles['dynamax']?.turns === 3) {
    // 				dynamaxEnding.push(pokemon);
    // 			}
    // 		}
    // 		if (dynamaxEnding.length > 1) {
    // 			this.updateSpeed();
    // 			this.speedSort(dynamaxEnding);
    // 		}
    // 		for (const pokemon of dynamaxEnding) {
    // 			pokemon.removeVolatile('dynamax');
    // 		}
    //
    // 		// Gen 1 partial trapping ends when either Pokemon or a switch in faints to residual damage
    // 		if (this.gen === 1) {
    // 			for (const pokemon of this.getAllActive()) {
    // 				if (pokemon.volatiles['partialtrappinglock']) {
    // 					const target = pokemon.volatiles['partialtrappinglock'].locked;
    // 					if (target.hp <= 0 || !target.volatiles['partiallytrapped']) {
    // 						delete pokemon.volatiles['partialtrappinglock'];
    // 					}
    // 				}
    // 				if (pokemon.volatiles['partiallytrapped']) {
    // 					const source = pokemon.volatiles['partiallytrapped'].source;
    // 					if (source.hp <= 0 || !source.volatiles['partialtrappinglock']) {
    // 						delete pokemon.volatiles['partiallytrapped'];
    // 					}
    // 				}
    // 				if (pokemon.volatiles['fakepartiallytrapped']) {
    // 					const counterpart = pokemon.volatiles['fakepartiallytrapped'].counterpart;
    // 					if (counterpart.hp <= 0 || !counterpart.volatiles['fakepartiallytrapped']) {
    // 						delete pokemon.volatiles['fakepartiallytrapped'];
    // 					}
    // 				}
    // 			}
    // 		}
    //
    // 		const trappedBySide: boolean[] = [];
    // 		const stalenessBySide: ('internal' | 'external' | undefined)[] = [];
    // 		for (const side of this.sides) {
    // 			let sideTrapped = true;
    // 			let sideStaleness: 'internal' | 'external' | undefined;
    // 			for (const pokemon of side.active) {
    // 				if (!pokemon) continue;
    // 				pokemon.moveThisTurn = '';
    // 				pokemon.newlySwitched = false;
    // 				pokemon.moveLastTurnResult = pokemon.moveThisTurnResult;
    // 				pokemon.moveThisTurnResult = undefined;
    // 				if (this.turn !== 1) {
    // 					pokemon.usedItemThisTurn = false;
    // 					pokemon.statsRaisedThisTurn = false;
    // 					pokemon.statsLoweredThisTurn = false;
    // 					// It shouldn't be possible in a normal battle for a Pokemon to be damaged before turn 1's move selection
    // 					// However, this could be potentially relevant in certain OMs
    // 					pokemon.hurtThisTurn = null;
    // 				}
    //
    // 				pokemon.maybeDisabled = false;
    // 				pokemon.maybeLocked = false;
    // 				for (const moveSlot of pokemon.moveSlots) {
    // 					moveSlot.disabled = false;
    // 					moveSlot.disabledSource = '';
    // 				}
    // 				this.runEvent('DisableMove', pokemon);
    // 				for (const moveSlot of pokemon.moveSlots) {
    // 					const activeMove = this.dex.getActiveMove(moveSlot.id);
    // 					this.singleEvent('DisableMove', activeMove, null, pokemon);
    // 					if (activeMove.flags['cantusetwice'] && pokemon.lastMove?.id === moveSlot.id) {
    // 						pokemon.disableMove(pokemon.lastMove.id);
    // 					}
    // 				}
    //
    // 				// If it was an illusion, it's not any more
    // 				if (pokemon.getLastAttackedBy() && this.gen >= 7) pokemon.knownType = true;
    //
    // 				for (let i = pokemon.attackedBy.length - 1; i >= 0; i--) {
    // 					const attack = pokemon.attackedBy[i];
    // 					if (attack.source.isActive) {
    // 						attack.thisTurn = false;
    // 					} else {
    // 						pokemon.attackedBy.splice(pokemon.attackedBy.indexOf(attack), 1);
    // 					}
    // 				}
    //
    // 				if (this.gen >= 7 && !pokemon.terastallized) {
    // 					// In Gen 7, the real type of every Pokemon is visible to all players via the bottom screen while making choices
    // 					const seenPokemon = pokemon.illusion || pokemon;
    // 					const realTypeString = seenPokemon.getTypes(true).join('/');
    // 					if (realTypeString !== seenPokemon.apparentType) {
    // 						this.add('-start', pokemon, 'typechange', realTypeString, '[silent]');
    // 						seenPokemon.apparentType = realTypeString;
    // 						if (pokemon.addedType) {
    // 							// The typechange message removes the added type, so put it back
    // 							this.add('-start', pokemon, 'typeadd', pokemon.addedType, '[silent]');
    // 						}
    // 					}
    // 				}
    //
    // 				pokemon.trapped = pokemon.maybeTrapped = false;
    // 				this.runEvent('TrapPokemon', pokemon);
    // 				if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon)) {
    // 					this.runEvent('MaybeTrapPokemon', pokemon);
    // 				}
    // 				// canceling switches would leak information
    // 				// if a foe might have a trapping ability
    // 				if (this.gen > 2) {
    // 					for (const source of pokemon.foes()) {
    // 						const species = (source.illusion || source).species;
    // 						if (!species.abilities) continue;
    // 						for (const abilitySlot in species.abilities) {
    // 							const abilityName = species.abilities[abilitySlot as keyof Species['abilities']];
    // 							if (abilityName === source.ability) {
    // 								// pokemon event was already run above so we don't need
    // 								// to run it again.
    // 								continue;
    // 							}
    // 							const ruleTable = this.ruleTable;
    // 							if ((ruleTable.has('+hackmons') || !ruleTable.has('obtainableabilities')) && !this.format.team) {
    // 								// hackmons format
    // 								continue;
    // 							} else if (abilitySlot === 'H' && species.unreleasedHidden) {
    // 								// unreleased hidden ability
    // 								continue;
    // 							}
    // 							const ability = this.dex.abilities.get(abilityName);
    // 							if (ruleTable.has('-ability:' + ability.id)) continue;
    // 							if (pokemon.knownType && !this.dex.getImmunity('trapped', pokemon)) continue;
    // 							this.singleEvent('FoeMaybeTrapPokemon', ability, {}, pokemon, source);
    // 						}
    // 					}
    // 				}
    //
    // 				if (pokemon.fainted) continue;
    //
    // 				sideTrapped = sideTrapped && pokemon.trapped;
    // 				const staleness = pokemon.volatileStaleness || pokemon.staleness;
    // 				if (staleness) sideStaleness = sideStaleness === 'external' ? sideStaleness : staleness;
    // 				pokemon.activeTurns++;
    // 			}
    // 			trappedBySide.push(sideTrapped);
    // 			stalenessBySide.push(sideStaleness);
    // 			side.faintedLastTurn = side.faintedThisTurn;
    // 			side.faintedThisTurn = null;
    // 		}
    //
    // 		if (this.maybeTriggerEndlessBattleClause(trappedBySide, stalenessBySide)) return;
    //
    // 		if (this.gameType === 'triples' && this.sides.every(side => side.pokemonLeft === 1)) {
    // 			// If both sides have one Pokemon left in triples and they are not adjacent, they are both moved to the center.
    // 			const actives = this.getAllActive();
    // 			if (actives.length > 1 && !actives[0].isAdjacent(actives[1])) {
    // 				this.swapPosition(actives[0], 1, '[silent]');
    // 				this.swapPosition(actives[1], 1, '[silent]');
    // 				this.add('-center');
    // 			}
    // 		}
    //
    // 		this.add('turn', this.turn);
    // 		if (this.gameType === 'multi') {
    // 			for (const side of this.sides) {
    // 				if (side.canDynamaxNow()) {
    // 					if (this.turn === 1) {
    // 						this.addSplit(side.id, ['-candynamax', side.id]);
    // 					} else {
    // 						this.add('-candynamax', side.id);
    // 					}
    // 				}
    // 			}
    // 		}
    // 		if (this.gen === 2) this.quickClawRoll = this.randomChance(60, 256);
    // 		if (this.gen === 3) this.quickClawRoll = this.randomChance(1, 5);
    //
    // 		this.makeRequest('move');
    // 	}
    //
    pub fn end_turn(&mut self) {
        eprintln!("[END_TURN] Called, turn {} -> {}", self.turn, self.turn + 1);
        self.turn += 1;

        // JS: this.lastSuccessfulMoveThisTurn = null;
        self.last_successful_move_this_turn = None;

        // Dynamax 3-turn removal
        // JS: const dynamaxEnding: Pokemon[] = [];
        // JS: for (const pokemon of this.getAllActive()) {
        // JS:     if (pokemon.volatiles['dynamax']?.turns === 3) {
        // JS:         dynamaxEnding.push(pokemon);
        // JS:     }
        // JS: }
        let dynamax_id = ID::new("dynamax");
        let mut dynamax_ending: Vec<(usize, usize)> = Vec::new();

        for side_idx in 0..self.sides.len() {
            for active_idx in 0..self.sides[side_idx].active.len() {
                if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(active_idx) {
                    if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                        // Check if Pokemon has dynamax volatile with turns === 3
                        if let Some(dynamax_state) = pokemon.volatiles.get(&dynamax_id) {
                            let turns = dynamax_state
                                .data
                                .get("turns")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0);
                            if turns == 3 {
                                dynamax_ending.push((side_idx, *poke_idx));
                            }
                        }
                    }
                }
            }
        }

        // JS: if (dynamaxEnding.length > 1) {
        // JS:     this.updateSpeed();
        // JS:     this.speedSort(dynamaxEnding);
        // JS: }
        if dynamax_ending.len() > 1 {
            // JS: this.updateSpeed() - updates ALL pokemon, not just dynamax ending ones
            self.update_speed();

            // Speed sort the Pokemon ending Dynamax
            dynamax_ending.sort_by(|&(side_a, poke_a), &(side_b, poke_b)| {
                let speed_a = self
                    .sides
                    .get(side_a)
                    .and_then(|s| s.pokemon.get(poke_a))
                    .map(|p| p.speed)
                    .unwrap_or(0);
                let speed_b = self
                    .sides
                    .get(side_b)
                    .and_then(|s| s.pokemon.get(poke_b))
                    .map(|p| p.speed)
                    .unwrap_or(0);
                speed_b.cmp(&speed_a) // Higher speed first
            });
        }

        // JS: for (const pokemon of dynamaxEnding) {
        // JS:     pokemon.removeVolatile('dynamax');
        // JS: }
        for (side_idx, poke_idx) in dynamax_ending {
            if let Some(pokemon) = self
                .sides
                .get_mut(side_idx)
                .and_then(|s| s.pokemon.get_mut(poke_idx))
            {
                pokemon.volatiles.remove(&dynamax_id);
            }
        }

        // Gen 1 partial trapping cleanup
        // JS: if (this.gen === 1) { ... }
        if self.gen == 1 {
            let partialtrappinglock_id = ID::new("partialtrappinglock");
            let partiallytrapped_id = ID::new("partiallytrapped");
            let fakepartiallytrapped_id = ID::new("fakepartiallytrapped");

            // Collect which volatiles need to be removed (to avoid borrow checker issues)
            let mut volatiles_to_remove: Vec<((usize, usize), ID)> = Vec::new();

            for side_idx in 0..self.sides.len() {
                for active_idx in 0..self.sides[side_idx].active.len() {
                    if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(active_idx) {
                        let pos = (side_idx, *poke_idx);

                        // Check partialtrappinglock
                        if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                            if let Some(lock_state) = pokemon.volatiles.get(&partialtrappinglock_id)
                            {
                                // JS: const target = pokemon.volatiles['partialtrappinglock'].locked;
                                // The locked target is stored in the volatile's data
                                let should_remove =
                                    if let Some(locked_data) = lock_state.data.get("locked") {
                                        // Extract target position (stored as side_idx * 10 + poke_idx)
                                        if let Some(locked_val) = locked_data.as_i64() {
                                            let target_side = (locked_val / 10) as usize;
                                            let target_poke = (locked_val % 10) as usize;

                                            // JS: if (target.hp <= 0 || !target.volatiles['partiallytrapped'])
                                            if let Some(target) = self
                                                .sides
                                                .get(target_side)
                                                .and_then(|s| s.pokemon.get(target_poke))
                                            {
                                                target.hp <= 0
                                                    || !target
                                                        .volatiles
                                                        .contains_key(&partiallytrapped_id)
                                            } else {
                                                true // Target doesn't exist, remove
                                            }
                                        } else {
                                            true // Invalid data, remove
                                        }
                                    } else {
                                        true // No locked data, remove
                                    };

                                if should_remove {
                                    volatiles_to_remove.push((pos, partialtrappinglock_id.clone()));
                                }
                            }

                            // Check partiallytrapped
                            if let Some(trapped_state) = pokemon.volatiles.get(&partiallytrapped_id)
                            {
                                // JS: const source = pokemon.volatiles['partiallytrapped'].source;
                                let should_remove =
                                    if let Some(source_data) = trapped_state.data.get("source") {
                                        // Extract source position
                                        if let Some(source_val) = source_data.as_i64() {
                                            let source_side = (source_val / 10) as usize;
                                            let source_poke = (source_val % 10) as usize;

                                            // JS: if (source.hp <= 0 || !source.volatiles['partialtrappinglock'])
                                            if let Some(source) = self
                                                .sides
                                                .get(source_side)
                                                .and_then(|s| s.pokemon.get(source_poke))
                                            {
                                                source.hp <= 0
                                                    || !source
                                                        .volatiles
                                                        .contains_key(&partialtrappinglock_id)
                                            } else {
                                                true // Source doesn't exist, remove
                                            }
                                        } else {
                                            true // Invalid data, remove
                                        }
                                    } else {
                                        true // No source data, remove
                                    };

                                if should_remove {
                                    volatiles_to_remove.push((pos, partiallytrapped_id.clone()));
                                }
                            }

                            // Check fakepartiallytrapped
                            if let Some(fake_state) =
                                pokemon.volatiles.get(&fakepartiallytrapped_id)
                            {
                                // JS: const counterpart = pokemon.volatiles['fakepartiallytrapped'].counterpart;
                                let should_remove = if let Some(counterpart_data) =
                                    fake_state.data.get("counterpart")
                                {
                                    // Extract counterpart position
                                    if let Some(counterpart_val) = counterpart_data.as_i64() {
                                        let counterpart_side = (counterpart_val / 10) as usize;
                                        let counterpart_poke = (counterpart_val % 10) as usize;

                                        // JS: if (counterpart.hp <= 0 || !counterpart.volatiles['fakepartiallytrapped'])
                                        if let Some(counterpart) = self
                                            .sides
                                            .get(counterpart_side)
                                            .and_then(|s| s.pokemon.get(counterpart_poke))
                                        {
                                            counterpart.hp <= 0
                                                || !counterpart
                                                    .volatiles
                                                    .contains_key(&fakepartiallytrapped_id)
                                        } else {
                                            true // Counterpart doesn't exist, remove
                                        }
                                    } else {
                                        true // Invalid data, remove
                                    }
                                } else {
                                    true // No counterpart data, remove
                                };

                                if should_remove {
                                    volatiles_to_remove
                                        .push((pos, fakepartiallytrapped_id.clone()));
                                }
                            }
                        }
                    }
                }
            }

            // Remove the volatiles
            for ((side_idx, poke_idx), volatile_id) in volatiles_to_remove {
                if let Some(pokemon) = self
                    .sides
                    .get_mut(side_idx)
                    .and_then(|s| s.pokemon.get_mut(poke_idx))
                {
                    pokemon.volatiles.remove(&volatile_id);
                }
            }
        }

        // Collect pokemon positions and move slots for DisableMove events (to avoid borrow checker issues)
        let mut pokemon_positions: Vec<(usize, usize)> = Vec::new();
        let mut disable_move_data: Vec<((usize, usize), ID)> = Vec::new();

        // Collect type change messages to execute after the loop (to avoid borrow checker issues)
        let mut type_change_messages: Vec<(String, String, Option<String>, (usize, usize))> = Vec::new(); // (target_slot, real_type, added_type, seen_pokemon_pos)

        // Collect attackedBy updates to process after the loop (to avoid borrow checker issues)
        let mut attacked_by_updates: Vec<(usize, usize)> = Vec::new(); // (side_idx, poke_idx)

        // Track trapped and staleness status per side
        let mut trapped_by_side: Vec<bool> = Vec::new();
        let mut staleness_by_side: Vec<Option<String>> = Vec::new();

        // Reset Pokemon turn-specific fields
        for side in &mut self.sides {
            let mut side_trapped = true;
            let mut side_staleness: Option<String> = None;

            for pokemon in &mut side.pokemon {
                if !pokemon.is_active {
                    continue;
                }

                // JS: pokemon.moveThisTurn = '';
                pokemon.move_this_turn = None;

                // JS: pokemon.newlySwitched = false;
                pokemon.newly_switched = false;

                // JS: pokemon.moveLastTurnResult = pokemon.moveThisTurnResult;
                // JS: pokemon.moveThisTurnResult = undefined;
                pokemon.move_last_turn_result = pokemon.move_this_turn_result;
                pokemon.move_this_turn_result = None;

                if self.turn != 1 {
                    // JS: pokemon.usedItemThisTurn = false;
                    pokemon.used_item_this_turn = false;

                    // JS: pokemon.statsRaisedThisTurn = false;
                    pokemon.stats_raised_this_turn = false;

                    // JS: pokemon.statsLoweredThisTurn = false;
                    pokemon.stats_lowered_this_turn = false;

                    // JS: pokemon.hurtThisTurn = null;
                    pokemon.hurt_this_turn = None;
                }

                // JS: pokemon.maybeDisabled = false;
                pokemon.maybe_disabled = false;

                // JS: pokemon.maybeLocked = false;
                pokemon.maybe_locked = false;

                // JS: for (const moveSlot of pokemon.moveSlots) { moveSlot.disabled = false; moveSlot.disabledSource = ''; }
                for move_slot in &mut pokemon.move_slots {
                    move_slot.disabled = false;
                    move_slot.disabled_source = None;
                }

                // Collect pokemon position for DisableMove event
                let pokemon_pos = (pokemon.side_index, pokemon.position);
                pokemon_positions.push(pokemon_pos);

                // Collect move slot data for later single_event calls
                for move_slot in &pokemon.move_slots {
                    disable_move_data.push((pokemon_pos, move_slot.id.clone()));
                }

                // JS: if (pokemon.getLastAttackedBy() && this.gen >= 7) pokemon.knownType = true;
                if self.gen >= 7 && pokemon.get_last_attacked_by().is_some() {
                    pokemon.known_type = Some(pokemon.types.join("/"));
                }

                // JS: for (let i = pokemon.attackedBy.length - 1; i >= 0; i--) {
                // JS:     const attack = pokemon.attackedBy[i];
                // JS:     if (attack.source.isActive) {
                // JS:         attack.thisTurn = false;
                // JS:     } else {
                // JS:         pokemon.attackedBy.splice(pokemon.attackedBy.indexOf(attack), 1);
                // JS:     }
                // JS: }
                // Collect for processing after the loop (to avoid borrow checker issues)
                if !pokemon.attacked_by.is_empty() {
                    attacked_by_updates.push(pokemon_pos);
                }

                // JS: if (this.gen >= 7 && !pokemon.terastallized) {
                // Gen 7+ type reveal logic
                if self.gen >= 7 && pokemon.terastallized.is_none() {
                    // JS: const seenPokemon = pokemon.illusion || pokemon;
                    let seen_pokemon_pos = if let Some(illusion_idx) = pokemon.illusion {
                        (pokemon_pos.0, illusion_idx)
                    } else {
                        pokemon_pos
                    };

                    // If seen pokemon is the current pokemon (no illusion), we can check directly
                    if seen_pokemon_pos == pokemon_pos {
                        // Get real type string from current pokemon
                        let real_type_string = pokemon.types.join("/");
                        let apparent_type_changed = pokemon.apparent_type.as_deref() != Some(&real_type_string);

                        if apparent_type_changed && !real_type_string.is_empty() {
                            // Collect for later to avoid borrow checker issues with self.add()
                            let target_arg = pokemon.get_slot();
                            let added_type_opt = pokemon.added_type.clone();
                            type_change_messages.push((target_arg, real_type_string, added_type_opt, seen_pokemon_pos));
                        }
                    } else {
                        // Different pokemon (illusion) - need to collect for later processing
                        // We can't access the illusion target here without conflicting borrows
                        // So we'll collect the data and process it after the loop
                        let target_arg = pokemon.get_slot();
                        let added_type_opt = pokemon.added_type.clone();
                        // Mark with empty string to indicate we need to fetch real_type later
                        type_change_messages.push((target_arg, String::new(), added_type_opt, seen_pokemon_pos));
                    }
                }

                // JS: pokemon.trapped = pokemon.maybeTrapped = false;
                pokemon.trapped = false;
                pokemon.maybe_trapped = false;

                // JS: pokemon.activeTurns++;
                pokemon.active_turns += 1;

                // JS: if (pokemon.fainted) continue;
                if pokemon.fainted {
                    continue;
                }

                // JS: sideTrapped = sideTrapped && pokemon.trapped;
                side_trapped = side_trapped && pokemon.trapped;

                // JS: const staleness = pokemon.volatileStaleness || pokemon.staleness;
                // JS: if (staleness) sideStaleness = sideStaleness === 'external' ? sideStaleness : staleness;
                let staleness = pokemon.volatile_staleness.clone().or(pokemon.staleness.clone());
                if let Some(s) = staleness {
                    side_staleness = if side_staleness.as_deref() == Some("external") {
                        side_staleness
                    } else {
                        Some(s)
                    };
                }
            }

            // JS: trappedBySide.push(sideTrapped);
            // JS: stalenessBySide.push(sideStaleness);
            trapped_by_side.push(side_trapped);
            staleness_by_side.push(side_staleness);

            // JS: side.faintedLastTurn = side.faintedThisTurn;
            // JS: side.faintedThisTurn = null;
            side.fainted_last_turn = side.fainted_this_turn;
            side.fainted_this_turn = None;
        }

        // Process attackedBy arrays (after the mutable borrow of sides ends)
        for (side_idx, poke_idx) in attacked_by_updates {
            // First pass: collect which indices should be removed vs marked (immutable borrows)
            let mut indices_to_remove: Vec<usize> = Vec::new();
            let mut indices_to_mark: Vec<usize> = Vec::new();

            if let Some(pokemon) = self.sides.get(side_idx)
                .and_then(|s| s.pokemon.get(poke_idx))
            {
                for (i, attack) in pokemon.attacked_by.iter().enumerate() {
                    let source_pos = attack.source;

                    // Check if source is still active
                    let source_is_active = if let Some(source_pokemon) = self.sides.get(source_pos.0)
                        .and_then(|s| s.pokemon.get(source_pos.1))
                    {
                        source_pokemon.is_active
                    } else {
                        false
                    };

                    if source_is_active {
                        indices_to_mark.push(i);
                    } else {
                        indices_to_remove.push(i);
                    }
                }
            }

            // Second pass: apply the changes (mutable borrow)
            if let Some(pokemon) = self.sides.get_mut(side_idx)
                .and_then(|s| s.pokemon.get_mut(poke_idx))
            {
                // Mark as not this turn
                for i in indices_to_mark {
                    pokemon.attacked_by[i].this_turn = false;
                }

                // Remove inactive sources (in reverse order to maintain indices)
                for i in indices_to_remove.iter().rev() {
                    pokemon.attacked_by.remove(*i);
                }
            }
        }

        // Process collected type change messages (after the mutable borrow of sides ends)
        for (target_slot, mut real_type, added_type_opt, seen_pokemon_pos) in type_change_messages {
            // If real_type is empty, we need to fetch it from the seen_pokemon (illusion case)
            if real_type.is_empty() {
                if let Some(seen_pokemon) = self.sides.get(seen_pokemon_pos.0)
                    .and_then(|s| s.pokemon.get(seen_pokemon_pos.1))
                {
                    real_type = seen_pokemon.types.join("/");

                    // Check if type changed
                    let apparent_type_changed = seen_pokemon.apparent_type.as_deref() != Some(&real_type);
                    if !apparent_type_changed || real_type.is_empty() {
                        continue; // Skip if no change or empty type
                    }
                } else {
                    continue; // Skip if pokemon not found
                }
            }

            // JS: this.add('-start', pokemon, 'typechange', realTypeString, '[silent]');
            self.add("-start", &[
                target_slot.as_str().into(),
                "typechange".into(),
                real_type.clone().into(),
                "[silent]".into()
            ]);

            // JS: seenPokemon.apparentType = realTypeString;
            if let Some(seen_pokemon_mut) = self.sides.get_mut(seen_pokemon_pos.0)
                .and_then(|s| s.pokemon.get_mut(seen_pokemon_pos.1))
            {
                seen_pokemon_mut.apparent_type = Some(real_type);
            }

            // JS: if (pokemon.addedType) { this.add('-start', pokemon, 'typeadd', pokemon.addedType, '[silent]'); }
            if let Some(added_type) = added_type_opt {
                self.add("-start", &[
                    target_slot.as_str().into(),
                    "typeadd".into(),
                    added_type.as_str().into(),
                    "[silent]".into()
                ]);
            }
        }

        // Call runEvent('DisableMove') for each active pokemon (after the mutable borrow ends)
        // JS: this.runEvent('DisableMove', pokemon);
        // This allows abilities like Assault Vest, Gorilla Tactics, etc. to disable moves
        for pokemon_pos in &pokemon_positions {
            self.run_event("DisableMove", Some(*pokemon_pos), None, None, None);
        }

        // Call singleEvent('DisableMove') for each move (allows move-specific disable logic)
        // JS: for (const moveSlot of pokemon.moveSlots) { this.singleEvent('DisableMove', activeMove, null, pokemon); }
        for (pokemon_pos, move_id) in disable_move_data {
            self.single_event("DisableMove", &move_id, Some(pokemon_pos), None, None);

            // JS: if (activeMove.flags['cantusetwice'] && pokemon.lastMove?.id === moveSlot.id) {
            // JS:     pokemon.disableMove(pokemon.lastMove.id);
            // JS: }
            // Check if move has cantusetwice flag and was last move used
            if let Some(move_def) = self.dex.moves().get_by_id(&move_id) {
                if move_def.flags.contains_key("cantusetwice") {
                    // Check if this was the last move used
                    if let Some(pokemon) = self.sides.get(pokemon_pos.0)
                        .and_then(|s| s.pokemon.get(pokemon_pos.1))
                    {
                        if let Some(ref last_move) = pokemon.last_move {
                            if last_move == &move_id {
                                // Disable this move
                                if let Some(pokemon_mut) = self.sides.get_mut(pokemon_pos.0)
                                    .and_then(|s| s.pokemon.get_mut(pokemon_pos.1))
                                {
                                    // Find and disable the move slot
                                    for slot in &mut pokemon_mut.move_slots {
                                        if slot.id == move_id {
                                            slot.disabled = true;
                                            slot.disabled_source = Some("cantusetwice".to_string());
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Call TrapPokemon and MaybeTrapPokemon events for each active pokemon
        // JS: this.runEvent('TrapPokemon', pokemon);
        // JS: if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon)) { this.runEvent('MaybeTrapPokemon', pokemon); }
        for &pokemon_pos in &pokemon_positions {
            // TrapPokemon event - allows moves/abilities to trap pokemon (e.g., Mean Look)
            self.run_event("TrapPokemon", Some(pokemon_pos), None, None, None);

            // MaybeTrapPokemon event - conditional trapping based on type immunity
            // JS: if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon))
            let should_run_maybe_trap = {
                let pokemon = &self.sides[pokemon_pos.0].pokemon[pokemon_pos.1];
                // Run if type is not known OR if not immune to trapped status
                pokemon.known_type.is_none() || pokemon.run_status_immunity(self, "trapped")
            };

            if should_run_maybe_trap {
                self.run_event("MaybeTrapPokemon", Some(pokemon_pos), None, None, None);
            }
        }

        // Check for foe abilities that might trap pokemon (Gen 3+)
        // JS: for (const source of pokemon.foes()) { ... check species.abilities ... }
        if self.gen >= 3 {
            for &(side_idx, poke_idx) in &pokemon_positions {
                // Get adjacent foes for this pokemon
                let foes = if let Some(pokemon) = self.sides.get(side_idx)
                    .and_then(|s| s.pokemon.get(poke_idx))
                {
                    pokemon.adjacent_foes(self)
                } else {
                    Vec::new()
                };

                for (_foe_side_idx, _foe_idx) in foes {
                    // TODO: Full implementation requires:
                    // 1. Check species.abilities for all ability slots
                    // 2. Check ruleTable for +hackmons and obtainableabilities
                    // 3. Check for unreleased hidden abilities
                    // 4. Call singleEvent('FoeMaybeTrapPokemon', ability, {}, pokemon, source)
                    // For now, this is a stub to document the requirement

                    // The simplified version just notes that foe abilities could affect trapping
                    // Full implementation would iterate through all possible abilities of the foe's species
                }
            }
        }

        // JS: if (this.maybeTriggerEndlessBattleClause(trappedBySide, stalenessBySide)) return;
        if self.maybe_trigger_endless_battle_clause(&trapped_by_side, &staleness_by_side) {
            return;
        }

        // JS: if (this.gameType === 'triples' && this.sides.every(side => side.pokemonLeft === 1)) {
        // Triples center logic
        if self.game_type == GameType::Triples && self.sides.iter().all(|s| s.pokemon_left == 1) {
            // JS: const actives = this.getAllActive();
            let actives = self.get_all_active(false);

            // JS: if (actives.length > 1 && !actives[0].isAdjacent(actives[1])) {
            if actives.len() > 1 {
                let active0_pos = actives[0];
                let active1_pos = actives[1];

                // Check if not adjacent using Battle::is_adjacent (fully implemented)
                let is_adjacent = self.is_adjacent(active0_pos, active1_pos);

                if !is_adjacent {
                    // JS: this.swapPosition(actives[0], 1, '[silent]');
                    // JS: this.swapPosition(actives[1], 1, '[silent]');
                    // TODO: Implement swapPosition method
                    // For now, this is a stub

                    // JS: this.add('-center');
                    self.add("-center", &[]);
                }
            }
        }

        self.add("turn", &[self.turn.to_string().into()]);

        // JS: if (this.gameType === 'multi') {
        // JS:     for (const side of this.sides) {
        // JS:         if (side.canDynamaxNow()) {
        // JS:             if (this.turn === 1) {
        // JS:                 this.addSplit(side.id, ['-candynamax', side.id]);
        // JS:             } else {
        // JS:                 this.add('-candynamax', side.id);
        // JS:             }
        // JS:         }
        // JS:     }
        // JS: }
        if self.game_type == GameType::Multi {
            for _side in &self.sides {
                // Check if side can dynamax now (would need canDynamaxNow() method)
                // For now, this is a stub as canDynamaxNow() is not yet implemented
                // TODO: Implement canDynamaxNow() method on Side

                // If we had the method:
                // if side.can_dynamax_now() {
                //     if self.turn == 1 {
                //         self.add_split(&side.id, &["-candynamax".into(), side.id.to_str().into()]);
                //     } else {
                //         self.add("-candynamax", &[side.id.to_str().into()]);
                //     }
                // }
            }
        }

        // JS: if (this.gen === 2) this.quickClawRoll = this.randomChance(60, 256);
        if self.gen == 2 {
            self.quick_claw_roll = Some(self.random_chance(60, 256));
        }

        // JS: if (this.gen === 3) this.quickClawRoll = this.randomChance(1, 5);
        if self.gen == 3 {
            self.quick_claw_roll = Some(self.random_chance(1, 5));
        }

        // JS: this.makeRequest('move');
        self.make_request(Some(BattleRequestState::Move));
    }
}
