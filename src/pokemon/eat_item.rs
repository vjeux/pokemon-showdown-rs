use crate::*;

impl Pokemon {

    /// Eat held item (berries)
    /// Equivalent to eatItem in pokemon.ts
    //
    // 	eatItem(force?: boolean, source?: Pokemon, sourceEffect?: Effect) {
    // 		if (!this.item) return false;
    // 		if ((!this.hp && this.item !== 'jabocaberry' && this.item !== 'rowapberry') || !this.isActive) return false;
    //
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		const item = this.getItem();
    // 		if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
    // 			// if an item is telling us to eat it but we aren't holding it, we probably shouldn't eat what we are holding
    // 			return false;
    // 		}
    // 		if (
    // 			this.battle.runEvent('UseItem', this, null, null, item) &&
    // 			(force || this.battle.runEvent('TryEatItem', this, null, null, item))
    // 		) {
    // 			this.battle.add('-enditem', this, item, '[eat]');
    //
    // 			this.battle.singleEvent('Eat', item, this.itemState, this, source, sourceEffect);
    // 			this.battle.runEvent('EatItem', this, source, sourceEffect, item);
    //
    // 			if (RESTORATIVE_BERRIES.has(item.id)) {
    // 				switch (this.pendingStaleness) {
    // 				case 'internal':
    // 					if (this.staleness !== 'external') this.staleness = 'internal';
    // 					break;
    // 				case 'external':
    // 					this.staleness = 'external';
    // 					break;
    // 				}
    // 				this.pendingStaleness = undefined;
    // 			}
    //
    // 			this.lastItem = this.item;
    // 			this.item = '';
    // 			this.battle.clearEffectState(this.itemState);
    // 			this.usedItemThisTurn = true;
    // 			this.ateBerry = true;
    // 			this.battle.runEvent('AfterUseItem', this, null, null, item);
    // 			return true;
    // 		}
    // 		return false;
    // 	}
    //
    pub fn eat_item(&mut self, _is_forced: bool) -> Option<ID> {
        if self.item.is_empty() {
            return None;
        }

        // Would check if item is edible (berry)
        // For now, same as use_item
        self.use_item()
    }
}
