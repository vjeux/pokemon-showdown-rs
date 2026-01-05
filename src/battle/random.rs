use crate::*;

impl Battle {

    /// Random number generator
    /// Matches TypeScript Battle.random(m?, n?)
    //
    // TypeScript source:
    // 	random(m?: number, n?: number) {
    // 		return this.prng.random(m, n);
    // 	}
    //
    // JavaScript: random(from, to)
    // - random() returns a real number in [0, 1)
    // - random(n) returns an integer in [0, n)
    // - random(m, n) returns an integer in [m, n)
    //
    // Rust version supports the two most common cases:
    // - random(n) returns an integer in [0, n)
    // - random_with_range(m, n) returns an integer in [m, n)
    pub fn random(&mut self, n: i32) -> i32 {
        let before = self.prng.call_count;
        let result = self.prng.random_int(n);
        let after = self.prng.call_count;
        eprintln!("[RANDOM] turn={}, n={}, result={}, PRNG: {}->{}",  self.turn, n, result, before, after);
        result
    }

    /// Random number in range [from, to)
    /// Equivalent to TypeScript random(from, to)
    pub fn random_with_range(&mut self, from: i32, to: i32) -> i32 {
        let before = self.prng.call_count;
        let result = self.prng.random_range(from, to);
        let after = self.prng.call_count;
        eprintln!("[RANDOM_WITH_RANGE] turn={}, from={}, to={}, result={}, PRNG: {}->{}",
                  self.turn, from, to, result, before, after);
        result
    }
}
