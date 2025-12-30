use crate::*;

impl Battle {

    /// Reset the PRNG with a new seed
    /// Equivalent to battle.ts resetRNG()
    // TypeScript source:
    // /** Note that passing `undefined` resets to the starting seed, but `null` will roll a new seed */
    // 	resetRNG(seed: PRNGSeed | null = this.prngSeed) {
    // 		this.prng = new PRNG(seed);
    // 		this.add('message', "The battle's RNG was reset.");
    // 	}
    //
    pub fn reset_rng(&mut self, seed: Option<crate::prng::PRNGSeed>) {
        let new_seed = seed.unwrap_or_else(|| self.prng_seed.clone());
        self.prng = PRNG::new(Some(new_seed.clone()));
        self.prng_seed = new_seed;
        self.add("message", &["The battle's RNG was reset.".into()]);
    }
}
