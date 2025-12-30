use crate::*;

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
    pub fn set_species(&mut self, species_id: ID, types: Vec<String>, weight_hg: i32) {
        self.species_id = species_id;
        self.types = types.clone();
        self.base_types = types;
        self.weight_hg = weight_hg;
    }
}
