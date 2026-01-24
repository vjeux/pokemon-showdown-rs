// 1:1 port of findPokemonEventHandlers from battle.ts

use crate::*;
use crate::battle::{Effect, EffectHolder, EventListener, EffectType};

impl Battle {
    /// Find Pokemon event handlers
    /// Equivalent to battle.ts findPokemonEventHandlers()
    ///
    // JS Source:
    // 	findPokemonEventHandlers(pokemon: Pokemon, callbackName: string, getKey?: 'duration') {
    // 		const handlers: EventListener[] = [];
    //
    // 		const status = pokemon.getStatus();
    // 		let callback = this.getCallback(pokemon, status, callbackName);
    // 		if (callback !== undefined || (getKey && pokemon.statusState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: status, callback, state: pokemon.statusState, end: pokemon.clearStatus, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		for (const id in pokemon.volatiles) {
    // 			const volatileState = pokemon.volatiles[id];
    // 			const volatile = this.dex.conditions.getByID(id as ID);
    // 			callback = this.getCallback(pokemon, volatile, callbackName);
    // 			if (callback !== undefined || (getKey && volatileState[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: volatile, callback, state: volatileState, end: pokemon.removeVolatile, effectHolder: pokemon,
    // 				}, callbackName));
    // 			}
    // 		}
    // 		const ability = pokemon.getAbility();
    // 		callback = this.getCallback(pokemon, ability, callbackName);
    // 		if (callback !== undefined || (getKey && pokemon.abilityState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: ability, callback, state: pokemon.abilityState, end: pokemon.clearAbility, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		const item = pokemon.getItem();
    // 		callback = this.getCallback(pokemon, item, callbackName);
    // 		if (callback !== undefined || (getKey && pokemon.itemState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: item, callback, state: pokemon.itemState, end: pokemon.clearItem, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		const species = pokemon.baseSpecies;
    // 		callback = this.getCallback(pokemon, species, callbackName);
    // 		if (callback !== undefined) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: species, callback, state: pokemon.speciesState, end() {}, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		const side = pokemon.side;
    // 		for (const conditionid in side.slotConditions[pokemon.position]) {
    // 			const slotConditionState = side.slotConditions[pokemon.position][conditionid];
    // 			const slotCondition = this.dex.conditions.getByID(conditionid as ID);
    // 			callback = this.getCallback(pokemon, slotCondition, callbackName);
    // 			if (callback !== undefined || (getKey && slotConditionState[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: slotCondition, callback, state: slotConditionState,
    // 					end: side.removeSlotCondition, endCallArgs: [side, pokemon, slotCondition.id], effectHolder: pokemon,
    // 				}, callbackName));
    // 			}
    // 		}
    //
    // 		return handlers;
    // 	}
    pub fn find_pokemon_event_handlers(
        &self,
        callback_name: &str,
        target: (usize, usize),
        get_key: Option<&str>,
    ) -> Vec<EventListener> {
        // JS: const handlers: EventListener[] = [];
        let mut handlers: Vec<EventListener> = Vec::new();
        let (side_idx, poke_idx) = target;

        // Get pokemon reference
        let pokemon = match self.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            Some(p) => p,
            None => return handlers,
        };

        // JS: const status = pokemon.getStatus();
        // JS: let callback = this.getCallback(pokemon, status, callbackName);
        // JS: if (callback !== undefined || (getKey && pokemon.statusState[getKey])) {
        if !pokemon.status.is_empty() {
            let has_callback = self.has_status_callback(&pokemon.status, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks statusState[getKey], which means checking if duration exists
                key == "duration" && pokemon.status_state.borrow().duration.is_some()
            });

            if has_callback || has_get_key {
                // Get status name from dex
                let status_name = self.dex.conditions().get_by_id(&pokemon.status)
                    .and_then(|c| c.name.clone())
                    .unwrap_or_else(|| pokemon.status.to_string());

                handlers.push(EventListener {
                    callback_name: String::new(),
                    effect: Effect {
                        id: pokemon.status.clone(),
                        name: status_name.into(),
                        effect_type: EffectType::Status,
                        effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                        side_index: Some(target.0),
                        prankster_boosted: false,
                    },
                    target: Some(target),
                    index: None,
                    state: Some(pokemon.status_state.clone()),
                    effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    // JavaScript: handler.effectOrder is NOT set in findPokemonEventHandlers
                    // Only resolve_priority sets it for SwitchIn/RedirectTarget events
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: for (const id in pokemon.volatiles) {
        if callback_name.contains("Damaging") {
            debug_elog!("[FIND_POKEMON_EVENT_HANDLERS] callback_name={}, pokemon={}, volatiles count={}, volatiles={:?}",
                callback_name, pokemon.species_id.as_str(), pokemon.volatiles.len(),
                pokemon.volatiles.keys().map(|k| k.as_str()).collect::<Vec<_>>());
        }
        for (volatile_id, volatile_state) in &pokemon.volatiles {
            // JS: const volatileState = pokemon.volatiles[id];
            // JS: const volatile = this.dex.conditions.getByID(id as ID);
            // JS: callback = this.getCallback(pokemon, volatile, callbackName);
            // JS: if (callback !== undefined || (getKey && volatileState[getKey])) {
            let has_callback = self.has_volatile_callback(volatile_id, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks volatileState[getKey], which means checking if duration exists
                key == "duration" && volatile_state.borrow().duration.is_some()
            });

            if has_callback || has_get_key {
                debug_elog!("[FIND_POKEMON_EVENT_HANDLERS] Adding handler for volatile={} with callback={}", volatile_id.as_str(), callback_name);
                // Get volatile name from dex
                let volatile_name = self.dex.conditions().get_by_id(volatile_id)
                    .and_then(|c| c.name.clone())
                    .unwrap_or_else(|| volatile_id.to_string());

                handlers.push(EventListener {
                    callback_name: String::new(),
                    effect: Effect {
                        id: volatile_id.clone(),
                        name: volatile_name.into(),
                        effect_type: EffectType::Condition,
                        effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                        side_index: Some(target.0),
                        prankster_boosted: false,
                    },
                    target: Some(target),
                    index: None,
                    // JavaScript passes volatileState in handler.state:
                    // handlers.push(this.resolvePriority({
                    //   effect: volatile, callback, state: volatileState, ...
                    // }, callbackName));
                    // This is critical for volatiles like "stall" which have counter state
                    state: Some(volatile_state.clone()),
                    effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    // JavaScript: handler.effectOrder is NOT set for volatiles in findPokemonEventHandlers
                    // The state.effectOrder exists but is not copied to handler.effectOrder
                    // So when comparePriority does (effectOrder || 0), it uses 0 for both
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: const ability = pokemon.getAbility();
        // JS: callback = this.getCallback(pokemon, ability, callbackName);
        // JS: if (callback !== undefined || (getKey && pokemon.abilityState[getKey])) {
        //
        // IMPORTANT: For abilities, we should only check the ability's DIRECT callbacks
        // (like onTryHit, onEnd), NOT the ability's embedded condition callbacks.
        // The embedded condition callbacks (like onModifySpA) are for the VOLATILE
        // that gets added when the ability triggers, not for the ability itself.
        if !pokemon.ability.is_empty() {
            // Use ability_has_callback directly instead of has_callback
            // This ensures we only check the ability's direct callbacks
            let has_callback = self.ability_has_callback(&pokemon.ability.as_str(), callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks abilityState[getKey], which means checking if duration exists
                key == "duration" && pokemon.ability_state.borrow().duration.is_some()
            });

            if has_callback || has_get_key {
                // Get ability name from dex
                let ability_name = self.dex.abilities().get(pokemon.ability.as_str())
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| pokemon.ability.to_string());

                handlers.push(EventListener {
                    callback_name: String::new(),
                    effect: Effect {
                        id: pokemon.ability.clone(),
                        name: ability_name.into(),
                        effect_type: EffectType::Ability,
                        effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                        side_index: Some(target.0),
                        prankster_boosted: false,
                    },
                    target: Some(target),
                    index: None,
                    state: Some(pokemon.ability_state.clone()),
                    effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    // JavaScript: handler.effectOrder is NOT set in findPokemonEventHandlers
                    // Only resolve_priority sets it for SwitchIn/RedirectTarget events
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: const item = pokemon.getItem();
        // JS: callback = this.getCallback(pokemon, item, callbackName);
        // JS: if (callback !== undefined || (getKey && pokemon.itemState[getKey])) {
        if !pokemon.item.is_empty() {
            let has_callback = self.has_item_id_callback(&pokemon.item, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks itemState[getKey], which means checking if duration exists
                key == "duration" && pokemon.item_state.borrow().duration.is_some()
            });

            if has_callback || has_get_key {
                // Get item name from dex
                let item_name = self.dex.items().get(pokemon.item.as_str())
                    .map(|i| i.name.clone())
                    .unwrap_or_else(|| pokemon.item.to_string());

                handlers.push(EventListener {
                    callback_name: String::new(),
                    effect: Effect {
                        id: pokemon.item.clone(),
                        name: item_name.into(),
                        effect_type: EffectType::Item,
                        effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                        side_index: Some(target.0),
                        prankster_boosted: false,
                    },
                    target: Some(target),
                    index: None,
                    state: Some(pokemon.item_state.clone()),
                    effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    // JavaScript: handler.effectOrder is NOT set in findPokemonEventHandlers
                    // Only resolve_priority sets it for SwitchIn/RedirectTarget events
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: const species = pokemon.baseSpecies;
        // JS: callback = this.getCallback(pokemon, species, callbackName);
        // JS: if (callback !== undefined) {
        // Note: Species don't have getKey (no duration field)
        // Note: Species callbacks use Condition effectType (like volatiles) with subOrder 2
        //       per JavaScript effectTypeOrder in battle.ts resolvePriority
        if self.has_species_id_callback(&pokemon.species_id, callback_name) {
            // Get species name from dex
            let species_name = self.dex.species().get(pokemon.species_id.as_str())
                .map(|s| s.name.clone())
                .unwrap_or_else(|| pokemon.species_id.to_string());

            handlers.push(EventListener {
                    callback_name: String::new(),
                effect: Effect {
                    id: pokemon.species_id.clone(),
                    name: species_name.into(),
                    effect_type: EffectType::Condition,
                    effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                    side_index: Some(target.0),
                    prankster_boosted: false,
                },
                target: Some(target),
                index: None,
                state: Some(pokemon.species_state.clone()),
                effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),
                order: None,
                priority: 0,
                sub_order: 0,
                effect_order: None,
                speed: None,
            });
        }

        // JS: const side = pokemon.side;
        // JS: for (const conditionid in side.slotConditions[pokemon.position]) {
        if let Some(slot_conds) = self.sides[side_idx].slot_conditions.get(pokemon.position) {
            for (slot_cond_id, slot_cond_state) in slot_conds {
                // JS: const slotConditionState = side.slotConditions[pokemon.position][conditionid];
                // JS: const slotCondition = this.dex.conditions.getByID(conditionid as ID);
                // JS: callback = this.getCallback(pokemon, slotCondition, callbackName);
                // JS: if (callback !== undefined || (getKey && slotConditionState[getKey])) {
                let has_callback = self.has_slot_condition_callback(slot_cond_id, callback_name);
                let has_get_key = get_key.is_some_and(|key| {
                    // JavaScript checks slotConditionState[getKey], which means checking if duration exists
                    key == "duration" && slot_cond_state.borrow().duration.is_some()
                });

                if has_callback || has_get_key {
                    // Get slot condition name from dex
                    let slot_cond_name = self.dex.conditions().get_by_id(slot_cond_id)
                        .and_then(|c| c.name.clone())
                        .unwrap_or_else(|| slot_cond_id.to_string());

                    // JavaScript sets effectHolder: pokemon - the Pokemon object itself
                    // We use (side_idx, poke_idx) which is the party index, not the slot position
                    // with_effect_state_ref will convert to slot position using pokemon.position
                    handlers.push(EventListener {
                    callback_name: String::new(),
                        effect: Effect {
                            id: slot_cond_id.clone(),
                            name: slot_cond_name.into(),
                            effect_type: EffectType::SlotCondition,
                            effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),  // Party index for callbacks
                            side_index: Some(side_idx),
                            prankster_boosted: false,
                        },
                        target: Some(target),  // Party index for event dispatch
                        index: None,
                        state: Some(slot_cond_state.clone()),
                        effect_holder: Some(EffectHolder::Pokemon(target.0, target.1)),  // Party index - with_effect_state converts to slot
                        order: None,
                        priority: 0,
                        sub_order: 0,
                        effect_order: None,
                        speed: None,
                    });
                }
            }
        }

        // JS: return handlers;
        handlers
    }
}
