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
        // JS: linkedStatus = linkedStatus.toString();
        // (ID is already string-like in Rust)

        // JS: for (const linkedPoke of linkedPokemon) {
        for &linked_poke_pos in linked_pokemon {
            // Track whether we should remove the volatile
            let should_remove_volatile = {
                // Get the linked pokemon
                let Some(linked_poke) = battle.pokemon_at_mut(linked_poke_pos.0, linked_poke_pos.1) else {
                    continue;
                };

                // JS: const volatileData = linkedPoke.volatiles[linkedStatus];
                // JS: if (!volatileData) continue;
                let Some(volatile_data) = linked_poke.volatiles.get_mut(linked_status) else {
                    continue;
                };

                // JS: volatileData.linkedPokemon.splice(volatileData.linkedPokemon.indexOf(this), 1);
                // Access data["linkedPokemon"] as array, remove this_pokemon
                if let Some(linked_pokemon_value) = volatile_data.data.get_mut("linkedPokemon") {
                    if let Some(linked_array) = linked_pokemon_value.as_array_mut() {
                        // Find and remove this_pokemon from the array
                        // linkedPokemon is stored as [[side, slot], ...]
                        linked_array.retain(|value| {
                            if let Some(arr) = value.as_array() {
                                if arr.len() == 2 {
                                    let side = arr[0].as_u64().unwrap_or(999) as usize;
                                    let slot = arr[1].as_u64().unwrap_or(999) as usize;
                                    (side, slot) != this_pokemon
                                } else {
                                    true // Keep malformed entries
                                }
                            } else {
                                true // Keep non-array entries
                            }
                        });

                        // JS: if (volatileData.linkedPokemon.length === 0) {
                        linked_array.is_empty()
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            // JS: linkedPoke.removeVolatile(linkedStatus);
            if should_remove_volatile {
                Pokemon::remove_volatile(battle, linked_poke_pos, linked_status);
            }
        }
    }
}
