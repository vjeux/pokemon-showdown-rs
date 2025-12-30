use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Set item
    //
    // 	setItem(item: string | Item, source?: Pokemon, effect?: Effect) {
    // 		if (!this.hp || !this.isActive) return false;
    // 		if (typeof item === 'string') item = this.battle.dex.items.get(item);
    //
    // 		const effectid = this.battle.effect ? this.battle.effect.id : '';
    // 		if (RESTORATIVE_BERRIES.has('leppaberry' as ID)) {
    // 			const inflicted = ['trick', 'switcheroo'].includes(effectid);
    // 			const external = inflicted && source && !source.isAlly(this);
    // 			this.pendingStaleness = external ? 'external' : 'internal';
    // 		} else {
    // 			this.pendingStaleness = undefined;
    // 		}
    // 		const oldItem = this.getItem();
    // 		const oldItemState = this.itemState;
    // 		this.item = item.id;
    // 		this.itemState = this.battle.initEffectState({ id: item.id, target: this });
    // 		if (oldItem.exists) this.battle.singleEvent('End', oldItem, oldItemState, this);
    // 		if (item.id) {
    // 			this.battle.singleEvent('Start', item, this.itemState, this, source, effect);
    // 		}
    // 		return true;
    // 	}
    //
    pub fn set_item(&mut self, item_id: ID) -> bool {
        if self.hp == 0 || !self.is_active {
            return false;
        }
        self.item = item_id.clone();
        self.item_state = EffectState::new(item_id);
        true
    }
}
