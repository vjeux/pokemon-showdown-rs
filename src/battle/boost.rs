use crate::*;
use crate::event::EventResult;

impl Battle {

    /// Apply stat boosts to a Pokemon
    /// Equivalent to battle.ts boost() (battle.ts:1974-2043)
    ///
    //
    // 	boost(
    // 		boost: SparseBoostsTable, target: Pokemon | null = null, source: Pokemon | null = null,
    // 		effect: Effect | null = null, isSecondary = false, isSelf = false
    // 	) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		if (!target?.hp) return 0;
    // 		if (!target.isActive) return false;
    // 		if (this.gen > 5 && !target.side.foePokemonLeft()) return false;
    // 		boost = this.runEvent('ChangeBoost', target, source, effect, { ...boost });
    // 		boost = target.getCappedBoost(boost);
    // 		boost = this.runEvent('TryBoost', target, source, effect, { ...boost });
    // 		let success = null;
    // 		let boosted = isSecondary;
    // 		let boostName: BoostID;
    // 		for (boostName in boost) {
    // 			const currentBoost: SparseBoostsTable = {
    // 				[boostName]: boost[boostName],
    // 			};
    // 			let boostBy = target.boostBy(currentBoost);
    // 			let msg = '-boost';
    // 			if (boost[boostName]! < 0 || target.boosts[boostName] === -6) {
    // 				msg = '-unboost';
    // 				boostBy = -boostBy;
    // 			}
    // 			if (boostBy) {
    // 				success = true;
    // 				switch (effect?.id) {
    // 				case 'bellydrum': case 'angerpoint':
    // 					this.add('-setboost', target, 'atk', target.boosts['atk'], '[from] ' + effect.fullname);
    // 					break;
    // 				case 'bellydrum2':
    // 					this.add(msg, target, boostName, boostBy, '[silent]');
    // 					this.hint("In Gen 2, Belly Drum boosts by 2 when it fails.");
    // 					break;
    // 				case 'zpower':
    // 					this.add(msg, target, boostName, boostBy, '[zeffect]');
    // 					break;
    // 				default:
    // 					if (!effect) break;
    // 					if (effect.effectType === 'Move') {
    // 						this.add(msg, target, boostName, boostBy);
    // 					} else if (effect.effectType === 'Item') {
    // 						this.add(msg, target, boostName, boostBy, '[from] item: ' + effect.name);
    // 					} else {
    // 						if (effect.effectType === 'Ability' && !boosted) {
    // 							this.add('-ability', target, effect.name, 'boost');
    // 							boosted = true;
    // 						}
    // 						this.add(msg, target, boostName, boostBy);
    // 					}
    // 					break;
    // 				}
    // 				this.runEvent('AfterEachBoost', target, source, effect, currentBoost);
    // 			} else if (effect?.effectType === 'Ability') {
    // 				if (isSecondary || isSelf) this.add(msg, target, boostName, boostBy);
    // 			} else if (!isSecondary && !isSelf) {
    // 				this.add(msg, target, boostName, boostBy);
    // 			}
    // 		}
    // 		this.runEvent('AfterBoost', target, source, effect, boost);
    // 		if (success) {
    // 			if (Object.values(boost).some(x => x > 0)) target.statsRaisedThisTurn = true;
    // 			if (Object.values(boost).some(x => x < 0)) target.statsLoweredThisTurn = true;
    // 		}
    // 		return success;
    // 	}
    //
    pub fn boost(
        &mut self,
        boosts: &[(&str, i8)],
        target: (usize, usize),
        source: Option<(usize, usize)>,
        effect: Option<&str>,
        is_secondary: bool,
        is_self: bool,
    ) -> bool {
        let (target_side, target_idx) = target;

        // Get Pokemon name for logging
        let pokemon_name = if let Some(pokemon) = self.pokemon_at(target_side, target_idx) {
            pokemon.name.clone()
        } else {
            "Unknown".to_string()
        };

        let boost_str: Vec<String> = boosts.iter()
            .map(|(stat, value)| format!("{}:{:+}", stat, value))
            .collect();
        crate::trace_boost!("turn={}, target={}, boosts=[{}], effect={:?}",
            self.turn, pokemon_name, boost_str.join(", "), effect);

        // JS: if (!target?.hp) return 0;
        // JS: if (!target.isActive) return false;
        {
            let side = match self.sides.get(target_side) {
                Some(s) => s,
                None => return false,
            };
            let pokemon = match side.pokemon.get(target_idx) {
                Some(p) => p,
                None => return false,
            };
            if pokemon.hp == 0 || !pokemon.is_active {
                return false;
            }
        }

        // JS: if (this.gen > 5 && !target.side.foePokemonLeft()) return false;
        if self.gen > 5 {
            let foe_side = if target_side == 0 { 1 } else { 0 };
            if foe_side < self.sides.len() {
                let foe_pokemon_left = self.sides[foe_side].pokemon.iter().any(|p| p.hp > 0);
                if !foe_pokemon_left {
                    return false;
                }
            }
        }

        // JS: boost = this.runEvent('ChangeBoost', target, source, effect, { ...boost });
        // This event allows abilities/items to modify boost amounts before they're applied
        // Note: Full boost modification would require infrastructure changes to return modified boosts
        // For now, we call the event so abilities can react, even if they can't modify the boost amounts
        let effect_id = effect.map(ID::new);
        self.run_event(
                "ChangeBoost",
                Some(crate::event::EventTarget::Pokemon(target)),
            source,
            effect_id.as_ref(),
            EventResult::Continue,
            false,
            false
        );

        // JS: boost = target.getCappedBoost(boost);
        // Clamp boosts to [-6, 6] range - done per-stat below

        // JS: boost = this.runEvent('TryBoost', target, source, effect, { ...boost });
        // This event can prevent boosts from being applied (e.g., Clear Body ability)
        // If the event handler needs to cancel boosts, it should set a flag or modify Pokemon state
        // Note: JavaScript can return null to cancel all boosts - we call the event for side effects
        self.run_event("TryBoost", Some(crate::event::EventTarget::Pokemon(target)), source, effect_id.as_ref(), EventResult::Continue, false, false);

        let mut success = false;
        let mut stats_raised = false;
        let mut stats_lowered = false;
        let mut boosted = is_secondary; // JS: let boosted = isSecondary;

        // Get Pokemon name for logging
        let pokemon_name = if let Some(side) = self.sides.get(target_side) {
            if let Some(pokemon) = side.pokemon.get(target_idx) {
                format!("{}: {}", side.id_str(), pokemon.name)
            } else {
                return false;
            }
        } else {
            return false;
        };

        // JS: for (boostName in boost) { ... }
        for (stat, amount) in boosts {
            if let Some(side) = self.sides.get_mut(target_side) {
                if let Some(pokemon) = side.pokemon.get_mut(target_idx) {
                    let current = match *stat {
                        "atk" => &mut pokemon.boosts.atk,
                        "def" => &mut pokemon.boosts.def,
                        "spa" => &mut pokemon.boosts.spa,
                        "spd" => &mut pokemon.boosts.spd,
                        "spe" => &mut pokemon.boosts.spe,
                        "accuracy" => &mut pokemon.boosts.accuracy,
                        "evasion" => &mut pokemon.boosts.evasion,
                        _ => continue,
                    };

                    let old = *current;
                    // JS: boostBy = target.boostBy(currentBoost);
                    *current = (*current + amount).clamp(-6, 6);
                    let actual = *current - old;

                    // JS: if (boostBy) { success = true; ... }
                    if actual != 0 {
                        success = true;
                        if actual > 0 {
                            stats_raised = true;
                        } else {
                            stats_lowered = true;
                        }

                        let mut msg = if actual > 0 { "-boost" } else { "-unboost" };
                        let boost_by = if actual < 0 || *current == -6 {
                            msg = "-unboost";
                            actual.abs()
                        } else {
                            actual
                        };
                        let boost_str = boost_by.abs().to_string();
                        let current_value_str = current.to_string(); // Extract value before calling self.add()

                        // JS: Special effect handling (bellydrum, angerpoint, zpower, etc.)
                        if let Some(eff) = effect {
                            match eff {
                                // JS: case 'bellydrum': case 'angerpoint':
                                "bellydrum" | "angerpoint" => {
                                    if *stat == "atk" {
                                        self.add("-setboost", &[
                                            pokemon_name.as_str().into(),
                                            "atk".into(),
                                            current_value_str.as_str().into(),
                                            format!("[from] {}", eff).into(),
                                        ]);
                                    }
                                }
                                // JS: case 'bellydrum2':
                                "bellydrum2" => {
                                    self.add(msg, &[
                                        pokemon_name.as_str().into(),
                                        (*stat).into(),
                                        boost_str.as_str().into(),
                                        "[silent]".into(),
                                    ]);
                                    self.hint("In Gen 2, Belly Drum boosts by 2 when it fails.", false, None);
                                }
                                // JS: case 'zpower':
                                "zpower" => {
                                    self.add(msg, &[
                                        pokemon_name.as_str().into(),
                                        (*stat).into(),
                                        boost_str.as_str().into(),
                                        "[zeffect]".into(),
                                    ]);
                                }
                                // JS: default:
                                _ => {
                                    // Get effect type from dex
                                    let effect_type = self.get_effect_type(&ID::new(eff));

                                    match effect_type {
                                        // JS: if (effect.effectType === 'Move')
                                        "Move" => {
                                            self.add(msg, &[
                                                pokemon_name.as_str().into(),
                                                (*stat).into(),
                                                boost_str.as_str().into(),
                                            ]);
                                        }
                                        // JS: else if (effect.effectType === 'Item')
                                        "Item" => {
                                            self.add(msg, &[
                                                pokemon_name.as_str().into(),
                                                (*stat).into(),
                                                boost_str.as_str().into(),
                                                format!("[from] item: {}", eff).into(),
                                            ]);
                                        }
                                        // JS: else (including Ability)
                                        _ => {
                                            // JS: if (effect.effectType === 'Ability' && !boosted) {
                                            //       this.add('-ability', target, effect.name, 'boost');
                                            //       boosted = true;
                                            //     }
                                            //     this.add(msg, target, boostName, boostBy);
                                            if effect_type == "Ability" && !boosted {
                                                self.add("-ability", &[
                                                    pokemon_name.as_str().into(),
                                                    eff.into(),
                                                    "boost".into(),
                                                ]);
                                                boosted = true;
                                            }
                                            self.add(msg, &[
                                                pokemon_name.as_str().into(),
                                                (*stat).into(),
                                                boost_str.as_str().into(),
                                            ]);
                                        }
                                    }
                                }
                            }
                        } else {
                            self.add(msg, &[pokemon_name.as_str().into(), (*stat).into(), boost_str.as_str().into()]);
                        }

                        // JS: this.runEvent('AfterEachBoost', target, source, effect, currentBoost);
                        self.run_event("AfterEachBoost", Some(crate::event::EventTarget::Pokemon(target)), source, None, EventResult::Continue, false, false);
                    } else {
                        // JS: } else if (effect?.effectType === 'Ability') {
                        //       if (isSecondary || isSelf) this.add(msg, target, boostName, boostBy);
                        //     } else if (!isSecondary && !isSelf) {
                        //       this.add(msg, target, boostName, boostBy);
                        //     }
                        let msg = "-boost"; // When boost is 0, always use "-boost"
                        let boost_str = "0";

                        if let Some(eff) = effect {
                            let effect_type = self.get_effect_type(&ID::new(eff));
                            if effect_type == "Ability" {
                                if is_secondary || is_self {
                                    self.add(msg, &[
                                        pokemon_name.as_str().into(),
                                        (*stat).into(),
                                        boost_str.into(),
                                    ]);
                                }
                            } else if !is_secondary && !is_self {
                                self.add(msg, &[
                                    pokemon_name.as_str().into(),
                                    (*stat).into(),
                                    boost_str.into(),
                                ]);
                            }
                        } else if !is_secondary && !is_self {
                            self.add(msg, &[
                                pokemon_name.as_str().into(),
                                (*stat).into(),
                                boost_str.into(),
                            ]);
                        }
                    }
                }
            }
        }

        // JS: this.runEvent('AfterBoost', target, source, effect, boost);
        self.run_event("AfterBoost", Some(crate::event::EventTarget::Pokemon(target)), source, None, EventResult::Continue, false, false);

        // JS: if (Object.values(boost).some(x => x > 0)) target.statsRaisedThisTurn = true;
        // JS: if (Object.values(boost).some(x => x < 0)) target.statsLoweredThisTurn = true;
        if success {
            if let Some(side) = self.sides.get_mut(target_side) {
                if let Some(pokemon) = side.pokemon.get_mut(target_idx) {
                    if stats_raised {
                        pokemon.stats_raised_this_turn = true;
                    }
                    if stats_lowered {
                        pokemon.stats_lowered_this_turn = true;
                    }
                }
            }
        }

        success
    }
}
