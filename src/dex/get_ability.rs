use crate::*;
use crate::dex::AbilityData;

impl Dex {

    /// Get ability data by name or ID
    pub fn get_ability(&self, name: &str) -> Option<&AbilityData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(ability) = self.abilities.get(&id) {
            return Some(ability);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.abilities.get(&canonical_id);
        }
        None
    }
}
