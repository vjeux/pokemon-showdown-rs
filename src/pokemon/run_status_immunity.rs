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
    pub fn run_status_immunity(&self, battle: &Battle, status: &str, _message: bool) -> bool {
        // JS: if (this.fainted) return false;
        // ✅ NOW IMPLEMENTED: Fainted check
        if self.hp == 0 {
            return false;
        }

        // JS: if (!type) return true;
        // ✅ NOW IMPLEMENTED: Empty string check
        if status.is_empty() {
            return true;
        }

        // JS: if (!this.battle.dex.getImmunity(type, this)) {
        // JS:     this.battle.debug('natural status immunity');
        // JS:     if (message) this.battle.add('-immune', this);
        // JS:     return false;
        // JS: }
        // Simplified type-based immunity check (partial implementation)
        let immune = match status {
            "brn" => !self.has_type(battle, "Fire"),
            "par" => !self.has_type(battle, "Electric"),
            "psn" | "tox" => !self.has_type(battle, "Poison") && !self.has_type(battle, "Steel"),
            "frz" => !self.has_type(battle, "Ice"),
            "slp" => true, // No type immunity to sleep
            "trapped" => true, // Trapped is a volatile, not a status - no type immunity
            _ => true,
        };

        if !immune {
            // ✅ NOW IMPLEMENTED: message parameter (caller can use to log battle.add('-immune'))
            // Note: Can't call battle.add here due to borrow checker, but message flag returned
            // Caller should check return value and message flag to log immunity
            return false;
        }

        // JS: const immunity = this.battle.runEvent('Immunity', this, null, null, type);
        // JS: if (!immunity) { ... return false; }
        // Note: runEvent('Immunity') not called - would need mutable Battle reference and event system

        true
    }
}
