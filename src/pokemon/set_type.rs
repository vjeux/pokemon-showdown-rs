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
        // JS: if (!enforce) { ... }
        // Note: Missing enforce parameter (defaults to false in JS)

        // JS: if (typeof newType === 'string' ? newType === 'Stellar' : newType.includes('Stellar')) return false;
        // Note: Missing Stellar type check when !enforce

        // JS: if ((this.battle.gen >= 5 && (this.species.num === 493 || this.species.num === 773)) ||
        // JS:     (this.battle.gen === 4 && this.hasAbility('multitype'))) {
        // JS:     return false;
        // JS: }
        // Note: Missing Arceus (493) and Silvally (773) protection
        // Would need Battle reference for gen check and species data for num

        // JS: if (this.terastallized) return false;
        // Note: Missing Terastallized protection when !enforce

        // JS: if (!newType) throw new Error("Must pass type to setType");
        // Note: No error handling for empty newType

        // JS: this.types = (typeof newType === 'string' ? [newType] : newType);
        self.types = new_types;

        // JS: this.addedType = '';
        // Note: Missing addedType reset
        // Would need: self.added_type = None;

        // JS: this.knownType = true;
        // Note: Missing knownType field assignment

        // JS: this.apparentType = this.types.join('/');
        // Note: Missing apparentType field assignment

        // JS: return true;
        // Note: Returns nothing (void) instead of bool
    }
}
