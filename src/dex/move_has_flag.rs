use crate::*;

impl Dex {

    /// Check if a move has a specific flag
    pub fn move_has_flag(&self, move_name: &str, flag: &str) -> bool {
        if let Some(move_data) = self.get_move(move_name) {
            move_data.flags.contains_key(flag)
        } else {
            false
        }
    }
}
