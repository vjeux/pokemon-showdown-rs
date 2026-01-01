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
        let call_before = self.prng.call_count;
        let result = self.prng.random_int(n);
        let call_after = self.prng.call_count;

        // Log PRNG calls around turn 21 to track what's calling them
        if call_after >= 69 && call_after <= 77 {
            eprintln!("[Battle::random] Call #{} (was #{}): random({}) = {}, turn={}",
                call_after, call_before, n, result, self.turn);
        }

        result
    }

    /// Random number in range [from, to)
    /// Equivalent to TypeScript random(from, to)
    pub fn random_with_range(&mut self, from: i32, to: i32) -> i32 {
        let result = self.prng.random_range(from, to);

        // Log PRNG calls on turns 19-22 to debug divergence
        let call_num = self.prng.call_count;
        if self.turn >= 19 && self.turn <= 22 {
            eprintln!("[TURN {} PRNG #{}] random_with_range({}, {}) = {}", self.turn, call_num, from, to, result);
        }

        result
    }
}
