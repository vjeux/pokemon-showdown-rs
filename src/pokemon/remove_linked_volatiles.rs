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
        // JS: linkedStatus = linkedStatus.toString();
        // Note: ID is already a string type in Rust

        // JS: for (const linkedPoke of linkedPokemon) {
        // Note: Would need to loop through _linked_pokemon positions

        // JS:     const volatileData = linkedPoke.volatiles[linkedStatus];
        // JS:     if (!volatileData) continue;
        // Note: Would need to get each pokemon's volatile for _linked_status

        // JS:     volatileData.linkedPokemon.splice(volatileData.linkedPokemon.indexOf(this), 1);
        // Note: Would need to access EffectState.data["linkedPokemon"] as Vec<(usize, usize)>
        // Note: Remove _this_pokemon from the linkedPokemon list

        // JS:     if (volatileData.linkedPokemon.length === 0) {
        // JS:         linkedPoke.removeVolatile(linkedStatus);
        // JS:     }
        // Note: If linkedPokemon list becomes empty, remove the volatile entirely

        // Full implementation needs:
        // 1. EffectState.data HashMap to store linkedPokemon: Vec<(usize, usize)>
        // 2. Loop through each position in _linked_pokemon
        // 3. Get pokemon_mut at each position
        // 4. Get their volatile for _linked_status
        // 5. Access data["linkedPokemon"] and remove _this_pokemon
        // 6. If list is empty, call pokemon.remove_volatile(_linked_status)
        //
        // The infrastructure for storing/retrieving linkedPokemon from EffectState.data
        // needs to be implemented first before this can work properly.
    }
}
