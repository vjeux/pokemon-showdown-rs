use crate::*;

impl Battle {

    /// Sample from a slice
    //
    // 	sample<T>(items: readonly T[]): T {
    // 		return this.prng.sample(items);
    // 	}
    //
    pub fn sample<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        self.prng.sample(items)
    }
}
