use crate::*;

impl Pokemon {

    /// Check if Pokemon has a specific item
    //
    // 	hasItem(item: string | string[]) {
    // 		if (Array.isArray(item)) {
    // 			if (!item.map(toID).includes(this.item)) return false;
    // 		} else {
    // 			if (toID(item) !== this.item) return false;
    // 		}
    // 		return !this.ignoringItem();
    // 	}
    //
    pub fn has_item(&self, items: &[&str]) -> bool {
        // TODO: implement the same logic as JavaScript
        // ignoringItem() is not being called.
        let item_id = self.item.as_str();
        items.iter().any(|&i| crate::dex_data::to_id(i) == item_id)
    }
}
