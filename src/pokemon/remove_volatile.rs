use crate::*;

impl Pokemon {

    /// Remove a volatile condition
    /// Refactored to associated function for Battle access (Session 24 Part 84)
    //
    // 	removeVolatile(status: string | Effect) {
    // 		if (!this.hp) return false;
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.volatiles[status.id]) return false;
    // 		const { linkedPokemon, linkedStatus } = this.volatiles[status.id];
    // 		this.battle.singleEvent('End', status, this.volatiles[status.id], this);
    // 		delete this.volatiles[status.id];
    // 		if (linkedPokemon) {
    // 			this.removeLinkedVolatiles(linkedStatus, linkedPokemon);
    // 		}
    // 		return true;
    // 	}
    //
    pub fn remove_volatile(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        volatile_id: &ID,
    ) -> bool {
        // Get Pokemon name for logging
        let _pokemon_name = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.name.clone()
        } else {
            "Unknown".to_string()
        };

        crate::trace_volatile!("turn={}, REMOVE volatile='{}' from {}",
            battle.turn, volatile_id.as_str(), _pokemon_name);

        // Phase 1: Check HP and volatile existence
        let (_hp, _has_volatile, linked_pokemon, linked_status) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };

            // JS: if (!this.hp) return false;
            if pokemon.hp == 0 {
                return false;
            }

            // JS: if (!this.volatiles[status.id]) return false;
            let has_volatile = pokemon.volatiles.contains_key(volatile_id);
            if !has_volatile {
                return false;
            }

            // JS: const { linkedPokemon, linkedStatus } = this.volatiles[status.id];
            let linked_pokemon = pokemon.volatiles.get(volatile_id)
                .map(|state| state.borrow().linked_pokemon.clone())
                .flatten();

            let linked_status = pokemon.volatiles.get(volatile_id)
                .map(|state| state.borrow().linked_status.as_ref().map(|s| ID::from(s.as_str())))
                .flatten();

            (pokemon.hp, has_volatile, linked_pokemon, linked_status)
        };

        // JS: this.battle.singleEvent('End', status, this.volatiles[status.id], this);
        // ✅ NOW IMPLEMENTED (Session 24 Part 84): singleEvent('End') for removed volatile
        let condition_effect = battle.make_condition_effect(volatile_id);
        battle.single_event("End", &condition_effect, None, Some(pokemon_pos), None, None, None);

        // Phase 2: Remove the volatile
        // JS: delete this.volatiles[status.id];
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };
        pokemon_mut.volatiles.remove(volatile_id);

        // JS: if (linkedPokemon) { this.removeLinkedVolatiles(linkedStatus, linkedPokemon); }
        // ✅ NOW IMPLEMENTED (Session 24 Part 84): Call removeLinkedVolatiles for bidirectional cleanup
        if let (Some(linked_status_id), Some(linked_pokemon_positions)) = (linked_status, linked_pokemon) {
            Pokemon::remove_linked_volatiles(
                battle,
                pokemon_pos,
                &linked_status_id,
                &linked_pokemon_positions,
            );
        }

        // JS: return true;
        true
    }
}
