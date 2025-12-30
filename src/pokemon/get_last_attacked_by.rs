use crate::*;
use crate::pokemon::Attacker;

impl Pokemon {

    /// Get last attacker info
    /// Equivalent to getLastAttackedBy in pokemon.ts
    //
    // 	getLastAttackedBy() {
    // 		if (this.attackedBy.length === 0) return undefined;
    // 		return this.attackedBy[this.attackedBy.length - 1];
    // 	}
    //
    pub fn get_last_attacked_by(&self) -> Option<&Attacker> {
        // JS: if (this.attackedBy.length === 0) return undefined;
        // JS: return this.attackedBy[this.attackedBy.length - 1];
        self.attacked_by.last()
    }
}
