use crate::*;

impl Pokemon {

    /// Get the types of this Pokemon
    //
    // 	getTypes(excludeAdded?: boolean, preterastallized?: boolean): string[] {
    // 		if (!preterastallized && this.terastallized && this.terastallized !== 'Stellar') {
    // 			return [this.terastallized];
    // 		}
    // 		const types = this.battle.runEvent('Type', this, null, null, this.types);
    // 		if (!types.length) types.push(this.battle.gen >= 5 ? 'Normal' : '???');
    // 		if (!excludeAdded && this.addedType) return types.concat(this.addedType);
    // 		return types;
    // 	}
    //
    pub fn get_types(&self, battle: &Battle, exclude_added: bool) -> Vec<String> {
        self.get_types_full(battle, exclude_added, false)
    }

    pub fn get_types_full(&self, battle: &Battle, exclude_added: bool, preterastallized: bool) -> Vec<String> {
        // JS: if (!preterastallized && this.terastallized && this.terastallized !== 'Stellar') return [this.terastallized];
        if !preterastallized {
            if let Some(ref tera) = self.terastallized {
                if tera != "Stellar" {
                    return vec![tera.clone()];
                }
            }
        }

        // JS: const types = this.battle.runEvent('Type', this, null, null, this.types);
        // ✅ NOW IMPLEMENTED: runEvent('Type') handling for Arceus and Silvally
        // These species have conditions that modify their type based on held items
        let mut types = self.types.clone();

        // Handle Arceus type (from data/conditions.ts arceus.onType)
        // Arceus's base forme has types specified in Pokedex (e.g., Fire for Arceus-Fire)
        // but its actual type is determined by Multitype + held plate
        let base_species = if let Some(species) = battle.dex.species().get(self.species_id.as_str()) {
            species.base_species.clone()
        } else {
            None
        };

        if base_species.as_deref() == Some("Arceus") {
            // onType(types, pokemon) for Arceus condition
            // if (pokemon.transformed || pokemon.ability !== 'multitype' && this.gen >= 8) return types;
            if !self.transformed {
                let has_multitype = self.ability.as_str() == "multitype";
                if has_multitype || battle.gen < 8 {
                    let mut type_name = "Normal".to_string();
                    if has_multitype {
                        // type = pokemon.getItem().onPlate;
                        if let Some(item) = battle.dex.items().get_by_id(&self.item) {
                            if let Some(plate_type) = &item.on_plate {
                                type_name = plate_type.clone();
                            }
                        }
                    }
                    types = vec![type_name];
                }
            }
        }

        // Handle Silvally type (from data/conditions.ts silvally.onType)
        if base_species.as_deref() == Some("Silvally") {
            // if (pokemon.transformed || pokemon.ability !== 'rkssystem' && this.gen >= 8) return types;
            if !self.transformed {
                let has_rks = self.ability.as_str() == "rkssystem";
                if has_rks || battle.gen < 8 {
                    let mut type_name = "Normal".to_string();
                    if has_rks {
                        // type = pokemon.getItem().onMemory;
                        if let Some(item) = battle.dex.items().get_by_id(&self.item) {
                            if let Some(memory_type) = &item.on_memory {
                                type_name = memory_type.clone();
                            }
                        }
                    }
                    types = vec![type_name];
                }
            }
        }

        // JS: if (!types.length) types.push(this.battle.gen >= 5 ? 'Normal' : '???');
        // ✅ NOW IMPLEMENTED: Gen check for default type
        if types.is_empty() {
            if battle.gen >= 5 {
                types.push("Normal".to_string());
            } else {
                types.push("???".to_string());
            }
        }

        // JS: if (!excludeAdded && this.addedType) return types.concat(this.addedType);
        // JavaScript: addedType is string, check if not empty
        if !exclude_added && !self.added_type.is_empty() {
            if !types.contains(&self.added_type) {
                types.push(self.added_type.clone());
            }
        }
        types
    }
}
