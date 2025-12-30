use crate::*;
use crate::dex::ItemData;

impl Dex {

    /// Get item data by name or ID
    pub fn get_item(&self, name: &str) -> Option<&ItemData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(item) = self.items.get(&id) {
            return Some(item);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.items.get(&canonical_id);
        }
        None
    }
}
