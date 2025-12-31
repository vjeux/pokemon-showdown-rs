use crate::*;

impl Battle {

    /// Find Pokemon event handlers
    /// Equivalent to battle.ts findPokemonEventHandlers() (battle.ts:1098-1157)
    ///
    // TODO: INCOMPLETE IMPLEMENTATION - Returns simplified data structure
    // Missing from TypeScript version (battle.ts:1098-1157, 60 lines):
    // Return type: Should return EventListener[] with complete handler objects, not Vec<(ID, Option<(usize, usize)>)>
    // For each effect type (status, volatiles, ability, item, species, slotConditions), should include:
    // 1. Get callback via this.getCallback(pokemon, effect, callbackName)
    // 2. If callback exists or getKey is set, push complete handler with:
    //    - effect: the effect (status/volatile/ability/item/species/slotCondition)
    //    - callback: the callback function
    //    - state: corresponding state object (statusState/volatileState/abilityState/itemState/speciesState/slotConditionState)
    //    - end: removal function (clearStatus/removeVolatile/clearAbility/clearItem/removeSlotCondition) or empty for species
    //    - endCallArgs: [side, pokemon, slotCondition.id] for slot conditions
    //    - effectHolder: pokemon
    // 3. Resolve priority via this.resolvePriority()
    // Current implementation only returns IDs without callback/state/priority/end information
    // This is similar to get_callback.rs architectural difference (static vs dynamic dispatch)
    //
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
    // 					effect: slotCondition,
    // 					callback,
    // 					state: slotConditionState,
    // 					end: side.removeSlotCondition,
    // 					endCallArgs: [side, pokemon, slotCondition.id],
    // 					effectHolder: pokemon,
    // 				}, callbackName));
    // 			}
    // 		}
    //
    // 		return handlers;
    // 	}
    //
    pub fn find_pokemon_event_handlers(
        &self,
        event_id: &str,  // Changed from _event_id to event_id (removed underscore)
        target: (usize, usize),
        get_key: Option<&str>,  // NEW: 'duration' for Residual events
    ) -> Vec<(ID, Option<(usize, usize)>, crate::battle::EffectType)> {  // NEW: Include EffectType
        eprintln!("[FIND_POKEMON_HANDLERS] event_id={}, target={:?}, get_key={:?}", event_id, target, get_key);

        let mut handlers = Vec::new();
        let (side_idx, poke_idx) = target;

        if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                eprintln!("[FIND_POKEMON_HANDLERS] Pokemon: {}, item: {}, status: '{}'", pokemon.name, pokemon.item, pokemon.status);

                // JS: const status = pokemon.getStatus();
                // Add status handler if it has callback for this event
                // IMPORTANT: Must use condition_has_callback, not has_callback, to avoid checking other effect types
                // JS: if (callback !== undefined || (getKey && pokemon.statusState[getKey]))
                let has_callback = !pokemon.status.is_empty() && self.condition_has_callback(pokemon.status.as_str(), event_id);
                let has_get_key = get_key == Some("duration") && pokemon.status_state.duration.is_some();
                eprintln!("[FIND_POKEMON_HANDLERS] Status check: status='{}', has_callback={}, has_get_key={}", pokemon.status, has_callback, has_get_key);
                if has_callback || has_get_key {
                    eprintln!("[FIND_POKEMON_HANDLERS] Adding status handler: {}", pokemon.status);
                    handlers.push((pokemon.status.clone(), Some(target), crate::battle::EffectType::Condition));
                }

                // JS: for (const id in pokemon.volatiles)
                // Add volatile handlers if they have callbacks for this event
                // IMPORTANT: Must use condition_has_callback, not has_callback
                // Bug fix: Previously used has_callback which checked ALL effect types (abilities, items, MOVES, conditions)
                // This caused "substitute" volatile to match "substitute" MOVE's onHit callback, dealing double damage
                // JS: if (callback !== undefined || (getKey && volatileState[getKey]))
                eprintln!("[FIND_POKEMON_HANDLERS] Checking {} volatiles for {}", pokemon.volatiles.len(), pokemon.name);
                for (volatile_id, volatile_state) in &pokemon.volatiles {
                    let has_cb = self.condition_has_callback(volatile_id.as_str(), event_id);
                    let has_get_key = get_key == Some("duration") && volatile_state.duration.is_some();
                    eprintln!("[FIND_POKEMON_HANDLERS] Volatile '{}' condition_has_callback({})={}, has_get_key({:?})={}",
                        volatile_id, event_id, has_cb, get_key, has_get_key);
                    if has_cb || has_get_key {
                        eprintln!("[FIND_POKEMON_HANDLERS] Adding volatile handler: {}", volatile_id);
                        handlers.push((volatile_id.clone(), Some(target), crate::battle::EffectType::Condition));
                    }
                }

                // JS: const ability = pokemon.getAbility();
                // Add ability handler if it has callback for this event
                // IMPORTANT: Must use ability_has_callback, not has_callback
                // JS: if (callback !== undefined || (getKey && pokemon.abilityState[getKey]))
                if !pokemon.ability.is_empty() {
                    let has_cb = self.ability_has_callback(pokemon.ability.as_str(), event_id);
                    let has_get_key = get_key == Some("duration") && pokemon.ability_state.duration.is_some();
                    eprintln!("[FIND_POKEMON_HANDLERS] Ability {} ability_has_callback({})={}, has_get_key={}", pokemon.ability, event_id, has_cb, has_get_key);
                    if has_cb || has_get_key {
                        eprintln!("[FIND_POKEMON_HANDLERS] Adding ability handler: {}", pokemon.ability);
                        handlers.push((pokemon.ability.clone(), Some(target), crate::battle::EffectType::Ability));
                    }
                }

                // JS: const item = pokemon.getItem();
                // Add item handler if it has callback for this event
                // IMPORTANT: Must use item_has_callback, not has_callback
                // JS: if (callback !== undefined || (getKey && pokemon.itemState[getKey]))
                if !pokemon.item.is_empty() {
                    let has_cb = self.item_has_callback(pokemon.item.as_str(), event_id);
                    let has_get_key = get_key == Some("duration") && pokemon.item_state.duration.is_some();
                    eprintln!("[FIND_POKEMON_HANDLERS] Item {} item_has_callback({})={}, has_get_key={}", pokemon.item, event_id, has_cb, has_get_key);
                    if has_cb || has_get_key {
                        eprintln!("[FIND_POKEMON_HANDLERS] Adding item handler: {}", pokemon.item);
                        handlers.push((pokemon.item.clone(), Some(target), crate::battle::EffectType::Item));
                    }
                }

                // JS: const species = pokemon.baseSpecies;
                // Add species handler if it has callback for this event
                // IMPORTANT: Must use species_has_callback, not has_callback
                // Note: Species don't have state with getKey fields
                if self.species_has_callback(pokemon.species_id.as_str(), event_id) {
                    handlers.push((pokemon.species_id.clone(), Some(target), crate::battle::EffectType::Condition));
                }

                // JS: for (const conditionid in side.slotConditions[pokemon.position])
                // Add slot condition handlers if they have callbacks for this event
                // IMPORTANT: Must use condition_has_callback, not has_callback (slot conditions are conditions)
                // JS: if (callback !== undefined || (getKey && slotConditionState[getKey]))
                if let Some(slot_conds) = side.slot_conditions.get(pokemon.position) {
                    for (slot_cond_id, slot_cond_state) in slot_conds {
                        let has_cb = self.condition_has_callback(slot_cond_id.as_str(), event_id);
                        let has_get_key = get_key == Some("duration") && slot_cond_state.duration.is_some();
                        if has_cb || has_get_key {
                            handlers.push((slot_cond_id.clone(), Some(target), crate::battle::EffectType::Condition));
                        }
                    }
                }
            }
        }

        handlers
    }
}
