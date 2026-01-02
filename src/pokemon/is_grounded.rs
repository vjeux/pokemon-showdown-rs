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
    pub fn is_grounded(&self) -> bool {
        // JS: if ('gravity' in this.battle.field.pseudoWeather) return true;
        // Note: Missing Gravity check - would need Battle reference for field.pseudoWeather

        // JS: if ('ingrain' in this.volatiles && this.battle.gen >= 4) return true;
        // Note: Missing gen check - would need Battle reference
        if self.has_volatile(&ID::new("ingrain")) {
            return true;
        }

        // JS: if ('smackdown' in this.volatiles) return true;
        if self.has_volatile(&ID::new("smackdown")) {
            return true;
        }

        // JS: const item = (this.ignoringItem() ? '' : this.item);
        // JS: if (item === 'ironball') return true;
        // Note: Missing ignoringItem() check
        if self.item.as_str() == "ironball" {
            return true;
        }

        // JS: if (!negateImmunity && this.hasType('Flying') && !(this.hasType('???') && 'roost' in this.volatiles)) return false;
        // Note: Missing negateImmunity parameter
        // Note: Using case-sensitive comparison like JS (no toLowerCase)
        // Note: Missing special ??? + Roost case
        if self.has_type("Flying") {
            return false;
        }

        // JS: if (this.hasAbility('levitate') && !this.battle.suppressingAbility(this)) return null;
        // Note: Missing suppressingAbility check - would need Battle reference
        // Note: Should return Option<bool> to represent null, but signature is bool
        if self.ability.as_str() == "levitate" {
            return false;
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
        // Note: Missing ignoringItem() check
        self.item.as_str() != "airballoon"
    }
}
