use crate::*;

impl Battle {

    /// Random chance
    //
    // 	randomChance(numerator: number, denominator: number) {
    // 		if (this.forceRandomChance !== null) return this.forceRandomChance;
    // 		return this.prng.randomChance(numerator, denominator);
    // 	}
    //
    pub fn random_chance(&mut self, numerator: i32, denominator: i32) -> bool {
        // JavaScript: if (this.forceRandomChance !== null) return this.forceRandomChance;
        // Check forced value FIRST before making PRNG call
        if let Some(forced) = self.force_random_chance {
            debug_elog!("[RANDOM_CHANCE] turn={}, numerator={}, denominator={}, FORCED={}", self.turn, numerator, denominator, forced);
            return forced;
        }

        // JavaScript: return this.prng.randomChance(numerator, denominator);
        let result = self.prng.random_chance(numerator, denominator);
        debug_elog!("[RANDOM_CHANCE] turn={}, numerator={}, denominator={}, result={}, PRNG={}", self.turn, numerator, denominator, result, self.prng.call_count);
        result
    }
}
