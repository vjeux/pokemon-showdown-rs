use crate::*;
use crate::battle::Effect;

impl Battle {

    /// Deal direct damage (bypasses most effects)
    /// Matches JavaScript battle.ts:2177-2230 directDamage()
    ///
    //
    // 	directDamage(damage: number, target?: Pokemon, source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		if (!target?.hp) return 0;
    // 		if (!damage) return 0;
    // 		damage = this.clampIntRange(damage, 1);
    //
    // 		if (typeof effect === 'string' || !effect) effect = this.dex.conditions.getByID((effect || '') as ID);
    //
    // 		// In Gen 1 BUT NOT STADIUM, Substitute also takes confusion and HJK recoil damage
    // 		if (this.gen <= 1 && this.dex.currentMod !== 'gen1stadium' &&
    // 			['confusion', 'jumpkick', 'highjumpkick'].includes(effect.id)) {
    // 			// Confusion and recoil damage can be countered
    // 			this.lastDamage = damage;
    // 			if (target.volatiles['substitute']) {
    // 				const hint = "In Gen 1, if a Pokemon with a Substitute hurts itself due to confusion or Jump Kick/Hi Jump Kick recoil and the target";
    // 				// if the move was a self-targeting move, the source is the same as the target. We need to check the opposing substitute
    // 				const foe = target.side.foe.active[0];
    // 				if (foe?.volatiles['substitute']) {
    // 					foe.volatiles['substitute'].hp -= damage;
    // 					if (foe.volatiles['substitute'].hp <= 0) {
    // 						foe.removeVolatile('substitute');
    // 						foe.subFainted = true;
    // 					} else {
    // 						this.add('-activate', foe, 'Substitute', '[damage]');
    // 					}
    // 					this.hint(hint + " has a Substitute, the target's Substitute takes the damage.");
    // 					return damage;
    // 				} else {
    // 					this.hint(hint + " does not have a Substitute there is no damage dealt.");
    // 					return 0;
    // 				}
    // 			}
    // 		}
    //
    // 		damage = target.damage(damage, source, effect);
    // 		switch (effect.id) {
    // 		case 'strugglerecoil':
    // 			this.add('-damage', target, target.getHealth, '[from] recoil');
    // 			break;
    // 		case 'confusion':
    // 			this.add('-damage', target, target.getHealth, '[from] confusion');
    // 			break;
    // 		default:
    // 			this.add('-damage', target, target.getHealth);
    // 			break;
    // 		}
    // 		if (target.fainted) this.faint(target);
    // 		return damage;
    // 	}
    //
    pub fn direct_damage(
        &mut self,
        mut damage: i32,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&Effect>,
    ) -> i32 {
        // JS: if (this.event) { target ||= this.event.target; source ||= this.event.source; effect ||= this.effect; }
        // Extract event context values first to avoid borrow checker issues
        let (event_target, event_source, event_effect) = if let Some(ref event) = self.current_event
        {
            (event.target, event.source, event.effect.clone())
        } else {
            (None, None, None)
        };

        let target = target.or(event_target);
        let _source = source.or(event_source); // Not used in current implementation but matches JS signature
        let effect_owned: Option<Effect>;
        let effect = if effect.is_none() {
            effect_owned = event_effect;
            effect_owned.as_ref()
        } else {
            effect
        };

        // Get target, handle None case
        let target_pos = match target {
            Some(pos) => pos,
            None => return 0,
        };

        // Check if target has HP
        let (has_hp, _) = if let Some(side) = self.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                (pokemon.hp > 0, pokemon.is_active)
            } else {
                return 0;
            }
        } else {
            return 0;
        };

        if !has_hp {
            return 0;
        }
        if damage == 0 {
            return 0;
        }

        // Clamp damage to at least 1
        damage = damage.max(1);

        let effect_id = effect.map(|e| e.id.as_str()).unwrap_or("");

        // Gen 1 special case: Substitute takes confusion and HJK recoil damage
        // JavaScript: if (this.gen <= 1 && this.dex.currentMod !== 'gen1stadium' && ...)
        if self.gen <= 1 && ["confusion", "jumpkick", "highjumpkick"].contains(&effect_id) {
            // Confusion and recoil damage can be countered
            self.last_damage = damage;

            // Check if target has Substitute volatile
            let substitute_id = ID::new("substitute");
            let has_substitute = if let Some(side) = self.sides.get(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                    pokemon.volatiles.contains_key(&substitute_id)
                } else {
                    false
                }
            } else {
                false
            };

            if has_substitute {
                // Check if foe has Substitute
                let foe_side = if target_pos.0 == 0 { 1 } else { 0 };
                let foe_has_substitute = if foe_side < self.sides.len() {
                    if let Some(side) = self.sides.get(foe_side) {
                        if let Some(&Some(poke_idx)) = side.active.first() {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                pokemon.volatiles.contains_key(&substitute_id)
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                let hint = "In Gen 1, if a Pokemon with a Substitute hurts itself due to confusion or Jump Kick/Hi Jump Kick recoil and the target";

                if foe_has_substitute {
                    // Damage foe's substitute
                    // JS: foe.volatiles['substitute'].hp -= damage;
                    // Get foe position
                    let foe_pos = if foe_side < self.sides.len() {
                        if let Some(side) = self.sides.get(foe_side) {
                            if let Some(active_idx) = side.active.first() {
                                active_idx.map(|idx| (foe_side, idx))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some((foe_side_idx, foe_poke_idx)) = foe_pos {
                        // Deduct damage from substitute HP
                        let mut sub_destroyed = false;
                        if let Some(foe_pokemon) = self
                            .sides
                            .get_mut(foe_side_idx)
                            .and_then(|s| s.pokemon.get_mut(foe_poke_idx))
                        {
                            if let Some(sub_state) = foe_pokemon.volatiles.get_mut(&substitute_id) {
                                // Get current HP from volatile state
                                let current_hp = sub_state.hp.unwrap_or(0);

                                let new_hp = current_hp - damage;

                                if new_hp <= 0 {
                                    // JS: foe.volatiles['substitute'].hp <= 0
                                    sub_destroyed = true;
                                } else {
                                    // Update HP
                                    sub_state.hp = Some(new_hp);
                                }
                            }
                        }

                        // Handle substitute destruction or damage log
                        if sub_destroyed {
                            // JS: foe.removeVolatile('substitute'); foe.subFainted = true;
                            if let Some(foe_pokemon) = self
                                .sides
                                .get_mut(foe_side_idx)
                                .and_then(|s| s.pokemon.get_mut(foe_poke_idx))
                            {
                                foe_pokemon.volatiles.remove(&substitute_id);
                                // JavaScript: pokemon.subFainted = true (boolean | null type)
                                foe_pokemon.sub_fainted = Some(true);
                            }
                        } else {
                            // JS: this.add('-activate', foe, 'Substitute', '[damage]');
                            let foe_ident = if let Some(foe_pokemon) = self
                                .sides
                                .get(foe_side_idx)
                                .and_then(|s| s.pokemon.get(foe_poke_idx))
                            {
                                foe_pokemon.get_slot()
                            } else {
                                String::new()
                            };
                            if !foe_ident.is_empty() {
                                self.add("-activate", &[foe_ident.into(), "Substitute".into(), "[damage]".into()]);
                            }
                        }
                    }

                    self.hint(
                        &format!(
                            "{} has a Substitute, the target's Substitute takes the damage.",
                            hint
                        ),
                        false,
                        None,
                    );
                    return damage;
                } else {
                    self.hint(
                        &format!(
                            "{} does not have a Substitute there is no damage dealt.",
                            hint
                        ),
                        false,
                        None,
                    );
                    return 0;
                }
            }
        }

        // Apply damage using Pokemon's damage method
        let actual_damage = if let Some(side) = self.sides.get_mut(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                let old_hp = pokemon.hp;
                pokemon.hp = pokemon.hp.saturating_sub(damage).max(0);  // Clamp to 0
                let actual = old_hp - pokemon.hp;
                actual
            } else {
                0
            }
        } else {
            0
        };

        // Add damage log message based on effect
        // Inline logic from TypeScript battle.ts:2216-2226
        {
            let (side_idx, poke_idx) = target_pos;

            // Get target identifier and health string
            let (target_str, health_str) = if let Some(side) = self.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    (pokemon.get_slot(), format!("{}/{}", pokemon.hp, pokemon.maxhp))
                } else {
                    return actual_damage;
                }
            } else {
                return actual_damage;
            };

            let effect_id = effect.map(|e| e.id.as_str()).unwrap_or("");

            // Special case handling
            // JS: this.add('-damage', target, target.getHealth, '[from] recoil/confusion')
            match effect_id {
                "strugglerecoil" => {
                    self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into(), "[from] recoil".into()]);
                }
                "confusion" => {
                    self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into(), "[from] confusion".into()]);
                }
                _ => {
                    self.add("-damage", &[target_str.as_str().into(), health_str.as_str().into()]);
                }
            }
        }

        // Check if target fainted
        if let Some(side) = self.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                if pokemon.hp == 0 {
                    // JS: if (pokemon.hp <= 0) pokemon.faint(source, effect);
                    let effect_str = effect.map(|e| e.id.as_str());
                    self.faint(target_pos, source, effect_str);
                }
            }
        }

        actual_damage
    }
}
