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
    pub fn has_item(&self, battle: &Battle, items: &[&str]) -> bool {
        // JS: if (Array.isArray(item)) {
        // JS:     if (!item.map(toID).includes(this.item)) return false;
        // JS: } else {
        // JS:     if (toID(item) !== this.item) return false;
        // JS: }
        let item_id = self.item.as_str();
        let has_matching_item = items.iter().any(|&i| crate::dex_data::to_id(i) == item_id);

        if !has_matching_item {
            return false;
        }

        // JS: return !this.ignoringItem();
        !self.ignoring_item(battle, false)
    }
}
