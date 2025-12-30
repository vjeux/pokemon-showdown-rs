use crate::*;

impl Dex {

    // =========================================================================
    // Move-specific methods (from dex-moves.ts)
    // =========================================================================

    /// Check if a move is a status move
    pub fn is_status_move(&self, move_name: &str) -> bool {
        self.moves().get(move_name)
            .map(|m| m.category == "Status")
            .unwrap_or(false)
    }
}
