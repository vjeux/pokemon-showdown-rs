use crate::*;

impl Pokemon {

    /// Clear item
    //
    // 	clearItem() {
    // 		return this.setItem('');
    // 	}
    //
    pub fn clear_item(&mut self) -> bool {
        // JS: return this.setItem('');
        self.set_item(ID::empty())
    }
}
