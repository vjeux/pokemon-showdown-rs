use crate::*;
use crate::event::EventResult;
use crate::event_system::EffectState;
use crate::battle::Effect;

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
        _source_effect: Option<&Effect>,
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
        // ✅ NOW IMPLEMENTED (Session 24 Part 83): runEvent('UseItem') and runEvent('TryEatItem')
        // Note: JavaScript passes item as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check pokemon's item field
        let use_item_result = battle.run_event("UseItem", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);
        if matches!(use_item_result, EventResult::Number(0)) || matches!(use_item_result, EventResult::Null) {
            return None; // false in JavaScript
        }

        // Check TryEatItem unless forced
        if !_is_forced {
            let try_eat_result = battle.run_event("TryEatItem", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);
            if matches!(try_eat_result, EventResult::Number(0)) || matches!(try_eat_result, EventResult::Null) {
                return None; // false in JavaScript
            }
        }

        // JS: this.battle.add('-enditem', this, item, '[eat]');
        // ✅ NOW IMPLEMENTED (Session 24 Part 51): battle.add message for eating items
        // Prepare message arguments (extract data, then drop borrows before battle.add call)
        let message_args: Vec<Arg> = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };
            let pokemon_str = format!("{}", pokemon);
            let item_str = item_id.to_string();
            vec![Arg::String(pokemon_str), Arg::String(item_str), Arg::String("[eat]".to_string())]
        };
        // All borrows dropped - now safe to call battle.add
        battle.add("-enditem", &message_args);

        // JS: this.battle.singleEvent('Eat', item, this.itemState, this, source, sourceEffect);
        // JS: this.battle.runEvent('EatItem', this, source, sourceEffect, item);
        // ✅ NOW IMPLEMENTED (Session 24 Part 83): singleEvent('Eat') and runEvent('EatItem')
        battle.single_event("Eat", &crate::battle::Effect::item(item_id.clone()), Some(pokemon_pos), _source_pos, None, None);
        battle.run_event("EatItem", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), _source_pos, Some(&crate::battle::Effect::item(item_id.clone())), EventResult::Continue, false, false);

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
        // ✅ NOW IMPLEMENTED (Session 24 Part 52): RESTORATIVE_BERRIES staleness logic
        if is_restorative_berry(item_id.as_str()) {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };

            match pokemon_mut.pending_staleness.as_deref() {
                Some("internal") => {
                    // Only set to internal if not already external
                    if pokemon_mut.staleness.as_deref() != Some("external") {
                        pokemon_mut.staleness = Some("internal".to_string());
                    }
                }
                Some("external") => {
                    pokemon_mut.staleness = Some("external".to_string());
                }
                _ => {
                    // No pending staleness or other value - do nothing
                }
            }
            pokemon_mut.pending_staleness = None;
        }

        // Phase 2: Mutate pokemon to consume item
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return None,
        };

        // JS: this.lastItem = this.item;
        pokemon_mut.last_item = item_id.clone();

        // JS: this.item = '';
        pokemon_mut.item = ID::empty();

        // JS: this.battle.clearEffectState(this.itemState);
        pokemon_mut.item_state = EffectState::new(ID::empty());

        // JS: this.usedItemThisTurn = true;
        pokemon_mut.used_item_this_turn = true;

        // JS: this.ateBerry = true;
        // ✅ NOW IMPLEMENTED: ateBerry tracking (specific to eat_item)
        pokemon_mut.ate_berry = true;

        // JS: this.battle.runEvent('AfterUseItem', this, null, null, item);
        // ✅ NOW IMPLEMENTED (Session 24 Part 83): runEvent('AfterUseItem')
        // Note: JavaScript passes item as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check pokemon's item field which is now empty
        battle.run_event("AfterUseItem", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);

        // JS: return true;
        Some(item_id)
    }
}
