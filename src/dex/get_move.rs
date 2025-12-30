use crate::*;
use crate::dex::MoveData;

impl Dex {

    /// Get move data by name or ID
    pub fn get_move(&self, name: &str) -> Option<&MoveData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(move_data) = self.moves.get(&id) {
            return Some(move_data);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.moves.get(&canonical_id);
        }
        None
    }
}
