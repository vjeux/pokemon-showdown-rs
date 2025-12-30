use crate::*;

impl Battle {

    /// Random number in [0, n)
    //
    // 	random(m?: number, n?: number) {
    // 		return this.prng.random(m, n);
    // 	}
    //
    // JavaScript: random(from, to)
    // - random() returns a real number in [0, 1)
    // - random(n) returns an integer in [0, n)
    // - random(m, n) returns an integer in [m, n)
    pub fn random(&mut self, n: i32) -> i32 {
        self.prng.random_int(n)
    }
}
