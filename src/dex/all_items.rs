use crate::*;
use crate::dex::ItemData;

impl Dex {

    /// Get all items data
    /// Equivalent to DexItems.all() in dex-items.ts
    pub fn all_items(&self) -> Vec<&ItemData> {
        self.items.values().collect()
    }
}
