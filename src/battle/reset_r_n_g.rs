// 1:1 port of resetRNG from battle.ts
//
// JS Source:
// 	resetRNG(seed: PRNGSeed | null = this.prngSeed) {
// 		this.prng = new PRNG(seed);
// 		this.add('message', "The battle's RNG was reset.");
// 	}

use crate::*;
use crate::prng::PRNG;

impl Battle {
    /// Reset the battle's random number generator
    /// Equivalent to battle.ts resetRNG() (battle.ts:1996-1999)
    pub fn reset_rng(&mut self, seed: Option<[u32; 4]>) {
        // JS: this.prng = new PRNG(seed);
        // Use provided seed or the battle's original prngSeed
        let seed_to_use = seed.or(self.prng_seed);
        self.prng = PRNG::new(seed_to_use);

        // JS: this.add('message', "The battle's RNG was reset.");
        self.add("message", &[Arg::Str("The battle's RNG was reset.")]);
    }
}
