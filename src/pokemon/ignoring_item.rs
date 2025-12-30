use crate::*;

impl Pokemon {

    /// Check if item is being ignored
    //
    // 	ignoringItem(isFling = false) {
    // 		if (this.getItem().isPrimalOrb) return false;
    // 		if (this.battle.gen >= 5 && !this.isActive) return true;
    // 		if (this.volatiles['embargo'] || this.battle.field.pseudoWeather['magicroom']) return true;
    // 		// check Fling first to avoid infinite recursion
    // 		if (isFling) return this.battle.gen >= 5 && this.hasAbility('klutz');
    // 		return !this.getItem().ignoreKlutz && this.hasAbility('klutz');
    // 	}
    //
    pub fn ignoring_item(&self) -> bool {
        // Gen 5+: inactive Pokemon have items suppressed
        if !self.is_active {
            return true;
        }
        // Embargo volatile
        if self.has_volatile(&ID::new("embargo")) {
            return true;
        }
        // Klutz ability
        if self.ability.as_str() == "klutz" {
            return true;
        }
        false
    }
}
