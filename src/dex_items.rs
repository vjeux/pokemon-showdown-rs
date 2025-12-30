//! DexItems - Items lookup helper
//!
//! Equivalent to DexItems class in dex-items.ts

use crate::dex::{Dex, ItemData};
use crate::dex_data::ID;

/// Helper struct for items lookups
/// Equivalent to DexItems class in TypeScript
pub struct DexItems<'a> {
    pub(crate) dex: &'a Dex,
}

impl<'a> DexItems<'a> {
    /// Get item data by name or ID
    /// Equivalent to DexItems.get() in dex-items.ts
    pub fn get(&self, name: &str) -> Option<&'a ItemData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(item) = self.dex.items.get(&id) {
            return Some(item);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.dex.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.dex.items.get(&canonical_id);
        }
        None
    }

    /// Get item by ID
    /// Equivalent to DexItems.getByID() in dex-items.ts
    pub fn get_by_id(&self, id: &ID) -> Option<&'a ItemData> {
        self.dex.items.get(id)
    }

    /// Get all items data
    /// Equivalent to DexItems.all() in dex-items.ts
    pub fn all(&self) -> Vec<&'a ItemData> {
        self.dex.items.values().collect()
    }
}
