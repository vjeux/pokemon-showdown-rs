use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Clear item
    //
    // 	clearItem() {
    // 		return this.setItem('');
    // 	}
    //
    pub fn clear_item(&mut self) -> ID {
        let old = self.item.clone();
        self.item = ID::empty();
        self.item_state = EffectState::new(ID::empty());
        old
    }
}
