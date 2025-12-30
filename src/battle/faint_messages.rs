use crate::*;
use crate::battle::FaintData;

impl Battle {

    /// Process faint messages
    /// Equivalent to battle.ts faintMessages(lastFirst?, forceCheck?, checkWin?)
    ///
    //
    // 	faintMessages(lastFirst = false, forceCheck = false, checkWin = true) {
    // 		if (this.ended) return;
    // 		const length = this.faintQueue.length;
    // 		if (!length) {
    // 			if (forceCheck && this.checkWin()) return true;
    // 			return false;
    // 		}
    // 		if (lastFirst) {
    // 			this.faintQueue.unshift(this.faintQueue[this.faintQueue.length - 1]);
    // 			this.faintQueue.pop();
    // 		}
    // 		let faintQueueLeft, faintData;
    // 		while (this.faintQueue.length) {
    // 			faintQueueLeft = this.faintQueue.length;
    // 			faintData = this.faintQueue.shift()!;
    // 			const pokemon: Pokemon = faintData.target;
    // 			if (!pokemon.fainted && this.runEvent('BeforeFaint', pokemon, faintData.source, faintData.effect)) {
    // 				this.add('faint', pokemon);
    // 				if (pokemon.side.pokemonLeft) pokemon.side.pokemonLeft--;
    // 				if (pokemon.side.totalFainted < 100) pokemon.side.totalFainted++;
    // 				this.runEvent('Faint', pokemon, faintData.source, faintData.effect);
    // 				this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon);
    // 				this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
    // 				if (pokemon.formeRegression && !pokemon.transformed) {
    // 					// before clearing volatiles
    // 					pokemon.baseSpecies = this.dex.species.get(pokemon.set.species || pokemon.set.name);
    // 					pokemon.baseAbility = toID(pokemon.set.ability);
    // 				}
    // 				pokemon.clearVolatile(false);
    // 				pokemon.fainted = true;
    // 				pokemon.illusion = null;
    // 				pokemon.isActive = false;
    // 				pokemon.isStarted = false;
    // 				delete pokemon.terastallized;
    // 				if (pokemon.formeRegression) {
    // 					// after clearing volatiles
    // 					pokemon.details = pokemon.getUpdatedDetails();
    // 					this.add('detailschange', pokemon, pokemon.details, '[silent]');
    // 					pokemon.updateMaxHp();
    // 					pokemon.formeRegression = false;
    // 				}
    // 				pokemon.side.faintedThisTurn = pokemon;
    // 				if (this.faintQueue.length >= faintQueueLeft) checkWin = true;
    // 			}
    // 		}
    //
    // 		if (this.gen <= 1) {
    // 			// in gen 1, fainting skips the rest of the turn
    // 			// residuals don't exist in gen 1
    // 			this.queue.clear();
    // 			// Fainting clears accumulated Bide damage
    // 			for (const pokemon of this.getAllActive()) {
    // 				if (pokemon.volatiles['bide']?.damage) {
    // 					pokemon.volatiles['bide'].damage = 0;
    // 					this.hint("Desync Clause Mod activated!");
    // 					this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
    // 				}
    // 			}
    // 		} else if (this.gen <= 3 && this.gameType === 'singles') {
    // 			// in gen 3 or earlier, fainting in singles skips to residuals
    // 			for (const pokemon of this.getAllActive()) {
    // 				if (this.gen <= 2) {
    // 					// in gen 2, fainting skips moves only
    // 					this.queue.cancelMove(pokemon);
    // 				} else {
    // 					// in gen 3, fainting skips all moves and switches
    // 					this.queue.cancelAction(pokemon);
    // 				}
    // 			}
    // 		}
    //
    // 		if (checkWin && this.checkWin(faintData)) return true;
    //
    // 		if (faintData && length) {
    // 			this.runEvent('AfterFaint', faintData.target, faintData.source, faintData.effect, length);
    // 		}
    // 		return false;
    // 	}
    //
    pub fn faint_messages(&mut self, last_first: bool, force_check: bool, mut check_win: bool) -> bool {
        // JS: if (this.ended) return;
        if self.ended {
            return false;
        }

        // For compatibility: Add any Pokemon with HP=0 to faint_queue if they're not already there
        // This handles cases where damage was applied without calling Battle.faint()
        for side_idx in 0..self.sides.len() {
            for poke_idx in 0..self.sides[side_idx].pokemon.len() {
                if self.sides[side_idx].pokemon[poke_idx].hp == 0
                    && !self.sides[side_idx].pokemon[poke_idx].fainted
                {
                    // Check if already in queue
                    let already_queued = self
                        .faint_queue
                        .iter()
                        .any(|fd| fd.target == (side_idx, poke_idx));
                    if !already_queued {
                        self.faint_queue.push(FaintData {
                            target: (side_idx, poke_idx),
                            source: None,
                            effect: None,
                        });
                    }
                }
            }
        }

        // JS: const length = this.faintQueue.length;
        let length = self.faint_queue.len();

        // JS: if (!length) { if (forceCheck && this.checkWin()) return true; return false; }
        if length == 0 {
            if force_check && self.check_win(None) {
                return true;
            }
            return false;
        }

        // JS: if (lastFirst) { this.faintQueue.unshift(this.faintQueue[this.faintQueue.length - 1]); this.faintQueue.pop(); }
        if last_first && !self.faint_queue.is_empty() {
            let last = self.faint_queue.pop().unwrap();
            self.faint_queue.insert(0, last);
        }

        let mut last_faint_data: Option<FaintData> = None;

        // JS: while (this.faintQueue.length)
        while !self.faint_queue.is_empty() {
            let faint_queue_left = self.faint_queue.len();
            let faint_data = self.faint_queue.remove(0); // JS: faintData = this.faintQueue.shift()!;
            let (side_idx, poke_idx) = faint_data.target;

            // Check if pokemon is already fainted
            let already_fainted = self.sides[side_idx].pokemon[poke_idx].fainted;

            // JS: if (!pokemon.fainted && this.runEvent('BeforeFaint', pokemon, faintData.source, faintData.effect))
            if !already_fainted {
                // Run BeforeFaint event - can be cancelled by returning false
                let before_faint_result = self.run_event_bool(
                    "BeforeFaint",
                    Some((side_idx, poke_idx)),
                    faint_data.source,
                    faint_data.effect.as_ref(),
                );

                if !before_faint_result {
                    // BeforeFaint was cancelled, skip this faint
                    continue;
                }

                // JS: this.add('faint', pokemon);
                let pokemon_name = {
                    let side_id = self.sides[side_idx].id_str();
                    let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add("faint", &[pokemon_name.into()]);

                // JS: if (pokemon.side.pokemonLeft) pokemon.side.pokemonLeft--;
                if self.sides[side_idx].pokemon_left > 0 {
                    self.sides[side_idx].pokemon_left -= 1;
                }

                // JS: if (pokemon.side.totalFainted < 100) pokemon.side.totalFainted++;
                if self.sides[side_idx].total_fainted < 100 {
                    self.sides[side_idx].total_fainted += 1;
                }

                // JS: this.runEvent('Faint', pokemon, faintData.source, faintData.effect);
                self.run_event(
                    "Faint",
                    Some((side_idx, poke_idx)),
                    faint_data.source,
                    faint_data.effect.as_ref(),
                    None, // relay_var
                );

                // JS: this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon);
                // JS: this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
                // Get ability and item IDs before they're cleared
                let ability_id = self.sides[side_idx].pokemon[poke_idx].ability.clone();
                let item_id = self.sides[side_idx].pokemon[poke_idx].item.clone();

                // Call End event for ability
                if !ability_id.is_empty() {
                    self.single_event("End", &ability_id, Some((side_idx, poke_idx)), None, None);
                }

                // Call End event for item
                if !item_id.is_empty() {
                    self.single_event("End", &item_id, Some((side_idx, poke_idx)), None, None);
                }

                // JS: pokemon.clearVolatile(false);
                // Use unsafe pointer split to allow pokemon to access battle for set_species call
                let (pokemon_ptr, battle_ptr) = unsafe {
                    let pokemon = &mut self.sides[side_idx].pokemon[poke_idx] as *mut Pokemon;
                    let battle = self as *mut Battle;
                    (pokemon, battle)
                };
                unsafe {
                    (*pokemon_ptr).clear_volatile_full(&mut *battle_ptr, false);
                }

                // JS: pokemon.fainted = true;
                self.sides[side_idx].pokemon[poke_idx].fainted = true;

                // JS: pokemon.illusion = null;
                self.sides[side_idx].pokemon[poke_idx].illusion = None;

                // JS: pokemon.isActive = false;
                self.sides[side_idx].pokemon[poke_idx].is_active = false;

                // Note: In Pokemon Showdown, fainted Pokemon remain in the active array
                // until they are explicitly replaced by a switch. They are not removed immediately.
                // This allows the battle to track which slots need replacement.
                // The old code removed them here, but that's incorrect:
                // // Remove from active slots
                // for slot in 0..self.sides[side_idx].active.len() {
                //     if let Some(idx) = self.sides[side_idx].active[slot] {
                //         if idx == poke_idx {
                //             self.sides[side_idx].active[slot] = None;
                //             break;
                //         }
                //     }
                // }

                // JS: pokemon.isStarted = false;
                self.sides[side_idx].pokemon[poke_idx].is_started = false;

                // JS: delete pokemon.terastallized;
                self.sides[side_idx].pokemon[poke_idx].terastallized = None;

                // JS: if (this.faintQueue.length >= faintQueueLeft) checkWin = true;
                if self.faint_queue.len() >= faint_queue_left {
                    check_win = true;
                }
            }

            last_faint_data = Some(faint_data);
        }

        // JS: if (this.gen <= 1) { this.queue.clear(); ... }
        if self.gen <= 1 {
            // Gen 1: fainting skips the rest of the turn
            // JS: this.queue.clear();
            self.queue.clear();

            // JS: Fainting clears accumulated Bide damage
            // JS: for (const pokemon of this.getAllActive()) {
            // JS:     if (pokemon.volatiles['bide']?.damage) {
            // JS:         pokemon.volatiles['bide'].damage = 0;
            // JS:         this.hint("Desync Clause Mod activated!");
            // JS:         this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
            // JS:     }
            // JS: }
            let mut bide_cleared = false;
            let bide_id = ID::new("bide");

            for side in &mut self.sides {
                for pokemon in &mut side.pokemon {
                    if !pokemon.is_active {
                        continue;
                    }

                    // Check if pokemon has bide volatile with damage > 0
                    if let Some(bide_state) = pokemon.volatiles.get_mut(&bide_id) {
                        // Check if the bide state has a damage field
                        if let Some(damage_value) = bide_state.data.get("damage") {
                            if let Some(damage) = damage_value.as_i64() {
                                if damage > 0 {
                                    // Reset damage to 0
                                    bide_state
                                        .data
                                        .insert("damage".to_string(), serde_json::json!(0));
                                    bide_cleared = true;
                                }
                            }
                        }
                    }
                }
            }

            if bide_cleared {
                self.hint("Desync Clause Mod activated!", false, None);
                self.hint(
                    "In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.",
                    false,
                    None,
                );
            }
        }
        // JS: else if (this.gen <= 3 && this.gameType === 'singles') { ... }
        else if self.gen <= 3 && self.game_type == GameType::Singles {
            // in gen 3 or earlier, fainting in singles skips to residuals
            // JS: for (const pokemon of this.getAllActive()) { ... }

            // Collect active pokemon positions to avoid borrow checker issues
            let active_positions: Vec<(usize, usize)> = self
                .sides
                .iter()
                .enumerate()
                .flat_map(|(side_idx, side)| {
                    side.active
                        .iter()
                        .filter_map(move |&opt_idx| opt_idx.map(|poke_idx| (side_idx, poke_idx)))
                })
                .collect();

            for (side_idx, pokemon_idx) in active_positions {
                if self.gen <= 2 {
                    // in gen 2, fainting skips moves only
                    // JS: this.queue.cancelMove(pokemon);
                    self.queue.cancel_move(side_idx, pokemon_idx);
                } else {
                    // in gen 3, fainting skips all moves and switches
                    // JS: this.queue.cancelAction(pokemon);
                    self.queue.cancel_action(side_idx, pokemon_idx);
                }
            }
        }

        // JS: if (checkWin && this.checkWin(faintData)) return true;
        if check_win && self.check_win(last_faint_data.clone()) {
            return true;
        }

        // JS: if (faintData && length) { this.runEvent('AfterFaint', faintData.target, faintData.source, faintData.effect, length); }
        if let Some(ref faint_data) = last_faint_data {
            if length > 0 {
                self.run_event(
                    "AfterFaint",
                    Some(faint_data.target),
                    faint_data.source,
                    faint_data.effect.as_ref(),
                    None, // relay_var
                );
            }
        }

        // JS: return false;
        false
    }
}
