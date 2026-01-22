// JS Source:
//
// 	removeLinkedVolatiles(linkedStatus: string | Effect, linkedPokemon: Pokemon[]) {
// 		linkedStatus = linkedStatus.toString();
// 		for (const linkedPoke of linkedPokemon) {
// 			const volatileData = linkedPoke.volatiles[linkedStatus];
// 			if (!volatileData) continue;
// 			volatileData.linkedPokemon.splice(volatileData.linkedPokemon.indexOf(this), 1);
// 			if (volatileData.linkedPokemon.length === 0) {
// 				linkedPoke.removeVolatile(linkedStatus);
// 			}
// 		}
// 	}
//
// Note: In Rust, this is an associated function because it needs mutable access to
// multiple Pokemon through Battle

use crate::*;

impl Pokemon {
    /// Remove linked volatiles
    /// Equivalent to removeLinkedVolatiles in pokemon.ts
    ///
    /// This method removes this pokemon from the linkedPokemon list of other pokemon
    /// who have volatiles linked to it (e.g., Leech Seed)
    pub fn remove_linked_volatiles(
        battle: &mut Battle,
        this_pokemon: (usize, usize),
        linked_status: &ID,
        linked_pokemon: &[(usize, usize)],
    ) {
        debug_elog!("[REMOVE_LINKED_VOLATILES] this_pokemon=({}, {}), linked_status='{}', linked_pokemon={:?}",
            this_pokemon.0, this_pokemon.1, linked_status.as_str(), linked_pokemon);

        // JS: linkedStatus = linkedStatus.toString();
        // (ID is already string-like in Rust)

        // JS: for (const linkedPoke of linkedPokemon) {
        for &linked_poke_pos in linked_pokemon {
            debug_elog!("[REMOVE_LINKED_VOLATILES] Checking linked_poke at ({}, {})",
                linked_poke_pos.0, linked_poke_pos.1);

            // Track whether we should remove the volatile
            let should_remove_volatile = {
                // Get the linked pokemon
                let Some(linked_poke) = battle.pokemon_at_mut(linked_poke_pos.0, linked_poke_pos.1) else {
                    debug_elog!("[REMOVE_LINKED_VOLATILES]   Pokemon not found, skipping");
                    continue;
                };

                // JS: const volatileData = linkedPoke.volatiles[linkedStatus];
                // JS: if (!volatileData) continue;
                let Some(volatile_data) = linked_poke.volatiles.get_mut(linked_status) else {
                    debug_elog!("[REMOVE_LINKED_VOLATILES]   Volatile '{}' not found on linked pokemon, skipping",
                        linked_status.as_str());
                    continue;
                };

                debug_elog!("[REMOVE_LINKED_VOLATILES]   Found volatile '{}' with linked_pokemon={:?}",
                    linked_status.as_str(), volatile_data.linked_pokemon);

                // JS: volatileData.linkedPokemon.splice(volatileData.linkedPokemon.indexOf(this), 1);
                // Access linkedPokemon as typed field, remove this_pokemon
                if let Some(linked_pokemon_vec) = volatile_data.linked_pokemon.as_mut() {
                    let _before_len = linked_pokemon_vec.len();
                    // Find and remove this_pokemon from the Vec
                    linked_pokemon_vec.retain(|&pos| pos != this_pokemon);
                    let _after_len = linked_pokemon_vec.len();
                    debug_elog!("[REMOVE_LINKED_VOLATILES]   Removed this_pokemon from linked_pokemon: {} -> {} items",
                        before_len, after_len);

                    // JS: if (volatileData.linkedPokemon.length === 0) {
                    linked_pokemon_vec.is_empty()
                } else {
                    debug_elog!("[REMOVE_LINKED_VOLATILES]   No linked_pokemon vec to modify");
                    false
                }
            };

            // JS: linkedPoke.removeVolatile(linkedStatus);
            if should_remove_volatile {
                debug_elog!("[REMOVE_LINKED_VOLATILES]   Calling remove_volatile for '{}' on ({}, {})",
                    linked_status.as_str(), linked_poke_pos.0, linked_poke_pos.1);
                Pokemon::remove_volatile(battle, linked_poke_pos, linked_status);
            }
        }
    }
}
