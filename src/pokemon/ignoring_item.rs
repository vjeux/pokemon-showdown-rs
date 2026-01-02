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
        // ✅ NOW IMPLEMENTED (Session 24 Part 47): Primal Orb check
        // Primal Orbs: Red Orb (Groudon), Blue Orb (Kyogre) - never suppressed
        if let Some(item_data) = battle.dex.items().get_by_id(&self.item) {
            if let Some(is_primal_orb) = item_data.extra.get("isPrimalOrb") {
                if is_primal_orb.as_bool().unwrap_or(false) {
                    return false;
                }
            }
        }

        // JS: if (this.battle.gen >= 5 && !this.isActive) return true;
        // ✅ NOW IMPLEMENTED: Gen check for inactive Pokemon
        if battle.gen >= 5 && !self.is_active {
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
        // ✅ NOW IMPLEMENTED: Gen check for Fling
        if is_fling {
            return battle.gen >= 5 && self.ability.as_str() == "klutz";
        }

        // JS: return !this.getItem().ignoreKlutz && this.hasAbility('klutz');
        // ✅ NOW IMPLEMENTED (Session 24 Part 47): ignoreKlutz flag check
        // Some items ignore Klutz (e.g., Macho Brace, Power items)

        // Ability Shield prevents ability suppression, including Klutz
        if self.has_item(battle, &["abilityshield"]) {
            return false;
        }

        if self.ability.as_str() == "klutz" {
            // Check if item has ignoreKlutz flag
            if let Some(item_data) = battle.dex.items().get_by_id(&self.item) {
                if let Some(ignore_klutz) = item_data.extra.get("ignoreKlutz") {
                    if ignore_klutz.as_bool().unwrap_or(false) {
                        // Item ignores Klutz, so don't suppress it
                        return false;
                    }
                }
            }
            return true;
        }

        false
    }
}
