// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle::{Effect, EffectType};
use crate::event::EventResult;

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
        damages: crate::battle_actions::SpreadMoveDamage,
        targets: &crate::battle_actions::SpreadMoveTargets,
        source: Option<(usize, usize)>,
        effect: Option<&Effect>,
        instafaint: bool,
    ) -> crate::battle_actions::SpreadMoveDamage {
        use crate::battle_actions::{DamageResult, SpreadMoveTarget};
        let mut ret_vals: crate::battle_actions::SpreadMoveDamage = Vec::new();

        // Process damage for each target
        for i in 0..damages.len() {
            let cur_damage = &damages[i];
            let target = &targets[i];

            // Handle undefined/failed damage
            let damage_value = match cur_damage {
                DamageResult::Damage(n) => *n,
                DamageResult::Failed | DamageResult::Undefined | DamageResult::NotFail => {
                    ret_vals.push(*cur_damage);
                    continue;
                }
                DamageResult::HitSubstitute => {
                    // HIT_SUBSTITUTE - substitute blocked the hit, pass through
                    ret_vals.push(DamageResult::HitSubstitute);
                    continue;
                }
                DamageResult::Success => {
                    // Success means "true" in JS, which should calculate max damage
                    // For now, treat as 0
                    0
                }
            };

            // Handle missing or fainted target
            let target_pos = match target {
                SpreadMoveTarget::Target(pos) => *pos,
                _ => {
                    ret_vals.push(DamageResult::Damage(0));
                    continue;
                }
            };

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
                ret_vals.push(DamageResult::Damage(0));
                continue;
            }

            if !is_active {
                ret_vals.push(DamageResult::Failed); // JavaScript returns false
                continue;
            }

            let mut target_damage = damage_value;

            // Clamp damage to at least 1 if non-zero
            if target_damage != 0 {
                target_damage = target_damage.max(1);
            }

            // JavaScript: if (effect.id !== 'struggle-recoil')
            let effect_id = effect.map(|e| e.id.as_str()).unwrap_or("");
            if effect_id != "strugglerecoil" {
                // Check weather immunity
                // JavaScript: if (effect.effectType === 'Weather' && !target.runStatusImmunity(effect.id))
                if let Some(eff) = effect {
                    // Check if this effect is a weather effect by comparing with field.weather
                    // This is more reliable than get_effect_type which checks moves before conditions
                    let is_weather = self.field.weather == eff.id;

                    if is_weather {
                        // Check if target is immune to this weather effect
                        let is_immune = !Pokemon::run_status_immunity(self, (side_idx, poke_idx), effect_id, false);
                        if is_immune {
                            // Target is immune to this weather damage
                            ret_vals.push(DamageResult::Damage(0));
                            continue;
                        }
                    }
                }

                // Fire Damage event
                // JavaScript: targetDamage = this.runEvent('Damage', target, source, effect, targetDamage, true);
                // The 6th parameter (true) is onEffect - this allows the move's own onDamage callback to run
                // Only use on_effect=true when we have an actual effect (e.g., a move like False Swipe)
                let on_effect = effect.is_some();
                let event_result = self.run_event(
                "Damage",
                Some(crate::event::EventTarget::Pokemon(target_pos)),
                    source,
                    effect,
                    EventResult::Number(target_damage),
                    on_effect,   // onEffect: include the move's onDamage callback (e.g., False Swipe)
                    false,
                );

                match event_result {
                    EventResult::Number(modified_damage) => {
                        target_damage = modified_damage;
                    }
                    EventResult::Null | EventResult::Boolean(false) => {
                        // Event failed / returned false (e.g., Magic Guard blocking damage)
                        self.debug("damage event failed");
                        ret_vals.push(DamageResult::Undefined);
                        continue;
                    }
                    _ => {
                        // Continue with current damage value
                    }
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
                let is_gen1stadium = self.dex.current_mod.as_deref() == Some("gen1stadium");
                let is_status_effect = effect.map(|e| e.effect_type == EffectType::Status).unwrap_or(false);

                // Check second condition: !['recoil', 'drain', 'leechseed'].includes(effect.id) && effect.effectType !== 'Status'
                let second_condition = effect_id != "recoil"
                    && effect_id != "drain"
                    && effect_id != "leechseed"
                    && !is_status_effect;

                if is_gen1stadium || second_condition {
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

            ret_vals.push(DamageResult::Damage(target_damage));

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
            if source.is_some() && effect.is_some_and(|e| e.effect_type == EffectType::Move) {
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
                let effect_id_str = effect.map(|e| e.id.as_str()).unwrap_or("");

                // Get effect name for logging
                // JavaScript: const name = effect.fullname === 'tox' ? 'psn' : effect.fullname;
                let effect_name = if let Some(eff) = effect {
                    let id_str = eff.id.as_str();
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
                let effect_type = effect.map(|e| e.effect_type);
                let is_move_effect = effect_type == Some(EffectType::Move);
                let is_ability_effect = effect_type == Some(EffectType::Ability);

                // Special case handling (matches JavaScript switch statement)
                match effect_id_str {
                    "partiallytrapped" => {
                        // Get source effect name from volatiles
                        // JS: '[from] ' + target.volatiles['partiallytrapped'].sourceEffect.fullname
                        let source_effect_name = if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                let trap_id = ID::new("partiallytrapped");
                                if let Some(trap_state) = pokemon.volatiles.get(&trap_id) {
                                    // Use the typed source_effect field from EffectState
                                    if let Some(ref source_eff) = trap_state.source_effect {
                                        source_eff.id.as_str()
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

                        if is_move_effect || effect_name.is_empty() {
                            // Move damage or no effect name: log without [from]
                            self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into()]);
                        } else if let Some(src) = source {
                            // Check if source != target OR effectType is Ability
                            let source_is_different = src != target_pos;
                            if source_is_different || is_ability_effect {
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
            if target_damage > 0 && effect.is_some_and(|e| e.effect_type == EffectType::Move) {
                // Get move data to check for recoil and drain (extract data first to avoid borrow issues)
                let (recoil_data, drain_data) = if let Some(eff) = effect {
                    if let Some(move_data) = self.dex.moves().get(eff.id.as_str()) {
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
                        // JS: if (this.dex.currentMod !== 'gen1stadium' || target.hp > 0) {
                        let is_gen1stadium = self.dex.current_mod.as_deref() == Some("gen1stadium");
                        let target_hp = if let Some(side) = self.sides.get(target_pos.0) {
                            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                                pokemon.hp
                            } else {
                                0
                            }
                        } else {
                            0
                        };

                        if !is_gen1stadium || target_hp > 0 {
                            let amount = ((target_damage as f64 * recoil_num as f64)
                                / recoil_denom as f64)
                                .floor() as i32;
                            let amount = self.clamp_int_range(amount, Some(1), Some(i32::MAX));

                            let recoil_effect = Effect::condition("recoil");
                            self.damage(amount, Some(source_pos), Some(target_pos), Some(&recoil_effect), false);
                        }
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

                        let drain_effect = Effect::condition("drain");
                        self.heal(amount, Some(source_pos), Some(target_pos), Some(&drain_effect));
                    }
                }

                // Gen 5+ drain healing (uses round instead of floor)
                if let (Some((drain_num, drain_denom)), Some(source_pos)) = (drain_data, source) {
                    if self.gen > 4 {
                        let amount = ((target_damage as f64 * drain_num as f64)
                            / drain_denom as f64)
                            .round() as i32;

                        let drain_effect = Effect::condition("drain");
                        self.heal(amount, Some(source_pos), Some(target_pos), Some(&drain_effect));
                    }
                }
            }
        }

        // Handle instafaint
        if instafaint {
            for i in 0..targets.len() {
                // Check if this target has valid damage (skip Failed/Undefined)
                if matches!(ret_vals.get(i), Some(DamageResult::Failed) | Some(DamageResult::Undefined) | None) {
                    continue;
                }
                if let Some(SpreadMoveTarget::Target(target_pos)) = targets.get(i) {
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
                            Pokemon::faint(self, *target_pos, source, effect);

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
                                            // Check if the bide state has damage > 0
                                            if let Some(damage) = bide_state.damage {
                                                if damage > 0 {
                                                    // Clear Bide damage
                                                    bide_state.damage = Some(0);
                                                    bide_cleared = true;
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
