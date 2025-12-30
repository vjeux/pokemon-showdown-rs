use crate::*;

impl Pokemon {

    /// Get ability ID
    //
    // 	getAbility() {
    // 		return this.battle.dex.abilities.getByID(this.ability);
    // 	}
    //
    pub fn get_ability(&self) -> &ID {
        &self.ability
    }
}
