use crate::*;

impl Battle {

    pub fn random_range(&mut self, from: i32, to: i32) -> i32 {
        self.prng.random_range(from, to)
    }
}
