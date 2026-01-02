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
        // JS: if (this.fainted) return false;
        // Note: Fainted check not implemented

        // JS: if (!type) return true;
        // Note: Empty string check not needed in Rust

        // JS: if (!this.battle.dex.getImmunity(type, this)) {
        // JS:     this.battle.debug('natural status immunity');
        // JS:     if (message) this.battle.add('-immune', this);
        // JS:     return false;
        // JS: }
        // Simplified type-based immunity check (partial implementation)
        let immune = match status {
            "brn" => !self.has_type("Fire"),
            "par" => !self.has_type("Electric"),
            "psn" | "tox" => !self.has_type("Poison") && !self.has_type("Steel"),
            "frz" => !self.has_type("Ice"),
            "slp" => true, // No type immunity to sleep
            _ => true,
        };

        if !immune {
            return false;
        }

        // JS: const immunity = this.battle.runEvent('Immunity', this, null, null, type);
        // JS: if (!immunity) { ... return false; }
        // Note: runEvent('Immunity') not called - would need Battle reference

        true
    }
}
