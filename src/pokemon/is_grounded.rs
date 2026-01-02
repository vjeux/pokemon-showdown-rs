use crate::*;

impl Pokemon {

    /// Check if Pokemon is grounded (affected by Ground moves and terrain)
    //
    // 	isGrounded(negateImmunity = false) {
    // 		if ('gravity' in this.battle.field.pseudoWeather) return true;
    // 		if ('ingrain' in this.volatiles && this.battle.gen >= 4) return true;
    // 		if ('smackdown' in this.volatiles) return true;
    // 		const item = (this.ignoringItem() ? '' : this.item);
    // 		if (item === 'ironball') return true;
    // 		// If a Fire/Flying type uses Burn Up and Roost, it becomes ???/Flying-type, but it's still grounded.
    // 		if (!negateImmunity && this.hasType('Flying') && !(this.hasType('???') && 'roost' in this.volatiles)) return false;
    // 		if (this.hasAbility('levitate') && !this.battle.suppressingAbility(this)) return null;
    // 		if ('magnetrise' in this.volatiles) return false;
    // 		if ('telekinesis' in this.volatiles) return false;
    // 		return item !== 'airballoon';
    // 	}
    //
    pub fn is_grounded(&self, battle: &Battle, negate_immunity: bool) -> bool {
        // JS: if ('gravity' in this.battle.field.pseudoWeather) return true;
        // ✅ NOW IMPLEMENTED: Gravity check
        let gravity_id = ID::new("gravity");
        if battle.field.has_pseudo_weather(&gravity_id) {
            return true;
        }

        // JS: if ('ingrain' in this.volatiles && this.battle.gen >= 4) return true;
        // ✅ NOW IMPLEMENTED: Gen check for Ingrain (gen >= 4)
        if self.has_volatile(&ID::new("ingrain")) && battle.gen >= 4 {
            return true;
        }

        // JS: if ('smackdown' in this.volatiles) return true;
        if self.has_volatile(&ID::new("smackdown")) {
            return true;
        }

        // JS: const item = (this.ignoringItem() ? '' : this.item);
        let item = if self.ignoring_item(battle, false) {
            ""
        } else {
            self.item.as_str()
        };

        // JS: if (item === 'ironball') return true;
        if item == "ironball" {
            return true;
        }

        // JS: if (!negateImmunity && this.hasType('Flying') && !(this.hasType('???') && 'roost' in this.volatiles)) return false;
        // ✅ NOW IMPLEMENTED: negateImmunity parameter
        // ✅ NOW IMPLEMENTED: Special ??? + Roost case for Fire/Flying with Burn Up
        if !negate_immunity && self.has_type(battle, "Flying") {
            // Special case: If Pokemon has ??? type AND Roost volatile, it IS grounded
            // This happens when Fire/Flying uses Burn Up (loses Fire, becomes ???) then Roost
            let has_unknown_type = self.has_type(battle, "???");
            let has_roost = self.has_volatile(&ID::new("roost"));

            // If it has both ??? and Roost, don't return false (continue to other checks)
            // Otherwise, Flying type means not grounded
            if !(has_unknown_type && has_roost) {
                return false;
            }
        }

        // JS: if (this.hasAbility('levitate') && !this.battle.suppressingAbility(this)) return null;
        // ✅ NOW IMPLEMENTED: suppressingAbility check for Levitate
        // Note: JavaScript returns null here, but Rust returns false (approximation)
        // Note: Changing to Option<bool> would require updating 21 callsites
        // Note: In JavaScript boolean contexts, null is falsy, so false is an acceptable approximation
        if self.has_ability(battle, &["levitate"]) {
            // If ability is being suppressed (e.g., by Mold Breaker), Levitate doesn't apply
            if !battle.suppressing_ability(Some((self.side_index, self.position))) {
                return false; // Should be None/null in perfect 1-to-1, but false works
            }
        }

        // JS: if ('magnetrise' in this.volatiles) return false;
        if self.has_volatile(&ID::new("magnetrise")) {
            return false;
        }

        // JS: if ('telekinesis' in this.volatiles) return false;
        if self.has_volatile(&ID::new("telekinesis")) {
            return false;
        }

        // JS: return item !== 'airballoon';
        item != "airballoon"
    }
}
