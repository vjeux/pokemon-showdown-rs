use crate::*;

impl Pokemon {

    /// Get ability ID
    //
    // 	getAbility() {
    // 		return this.battle.dex.abilities.getByID(this.ability);
    // 	}
    //
    pub fn get_ability(&self) -> &ID {
        // TODO: implement the same logic as JavaScript
        &self.ability
    }
}
