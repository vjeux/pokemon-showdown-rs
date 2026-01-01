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
        let call_before = self.prng.call_count;
        let result = self.prng.random_chance(numerator, denominator);
        let call_after = self.prng.call_count;

        // Log calls around divergence point to track what's calling them
        if call_after >= 137 && call_after <= 150 {
            eprintln!("[Battle::random_chance] Call #{} (was #{}): random_chance({}/{}) = {}, turn={}",
                call_after, call_before, numerator, denominator, result, self.turn);
        }

        if let Some(forced) = self.force_random_chance {
            return forced;
        }
        result
    }
}
