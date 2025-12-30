use crate::*;
use crate::dex::Accuracy;

impl Dex {

    /// Check if a move always hits
    pub fn move_always_hits(&self, move_name: &str) -> bool {
        self.get_move(move_name)
            .map(|m| matches!(m.accuracy, Accuracy::AlwaysHits))
            .unwrap_or(false)
    }
}
