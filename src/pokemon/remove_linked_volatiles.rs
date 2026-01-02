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
        _battle: &mut Battle,
        _this_pokemon: (usize, usize),
        _linked_status: &ID,
        _linked_pokemon: &[(usize, usize)],
    ) {
        // TODO: Implement 1-to-1 from JavaScript
        // This requires:
        // 1. Store linkedPokemon and linkedStatus in EffectState.data (currently not implemented)
        // 2. For each linked pokemon, get their volatile for linked_status
        // 3. Remove this_pokemon from their linkedPokemon list (stored in data HashMap)
        // 4. If linkedPokemon list becomes empty, remove the volatile entirely
        //
        // The infrastructure for storing/retrieving linkedPokemon from EffectState.data
        // needs to be implemented first before this can work properly.
    }
}
