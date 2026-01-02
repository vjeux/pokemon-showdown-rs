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
        // JS: if (this.getItem().isPrimalOrb) return false;
        // Note: Primal Orb check not implemented - would need item data access

        // JS: if (this.battle.gen >= 5 && !this.isActive) return true;
        // Note: Gen check not implemented - assumes gen >= 5
        if !self.is_active {
            return true;
        }

        // JS: if (this.volatiles['embargo'] || this.battle.field.pseudoWeather['magicroom']) return true;
        if self.has_volatile(&ID::new("embargo")) {
            return true;
        }
        // Note: Magic Room check not implemented - would need Battle reference

        // JS: if (isFling) return this.battle.gen >= 5 && this.hasAbility('klutz');
        // Note: isFling parameter not in Rust signature

        // JS: return !this.getItem().ignoreKlutz && this.hasAbility('klutz');
        // Note: ignoreKlutz flag not checked - would need item data access
        if self.ability.as_str() == "klutz" {
            return true;
        }

        false
    }
}
