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
        // JS: return this.battle.dex.conditions.getByID(this.status);
        // In Rust, we return the ID directly (or None if no status)
        // Callers can use battle.dex.conditions.get() if they need the full condition data
        if self.status.is_empty() {
            None
        } else {
            Some(&self.status)
        }
    }
}
