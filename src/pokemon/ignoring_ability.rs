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
    pub fn ignoring_ability(&self, battle: &Battle) -> bool {
        // JS: if (this.battle.gen >= 5 && !this.isActive) return true;
        // ✅ NOW IMPLEMENTED: Gen check
        if battle.gen >= 5 && !self.is_active {
            return true;
        }

        // JS: if (this.getAbility().flags['notransform'] && this.transformed) return true;
        // ✅ NOW IMPLEMENTED (Session 24 Part 48): Check notransform flag
        // Certain Abilities won't activate while Transformed (e.g. Disguise)
        if self.transformed {
            if let Some(ability_data) = battle.dex.abilities().get_by_id(&self.ability) {
                if let Some(&notransform_flag) = ability_data.flags.get("notransform") {
                    if notransform_flag != 0 {
                        return true;
                    }
                }
            }
        }

        // JS: if (this.getAbility().flags['cantsuppress']) return false;
        // ✅ NOW IMPLEMENTED (Session 24 Part 48): Check cantsuppress flag
        // Some abilities cannot be suppressed (e.g. Commander, As One)
        if let Some(ability_data) = battle.dex.abilities().get_by_id(&self.ability) {
            if let Some(&cantsuppress_flag) = ability_data.flags.get("cantsuppress") {
                if cantsuppress_flag != 0 {
                    return false;
                }
            }
        }

        // JS: if (this.volatiles['gastroacid']) return true;
        if self.has_volatile(&ID::new("gastroacid")) {
            return true;
        }

        // JS: if (this.hasItem('Ability Shield') || this.ability === 'neutralizinggas') return false;
        // ✅ NOW IMPLEMENTED: Ability Shield and Neutralizing Gas check
        if self.has_item(battle, &["abilityshield"]) || self.ability.as_str() == "neutralizinggas" {
            return false;
        }

        // JS: for (const pokemon of this.battle.getAllActive()) {
        // JS:     if (pokemon.ability === 'neutralizinggas' && !pokemon.volatiles['gastroacid'] &&
        // JS:         !pokemon.transformed && !pokemon.abilityState.ending && !this.volatiles['commanding']) {
        // JS:         return true;
        // JS:     }
        // JS: }
        // ✅ NOW IMPLEMENTED: Check all active Pokemon for Neutralizing Gas
        for side in &battle.sides {
            for pokemon in &side.pokemon {
                if !pokemon.is_active {
                    continue;
                }
                if pokemon.ability.as_str() == "neutralizinggas"
                    && !pokemon.has_volatile(&ID::new("gastroacid"))
                    && !pokemon.transformed
                    && !pokemon.ability_state.borrow().ending.unwrap_or(false)
                    && !self.has_volatile(&ID::new("commanding"))
                {
                    return true;
                }
            }
        }

        false
    }
}
