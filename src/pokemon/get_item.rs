use crate::*;

impl Pokemon {

    /// Get item ID
    //
    // 	getItem() {
    // 		return this.battle.dex.items.getByID(this.item);
    // 	}
    //
    pub fn get_item(&self) -> &ID {
        &self.item
    }
}
