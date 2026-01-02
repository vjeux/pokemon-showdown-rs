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
    pub fn ignoring_item(&self, battle: &Battle, is_fling: bool) -> bool {
        // JS: if (this.getItem().isPrimalOrb) return false;
        // Note: Primal Orb check not implemented - would need item data access
        // Primal Orbs: Red Orb (Groudon), Blue Orb (Kyogre) - never suppressed

        // JS: if (this.battle.gen >= 5 && !this.isActive) return true;
        // Note: Gen check not implemented - assumes gen >= 5
        if !self.is_active {
            return true;
        }

        // JS: if (this.volatiles['embargo'] || this.battle.field.pseudoWeather['magicroom']) return true;
        if self.has_volatile(&ID::new("embargo")) {
            return true;
        }

        // ✅ NOW IMPLEMENTED: Magic Room check
        let magicroom_id = ID::new("magicroom");
        if battle.field.has_pseudo_weather(&magicroom_id) {
            return true;
        }

        // JS: if (isFling) return this.battle.gen >= 5 && this.hasAbility('klutz');
        // Check Fling first to avoid infinite recursion
        // Note: Gen check not implemented - assumes gen >= 5
        if is_fling {
            return self.ability.as_str() == "klutz";
        }

        // JS: return !this.getItem().ignoreKlutz && this.hasAbility('klutz');
        // Note: ignoreKlutz flag not checked - would need item data access
        // Some items ignore Klutz (e.g., Macho Brace, Power items) but we can't check without item data

        // Ability Shield prevents ability suppression, including Klutz
        if self.has_item(battle, &["abilityshield"]) {
            return false;
        }

        if self.ability.as_str() == "klutz" {
            return true;
        }

        false
    }
}
