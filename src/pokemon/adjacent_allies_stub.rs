use crate::*;

impl Pokemon {

    /// Get indices of adjacent allies (for triples)
    /// Equivalent to pokemon.ts adjacentAllies()
    pub fn adjacent_allies_stub(&self, active_per_half: usize) -> Vec<(usize, usize)> {
        // In singles/doubles, all allies are adjacent
        // In triples, only adjacent positions
        let _ = active_per_half;
        vec![]
    }
}
