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
    /// Refactored to associated function for Battle access (Session 24 Part 49)
    pub fn eat_item(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _is_forced: bool,
        _source_pos: Option<(usize, usize)>,
        _source_effect: Option<&ID>,
    ) -> Option<ID> {
        // Phase 1: Extract pokemon data to check conditions
        let (item_id, hp, is_active) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };

            // JS: if (!this.item) return false;
            if pokemon.item.is_empty() {
                return None;
            }

            (pokemon.item.clone(), pokemon.hp, pokemon.is_active)
        };

        // JS: if ((!this.hp && this.item !== 'jabocaberry' && this.item !== 'rowapberry') || !this.isActive) return false;
        // ✅ NOW IMPLEMENTED: HP check with Jaboca/Rowap Berry exception
        if hp == 0
            && item_id != ID::from("jabocaberry")
            && item_id != ID::from("rowapberry") {
            return None;
        }
        // ✅ NOW IMPLEMENTED: isActive check
        if !is_active {
            return None;
        }

        // JS: if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
        // JS: if (!source && this.battle.event?.target) source = this.battle.event.target;
        // Note: battle.event source/sourceEffect defaulting still missing (needs event system infrastructure)

        // JS: const item = this.getItem();
        // JS: if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
        // JS:     return false;
        // JS: }
        // Note: Missing sourceEffect item type check (needs event system infrastructure)

        // JS: if (
        // JS:     this.battle.runEvent('UseItem', this, null, null, item) &&
        // JS:     (force || this.battle.runEvent('TryEatItem', this, null, null, item))
        // JS: ) { ... }
        // Note: Missing runEvent('UseItem') and runEvent('TryEatItem') (needs event system infrastructure)

        // JS: this.battle.add('-enditem', this, item, '[eat]');
        // Note: Missing battle.add message (needs event system infrastructure)

        // JS: this.battle.singleEvent('Eat', item, this.itemState, this, source, sourceEffect);
        // JS: this.battle.runEvent('EatItem', this, source, sourceEffect, item);
        // Note: Missing singleEvent('Eat') and runEvent('EatItem') (needs event system infrastructure)

        // JS: if (RESTORATIVE_BERRIES.has(item.id)) {
        // JS:     switch (this.pendingStaleness) {
        // JS:     case 'internal':
        // JS:         if (this.staleness !== 'external') this.staleness = 'internal';
        // JS:         break;
        // JS:     case 'external':
        // JS:         this.staleness = 'external';
        // JS:         break;
        // JS:     }
        // JS:     this.pendingStaleness = undefined;
        // JS: }
        // Note: Missing RESTORATIVE_BERRIES staleness logic (needs constant and staleness field checks)

        // JS: this.lastItem = this.item;
        // JS: this.item = '';
        // JS: this.battle.clearEffectState(this.itemState);
        // JS: this.usedItemThisTurn = true;
        // JS: this.ateBerry = true;
        // ✅ NOW IMPLEMENTED: lastItem, usedItemThisTurn, ateBerry tracking

        // Call Pokemon::use_item() which handles lastItem and usedItemThisTurn
        // ✅ NOW IMPLEMENTED (Session 24 Part 49): Call refactored associated function
        let result = Pokemon::use_item(battle, pokemon_pos, _source_pos, _source_effect);

        // Additionally set ateBerry = true (specific to eating)
        if result.is_some() {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return result,
            };
            pokemon_mut.ate_berry = true;
        }

        // JS: this.battle.runEvent('AfterUseItem', this, null, null, item);
        // Note: Missing runEvent('AfterUseItem') (needs event system infrastructure)

        result
    }
}
