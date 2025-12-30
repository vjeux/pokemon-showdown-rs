use crate::*;
use crate::dex::ItemData;

impl Dex {

    /// Get item by ID (equivalent to DexItems.getByID)
    pub fn get_item_by_id(&self, id: &ID) -> Option<&ItemData> {
        self.items.get(id)
    }
}
