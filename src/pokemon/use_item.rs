use crate::*;
use crate::event::EventResult;
use crate::event_system::SharedEffectState;
use crate::battle::Effect;

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
    /// Use held item
    /// Refactored to associated function for Battle access (Session 24 Part 49)
    pub fn use_item(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos_param: Option<(usize, usize)>,
        source_effect_param: Option<&Effect>,
    ) -> Option<ID> {
        // Phase 1: Extract pokemon data immutably to check conditions
        let (hp, is_active, item_id, is_gem) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => {
                    return None;
                }
            };

            // JS: if (!this.item) return false;
            if pokemon.item.is_empty() {
                return None;
            }

            // Check if item is a Gem (needed for HP check)
            let is_gem = battle
                .dex
                .items()
                .get_by_id(&pokemon.item)
                .map(|item_data| item_data.is_gem)
                .unwrap_or(false);

            (pokemon.hp, pokemon.is_active, pokemon.item.clone(), is_gem)
        };

        // JS: if ((!this.hp && !this.getItem().isGem) || !this.isActive) return false;
        // ✅ NOW IMPLEMENTED (Session 24 Part 49): HP check with Gem exception
        if (hp == 0 && !is_gem) || !is_active {
            return None;
        }

        // JS: if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
        // JS: if (!source && this.battle.event?.target) source = this.battle.event.target;
        // ✅ NOW IMPLEMENTED: Default source/sourceEffect from battle.event/battle.effect
        let (source_pos, default_effect) = if battle.event.is_some() {
            // If source_pos is None, try to get it from battle.event.target
            // Note: use_item uses event.target, not event.source (per JavaScript)
            let event_target = battle.event.as_ref().and_then(|e| e.target);
            let resolved_source = source_pos_param.or(event_target);

            // If source_effect is None, get battle.effect as owned value
            let resolved_effect = if source_effect_param.is_none() {
                battle.effect.clone()
            } else {
                None  // We'll use the passed-in source_effect
            };
            (resolved_source, resolved_effect)
        } else {
            (source_pos_param, None)
        };

        // Use passed-in source_effect if available, otherwise use the default from battle.effect
        let source_effect_ref = source_effect_param.or(default_effect.as_ref());

        // JS: const item = this.getItem();
        // JS: if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
        // JS:     return false;
        // JS: }
        // ✅ NOW IMPLEMENTED: sourceEffect item type check
        if let Some(effect) = source_effect_ref {
            if effect.effect_type == crate::battle::EffectType::Item {
                // If an item is telling us to eat/use it but we aren't holding it,
                // and source is this pokemon, we probably shouldn't use what we are holding
                if item_id.as_str() != effect.id.as_str() {
                    // Check if source === this (source_pos matches pokemon_pos)
                    if source_pos == Some(pokemon_pos) {
                        return None;
                    }
                }
            }
        }

        // JS: if (this.battle.runEvent('UseItem', this, null, null, item)) {
        // ✅ NOW IMPLEMENTED (Session 24 Part 82): runEvent('UseItem')
        // Note: JavaScript passes item as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check pokemon's item field
        let use_item_result = battle.run_event("UseItem", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);
        // runEvent returns Option<i32>, None or Some(0) means failure
        if matches!(use_item_result, EventResult::Number(0)) || matches!(use_item_result, EventResult::Null) {
            return None; // false in JavaScript
        }

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
        // ✅ NOW IMPLEMENTED (Session 24 Part 50): battle.add messages for item consumption
        // Prepare message arguments (extract data, then drop borrows before battle.add call)
        let message_args: Vec<Arg> = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };
            let pokemon_str = format!("{}", pokemon);
            let item_str = item_id.to_string();

            if item_id.as_str() == "redcard" {
                // Red Card case: add source if available
                if let Some(src_pos) = source_pos {
                    if let Some(source) = battle.pokemon_at(src_pos.0, src_pos.1) {
                        let source_str = format!("[of] {}", source);
                        vec![Arg::String(pokemon_str), Arg::String(item_str), Arg::String(source_str)]
                    } else {
                        vec![Arg::String(pokemon_str), Arg::String(item_str)]
                    }
                } else {
                    vec![Arg::String(pokemon_str), Arg::String(item_str)]
                }
            } else if is_gem {
                // Gem case: add '[from] gem'
                vec![Arg::String(pokemon_str), Arg::String(item_str), Arg::String("[from] gem".to_string())]
            } else {
                // Default case: just pokemon and item
                vec![Arg::String(pokemon_str), Arg::String(item_str)]
            }
        };
        // All borrows dropped - now safe to call battle.add
        battle.add("-enditem", &message_args);

        // JS: if (item.boosts) {
        // JS:     this.battle.boost(item.boosts, this, source, item);
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 49): item.boosts handling
        // Check if item has boosts and apply them
        if let Some(item_data) = battle.dex.items().get_by_id(&item_id) {
            if let Some(boosts_map) = &item_data.boosts {
                // Convert HashMap boosts to array of tuples for battle.boost()
                let mut boosts_array: Vec<(&str, i8)> = Vec::new();

                if let Some(&atk) = boosts_map.get("atk") {
                    boosts_array.push(("atk", atk as i8));
                }
                if let Some(&def) = boosts_map.get("def") {
                    boosts_array.push(("def", def as i8));
                }
                if let Some(&spa) = boosts_map.get("spa") {
                    boosts_array.push(("spa", spa as i8));
                }
                if let Some(&spd) = boosts_map.get("spd") {
                    boosts_array.push(("spd", spd as i8));
                }
                if let Some(&spe) = boosts_map.get("spe") {
                    boosts_array.push(("spe", spe as i8));
                }
                if let Some(&accuracy) = boosts_map.get("accuracy") {
                    boosts_array.push(("accuracy", accuracy as i8));
                }
                if let Some(&evasion) = boosts_map.get("evasion") {
                    boosts_array.push(("evasion", evasion as i8));
                }

                // Apply boosts if any were found
                // JS: this.battle.boost(item.boosts, this, source, item);
                // battle.boost signature: (boosts, target, source, effect, is_secondary, is_self)
                if !boosts_array.is_empty() {
                    battle.boost(
                        &boosts_array,
                        pokemon_pos,
                        source_pos,
                        Some(item_id.as_str()),
                        false,  // is_secondary
                        false,  // is_self
                    );
                }
            }
        }

        // JS: this.battle.singleEvent('Use', item, this.itemState, this, source, sourceEffect);
        // ✅ NOW IMPLEMENTED (Session 24 Part 82): singleEvent('Use')
        battle.single_event("Use", &crate::battle::Effect::item(item_id.clone()), None, Some(pokemon_pos), source_pos, source_effect_ref, None);

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
        pokemon_mut.item_state = SharedEffectState::with_id(ID::empty());

        // JS: this.usedItemThisTurn = true;
        pokemon_mut.used_item_this_turn = true;

        // JS: this.battle.runEvent('AfterUseItem', this, null, null, item);
        // ✅ NOW IMPLEMENTED (Session 24 Part 82): runEvent('AfterUseItem')
        // Note: JavaScript passes item as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check pokemon's item field which is now empty
        battle.run_event("AfterUseItem", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);

        // JS: return true;
        Some(item_id)
    }
}
