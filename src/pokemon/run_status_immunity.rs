use crate::*;

impl Pokemon {

    /// Run status immunity check
    /// Equivalent to runStatusImmunity in pokemon.ts
    //
    // 	runStatusImmunity(type: string, message?: string) {
    // 		if (this.fainted) return false;
    // 		if (!type) return true;
    //
    // 		if (!this.battle.dex.getImmunity(type, this)) {
    // 			this.battle.debug('natural status immunity');
    // 			if (message) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		const immunity = this.battle.runEvent('Immunity', this, null, null, type);
    // 		if (!immunity) {
    // 			this.battle.debug('artificial status immunity');
    // 			if (message && immunity !== null) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn run_status_immunity(&self, status: &str) -> bool {
        match status {
            "brn" => !self.has_type("fire"),
            "par" => !self.has_type("electric"),
            "psn" | "tox" => !self.has_type("poison") && !self.has_type("steel"),
            "frz" => !self.has_type("ice"),
            "slp" => true, // No type immunity to sleep
            _ => true,
        }
    }
}
