use crate::*;
use crate::dex::MoveData;

impl Dex {

    /// Get all moves data
    /// Equivalent to DexMoves.all() in dex-moves.ts
    pub fn all_moves(&self) -> Vec<&MoveData> {
        self.moves.values().collect()
    }
}
