use crate::*;

impl Battle {

    /// Get move priority (-7 to +5)
    pub fn get_move_priority(&self, move_id: &ID) -> i8 {
        // Use move data from MoveDef
        if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            move_def.priority
        } else {
            0 // Default priority for unknown moves
        }
    }
}
