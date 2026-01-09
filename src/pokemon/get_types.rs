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
        // This dispatches to species conditions (arceus, silvally) which can modify types.
        // The condition is looked up by baseSpecies name from the species data.
        let mut types = self.run_type_event(battle);

        // JS: if (!types.length) types.push(this.battle.gen >= 5 ? 'Normal' : '???');
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

    /// Run the 'Type' event for this Pokemon
    /// This handles species conditions like Arceus/Silvally that modify types based on held items.
    ///
    /// JavaScript uses runEvent('Type') which dispatches to conditions defined in data/conditions.ts.
    /// These conditions are keyed by the base species name (e.g., "arceus" for all Arceus formes).
    /// The condition is inherited: dex-species.ts line 531 does:
    ///   const baseSpeciesStatuses = this.dex.data.Conditions[toID(species.baseSpecies)];
    ///
    /// So we look up the species data to get baseSpecies, then match on that.
    fn run_type_event(&self, battle: &Battle) -> Vec<String> {
        // Look up the species data to get the baseSpecies field
        // JavaScript: species.baseSpecies is a string like "Arceus" for all Arceus formes
        let base_species_name = if let Some(species_data) = battle.dex.species().get_by_id(&self.species_id) {
            // species_data.base_species is Option<String>, None means same as name
            species_data.base_species.as_ref()
                .map(|s| ID::new(s))
                .unwrap_or_else(|| ID::new(&species_data.name))
        } else {
            self.species_id.clone()
        };

        // Check for species-specific type modifications
        // These are defined in data/conditions.ts and inherit to all formes
        match base_species_name.as_str() {
            "arceus" => self.arceus_on_type(battle),
            "silvally" => self.silvally_on_type(battle),
            _ => self.types.clone(),
        }
    }

    /// Arceus onType callback
    /// JavaScript source (data/conditions.ts):
    /// onType(types, pokemon) {
    ///     if (pokemon.transformed || pokemon.ability !== 'multitype' && this.gen >= 8) return types;
    ///     let type: string | undefined = 'Normal';
    ///     if (pokemon.ability === 'multitype') {
    ///         type = pokemon.getItem().onPlate;
    ///         if (!type) { type = 'Normal'; }
    ///     }
    ///     return [type];
    /// }
    fn arceus_on_type(&self, battle: &Battle) -> Vec<String> {
        let has_multitype = self.has_ability(battle, &["multitype"]);

        // if (pokemon.transformed || pokemon.ability !== 'multitype' && this.gen >= 8) return types;
        if self.transformed || (!has_multitype && battle.gen >= 8) {
            return self.types.clone();
        }

        // let type = 'Normal';
        let mut type_name = "Normal".to_string();

        // if (pokemon.ability === 'multitype')
        if has_multitype {
            // type = pokemon.getItem().onPlate;
            let item_id = self.get_item();
            if let Some(item) = battle.dex.items().get_by_id(item_id) {
                if let Some(ref plate_type) = item.on_plate {
                    type_name = plate_type.clone();
                }
            }
            // if (!type) { type = 'Normal'; } is already handled by initialization
        }

        // return [type];
        vec![type_name]
    }

    /// Silvally onType callback
    /// JavaScript source (data/conditions.ts):
    /// onType(types, pokemon) {
    ///     if (pokemon.transformed || pokemon.ability !== 'rkssystem' && this.gen >= 8) return types;
    ///     let type: string | undefined = 'Normal';
    ///     if (pokemon.ability === 'rkssystem') {
    ///         type = pokemon.getItem().onMemory;
    ///         if (!type) { type = 'Normal'; }
    ///     }
    ///     return [type];
    /// }
    fn silvally_on_type(&self, battle: &Battle) -> Vec<String> {
        let has_rkssystem = self.has_ability(battle, &["rkssystem"]);

        // if (pokemon.transformed || pokemon.ability !== 'rkssystem' && this.gen >= 8) return types;
        if self.transformed || (!has_rkssystem && battle.gen >= 8) {
            return self.types.clone();
        }

        // let type = 'Normal';
        let mut type_name = "Normal".to_string();

        // if (pokemon.ability === 'rkssystem')
        if has_rkssystem {
            // type = pokemon.getItem().onMemory;
            let item_id = self.get_item();
            if let Some(item) = battle.dex.items().get_by_id(item_id) {
                if let Some(ref memory_type) = item.on_memory {
                    type_name = memory_type.clone();
                }
            }
            // if (!type) { type = 'Normal'; } is already handled by initialization
        }

        // return [type];
        vec![type_name]
    }
}
