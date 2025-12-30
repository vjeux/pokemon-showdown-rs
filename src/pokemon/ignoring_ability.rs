use crate::*;

impl Pokemon {

    /// Check if ability is being suppressed
    //
    // 	ignoringAbility() {
    // 		if (this.battle.gen >= 5 && !this.isActive) return true;
    //
    // 		// Certain Abilities won't activate while Transformed, even if they ordinarily couldn't be suppressed (e.g. Disguise)
    // 		if (this.getAbility().flags['notransform'] && this.transformed) return true;
    // 		if (this.getAbility().flags['cantsuppress']) return false;
    // 		if (this.volatiles['gastroacid']) return true;
    //
    // 		// Check if any active pokemon have the ability Neutralizing Gas
    // 		if (this.hasItem('Ability Shield') || this.ability === ('neutralizinggas' as ID)) return false;
    // 		for (const pokemon of this.battle.getAllActive()) {
    // 			// can't use hasAbility because it would lead to infinite recursion
    // 			if (pokemon.ability === ('neutralizinggas' as ID) && !pokemon.volatiles['gastroacid'] &&
    // 				!pokemon.transformed && !pokemon.abilityState.ending && !this.volatiles['commanding']) {
    // 				return true;
    // 			}
    // 		}
    //
    // 		return false;
    // 	}
    //
    pub fn ignoring_ability(&self) -> bool {
        // Gen 5+: inactive Pokemon have abilities suppressed
        if !self.is_active {
            return true;
        }
        // Transformed Pokemon with certain abilities
        if self.transformed {
            // Would need to check ability flags
        }
        // Gastro Acid volatile
        if self.has_volatile(&ID::new("gastroacid")) {
            return true;
        }
        false
    }
}
