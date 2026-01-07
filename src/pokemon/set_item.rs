use crate::*;
use crate::event_system::EffectState;

/// Check if an item is a restorative berry
/// JavaScript: RESTORATIVE_BERRIES.has(item.id)
fn is_restorative_berry(item_id: &str) -> bool {
    matches!(
        item_id,
        "leppaberry"
            | "aguavberry"
            | "enigmaberry"
            | "figyberry"
            | "iapapaberry"
            | "magoberry"
            | "sitrusberry"
            | "wikiberry"
            | "oranberry"
    )
}

impl Pokemon {

    /// Set item
    /// Refactored to associated function for Battle access (Session 24 Part 53)
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
    pub fn set_item(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        item_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&ID>,
    ) -> bool {
        // Phase 1: Extract pokemon data to check conditions
        let (hp, is_active, pokemon_side_idx) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };

            // JS: if (!this.hp || !this.isActive) return false;
            (pokemon.hp, pokemon.is_active, pokemon.side_index)
        };

        if hp == 0 || !is_active {
            return false;
        }

        // JS: if (typeof item === 'string') item = this.battle.dex.items.get(item);
        // Note: In Rust we receive ID directly

        // JS: const effectid = this.battle.effect ? this.battle.effect.id : '';
        // JS: if (RESTORATIVE_BERRIES.has('leppaberry' as ID)) {
        // JS:     const inflicted = ['trick', 'switcheroo'].includes(effectid);
        // JS:     const external = inflicted && source && !source.isAlly(this);
        // JS:     this.pendingStaleness = external ? 'external' : 'internal';
        // JS: } else {
        // JS:     this.pendingStaleness = undefined;
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 53): RESTORATIVE_BERRIES staleness logic
        if is_restorative_berry(item_id.as_str()) {
            // Check if effect is 'trick' or 'switcheroo'
            let is_inflicted = source_effect
                .map(|e| e.as_str() == "trick" || e.as_str() == "switcheroo")
                .unwrap_or(false);

            // Check if source exists and is not an ally
            let is_external = if is_inflicted {
                if let Some(src_pos) = source_pos {
                    // Check if source is not an ally (different side)
                    src_pos.0 != pokemon_side_idx
                } else {
                    false
                }
            } else {
                false
            };

            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };

            pokemon_mut.pending_staleness = if is_external {
                Some("external".to_string())
            } else {
                Some("internal".to_string())
            };
        } else {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            pokemon_mut.pending_staleness = None;
        }

        // JS: const oldItem = this.getItem();
        // JS: const oldItemState = this.itemState;
        // ✅ NOW IMPLEMENTED (Session 24 Part 74): Store old item for singleEvent('End')
        // Note: old_item_state extracted but not used - Rust event system handles state internally
        let (old_item_id, _old_item_state) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            (pokemon.item.clone(), pokemon.item_state.clone())
        };

        // Phase 2: Mutate pokemon to set new item
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: this.item = item.id;
        // JS: this.itemState = this.battle.initEffectState({ id: item.id, target: this });
        pokemon_mut.item = item_id.clone();
        pokemon_mut.item_state = EffectState::new(item_id.clone());
        pokemon_mut.item_state.target = Some((pokemon_pos.0, pokemon_pos.1));

        // JS: if (oldItem.exists) this.battle.singleEvent('End', oldItem, oldItemState, this);
        // ✅ NOW IMPLEMENTED (Session 24 Part 74): singleEvent('End') for old item
        if !old_item_id.as_str().is_empty() {
            battle.single_event("End", &crate::battle::Effect::item(old_item_id.clone()), Some(pokemon_pos), None, None, None);
        }

        // JS: if (item.id) {
        // JS:     this.battle.singleEvent('Start', item, this.itemState, this, source, effect);
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 74): singleEvent('Start') for new item
        if !item_id.as_str().is_empty() {
            battle.single_event("Start", &crate::battle::Effect::item(item_id.clone()), Some(pokemon_pos), source_pos, source_effect, None);
        }

        // JS: return true;
        true
    }
}
