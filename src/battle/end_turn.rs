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
            // Update speed for all Pokemon ending Dynamax
            for &(side_idx, poke_idx) in &dynamax_ending {
                if let Some(pokemon) = self
                    .sides
                    .get_mut(side_idx)
                    .and_then(|s| s.pokemon.get_mut(poke_idx))
                {
                    pokemon.update_speed();
                }
            }

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

        // Reset Pokemon turn-specific fields
        for side in &mut self.sides {
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

                // JS: for (const moveSlot of pokemon.moveSlots) { moveSlot.disabled = false; }
                // Reset all move slots to not disabled (this happens in Pokemon struct reset)

                // Collect pokemon position for DisableMove event
                let pokemon_pos = (pokemon.side_index, pokemon.position);
                pokemon_positions.push(pokemon_pos);

                // Collect move slot data for later single_event calls
                for move_slot in &pokemon.move_slots {
                    disable_move_data.push((pokemon_pos, move_slot.id.clone()));
                }

                // JS: pokemon.trapped = pokemon.maybeTrapped = false;
                pokemon.trapped = false;
                pokemon.maybe_trapped = false;
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
                pokemon.known_type.is_none() || pokemon.run_status_immunity("trapped")
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

        self.add("", &[]);
        self.add("turn", &[self.turn.to_string().into()]);

        // JS: this.makeRequest('move');
        eprintln!("DEBUG [end_turn]: Calling make_request(Move)");
        self.make_request(Some(BattleRequestState::Move));
    }
}
