use crate::*;

impl Battle {

    /// Find Pokemon event handlers
    /// Equivalent to battle.ts findPokemonEventHandlers() (battle.ts:1098-1157)
    ///
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
        _event_id: &str,
        target: (usize, usize),
    ) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();
        let (side_idx, poke_idx) = target;

        if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                // JS: const status = pokemon.getStatus();
                // Add status handler
                if !pokemon.status.is_empty() {
                    handlers.push((pokemon.status.clone(), Some(target)));
                }

                // JS: for (const id in pokemon.volatiles)
                // Add volatile handlers
                for volatile_id in pokemon.volatiles.keys() {
                    handlers.push((volatile_id.clone(), Some(target)));
                }

                // JS: const ability = pokemon.getAbility();
                // Add ability handler
                if !pokemon.ability.is_empty() {
                    handlers.push((pokemon.ability.clone(), Some(target)));
                }

                // JS: const item = pokemon.getItem();
                // Add item handler
                if !pokemon.item.is_empty() {
                    handlers.push((pokemon.item.clone(), Some(target)));
                }

                // JS: const species = pokemon.baseSpecies;
                // Add species handler (NEW! - was missing)
                handlers.push((pokemon.species_id.clone(), Some(target)));

                // JS: for (const conditionid in side.slotConditions[pokemon.position])
                // Add slot condition handlers (NEW! - was missing)
                if let Some(slot_conds) = side.slot_conditions.get(pokemon.position) {
                    for slot_cond_id in slot_conds.keys() {
                        handlers.push((slot_cond_id.clone(), Some(target)));
                    }
                }
            }
        }

        handlers
    }
}
