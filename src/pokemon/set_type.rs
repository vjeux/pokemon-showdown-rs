use crate::*;

impl Pokemon {

    /// Set a new type (for moves like Soak, Forest's Curse, etc.)
    ///
    /// This is an associated function (not a method) because it needs
    /// mutable access to Battle while operating on a Pokemon within that Battle.
    /// Call as: Pokemon::set_type(battle, pokemon_pos, new_types, enforce)
    // TypeScript source:
    // /**
    // 	 * Sets a type (except on Arceus, who resists type changes)
    // 	 */
    // 	setType(newType: string | string[], enforce = false) {
    // 		if (!enforce) {
    // 			// No Pokemon should be able to have Stellar as a base type
    // 			if (typeof newType === 'string' ? newType === 'Stellar' : newType.includes('Stellar')) return false;
    // 			// First type of Arceus, Silvally cannot be normally changed
    // 			if ((this.battle.gen >= 5 && (this.species.num === 493 || this.species.num === 773)) ||
    // 				(this.battle.gen === 4 && this.hasAbility('multitype'))) {
    // 				return false;
    // 			}
    // 			// Terastallized Pokemon cannot have their base type changed except via forme change
    // 			if (this.terastallized) return false;
    // 		}
    //
    // 		if (!newType) throw new Error("Must pass type to setType");
    // 		this.types = (typeof newType === 'string' ? [newType] : newType);
    // 		this.addedType = '';
    // 		this.knownType = true;
    // 		this.apparentType = this.types.join('/');
    //
    // 		return true;
    // 	}
    //
    pub fn set_type(battle: &mut Battle, pokemon_pos: (usize, usize), new_types: Vec<String>, enforce: bool) -> bool {
        // JS: if (!enforce) { ... }
        if !enforce {
            // JS: if (typeof newType === 'string' ? newType === 'Stellar' : newType.includes('Stellar')) return false;
            // ✅ NOW IMPLEMENTED: Stellar type check
            if new_types.iter().any(|t| t == "Stellar") {
                return false;
            }

            // Phase 1: Extract data immutably
            let (_species_id, species_num, terastallized) = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return false,
                };

                let species_num = battle.dex.species.get(&pokemon.species_id)
                    .map(|s| s.num)
                    .unwrap_or(0);

                (pokemon.species_id.clone(), species_num, pokemon.terastallized.is_some())
            };

            // JS: if ((this.battle.gen >= 5 && (this.species.num === 493 || this.species.num === 773)) ||
            // JS:     (this.battle.gen === 4 && this.hasAbility('multitype'))) {
            // JS:     return false;
            // JS: }
            // ✅ NOW IMPLEMENTED: Arceus (493) and Silvally (773) protection
            // Gen 5+: Block Arceus and Silvally type changes
            if battle.gen >= 5 && (species_num == 493 || species_num == 773) {
                return false;
            }

            // Gen 4: Block Arceus with Multitype ability
            if battle.gen == 4 {
                let has_multitype = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return false,
                    };
                    pokemon.has_ability(battle, &["multitype"])
                };

                if has_multitype {
                    return false;
                }
            }

            // JS: if (this.terastallized) return false;
            // ✅ NOW IMPLEMENTED: Terastallized protection
            if terastallized {
                return false;
            }
        }

        // JS: if (!newType) throw new Error("Must pass type to setType");
        // ✅ NOW IMPLEMENTED: Empty type check
        if new_types.is_empty() {
            return false; // Return false instead of panic
        }

        // Phase 2: Get mutable reference and apply changes
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: this.types = (typeof newType === 'string' ? [newType] : newType);
        pokemon.types = new_types;

        // JS: this.addedType = '';
        // ✅ NOW IMPLEMENTED: addedType reset
        pokemon.added_type = None;

        // JS: this.knownType = true;
        // Note: Missing knownType field assignment (field doesn't exist)

        // JS: this.apparentType = this.types.join('/');
        // Note: Missing apparentType field assignment (field doesn't exist)

        // JS: return true;
        true
    }
}
