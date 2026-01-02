use crate::*;
use crate::dex_data::StatsTable;
use crate::pokemon::PokemonSet;

impl Pokemon {

    /// Set species (for forme changes and Transform)
    // TypeScript source:
    // /**
    // 	 * Changes this Pokemon's species to the given speciesId (or species).
    // 	 * This function only handles changes to stats and type.
    // 	 * Use formeChange to handle changes to ability and sending client messages.
    // 	 */
    // 	setSpecies(rawSpecies: Species, source: Effect | null = this.battle.effect, isTransform = false) {
    // 		const species = this.battle.runEvent('ModifySpecies', this, null, source, rawSpecies);
    // 		if (!species) return null;
    // 		this.species = species;
    //
    // 		this.setType(species.types, true);
    // 		this.apparentType = rawSpecies.types.join('/');
    // 		this.addedType = species.addedType || '';
    // 		this.knownType = true;
    // 		this.weighthg = species.weighthg;
    //
    // 		const stats = this.battle.spreadModify(this.species.baseStats, this.set);
    // 		if (this.species.maxHP) stats.hp = this.species.maxHP;
    //
    // 		if (!this.maxhp) {
    // 			this.baseMaxhp = stats.hp;
    // 			this.maxhp = stats.hp;
    // 			this.hp = stats.hp;
    // 		}
    //
    // 		if (!isTransform) this.baseStoredStats = stats;
    // 		let statName: StatIDExceptHP;
    // 		for (statName in this.storedStats) {
    // 			this.storedStats[statName] = stats[statName];
    // 			if (this.modifiedStats) this.modifiedStats[statName] = stats[statName]; // Gen 1: Reset modified stats.
    // 		}
    // 		if (this.battle.gen <= 1) {
    // 			// Gen 1: Re-Apply burn and para drops.
    // 			if (this.status === 'par') this.modifyStat!('spe', 0.25);
    // 			if (this.status === 'brn') this.modifyStat!('atk', 0.5);
    // 		}
    // 		this.speed = this.storedStats.spe;
    // 		return species;
    // 	}
    //
    pub fn set_species(
        &mut self,
        battle: &mut Battle,
        species_id: &ID,
        _source: Option<&ID>,
        is_transform: bool,
    ) -> bool {
        // JS: const species = this.battle.runEvent('ModifySpecies', this, null, source, rawSpecies);
        // Note: ModifySpecies event not called - would need to refactor for mutable battle access
        // For now, just use the species directly

        // Get species data from dex
        let (types, weightkg, base_stats) = {
            if let Some(species_data) = battle.dex.species().get(species_id.as_str()) {
                (
                    species_data.types.clone(),
                    species_data.weightkg,
                    species_data.base_stats.clone(),
                )
            } else {
                // Species not found, return false
                return false;
            }
        };

        // JS: this.species = species;
        self.species_id = species_id.clone();

        // JS: this.setType(species.types, true);
        // JS: this.apparentType = rawSpecies.types.join('/');
        // JS: this.addedType = species.addedType || '';
        // JS: this.knownType = true;
        self.types = types.clone();
        self.base_types = types.clone();
        self.added_type = None; // TypeScript uses empty string, Rust uses None
        // Note: knownType field doesn't exist in Rust Pokemon struct

        // JS: this.weighthg = species.weighthg;
        self.weight_hg = (weightkg as f64 * 10.0) as i32;

        // JS: const stats = this.battle.spreadModify(this.species.baseStats, this.set);
        // Get the pokemon's set (we need position to find it in side.team)
        let pokemon_set = {
            // Find this pokemon's set from the side's team
            if self.position < battle.sides[self.side_index].team.len() {
                battle.sides[self.side_index].team[self.position].clone()
            } else {
                // No set found, use defaults
                PokemonSet::default()
            }
        };

        let stats = battle.spread_modify(&StatsTable::from(base_stats), &pokemon_set);

        // JS: if (this.species.maxHP) stats.hp = this.species.maxHP;
        // Note: species.maxHP override not implemented - would need species data field

        // JS: if (!this.maxhp) { ... }
        if self.maxhp == 0 {
            self.base_maxhp = stats.hp;
            self.maxhp = stats.hp;
            self.hp = stats.hp;
        }

        // JS: if (!isTransform) this.baseStoredStats = stats;
        // ✅ NOW IMPLEMENTED: isTransform parameter handling - only set base_stored_stats if NOT transform
        if !is_transform {
            self.base_stored_stats = StatsTable {
                hp: 0, // HP not stored in storedStats
                atk: stats.atk,
                def: stats.def,
                spa: stats.spa,
                spd: stats.spd,
                spe: stats.spe,
            };
        }

        // JS: for (statName in this.storedStats) { this.storedStats[statName] = stats[statName]; ... }
        self.stored_stats = StatsTable {
            hp: 0, // HP not stored in storedStats
            atk: stats.atk,
            def: stats.def,
            spa: stats.spa,
            spd: stats.spd,
            spe: stats.spe,
        };

        // JS: if (this.battle.gen <= 1) {
        // JS:     // Gen 1: Re-Apply burn and para drops.
        // JS:     if (this.status === 'par') this.modifyStat!('spe', 0.25);
        // JS:     if (this.status === 'brn') this.modifyStat!('atk', 0.5);
        // JS: }
        // ✅ NOW IMPLEMENTED: Gen 1 burn/para stat drops
        if battle.gen <= 1 {
            // Gen 1: Re-Apply burn and para drops
            if self.status.as_str() == "par" {
                // Paralysis: Speed is 25% of normal
                self.stored_stats.spe = ((self.stored_stats.spe as f64) * 0.25) as i32;
            }
            if self.status.as_str() == "brn" {
                // Burn: Attack is 50% of normal
                self.stored_stats.atk = ((self.stored_stats.atk as f64) * 0.5) as i32;
            }
        }

        // JS: this.speed = this.storedStats.spe;
        self.speed = self.stored_stats.spe as i32;

        true
    }
}
