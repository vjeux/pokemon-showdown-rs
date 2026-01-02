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
    /// Use held item
    /// Refactored to associated function for Battle access (Session 24 Part 49)
    pub fn use_item(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _source_effect: Option<&ID>,
    ) -> Option<ID> {
        // Phase 1: Extract pokemon data immutably to check conditions
        let (hp, is_active, item_id, is_gem) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
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
                .and_then(|item_data| item_data.extra.get("isGem"))
                .and_then(|v| v.as_bool())
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
        // Note: battle.event source/sourceEffect defaulting still missing (needs event system infrastructure)

        // JS: const item = this.getItem();
        // JS: if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
        // JS:     return false;
        // JS: }
        // Note: Missing sourceEffect item type check (needs event system infrastructure)

        // JS: if (this.battle.runEvent('UseItem', this, null, null, item)) {
        // ✅ NOW IMPLEMENTED (Session 24 Part 82): runEvent('UseItem')
        // Note: JavaScript passes item as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check pokemon's item field
        let use_item_result = battle.run_event("UseItem", Some(pokemon_pos), None, None, None);
        // runEvent returns Option<i32>, None or Some(0) means failure
        if use_item_result == Some(0) || use_item_result == None {
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
                if let Some(source_pos) = _source_pos {
                    if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
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
            if let Some(boosts_value) = item_data.extra.get("boosts") {
                if let Some(boosts_obj) = boosts_value.as_object() {
                    // Convert JSON boosts to array of tuples for battle.boost()
                    let mut boosts_array: Vec<(&str, i8)> = Vec::new();

                    if let Some(atk) = boosts_obj.get("atk").and_then(|v| v.as_i64()) {
                        boosts_array.push(("atk", atk as i8));
                    }
                    if let Some(def) = boosts_obj.get("def").and_then(|v| v.as_i64()) {
                        boosts_array.push(("def", def as i8));
                    }
                    if let Some(spa) = boosts_obj.get("spa").and_then(|v| v.as_i64()) {
                        boosts_array.push(("spa", spa as i8));
                    }
                    if let Some(spd) = boosts_obj.get("spd").and_then(|v| v.as_i64()) {
                        boosts_array.push(("spd", spd as i8));
                    }
                    if let Some(spe) = boosts_obj.get("spe").and_then(|v| v.as_i64()) {
                        boosts_array.push(("spe", spe as i8));
                    }
                    if let Some(accuracy) = boosts_obj.get("accuracy").and_then(|v| v.as_i64()) {
                        boosts_array.push(("accuracy", accuracy as i8));
                    }
                    if let Some(evasion) = boosts_obj.get("evasion").and_then(|v| v.as_i64()) {
                        boosts_array.push(("evasion", evasion as i8));
                    }

                    // Apply boosts if any were found
                    // JS: this.battle.boost(item.boosts, this, source, item);
                    // battle.boost signature: (boosts, target, source, effect, is_secondary, is_self)
                    if !boosts_array.is_empty() {
                        battle.boost(
                            &boosts_array,
                            pokemon_pos,
                            _source_pos,
                            Some(item_id.as_str()),
                            false,  // is_secondary
                            false,  // is_self
                        );
                    }
                }
            }
        }

        // JS: this.battle.singleEvent('Use', item, this.itemState, this, source, sourceEffect);
        // ✅ NOW IMPLEMENTED (Session 24 Part 82): singleEvent('Use')
        battle.single_event("Use", &item_id, Some(pokemon_pos), _source_pos, None);

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

        // JS: this.battle.runEvent('AfterUseItem', this, null, null, item);
        // ✅ NOW IMPLEMENTED (Session 24 Part 82): runEvent('AfterUseItem')
        // Note: JavaScript passes item as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check pokemon's item field which is now empty
        battle.run_event("AfterUseItem", Some(pokemon_pos), None, None, None);

        // JS: return true;
        Some(item_id)
    }
}
