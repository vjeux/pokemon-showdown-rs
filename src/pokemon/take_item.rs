use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Take item (remove and return it)
    //
    // 	takeItem(source?: Pokemon) {
    // 		if (!this.item) return false;
    // 		if (!source) source = this;
    // 		if (this.battle.gen <= 4) {
    // 			if (source.itemKnockedOff) return false;
    // 			if (toID(this.ability) === 'multitype') return false;
    // 			if (toID(source.ability) === 'multitype') return false;
    // 		}
    // 		const item = this.getItem();
    // 		if (this.battle.runEvent('TakeItem', this, source, null, item)) {
    // 			this.item = '';
    // 			const oldItemState = this.itemState;
    // 			this.battle.clearEffectState(this.itemState);
    // 			this.pendingStaleness = undefined;
    // 			this.battle.singleEvent('End', item, oldItemState, this);
    // 			this.battle.runEvent('AfterTakeItem', this, null, null, item);
    // 			return item;
    // 		}
    // 		return false;
    // 	}
    //
    pub fn take_item(&mut self) -> Option<ID> {
        // TODO: implement the same logic as JavaScript
        if self.item.is_empty() {
            return None;
        }
        let item = self.item.clone();
        self.item = ID::empty();
        self.item_state = EffectState::new(ID::empty());
        Some(item)
    }
}
