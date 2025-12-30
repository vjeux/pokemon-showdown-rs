use crate::*;
use crate::dex_moves::DexMoves;

impl Dex {
    /// Get moves helper
    /// Equivalent to accessing `dex.moves` in TypeScript
    pub fn moves(&self) -> DexMoves<'_> {
        DexMoves { dex: self }
    }
}
