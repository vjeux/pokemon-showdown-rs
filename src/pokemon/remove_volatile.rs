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
        // Phase 1: Check HP and volatile existence
        let (hp, has_volatile, linked_pokemon, linked_status) = {
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
                .and_then(|state| state.data.get("linkedPokemon"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| {
                            v.as_array().and_then(|pos| {
                                if pos.len() == 2 {
                                    Some((
                                        pos[0].as_u64().unwrap() as usize,
                                        pos[1].as_u64().unwrap() as usize,
                                    ))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect::<Vec<(usize, usize)>>()
                });

            let linked_status = pokemon.volatiles.get(volatile_id)
                .and_then(|state| state.data.get("linkedStatus"))
                .and_then(|v| v.as_str())
                .map(|s| ID::from(s));

            (pokemon.hp, has_volatile, linked_pokemon, linked_status)
        };

        // JS: this.battle.singleEvent('End', status, this.volatiles[status.id], this);
        // ✅ NOW IMPLEMENTED (Session 24 Part 84): singleEvent('End') for removed volatile
        battle.single_event("End", volatile_id, Some(pokemon_pos), None, None);

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
