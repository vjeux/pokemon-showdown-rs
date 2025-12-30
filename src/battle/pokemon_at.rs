use crate::*;

impl Battle {

    // ========================================================================
    // Pokemon Iterator Helpers
    // ========================================================================
    // These methods provide a cleaner API for iterating over Pokemon without
    // exposing the internal (side_idx, poke_idx) representation.

    /// Get a Pokemon reference by indices (immutable)
    /// Helper method to reduce boilerplate when accessing Pokemon by (side_idx, poke_idx)
    #[inline]
    pub fn pokemon_at(&self, side_idx: usize, poke_idx: usize) -> Option<&Pokemon> {
        self.sides.get(side_idx)?.pokemon.get(poke_idx)
    }
}
