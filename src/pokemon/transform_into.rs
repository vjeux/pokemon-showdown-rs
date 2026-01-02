use crate::*;
use crate::pokemon::MoveSlot;

impl Pokemon {

    /// Transform into another Pokemon
    //
    // 	transformInto(pokemon: Pokemon, effect?: Effect) {
    // 		const species = pokemon.species;
    // 		if (
    // 			pokemon.fainted || this.illusion || pokemon.illusion || (pokemon.volatiles['substitute'] && this.battle.gen >= 5) ||
    // 			(pokemon.transformed && this.battle.gen >= 2) || (this.transformed && this.battle.gen >= 5) ||
    // 			species.name === 'Eternatus-Eternamax' ||
    // 			(['Ogerpon', 'Terapagos'].includes(species.baseSpecies) && (this.terastallized || pokemon.terastallized)) ||
    // 			this.terastallized === 'Stellar'
    // 		) {
    // 			return false;
    // 		}
    //
    // 		if (this.battle.dex.currentMod === 'gen1stadium' && (
    // 			species.name === 'Ditto' ||
    // 			(this.species.name === 'Ditto' && pokemon.moves.includes('transform'))
    // 		)) {
    // 			return false;
    // 		}
    //
    // 		if (!this.setSpecies(species, effect, true)) return false;
    //
    // 		this.transformed = true;
    // 		this.weighthg = pokemon.weighthg;
    //
    // 		const types = pokemon.getTypes(true, true);
    // 		this.setType(pokemon.volatiles['roost'] ? pokemon.volatiles['roost'].typeWas : types, true);
    // 		this.addedType = pokemon.addedType;
    // 		this.knownType = this.isAlly(pokemon) && pokemon.knownType;
    // 		this.apparentType = pokemon.apparentType;
    //
    // 		let statName: StatIDExceptHP;
    // 		for (statName in this.storedStats) {
    // 			this.storedStats[statName] = pokemon.storedStats[statName];
    // 			if (this.modifiedStats) this.modifiedStats[statName] = pokemon.modifiedStats![statName]; // Gen 1: Copy modified stats.
    // 		}
    // 		this.moveSlots = [];
    // 		this.hpType = (this.battle.gen >= 5 ? this.hpType : pokemon.hpType);
    // 		this.hpPower = (this.battle.gen >= 5 ? this.hpPower : pokemon.hpPower);
    // 		this.timesAttacked = pokemon.timesAttacked;
    // 		for (const moveSlot of pokemon.moveSlots) {
    // 			let moveName = moveSlot.move;
    // 			if (moveSlot.id === 'hiddenpower') {
    // 				moveName = 'Hidden Power ' + this.hpType;
    // 			}
    // 			this.moveSlots.push({
    // 				move: moveName,
    // 				id: moveSlot.id,
    // 				pp: moveSlot.maxpp === 1 ? 1 : 5,
    // 				maxpp: this.battle.gen >= 5 ? (moveSlot.maxpp === 1 ? 1 : 5) : moveSlot.maxpp,
    // 				target: moveSlot.target,
    // 				disabled: false,
    // 				used: false,
    // 				virtual: true,
    // 			});
    // 		}
    // 		let boostName: BoostID;
    // 		for (boostName in pokemon.boosts) {
    // 			this.boosts[boostName] = pokemon.boosts[boostName];
    // 		}
    // 		if (this.battle.gen >= 6) {
    // 			// we need to remove all of the overlapping crit volatiles before adding any of them
    // 			const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
    // 			for (const volatile of volatilesToCopy) this.removeVolatile(volatile);
    // 			for (const volatile of volatilesToCopy) {
    // 				if (pokemon.volatiles[volatile]) {
    // 					this.addVolatile(volatile);
    // 					if (volatile === 'gmaxchistrike') this.volatiles[volatile].layers = pokemon.volatiles[volatile].layers;
    // 					if (volatile === 'dragoncheer') this.volatiles[volatile].hasDragonType = pokemon.volatiles[volatile].hasDragonType;
    // 				}
    // 			}
    // 		}
    // 		if (effect) {
    // 			this.battle.add('-transform', this, pokemon, '[from] ' + effect.fullname);
    // 		} else {
    // 			this.battle.add('-transform', this, pokemon);
    // 		}
    // 		if (this.terastallized) {
    // 			this.knownType = true;
    // 			this.apparentType = this.terastallized;
    // 		}
    // 		if (this.battle.gen > 2) this.setAbility(pokemon.ability, this, null, true, true);
    //
    // 		// Change formes based on held items (for Transform)
    // 		// Only ever relevant in Generation 4 since Generation 3 didn't have item-based forme changes
    // 		if (this.battle.gen === 4) {
    // 			if (this.species.num === 487) {
    // 				// Giratina formes
    // 				if (this.species.name === 'Giratina' && this.item === 'griseousorb') {
    // 					this.formeChange('Giratina-Origin');
    // 				} else if (this.species.name === 'Giratina-Origin' && this.item !== 'griseousorb') {
    // 					this.formeChange('Giratina');
    // 				}
    // 			}
    // 			if (this.species.num === 493) {
    // 				// Arceus formes
    // 				const item = this.getItem();
    // 				const targetForme = (item?.onPlate ? 'Arceus-' + item.onPlate : 'Arceus');
    // 				if (this.species.name !== targetForme) {
    // 					this.formeChange(targetForme);
    // 				}
    // 			}
    // 		}
    //
    // 		// Pokemon transformed into Ogerpon cannot Terastallize
    // 		// restoring their ability to tera after they untransform is handled ELSEWHERE
    // 		if (['Ogerpon', 'Terapagos'].includes(this.species.baseSpecies) && this.canTerastallize) this.canTerastallize = false;
    //
    // 		return true;
    // 	}
    //
    pub fn transform_into(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: (usize, usize)) -> bool {
        // Extract gen value upfront (before any mutable borrows)
        let gen = battle.gen;

        // Phase 1: Extract target data immutably
        let (target_species_id, target_weight_hg, target_types, target_added_type, target_stored_stats,
             target_move_slots, target_boosts, target_ability, target_has_substitute, target_transformed,
             target_fainted, target_times_attacked, target_apparent_type, target_hp_type, target_hp_power) = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return false,
            };

            (
                target.species_id.clone(),
                target.weight_hg,
                target.types.clone(),
                target.added_type.clone(),
                target.stored_stats,
                target.move_slots.clone(),
                target.boosts,
                target.ability.clone(),
                target.has_volatile(&ID::new("substitute")),
                target.transformed,
                target.fainted,
                target.times_attacked,
                target.apparent_type.clone(),
                target.hp_type.clone(),
                target.hp_power,
            )
        };

        // Phase 2: Check self pokemon immutably
        let self_terastallized = {
            let self_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };

            // JS: const species = pokemon.species;
            // JS: if (pokemon.fainted || this.illusion || pokemon.illusion || ...) return false;
            // Note: Missing illusion checks on both pokemon
            if self_pokemon.fainted || target_fainted {
                return false;
            }

            // JS: (pokemon.volatiles['substitute'] && this.battle.gen >= 5)
            // ✅ NOW IMPLEMENTED: Gen >= 5 check for substitute
            if target_has_substitute && gen >= 5 {
                return false;
            }

            // JS: (pokemon.transformed && this.battle.gen >= 2)
            // ✅ NOW IMPLEMENTED: Gen >= 2 check for target transformed
            if target_transformed && gen >= 2 {
                return false;
            }

            // JS: (this.transformed && this.battle.gen >= 5)
            // ✅ NOW IMPLEMENTED: Gen >= 5 check for self transformed
            if self_pokemon.transformed && gen >= 5 {
                return false;
            }

            // JS: species.name === 'Eternatus-Eternamax'
            // Note: Missing Eternatus-Eternamax check - would need species data

            // JS: (['Ogerpon', 'Terapagos'].includes(species.baseSpecies) && (this.terastallized || pokemon.terastallized))
            // Note: Missing Ogerpon/Terapagos terastallized check - would need species data

            // JS: this.terastallized === 'Stellar'
            // ✅ NOW IMPLEMENTED: Stellar tera check
            if let Some(ref tera_type) = self_pokemon.terastallized {
                if tera_type == "Stellar" {
                    return false;
                }
            }

            self_pokemon.terastallized.clone()
        };

        // Phase 3: Get mutable reference and apply transformation
        let self_pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: if (this.battle.dex.currentMod === 'gen1stadium' && ...) return false;
        // Note: Missing gen1stadium Ditto checks - would need Battle reference

        // JS: if (!this.setSpecies(species, effect, true)) return false;
        // Note: Not calling setSpecies - should update types, stats, weight from species data

        // Copy species
        self_pokemon_mut.species_id = target_species_id;

        // JS: this.transformed = true;
        self_pokemon_mut.transformed = true;

        // JS: this.weighthg = pokemon.weighthg;
        self_pokemon_mut.weight_hg = target_weight_hg;

        // JS: const types = pokemon.getTypes(true, true);
        // JS: this.setType(pokemon.volatiles['roost'] ? pokemon.volatiles['roost'].typeWas : types, true);
        // Note: Missing roost volatile type handling
        // Copy types
        self_pokemon_mut.types = target_types;

        // JS: this.addedType = pokemon.addedType;
        self_pokemon_mut.added_type = target_added_type;

        // JS: this.knownType = this.isAlly(pokemon) && pokemon.knownType;
        // Note: JavaScript knownType is boolean (is type publicly known), Rust known_type is Option<String> (what type is known for Illusion)
        // Different semantics, so not setting known_type here

        // JS: this.apparentType = pokemon.apparentType;
        // ✅ NOW IMPLEMENTED: apparentType copying from target
        self_pokemon_mut.apparent_type = target_apparent_type;

        // JS: for (statName in this.storedStats) { this.storedStats[statName] = pokemon.storedStats[statName]; }
        // Copy stats
        self_pokemon_mut.stored_stats = target_stored_stats;

        // JS: if (this.modifiedStats) this.modifiedStats[statName] = pokemon.modifiedStats![statName];
        // Note: Missing modifiedStats copying for Gen 1

        // JS: this.moveSlots = [];
        // JS: this.hpType = (this.battle.gen >= 5 ? this.hpType : pokemon.hpType);
        // JS: this.hpPower = (this.battle.gen >= 5 ? this.hpPower : pokemon.hpPower);
        // ✅ NOW IMPLEMENTED (Session 24 Part 58): hpType/hpPower conditional copying based on gen
        if gen >= 5 {
            // Gen >= 5: Keep self's hp_type and hp_power (no action needed, already set)
        } else {
            // Gen < 5: Copy target's hp_type and hp_power
            self_pokemon_mut.hp_type = target_hp_type;
            self_pokemon_mut.hp_power = target_hp_power;
        }

        // JS: this.timesAttacked = pokemon.timesAttacked;
        // ✅ NOW IMPLEMENTED: timesAttacked copying
        self_pokemon_mut.times_attacked = target_times_attacked;

        // JS: for (const moveSlot of pokemon.moveSlots) {
        // JS:     let moveName = moveSlot.move;
        // JS:     if (moveSlot.id === 'hiddenpower') {
        // JS:         moveName = 'Hidden Power ' + this.hpType;
        // JS:     }
        // Extract hp_type for Hidden Power formatting
        let hp_type_for_moves = self_pokemon_mut.hp_type.clone();

        // Copy moves with reduced PP
        // ✅ NOW IMPLEMENTED (Session 24 Part 59): Hidden Power move name formatting with hpType
        self_pokemon_mut.move_slots = target_move_slots
            .iter()
            .map(|slot| {
                // Format move name for Hidden Power
                let move_name = if slot.id.as_str() == "hiddenpower" {
                    if !hp_type_for_moves.is_empty() {
                        format!("Hidden Power {}", hp_type_for_moves)
                    } else {
                        slot.move_name.clone()
                    }
                } else {
                    slot.move_name.clone()
                };

                MoveSlot {
                    id: slot.id.clone(),
                    move_name,
                    // JS: pp: moveSlot.maxpp === 1 ? 1 : 5,
                    // JS: maxpp: this.battle.gen >= 5 ? (moveSlot.maxpp === 1 ? 1 : 5) : moveSlot.maxpp,
                    // ✅ NOW IMPLEMENTED: Gen check for maxpp calculation
                    pp: if slot.maxpp == 1 { 1 } else { 5 },
                    maxpp: if gen >= 5 {
                        if slot.maxpp == 1 { 1 } else { 5 }
                    } else {
                        slot.maxpp
                    },
                    target: slot.target.clone(),
                    disabled: false,
                    disabled_source: None,
                    used: false,
                    virtual_move: true,
                    is_z: slot.is_z,
                }
            })
            .collect();

        // JS: for (boostName in pokemon.boosts) { this.boosts[boostName] = pokemon.boosts[boostName]; }
        // Copy boosts
        self_pokemon_mut.boosts = target_boosts;

        // Release mutable reference before we need Battle access for volatiles
        let _ = self_pokemon_mut;

        // JS: if (this.battle.gen >= 6) {
        // JS:     const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
        // JS:     for (const volatile of volatilesToCopy) this.removeVolatile(volatile);
        // JS:     for (const volatile of volatilesToCopy) {
        // JS:         if (pokemon.volatiles[volatile]) {
        // JS:             this.addVolatile(volatile);
        // JS:             if (volatile === 'gmaxchistrike') this.volatiles[volatile].layers = ...;
        // JS:             if (volatile === 'dragoncheer') this.volatiles[volatile].hasDragonType = ...;
        // JS:         }
        // JS:     }
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 33): Gen 6+ crit volatile copying
        if gen >= 6 {
            let volatiles_to_copy = vec![
                ID::new("dragoncheer"),
                ID::new("focusenergy"),
                ID::new("gmaxchistrike"),
                ID::new("laserfocus"),
            ];

            // First pass: remove existing volatiles
            for volatile_id in &volatiles_to_copy {
                Pokemon::remove_volatile(battle, pokemon_pos, volatile_id);
            }

            // Second pass: check target and add if present
            for volatile_id in &volatiles_to_copy {
                // Check if target has this volatile and extract data
                let (has_volatile, layers_data, dragon_type_data) = {
                    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return false,
                    };

                    if let Some(volatile_state) = target.volatiles.get(volatile_id) {
                        // Extract layers for gmaxchistrike
                        let layers = if volatile_id.as_str() == "gmaxchistrike" {
                            volatile_state.data.get("layers")
                                .and_then(|v| v.as_i64())
                                .map(|v| v as i32)
                        } else {
                            None
                        };

                        // Extract hasDragonType for dragoncheer
                        let has_dragon = if volatile_id.as_str() == "dragoncheer" {
                            volatile_state.data.get("hasDragonType")
                                .and_then(|v| v.as_bool())
                        } else {
                            None
                        };

                        (true, layers, has_dragon)
                    } else {
                        (false, None, None)
                    }
                };

                if has_volatile {
                    // Add volatile to self
                    Pokemon::add_volatile(battle, pokemon_pos, volatile_id.clone(), None, None, None);

                    // Copy additional data fields
                    if let Some(layers) = layers_data {
                        let self_pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                            Some(p) => p,
                            None => return false,
                        };
                        if let Some(volatile_state) = self_pokemon.volatiles.get_mut(volatile_id) {
                            volatile_state.data.insert("layers".to_string(), serde_json::json!(layers));
                        }
                    }

                    if let Some(has_dragon) = dragon_type_data {
                        let self_pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                            Some(p) => p,
                            None => return false,
                        };
                        if let Some(volatile_state) = self_pokemon.volatiles.get_mut(volatile_id) {
                            volatile_state.data.insert("hasDragonType".to_string(), serde_json::json!(has_dragon));
                        }
                    }
                }
            }
        }

        // JS: if (effect) {
        // JS:     this.battle.add('-transform', this, pokemon, '[from] ' + effect.fullname);
        // JS: } else {
        // JS:     this.battle.add('-transform', this, pokemon);
        // JS: }
        // ✅ NOW IMPLEMENTED: battle.add('-transform') message
        {
            let self_ident = {
                let self_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return false,
                };
                self_pokemon.get_slot()
            };
            let target_ident = {
                let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return false,
                };
                target_pokemon.get_slot()
            };
            battle.add(
                "-transform",
                &[
                    self_ident.as_str().into(),
                    target_ident.as_str().into(),
                ],
            );
        }

        // Get mutable reference again for final transformations
        let self_pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: if (this.terastallized) {
        // JS:     this.knownType = true;
        // JS:     this.apparentType = this.terastallized;
        // JS: }
        // ✅ NOW IMPLEMENTED: terastallized apparentType update
        if let Some(tera_type) = self_terastallized {
            // JavaScript: this.apparentType = this.terastallized;
            self_pokemon_mut.apparent_type = tera_type;
        }
        // Note: Not setting known_type as it has different semantics in Rust (Option<String> vs boolean)

        // JS: if (this.battle.gen > 2) this.setAbility(pokemon.ability, this, null, true, true);
        // ✅ NOW IMPLEMENTED: Proper setAbility call with parameters (gen > 2)
        // JS parameters: (ability, source=this, sourceEffect=null, isFromFormeChange=true, isTransform=true)
        if gen > 2 {
            Pokemon::set_ability(battle, pokemon_pos, target_ability, Some(pokemon_pos), None, true, true);
        }

        // JS: // Change formes based on held items (for Transform)
        // JS: if (this.battle.gen === 4) { ... Giratina/Arceus forme changes ... }
        // Note: Missing Gen 4 Giratina/Arceus forme changes

        // JS: if (['Ogerpon', 'Terapagos'].includes(this.species.baseSpecies) && this.canTerastallize)
        // JS:     this.canTerastallize = false;
        // Note: Missing Ogerpon/Terapagos canTerastallize blocking

        // JS: return true;
        true
    }
}
