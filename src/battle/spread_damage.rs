// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Spread damage to multiple targets
    /// Matches JavaScript battle.ts:2045-2164 spreadDamage()
    //
    // 	spreadDamage(
    // 		damage: SpreadMoveDamage, targetArray: (false | Pokemon | null)[] | null = null,
    // 		source: Pokemon | null = null, effect: 'drain' | 'recoil' | Effect | null = null, instafaint = false
    // 	) {
    // 		if (!targetArray) return [0];
    // 		const retVals: (number | false | undefined)[] = [];
    // 		if (typeof effect === 'string' || !effect) effect = this.dex.conditions.getByID((effect || '') as ID);
    // 		for (const [i, curDamage] of damage.entries()) {
    // 			const target = targetArray[i];
    // 			let targetDamage = curDamage;
    // 			if (!(targetDamage || targetDamage === 0)) {
    // 				retVals[i] = targetDamage;
    // 				continue;
    // 			}
    // 			if (!target || !target.hp) {
    // 				retVals[i] = 0;
    // 				continue;
    // 			}
    // 			if (!target.isActive) {
    // 				retVals[i] = false;
    // 				continue;
    // 			}
    // 			if (targetDamage !== 0) targetDamage = this.clampIntRange(targetDamage, 1);
    //
    // 			if (effect.id !== 'struggle-recoil') { // Struggle recoil is not affected by effects
    // 				if (effect.effectType === 'Weather' && !target.runStatusImmunity(effect.id)) {
    // 					this.debug('weather immunity');
    // 					retVals[i] = 0;
    // 					continue;
    // 				}
    // 				targetDamage = this.runEvent('Damage', target, source, effect, targetDamage, true);
    // 				if (!(targetDamage || targetDamage === 0)) {
    // 					this.debug('damage event failed');
    // 					retVals[i] = curDamage === true ? undefined : targetDamage;
    // 					continue;
    // 				}
    // 			}
    // 			if (targetDamage !== 0) targetDamage = this.clampIntRange(targetDamage, 1);
    //
    // 			if (this.gen <= 1) {
    // 				if (this.dex.currentMod === 'gen1stadium' ||
    // 					!['recoil', 'drain', 'leechseed'].includes(effect.id) && effect.effectType !== 'Status') {
    // 					this.lastDamage = targetDamage;
    // 				}
    // 			}
    //
    // 			retVals[i] = targetDamage = target.damage(targetDamage, source, effect);
    // 			if (targetDamage !== 0) target.hurtThisTurn = target.hp;
    // 			if (source && effect.effectType === 'Move') source.lastDamage = targetDamage;
    //
    // 			const name = effect.fullname === 'tox' ? 'psn' : effect.fullname;
    // 			switch (effect.id) {
    // 			case 'partiallytrapped':
    // 				this.add('-damage', target, target.getHealth, '[from] ' + target.volatiles['partiallytrapped'].sourceEffect.fullname, '[partiallytrapped]');
    // 				break;
    // 			case 'powder':
    // 				this.add('-damage', target, target.getHealth, '[silent]');
    // 				break;
    // 			case 'confused':
    // 				this.add('-damage', target, target.getHealth, '[from] confusion');
    // 				break;
    // 			default:
    // 				if (effect.effectType === 'Move' || !name) {
    // 					this.add('-damage', target, target.getHealth);
    // 				} else if (source && (source !== target || effect.effectType === 'Ability')) {
    // 					this.add('-damage', target, target.getHealth, `[from] ${name}`, `[of] ${source}`);
    // 				} else {
    // 					this.add('-damage', target, target.getHealth, `[from] ${name}`);
    // 				}
    // 				break;
    // 			}
    //
    // 			if (targetDamage && effect.effectType === 'Move') {
    // 				if (this.gen <= 1 && effect.recoil && source) {
    // 					if (this.dex.currentMod !== 'gen1stadium' || target.hp > 0) {
    // 						const amount = this.clampIntRange(Math.floor(targetDamage * effect.recoil[0] / effect.recoil[1]), 1);
    // 						this.damage(amount, source, target, 'recoil');
    // 					}
    // 				}
    // 				if (this.gen <= 4 && effect.drain && source) {
    // 					const amount = this.clampIntRange(Math.floor(targetDamage * effect.drain[0] / effect.drain[1]), 1);
    // 					// Draining can be countered in gen 1
    // 					if (this.gen <= 1) this.lastDamage = amount;
    // 					this.heal(amount, source, target, 'drain');
    // 				}
    // 				if (this.gen > 4 && effect.drain && source) {
    // 					const amount = Math.round(targetDamage * effect.drain[0] / effect.drain[1]);
    // 					this.heal(amount, source, target, 'drain');
    // 				}
    // 			}
    // 		}
    //
    // 		if (instafaint) {
    // 			for (const [i, target] of targetArray.entries()) {
    // 				if (!retVals[i] || !target) continue;
    //
    // 				if (target.hp <= 0) {
    // 					this.debug(`instafaint: ${this.faintQueue.map(entry => entry.target.name)}`);
    // 					this.faintMessages(true);
    // 					if (this.gen <= 2) {
    // 						target.faint();
    // 						if (this.gen <= 1) {
    // 							this.queue.clear();
    // 							// Fainting clears accumulated Bide damage
    // 							for (const pokemon of this.getAllActive()) {
    // 								if (pokemon.volatiles['bide']?.damage) {
    // 									pokemon.volatiles['bide'].damage = 0;
    // 									this.hint("Desync Clause Mod activated!");
    // 									this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
    // 								}
    // 							}
    // 						}
    // 					}
    // 				}
    // 			}
    // 		}
    //
    // 		return retVals;
    // 	}
    //
    pub fn spread_damage(
        &mut self,
        damages: &[Option<i32>], // Can be true (max damage), false, number, or undefined
        targets: &[Option<(usize, usize)>],
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
        instafaint: bool,
    ) -> Vec<Option<i32>> {
        let mut ret_vals: Vec<Option<i32>> = Vec::new();

        // Process damage for each target
        for i in 0..damages.len() {
            let cur_damage = damages.get(i).copied().flatten();
            let target = targets.get(i).copied().flatten();

            // Handle undefined/null damage
            if cur_damage.is_none() {
                ret_vals.push(None);
                continue;
            }

            let mut target_damage = cur_damage.unwrap();

            // Handle missing or fainted target
            if target.is_none() {
                ret_vals.push(Some(0));
                continue;
            }

            let target_pos = target.unwrap();
            let (side_idx, poke_idx) = target_pos;

            // Check if target exists and has HP
            let (has_hp, is_active) = if let Some(side) = self.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    (pokemon.hp > 0, pokemon.is_active)
                } else {
                    (false, false)
                }
            } else {
                (false, false)
            };

            if !has_hp {
                ret_vals.push(Some(0));
                continue;
            }

            if !is_active {
                ret_vals.push(None); // JavaScript returns false
                continue;
            }

            // Clamp damage to at least 1 if non-zero
            if target_damage != 0 {
                target_damage = target_damage.max(1);
            }

            // JavaScript: if (effect.id !== 'struggle-recoil')
            let effect_id = effect.map(|e| e.as_str()).unwrap_or("");
            if effect_id != "strugglerecoil" {
                // Check weather immunity
                // JavaScript: if (effect.effectType === 'Weather' && !target.runStatusImmunity(effect.id))
                if let Some(eff) = effect {
                    let effect_type = self.get_effect_type(eff);
                    if effect_type == "Weather" {
                        // Check if target is immune to this weather effect
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                if !pokemon.run_status_immunity(self, effect_id) {
                                    // Target is immune to this weather damage
                                    ret_vals.push(Some(0));
                                    continue;
                                }
                            }
                        }
                    }
                }

                // Fire Damage event
                // JavaScript: targetDamage = this.runEvent('Damage', target, source, effect, targetDamage, true);
                eprintln!("[SPREAD_DAMAGE] Before Damage event: target_damage={}", target_damage);
                let event_result = self.run_event(
                    "Damage",
                    Some(target_pos),
                    source,
                    effect,
                    Some(target_damage),
                );

                if let Some(modified_damage) = event_result {
                    eprintln!("[SPREAD_DAMAGE] After Damage event: modified_damage={}", modified_damage);
                    target_damage = modified_damage;
                } else {
                    // Event failed
                    self.debug("damage event failed");
                    ret_vals.push(None);
                    continue;
                }
            }

            // Clamp damage again after event
            if target_damage != 0 {
                target_damage = target_damage.max(1);
            }

            // Gen 1: set lastDamage for certain effects
            // JavaScript: if (this.gen <= 1) {
            //     if (this.dex.currentMod === 'gen1stadium' ||
            //         !['recoil', 'drain', 'leechseed'].includes(effect.id) && effect.effectType !== 'Status')
            //         this.lastDamage = targetDamage;
            // }
            if self.gen <= 1 {
                // TODO: Add gen1stadium check when currentMod is implemented
                // For now, we implement the second condition only
                let effect_type = effect.map(|e| self.get_effect_type(e)).unwrap_or("");
                if effect_id != "recoil"
                    && effect_id != "drain"
                    && effect_id != "leechseed"
                    && effect_type != "Status"
                {
                    self.last_damage = target_damage;
                }
            }

            // Apply damage using Pokemon's damage method
            let actual_damage = {
                let faint_queue = &mut self.faint_queue;
                if let Some(side) = self.sides.get_mut(side_idx) {
                    if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                        pokemon.damage(target_damage, target_pos, source, effect, faint_queue)
                    } else {
                        0
                    }
                } else {
                    0
                }
            };

            target_damage = actual_damage;

            // Debug: Show which Pokemon took damage
            if let Some((side, idx)) = target {
                if let Some(s) = self.sides.get(side) {
                    if let Some(p) = s.pokemon.get(idx) {
                        eprintln!("[SPREAD_DAMAGE DEBUG] {} took {} damage (HP: {}/{})", p.name, target_damage, p.hp, p.maxhp);
                    }
                }
            }

            ret_vals.push(Some(target_damage));

            // Set hurtThisTurn
            if target_damage != 0 {
                if let Some(side) = self.sides.get_mut(side_idx) {
                    if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                        pokemon.hurt_this_turn = Some(pokemon.hp);
                    }
                }
            }

            // Set source.lastDamage for moves
            // JS: if (source && effect.effectType === 'Move') source.lastDamage = targetDamage;
            if source.is_some() && effect.is_some_and(|e| self.get_effect_type(e) == "Move") {
                if let Some((src_side, src_idx)) = source {
                    if let Some(side) = self.sides.get_mut(src_side) {
                        if let Some(pokemon) = side.pokemon.get_mut(src_idx) {
                            pokemon.last_damage = target_damage;
                        }
                    }
                }
            }

            // Add damage log message
            // Inline logic from TypeScript battle.ts:2095-2114
            {
                let (side_idx, poke_idx) = target_pos;

                // Get target health string
                let health_str = if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        format!("{}/{}", pokemon.hp, pokemon.maxhp)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };

                let target_str = format!("p{}a", side_idx + 1);
                let effect_id_str = effect.map(|e| e.as_str()).unwrap_or("");

                // Get effect name for logging
                // JavaScript: const name = effect.fullname === 'tox' ? 'psn' : effect.fullname;
                let effect_name = if let Some(eff) = effect {
                    let id_str = eff.as_str();
                    // Handle special case: tox -> psn
                    if id_str == "tox" {
                        "psn".to_string()
                    } else {
                        // Try to get name from move/ability/item data
                        if let Some(move_data) = self.dex.moves().get(id_str) {
                            move_data.name.clone()
                        } else if let Some(ability_data) = self.dex.abilities().get(id_str) {
                            ability_data.name.clone()
                        } else if let Some(item_data) = self.dex.items().get(id_str) {
                            item_data.name.clone()
                        } else {
                            // For conditions/status effects, use the ID as name
                            id_str.to_string()
                        }
                    }
                } else {
                    String::new()
                };

                // Get effect type for conditional logic
                let effect_type = effect.map(|e| self.get_effect_type(e)).unwrap_or("");

                // Special case handling (matches JavaScript switch statement)
                match effect_id_str {
                    "partiallytrapped" => {
                        // Get source effect name from volatiles
                        // JS: '[from] ' + target.volatiles['partiallytrapped'].sourceEffect.fullname
                        let source_effect_name = if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                let trap_id = ID::new("partiallytrapped");
                                if let Some(trap_state) = pokemon.volatiles.get(&trap_id) {
                                    // Extract sourceEffect.fullname from data HashMap
                                    if let Some(source_effect) = trap_state.data.get("sourceEffect") {
                                        source_effect
                                            .get("fullname")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("partiallytrapped")
                                    } else {
                                        "partiallytrapped"
                                    }
                                } else {
                                    "partiallytrapped"
                                }
                            } else {
                                "partiallytrapped"
                            }
                        } else {
                            "partiallytrapped"
                        };

                        let from_str = format!("[from] {}", source_effect_name);
                        self.add(
                            "-damage",
                            &[target_str.as_str().into(), health_str.as_str().into(), from_str.into(), "[partiallytrapped]".into()],
                        );
                    }
                    "powder" => {
                        self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into(), "[silent]".into()]);
                    }
                    "confused" => {
                        self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into(), "[from] confusion".into()]);
                    }
                    _ => {
                        // Default damage log
                        // JavaScript logic:
                        // if (effect.effectType === 'Move' || !name) {
                        //     this.add('-damage', target, target.getHealth);
                        // } else if (source && (source !== target || effect.effectType === 'Ability')) {
                        //     this.add('-damage', target, target.getHealth, `[from] ${name}`, `[of] ${source}`);
                        // } else {
                        //     this.add('-damage', target, target.getHealth, `[from] ${name}`);
                        // }

                        if effect_type == "Move" || effect_name.is_empty() {
                            // Move damage or no effect name: log without [from]
                            self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into()]);
                        } else if let Some(src) = source {
                            // Check if source != target OR effectType is Ability
                            let source_is_different = src != target_pos;
                            if source_is_different || effect_type == "Ability" {
                                let src_str = format!("p{}a", src.0 + 1);
                                let from_str = format!("[from] {}", effect_name);
                                let of_str = format!("[of] {}", src_str);
                                self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into(), from_str.into(), of_str.into()]);
                            } else {
                                let from_str = format!("[from] {}", effect_name);
                                self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into(), from_str.into()]);
                            }
                        } else {
                            // No source: log with [from] only
                            let from_str = format!("[from] {}", effect_name);
                            self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into(), from_str.into()]);
                        }
                    }
                }
            }

            // Handle recoil and drain for moves
            // JavaScript: if (targetDamage && effect.effectType === 'Move')
            // JS: 			if (targetDamage && effect.effectType === 'Move') {
            // JS: 				if (this.gen <= 1 && effect.recoil && source) {
            // JS: 					if (this.dex.currentMod !== 'gen1stadium' || target.hp > 0) {
            // JS: 						const amount = this.clampIntRange(Math.floor(targetDamage * effect.recoil[0] / effect.recoil[1]), 1);
            // JS: 						this.damage(amount, source, target, 'recoil');
            // JS: 					}
            // JS: 				}
            // JS: 				if (this.gen <= 4 && effect.drain && source) {
            // JS: 					const amount = this.clampIntRange(Math.floor(targetDamage * effect.drain[0] / effect.drain[1]), 1);
            // JS: 					// Draining can be countered in gen 1
            // JS: 					if (this.gen <= 1) this.lastDamage = amount;
            // JS: 					this.heal(amount, source, target, 'drain');
            // JS: 				}
            // JS: 				if (this.gen > 4 && effect.drain && source) {
            // JS: 					const amount = Math.round(targetDamage * effect.drain[0] / effect.drain[1]);
            // JS: 					this.heal(amount, source, target, 'drain');
            // JS: 				}
            // JS: 			}
            if target_damage > 0 && effect.is_some_and(|e| self.get_effect_type(e) == "Move") {
                // Get move data to check for recoil and drain (extract data first to avoid borrow issues)
                let (recoil_data, drain_data) = if let Some(eff) = effect {
                    if let Some(move_data) = self.dex.moves().get(eff.as_str()) {
                        (move_data.recoil, move_data.drain)
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };

                // Gen 1 recoil damage
                if let (Some((recoil_num, recoil_denom)), Some(source_pos)) = (recoil_data, source)
                {
                    if self.gen <= 1 {
                        let amount = ((target_damage as f64 * recoil_num as f64)
                            / recoil_denom as f64)
                            .floor() as i32;
                        let amount = self.clamp_int_range(amount, Some(1), Some(i32::MAX));

                        let recoil_id = ID::new("recoil");
                        self.damage(amount, Some(source_pos), target, Some(&recoil_id), false);
                    }
                }

                // Gen 1-4 drain healing
                if let (Some((drain_num, drain_denom)), Some(source_pos)) = (drain_data, source) {
                    if self.gen <= 4 {
                        let amount = ((target_damage as f64 * drain_num as f64)
                            / drain_denom as f64)
                            .floor() as i32;
                        let amount = self.clamp_int_range(amount, Some(1), Some(i32::MAX));

                        // Draining can be countered in gen 1
                        if self.gen <= 1 {
                            self.last_damage = amount;
                        }

                        let drain_id = ID::new("drain");
                        self.heal(amount, Some(source_pos), target, Some(&drain_id));
                    }
                }

                // Gen 5+ drain healing (uses round instead of floor)
                if let (Some((drain_num, drain_denom)), Some(source_pos)) = (drain_data, source) {
                    if self.gen > 4 {
                        let amount = ((target_damage as f64 * drain_num as f64)
                            / drain_denom as f64)
                            .round() as i32;

                        eprintln!("[DRAIN DEBUG] target_damage={}, drain={}/{}, calculated amount={}",
                            target_damage, drain_num, drain_denom, amount);

                        let drain_id = ID::new("drain");
                        self.heal(amount, Some(source_pos), target, Some(&drain_id));
                    }
                }
            }
        }

        // Handle instafaint
        if instafaint {
            for i in 0..targets.len() {
                if ret_vals.get(i).copied().flatten().is_none() {
                    continue;
                }
                if let Some(Some(target_pos)) = targets.get(i) {
                    let should_faint = if let Some(side) = self.sides.get(target_pos.0) {
                        if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                            pokemon.hp == 0
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if should_faint {
                        self.debug("instafaint");
                        // JS: this.faintMessages(true);
                        self.faint_messages(true, false, true);

                        // Gen 1-2 special handling
                        if self.gen <= 2 {
                            if let Some(side) = self.sides.get_mut(target_pos.0) {
                                if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                                    pokemon.faint();
                                }
                            }

                            // Gen 1: Clear queue and reset Bide
                            // JS: if (this.gen <= 1) {
                            // JS:     this.queue.clear();
                            // JS:     // Fainting clears accumulated Bide damage
                            // JS:     for (const pokemon of this.getAllActive()) {
                            // JS:         if (pokemon.volatiles['bide']?.damage) {
                            // JS:             pokemon.volatiles['bide'].damage = 0;
                            // JS:             this.hint("Desync Clause Mod activated!");
                            // JS:             this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
                            // JS:         }
                            // JS:     }
                            // JS: }
                            if self.gen <= 1 {
                                self.queue.clear();

                                // Reset Bide damage for all active Pokemon
                                let mut bide_cleared = false;
                                let bide_id = ID::new("bide");

                                for side in &mut self.sides {
                                    for pokemon in &mut side.pokemon {
                                        if !pokemon.is_active {
                                            continue;
                                        }

                                        // Check if pokemon has bide volatile with damage > 0
                                        if let Some(bide_state) =
                                            pokemon.volatiles.get_mut(&bide_id)
                                        {
                                            // Check if the bide state has a damage field
                                            if let Some(damage_value) =
                                                bide_state.data.get("damage")
                                            {
                                                if let Some(damage) = damage_value.as_i64() {
                                                    if damage > 0 {
                                                        // Clear Bide damage
                                                        bide_state.data.insert(
                                                            "damage".to_string(),
                                                            serde_json::json!(0),
                                                        );
                                                        bide_cleared = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if bide_cleared {
                                    self.hint("Desync Clause Mod activated!", false, None);
                                    self.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.", false, None);
                                }
                            }
                        }
                    }
                }
            }
        }

        ret_vals
    }
}
