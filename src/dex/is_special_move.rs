use crate::*;

impl Dex {

    /// Check if a move is a special move
    pub fn is_special_move(&self, move_name: &str) -> bool {
        self.moves().get(move_name)
            .map(|m| m.category == "Special")
            .unwrap_or(false)
    }
}
