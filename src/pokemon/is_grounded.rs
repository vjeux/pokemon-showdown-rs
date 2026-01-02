use crate::*;

impl Pokemon {

    /// Check if Pokemon is grounded (affected by Ground moves and terrain)
    /// Returns Option<bool>:
    ///   - Some(true) = grounded
    ///   - Some(false) = not grounded
    ///   - None = null (Levitate without suppression)
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
    pub fn is_grounded(&self, battle: &Battle, negate_immunity: bool) -> Option<bool> {
        // JS: if ('gravity' in this.battle.field.pseudoWeather) return true;
        // ✅ NOW IMPLEMENTED: Gravity check
        let gravity_id = ID::new("gravity");
        if battle.field.has_pseudo_weather(&gravity_id) {
            return Some(true);
        }

        // JS: if ('ingrain' in this.volatiles && this.battle.gen >= 4) return true;
        // ✅ NOW IMPLEMENTED: Gen check for Ingrain (gen >= 4)
        if self.has_volatile(&ID::new("ingrain")) && battle.gen >= 4 {
            return Some(true);
        }

        // JS: if ('smackdown' in this.volatiles) return true;
        if self.has_volatile(&ID::new("smackdown")) {
            return Some(true);
        }

        // JS: const item = (this.ignoringItem() ? '' : this.item);
        let item = if self.ignoring_item(battle, false) {
            ""
        } else {
            self.item.as_str()
        };

        // JS: if (item === 'ironball') return true;
        if item == "ironball" {
            return Some(true);
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
                return Some(false);
            }
        }

        // JS: if (this.hasAbility('levitate') && !this.battle.suppressingAbility(this)) return null;
        // ✅ NOW IMPLEMENTED (Session 24 Part 94): Return None for Levitate (1:1 with JavaScript null)
        if self.has_ability(battle, &["levitate"]) {
            // If ability is being suppressed (e.g., by Mold Breaker), Levitate doesn't apply
            if !battle.suppressing_ability(Some((self.side_index, self.position))) {
                return None; // JavaScript returns null here
            }
        }

        // JS: if ('magnetrise' in this.volatiles) return false;
        if self.has_volatile(&ID::new("magnetrise")) {
            return Some(false);
        }

        // JS: if ('telekinesis' in this.volatiles) return false;
        if self.has_volatile(&ID::new("telekinesis")) {
            return Some(false);
        }

        // JS: return item !== 'airballoon';
        // Returns true if no airballoon, false if has airballoon
        Some(item != "airballoon")
    }
}
