use crate::*;

impl Battle {

    pub fn random_range(&mut self, from: i32, to: i32) -> i32 {
        let result = self.prng.random_range(from, to);
        eprintln!("PRNG [random]: Battle.random_range({}, {}) = {}", from, to, result);
        result
    }
}
