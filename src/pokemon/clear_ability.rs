use crate::*;

impl Pokemon {

    /// Clear ability (set to empty)
    //
    // 	clearAbility() {
    // 		return this.setAbility('');
    // 	}
    //
    pub fn clear_ability(&mut self) -> ID {
        // JS: return this.setAbility('');
        self.set_ability(ID::empty())
    }
}
