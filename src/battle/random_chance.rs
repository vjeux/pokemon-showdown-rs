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
        if let Some(forced) = self.force_random_chance {
            eprintln!("PRNG: random_chance({}, {}) = {} [FORCED]", numerator, denominator, forced);
            return forced;
        }
        let result = self.prng.random_chance(numerator, denominator);
        eprintln!("PRNG: random_chance({}, {}) = {}", numerator, denominator, result);
        result
    }
}
