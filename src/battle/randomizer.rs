use crate::*;

impl Battle {

    // =========================================================================
    // ADDITIONAL METHODS (ported from battle.ts)
    // =========================================================================

    /// Apply damage randomization (85%-100%)
    /// Equivalent to battle.ts randomizer(baseDamage)
    ///
    //
    // 	randomizer(baseDamage: number) {
    // 		const tr = this.trunc;
    // 		return tr(tr(baseDamage * (100 - this.random(16))) / 100);
    // 	}
    //
    pub fn randomizer(&mut self, base_damage: i32) -> i32 {
        // JS: return tr(tr(baseDamage * (100 - this.random(16))) / 100);
        // Damage = baseDamage * (100 - random(16)) / 100
        // This gives range 85% to 100% damage
        let roll = 100 - self.random(16);  // Use self.random() instead of self.prng.random_int()
        let inner = self.trunc(base_damage as f64 * roll as f64, None);
        self.trunc(inner as f64 / 100.0, None) as i32
    }
}
