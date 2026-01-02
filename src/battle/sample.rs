// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Sample from a slice
    //
    // 	sample<T>(items: readonly T[]): T {
    // 		return this.prng.sample(items);
    // 	}
    //
    pub fn sample<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        let call_num_before = self.prng.call_count;

        let result = self.prng.sample(items);

        let call_num_after = self.prng.call_count;

        // Log sample calls on turns 19-22 to debug divergence
        if self.turn >= 19 && self.turn <= 22 {
            eprintln!("[TURN {} PRNG #{}] sample(len={}) (was #{})",
                self.turn, call_num_after, items.len(), call_num_before);
        }

        result
    }
}
