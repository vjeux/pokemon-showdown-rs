use crate::*;

impl Battle {

    /// Chain two modifiers together
    /// Equivalent to battle.ts chain(previousMod, nextMod)
    ///
    //
    // 	chain(previousMod: number | number[], nextMod: number | number[]) {
    // 		// previousMod or nextMod can be either a number or an array [numerator, denominator]
    // 		if (Array.isArray(previousMod)) {
    // 			previousMod = this.trunc(previousMod[0] * 4096 / previousMod[1]);
    // 		} else {
    // 			previousMod = this.trunc(previousMod * 4096);
    // 		}
    //
    // 		if (Array.isArray(nextMod)) {
    // 			nextMod = this.trunc(nextMod[0] * 4096 / nextMod[1]);
    // 		} else {
    // 			nextMod = this.trunc(nextMod * 4096);
    // 		}
    // 		return ((previousMod * nextMod + 2048) >> 12) / 4096; // M'' = ((M * M') + 0x800) >> 12
    // 	}
    //
    pub fn chain(&self, previous_mod: (i32, i32), next_mod: (i32, i32)) -> f64 {
        // JS: previousMod = this.trunc(previousMod[0] * 4096 / previousMod[1]);
        let prev = self.trunc((previous_mod.0 * 4096) as f64 / previous_mod.1 as f64, None) as i32;
        // JS: nextMod = this.trunc(nextMod[0] * 4096 / nextMod[1]);
        let next = self.trunc((next_mod.0 * 4096) as f64 / next_mod.1 as f64, None) as i32;
        // JS: return ((previousMod * nextMod + 2048) >> 12) / 4096;
        let result = ((prev * next) + 2048) >> 12;
        result as f64 / 4096.0
    }
}
