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
            eprintln!("[RANDOM_CHANCE] turn={}, force_random_chance={}, returning without PRNG call", self.turn, forced);
            return forced;
        }

        let prng_before = self.prng.call_count;
        // JavaScript: return this.prng.randomChance(numerator, denominator);
        let result = self.prng.random_chance(numerator, denominator);
        let prng_after = self.prng.call_count;
        eprintln!("[RANDOM_CHANCE] turn={}, randomChance({}, {}) = {}, PRNG: {}->{}",
            self.turn, numerator, denominator, result, prng_before, prng_after);
        result
    }
}
