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
        // JS: if ((!this.hp && !this.getItem().isGem) || !this.isActive) return false;
        // Note: Missing HP check with Gem exception
        // Note: Missing isActive check

        // JS: if (!this.item) return false;
        if self.item.is_empty() {
            return None;
        }

        // JS: if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
        // JS: if (!source && this.battle.event?.target) source = this.battle.event.target;
        // Note: Missing source and sourceEffect parameters

        // JS: const item = this.getItem();
        // JS: if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
        // JS:     return false;
        // JS: }
        // Note: Missing sourceEffect item type check

        // JS: if (this.battle.runEvent('UseItem', this, null, null, item)) {
        // Note: Missing runEvent('UseItem')

        // JS: switch (item.id) {
        // JS: case 'redcard':
        // JS:     this.battle.add('-enditem', this, item, `[of] ${source}`);
        // JS:     break;
        // JS: default:
        // JS:     if (item.isGem) {
        // JS:         this.battle.add('-enditem', this, item, '[from] gem');
        // JS:     } else {
        // JS:         this.battle.add('-enditem', this, item);
        // JS:     }
        // JS:     break;
        // JS: }
        // Note: Missing battle.add message with special cases for Red Card and Gems

        // JS: if (item.boosts) {
        // JS:     this.battle.boost(item.boosts, this, source, item);
        // JS: }
        // Note: Missing item.boosts handling (would need item data access)

        // JS: this.battle.singleEvent('Use', item, this.itemState, this, source, sourceEffect);
        // Note: Missing singleEvent('Use')

        // JS: this.lastItem = this.item;
        let item = self.item.clone();
        self.last_item = item.clone();

        // JS: this.item = '';
        self.item = ID::empty();

        // JS: this.battle.clearEffectState(this.itemState);
        self.item_state = EffectState::new(ID::empty());

        // JS: this.usedItemThisTurn = true;
        self.used_item_this_turn = true;

        // JS: this.battle.runEvent('AfterUseItem', this, null, null, item);
        // Note: Missing runEvent('AfterUseItem')

        // JS: return true;
        Some(item)
    }
}
