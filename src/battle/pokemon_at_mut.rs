use crate::*;

impl Battle {

    /// Get a Pokemon reference by indices (mutable)
    /// Helper method to reduce boilerplate when accessing Pokemon by (side_idx, poke_idx)
    #[inline]
    pub fn pokemon_at_mut(&mut self, side_idx: usize, poke_idx: usize) -> Option<&mut Pokemon> {
        self.sides.get_mut(side_idx)?.pokemon.get_mut(poke_idx)
    }
}
