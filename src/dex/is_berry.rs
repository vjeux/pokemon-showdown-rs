use crate::*;

impl Dex {

    // =========================================================================
    // Item-specific methods (from dex-items.ts)
    // =========================================================================

    /// Check if an item is a berry
    pub fn is_berry(&self, item_name: &str) -> bool {
        let id = ID::new(item_name);
        id.as_str().ends_with("berry")
    }
}
