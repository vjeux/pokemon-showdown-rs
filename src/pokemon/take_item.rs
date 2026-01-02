use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Take item (remove and return it)
    ///
    /// This is an associated function (not a method) because it needs
    /// access to Battle for the gen check and source Pokemon.
    /// Call as: Pokemon::take_item(battle, pokemon_pos, source_pos)
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
    pub fn take_item(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> Option<ID> {
        // Phase 1: Check if item exists and get gen/ability info
        let (item, pokemon_ability, gen) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };

            // JS: if (!this.item) return false;
            if pokemon.item.is_empty() {
                return None;
            }

            (pokemon.item.clone(), pokemon.ability.clone(), battle.gen)
        };

        // JS: if (!source) source = this;
        // ✅ NOW IMPLEMENTED: source parameter (defaults to self if None)
        let source_pos = source_pos.unwrap_or(pokemon_pos);

        // JS: if (this.battle.gen <= 4) {
        // JS:     if (source.itemKnockedOff) return false;
        // JS:     if (toID(this.ability) === 'multitype') return false;
        // JS:     if (toID(source.ability) === 'multitype') return false;
        // JS: }
        // ✅ NOW IMPLEMENTED: Gen 4 and earlier Multitype/itemKnockedOff checks
        if gen <= 4 {
            // Check source's itemKnockedOff flag
            let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return None,
            };

            if source.item_knocked_off {
                return None; // false in JS
            }

            // Check if target has Multitype ability (Arceus)
            if pokemon_ability.as_str() == "multitype" {
                return None; // false in JS
            }

            // Check if source has Multitype ability (Arceus)
            if source.ability.as_str() == "multitype" {
                return None; // false in JS
            }
        }

        // JS: const item = this.getItem();
        // Already have item from Phase 1

        // JS: if (this.battle.runEvent('TakeItem', this, source, null, item)) { ... }
        // Note: Missing runEvent('TakeItem') - would need event system
        // Currently always succeeds

        // JS: const oldItemState = this.itemState;
        // ✅ NOW IMPLEMENTED (Session 24 Part 75): Store oldItemState for singleEvent('End')
        // Note: old_item_state extracted but not used - Rust event system handles state internally
        let _old_item_state = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };
            pokemon.item_state.clone()
        };

        // Phase 2: Remove item mutably
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return None,
        };

        // JS: this.item = '';
        pokemon_mut.item = ID::empty();

        // JS: this.battle.clearEffectState(this.itemState);
        // Note: Not implementing clearEffectState - resetting item_state instead

        // JS: this.pendingStaleness = undefined;
        // ✅ NOW IMPLEMENTED: pendingStaleness reset (field exists in Rust)
        pokemon_mut.pending_staleness = None;

        pokemon_mut.item_state = EffectState::new(ID::empty());

        // JS: this.battle.singleEvent('End', item, oldItemState, this);
        // ✅ NOW IMPLEMENTED (Session 24 Part 75): singleEvent('End') for removed item
        battle.single_event("End", &item, Some(pokemon_pos), None, None);

        // JS: this.battle.runEvent('AfterTakeItem', this, null, null, item);
        // Note: Missing runEvent('AfterTakeItem') - would need event system

        // JS: return item;
        Some(item)
    }
}
