// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Check and trigger Endless Battle Clause
    /// Equivalent to battle.ts maybeTriggerEndlessBattleClause() (battle.ts:1757-1856)
    ///
    //
    // 	maybeTriggerEndlessBattleClause(
    // 		trappedBySide: boolean[], stalenessBySide: ('internal' | 'external' | undefined)[]
    // 	) {
    // 		// Gen 1 Endless Battle Clause triggers
    // 		// These are checked before the 100 turn minimum as the battle cannot progress if they are true
    // 		if (this.gen <= 1) {
    // 			const noProgressPossible = this.sides.every(side => {
    // 				const foeAllGhosts = side.foe.pokemon.every(pokemon => pokemon.fainted || pokemon.hasType('Ghost'));
    // 				const foeAllTransform = side.foe.pokemon.every(pokemon => (
    // 					pokemon.fainted ||
    // 					// true if transforming into this pokemon would lead to an endless battle
    // 					// Transform will fail (depleting PP) if used against Ditto in Stadium 1
    // 					(this.dex.currentMod !== 'gen1stadium' || pokemon.species.id !== 'ditto') &&
    // 					// there are some subtleties such as a Mew with only Transform and auto-fail moves,
    // 					// but it's unlikely to come up in a real game so there's no need to handle it
    // 					pokemon.moves.every(moveid => moveid === 'transform')
    // 				));
    // 				return side.pokemon.every(pokemon => (
    // 					pokemon.fainted ||
    // 					// frozen pokemon can't thaw in gen 1 without outside help
    // 					pokemon.status === 'frz' ||
    // 					// a pokemon can't lose PP if it Transforms into a pokemon with only Transform
    // 					(pokemon.moves.every(moveid => moveid === 'transform') && foeAllTransform) ||
    // 					// Struggle can't damage yourself if every foe is a Ghost
    // 					(pokemon.moveSlots.every(slot => slot.pp === 0) && foeAllGhosts)
    // 				));
    // 			});
    // 			if (noProgressPossible) {
    // 				this.add('-message', `This battle cannot progress. Endless Battle Clause activated!`);
    // 				return this.tie();
    // 			}
    // 		}
    //
    // 		if (this.turn <= 100) return;
    //
    // 		// the turn limit is not a part of Endless Battle Clause
    // 		if (this.turn >= 1000) {
    // 			this.add('message', `It is turn 1000. You have hit the turn limit!`);
    // 			this.tie();
    // 			return true;
    // 		}
    // 		if (
    // 			(this.turn >= 500 && this.turn % 100 === 0) || // every 100 turns past turn 500,
    // 			(this.turn >= 900 && this.turn % 10 === 0) || // every 10 turns past turn 900,
    // 			this.turn >= 990 // every turn past turn 990
    // 		) {
    // 			const turnsLeft = 1000 - this.turn;
    // 			const turnsLeftText = (turnsLeft === 1 ? `1 turn` : `${turnsLeft} turns`);
    // 			this.add('bigerror', `You will auto-tie if the battle doesn't end in ${turnsLeftText} (on turn 1000).`);
    // 		}
    //
    // 		if (!this.ruleTable.has('endlessbattleclause')) return;
    // 		// for now, FFA doesn't support Endless Battle Clause
    // 		if (this.format.gameType === 'freeforall') return;
    //
    // 		// Are all Pokemon on every side stale, with at least one side containing an externally stale Pokemon?
    // 		if (!stalenessBySide.every(s => !!s) || !stalenessBySide.some(s => s === 'external')) return;
    //
    // 		// Can both sides switch to a non-stale Pokemon?
    // 		const canSwitch = [];
    // 		for (const [i, trapped] of trappedBySide.entries()) {
    // 			canSwitch[i] = false;
    // 			if (trapped) break;
    // 			const side = this.sides[i];
    //
    // 			for (const pokemon of side.pokemon) {
    // 				if (!pokemon.fainted && !(pokemon.volatileStaleness || pokemon.staleness)) {
    // 					canSwitch[i] = true;
    // 					break;
    // 				}
    // 			}
    // 		}
    // 		if (canSwitch.every(s => s)) return;
    //
    // 		// Endless Battle Clause activates - we determine the winner by looking at each side's sets.
    // 		const losers: Side[] = [];
    // 		for (const side of this.sides) {
    // 			let berry = false; // Restorative Berry
    // 			let cycle = false; // Harvest or Recycle
    // 			for (const pokemon of side.pokemon) {
    // 				berry = RESTORATIVE_BERRIES.has(toID(pokemon.set.item));
    // 				if (['harvest', 'pickup'].includes(toID(pokemon.set.ability)) ||
    // 					pokemon.set.moves.map(toID).includes('recycle' as ID)) {
    // 					cycle = true;
    // 				}
    // 				if (berry && cycle) break;
    // 			}
    // 			if (berry && cycle) losers.push(side);
    // 		}
    //
    // 		if (losers.length === 1) {
    // 			const loser = losers[0];
    // 			this.add('-message', `${loser.name}'s team started with the rudimentary means to perform restorative berry-cycling and thus loses.`);
    // 			return this.win(loser.foe);
    // 		}
    // 		if (losers.length === this.sides.length) {
    // 			this.add('-message', `Each side's team started with the rudimentary means to perform restorative berry-cycling.`);
    // 		}
    //
    // 		return this.tie();
    // 	}
    //
    pub fn maybe_trigger_endless_battle_clause(
        &mut self,
        trapped_by_side: &[bool],
        staleness_by_side: &[Option<String>],
    ) -> bool {
        // JavaScript: Gen 1 no-progress checks (lines 16-42)
        if self.gen <= 1 {
            // Collect all pokemon data to avoid borrow checker issues
            let mut pokemon_data: Vec<Vec<(bool, Vec<String>, bool, Vec<String>, Vec<u8>, ID)>> = Vec::new();
            let is_stadium = self.dex.current_mod.as_deref() == Some("gen1stadium");

            for side in &self.sides {
                let mut side_data = Vec::new();
                for p in &side.pokemon {
                    let types = p.get_types(self, false);
                    let moves = p.set.moves.clone();
                    let move_slot_pps: Vec<u8> = p.move_slots.iter().map(|slot| slot.pp).collect();
                    let species_id = p.species_id.clone();
                    side_data.push((
                        p.fainted,
                        types,
                        p.status == ID::from("frz"),
                        moves,
                        move_slot_pps,
                        species_id,
                    ));
                }
                pokemon_data.push(side_data);
            }

            // Now do the checks
            let no_progress_possible = (0..pokemon_data.len()).all(|i| {
                let foe_index = if i == 0 { 1 } else { 0 };

                // Check if all foes are Ghosts
                let foe_all_ghosts = pokemon_data[foe_index].iter().all(|(fainted, types, _, _, _, _)| {
                    *fainted || types.contains(&"Ghost".to_string())
                });

                // Check if all foes have only Transform
                let foe_all_transform = pokemon_data[foe_index].iter().all(|(fainted, _, _, moves, _, species_id)| {
                    if *fainted {
                        return true;
                    }
                    // Transform will fail (depleting PP) if used against Ditto in Stadium 1
                    if !is_stadium || species_id.as_str() != "ditto" {
                        return moves.iter().all(|m| m.as_str() == "transform");
                    }
                    false
                });

                // Check if all pokemon on this side are in an endless state
                pokemon_data[i].iter().all(|(fainted, _, is_frozen, moves, move_pps, _)| {
                    if *fainted {
                        return true;
                    }
                    // Frozen pokemon can't thaw in gen 1 without outside help
                    if *is_frozen {
                        return true;
                    }
                    // A pokemon can't lose PP if it Transforms into a pokemon with only Transform
                    if moves.iter().all(|m| m.as_str() == "transform") && foe_all_transform {
                        return true;
                    }
                    // Struggle can't damage yourself if every foe is a Ghost
                    if move_pps.iter().all(|&pp| pp == 0) && foe_all_ghosts {
                        return true;
                    }
                    false
                })
            });

            if no_progress_possible {
                self.add("message", &["This battle cannot progress. Endless Battle Clause activated!".into()]);
                return self.tie();
            }
        }

        // JS: if (this.turn <= 100) return;
        if self.turn <= 100 {
            return false;
        }

        // JS: if (this.turn >= 1000) { this.add('message', ...); this.tie(); return true; }
        if self.turn >= 1000 {
            self.add(
                "message",
                &["It is turn 1000. You have hit the turn limit!".into()],
            );
            self.tie();
            return true;
        }

        // JS: Turn limit warnings
        // if ((turn >= 500 && turn % 100 === 0) || (turn >= 900 && turn % 10 === 0) || turn >= 990)
        if (self.turn >= 500 && self.turn % 100 == 0)
            || (self.turn >= 900 && self.turn % 10 == 0)
            || self.turn >= 990
        {
            let turns_left = 1000 - self.turn;
            let turns_text = if turns_left == 1 {
                "1 turn".to_string()
            } else {
                format!("{} turns", turns_left)
            };
            self.add(
                "bigerror",
                &[format!(
                    "You will auto-tie if the battle doesn't end in {} (on turn 1000).",
                    turns_text
                ).into()],
            );
        }

        // TODO: Gen 1 no-progress checks (requires Pokemon.hasType())
        // TODO: Staleness checks (requires Pokemon.volatileStaleness, Pokemon.staleness)
        // TODO: Berry cycling checks (requires Harvest, Recycle, RESTORATIVE_BERRIES)

        false
    }
}
