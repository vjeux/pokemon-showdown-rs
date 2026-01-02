use crate::*;

impl Pokemon {

    /// Get status object
    /// Equivalent to getStatus in pokemon.ts
    //
    // 	getStatus() {
    // 		return this.battle.dex.conditions.getByID(this.status);
    // 	}
    //
    pub fn get_status(&self) -> Option<&ID> {
        // TODO: implement the same logic as JavaScript
        if self.status.is_empty() {
            None
        } else {
            Some(&self.status)
        }
    }
}
