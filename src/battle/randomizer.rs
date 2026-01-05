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
        let roll = self.random(16);
        let multiplier = 100 - roll;
        let product = base_damage as f64 * multiplier as f64;
        let inner = self.trunc(product, None);
        let division = inner as f64 / 100.0;
        let result = self.trunc(division, None) as i32;

        eprintln!("[RANDOMIZER] turn={}, base_damage={}, roll={}, multiplier={}, product={}, inner={}, division={}, result={}",
            self.turn, base_damage, roll, multiplier, product, inner, division, result);

        result
    }
}
