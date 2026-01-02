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
        // JS: if (!this.item) return false;
        if self.item.is_empty() {
            return None;
        }

        // JS: if (!source) source = this;
        // Note: Missing source parameter

        // JS: if (this.battle.gen <= 4) {
        // JS:     if (source.itemKnockedOff) return false;
        // JS:     if (toID(this.ability) === 'multitype') return false;
        // JS:     if (toID(source.ability) === 'multitype') return false;
        // JS: }
        // Note: Missing Gen 4 and earlier Multitype/itemKnockedOff checks
        // Would need Battle reference for gen check and source parameter

        // JS: const item = this.getItem();
        let item = self.item.clone();

        // JS: if (this.battle.runEvent('TakeItem', this, source, null, item)) { ... }
        // Note: Missing runEvent('TakeItem') - would need Battle reference
        // Currently always succeeds

        // JS: this.item = '';
        self.item = ID::empty();

        // JS: const oldItemState = this.itemState;
        // JS: this.battle.clearEffectState(this.itemState);
        // Note: Not storing oldItemState or calling clearEffectState

        // JS: this.pendingStaleness = undefined;
        // Note: Missing pendingStaleness reset - field doesn't exist in Rust

        self.item_state = EffectState::new(ID::empty());

        // JS: this.battle.singleEvent('End', item, oldItemState, this);
        // Note: Missing singleEvent('End') - would need Battle reference

        // JS: this.battle.runEvent('AfterTakeItem', this, null, null, item);
        // Note: Missing runEvent('AfterTakeItem') - would need Battle reference

        // JS: return item;
        Some(item)
    }
}
