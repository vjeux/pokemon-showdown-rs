use crate::*;

impl Pokemon {

    /// Set a new type (for moves like Soak, Forest's Curse, etc.)
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
    pub fn set_type(&mut self, new_types: Vec<String>) {
        self.types = new_types;
    }
}
