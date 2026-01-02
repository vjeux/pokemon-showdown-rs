use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Use held item
    /// Equivalent to useItem in pokemon.ts
    //
    // 	useItem(source?: Pokemon, sourceEffect?: Effect) {
    // 		if ((!this.hp && !this.getItem().isGem) || !this.isActive) return false;
    // 		if (!this.item) return false;
    //
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		const item = this.getItem();
    // 		if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
    // 			// if an item is telling us to eat it but we aren't holding it, we probably shouldn't eat what we are holding
    // 			return false;
    // 		}
    // 		if (this.battle.runEvent('UseItem', this, null, null, item)) {
    // 			switch (item.id) {
    // 			case 'redcard':
    // 				this.battle.add('-enditem', this, item, `[of] ${source}`);
    // 				break;
    // 			default:
    // 				if (item.isGem) {
    // 					this.battle.add('-enditem', this, item, '[from] gem');
    // 				} else {
    // 					this.battle.add('-enditem', this, item);
    // 				}
    // 				break;
    // 			}
    // 			if (item.boosts) {
    // 				this.battle.boost(item.boosts, this, source, item);
    // 			}
    //
    // 			this.battle.singleEvent('Use', item, this.itemState, this, source, sourceEffect);
    //
    // 			this.lastItem = this.item;
    // 			this.item = '';
    // 			this.battle.clearEffectState(this.itemState);
    // 			this.usedItemThisTurn = true;
    // 			this.battle.runEvent('AfterUseItem', this, null, null, item);
    // 			return true;
    // 		}
    // 		return false;
    // 	}
    //
    pub fn use_item(&mut self) -> Option<ID> {
        // TODO: implement the same logic as JavaScript

        if self.item.is_empty() {
            return None;
        }
        let item = self.item.clone();
        self.used_item_this_turn = true;
        self.last_item = item.clone();
        self.item = ID::empty();
        self.item_state = EffectState::new(ID::empty());
        Some(item)
    }
}
