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
        // Flying type or Levitate makes you not grounded
        if self.types.iter().any(|t| t.to_lowercase() == "flying") {
            return false;
        }
        if self.ability.as_str() == "levitate" {
            return false;
        }
        // Air Balloon makes you not grounded
        if self.item.as_str() == "airballoon" {
            return false;
        }
        // Magnet Rise volatile
        if self.has_volatile(&ID::new("magnetrise")) {
            return false;
        }
        // Telekinesis volatile
        if self.has_volatile(&ID::new("telekinesis")) {
            return false;
        }
        // Iron Ball or Gravity or Ingrain makes you grounded even if flying
        if self.item.as_str() == "ironball" {
            return true;
        }
        if self.has_volatile(&ID::new("ingrain")) {
            return true;
        }
        true
    }
}
