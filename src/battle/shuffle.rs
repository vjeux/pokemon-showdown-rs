use crate::*;

impl Battle {

    /// Shuffle a slice in place
    /// JavaScript calls this.prng.shuffle() directly (no Battle wrapper method)
    /// This is a Rust convenience wrapper following the pattern of sample/random/random_chance
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        self.prng.shuffle(items);
    }
}
