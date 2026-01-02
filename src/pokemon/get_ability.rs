use crate::*;

impl Pokemon {

    /// Get ability ID
    //
    // 	getAbility() {
    // 		return this.battle.dex.abilities.getByID(this.ability);
    // 	}
    //
    pub fn get_ability(&self) -> &ID {
        // JS: return this.battle.dex.abilities.getByID(this.ability);
        // In Rust, we return the ID directly rather than looking up the full ability object
        // Callers can use battle.dex.abilities.get() if they need the full ability data
        &self.ability
    }
}
