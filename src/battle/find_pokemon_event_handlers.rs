// 1:1 port of findPokemonEventHandlers from battle.ts

use crate::*;
use crate::event::EventResult;
use crate::battle::{EventListener, EffectType};

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
            let has_callback = self.has_callback(&pokemon.status, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks statusState[getKey], which means checking if duration exists
                key == "duration" && pokemon.status_state.duration.is_some()
            });

            if has_callback || has_get_key {
                handlers.push(EventListener {
                    event_name: String::new(),
                    effect_id: pokemon.status.clone(),
                    effect_type: EffectType::Status,
                    target: Some(target),
                    index: None,
                    state: Some(pokemon.status_state.clone()),
                    effect_holder: Some(target),
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: for (const id in pokemon.volatiles) {
        for (volatile_id, volatile_state) in &pokemon.volatiles {
            // JS: const volatileState = pokemon.volatiles[id];
            // JS: const volatile = this.dex.conditions.getByID(id as ID);
            // JS: callback = this.getCallback(pokemon, volatile, callbackName);
            // JS: if (callback !== undefined || (getKey && volatileState[getKey])) {
            let has_callback = self.has_callback(volatile_id, callback_name);
            eprintln!("[FIND_POKEMON_EVENT_HANDLERS] Checking volatile '{}' for callback '{}': has_callback={}",
                volatile_id.as_str(), callback_name, has_callback);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks volatileState[getKey], which means checking if duration exists
                key == "duration" && volatile_state.duration.is_some()
            });

            if has_callback || has_get_key {
                eprintln!("[FIND_POKEMON_EVENT_HANDLERS] Adding handler for volatile '{}'", volatile_id.as_str());
                handlers.push(EventListener {
                    event_name: String::new(),
                    effect_id: volatile_id.clone(),
                    effect_type: EffectType::Condition,
                    target: Some(target),
                    index: None,
                    state: Some(volatile_state.clone()),
                    effect_holder: Some(target),
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: const ability = pokemon.getAbility();
        // JS: callback = this.getCallback(pokemon, ability, callbackName);
        // JS: if (callback !== undefined || (getKey && pokemon.abilityState[getKey])) {
        if !pokemon.ability.is_empty() {
            let has_callback = self.has_callback(&pokemon.ability, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks abilityState[getKey], which means checking if duration exists
                key == "duration" && pokemon.ability_state.duration.is_some()
            });

            if has_callback || has_get_key {
                handlers.push(EventListener {
                    event_name: String::new(),
                    effect_id: pokemon.ability.clone(),
                    effect_type: EffectType::Ability,
                    target: Some(target),
                    index: None,
                    state: Some(pokemon.ability_state.clone()),
                    effect_holder: Some(target),
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: const item = pokemon.getItem();
        // JS: callback = this.getCallback(pokemon, item, callbackName);
        // JS: if (callback !== undefined || (getKey && pokemon.itemState[getKey])) {
        if !pokemon.item.is_empty() {
            let has_callback = self.has_callback(&pokemon.item, callback_name);
            eprintln!("[FIND_POKEMON_EVENT_HANDLERS] Checking item '{}' for callback '{}': has_callback={}",
                pokemon.item.as_str(), callback_name, has_callback);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks itemState[getKey], which means checking if duration exists
                key == "duration" && pokemon.item_state.duration.is_some()
            });

            if has_callback || has_get_key {
                eprintln!("[FIND_POKEMON_EVENT_HANDLERS] Adding handler for item '{}'", pokemon.item.as_str());
                handlers.push(EventListener {
                    event_name: String::new(),
                    effect_id: pokemon.item.clone(),
                    effect_type: EffectType::Item,
                    target: Some(target),
                    index: None,
                    state: Some(pokemon.item_state.clone()),
                    effect_holder: Some(target),
                    order: None,
                    priority: 0,
                    sub_order: 0,
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
        if self.has_callback(&pokemon.species_id, callback_name) {
            handlers.push(EventListener {
                    event_name: String::new(),
                effect_id: pokemon.species_id.clone(),
                effect_type: EffectType::Condition,
                target: Some(target),
                index: None,
                state: Some(pokemon.species_state.clone()),
                effect_holder: Some(target),
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
                let has_callback = self.has_callback(slot_cond_id, callback_name);
                let has_get_key = get_key.is_some_and(|key| {
                    // JavaScript checks slotConditionState[getKey], which means checking if duration exists
                    key == "duration" && slot_cond_state.duration.is_some()
                });

                if has_callback || has_get_key {
                    handlers.push(EventListener {
                    event_name: String::new(),
                        effect_id: slot_cond_id.clone(),
                        effect_type: EffectType::SlotCondition,
                        target: Some(target),
                        index: None,
                        state: Some(slot_cond_state.clone()),
                        effect_holder: Some(target),
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
