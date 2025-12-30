use crate::*;

impl Battle {
    #[allow(dead_code)]
    /// Get move accuracy (0-100, where 100+ means never miss)
    pub fn get_move_accuracy(&self, move_id: &ID) -> i32 {
        // Use move data from MoveData
        if let Some(move_def) = self.dex.moves().get(move_id.as_str()) {
            match move_def.accuracy {
                crate::dex::Accuracy::Percent(p) => p,
                crate::dex::Accuracy::AlwaysHits => 101, // > 100 means always hits
            }
        } else {
            100 // Default accuracy for unknown moves
        }
    }
}
