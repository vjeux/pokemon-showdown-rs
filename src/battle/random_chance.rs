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
        eprintln!("[RANDOM_CHANCE] Called with {}/{}", numerator, denominator);
        eprintln!("[RANDOM_CHANCE] Call #{}", {
            static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        });
        if let Some(forced) = self.force_random_chance {
            return forced;
        }
        self.prng.random_chance(numerator, denominator)
    }
}
