use crate::*;

impl Dex {

    /// Check if a move is a physical move
    pub fn is_physical_move(&self, move_name: &str) -> bool {
        self.moves().get(move_name)
            .map(|m| m.category == "Physical")
            .unwrap_or(false)
    }
}
