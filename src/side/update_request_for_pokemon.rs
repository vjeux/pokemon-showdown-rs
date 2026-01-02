// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Update request for specific Pokemon
    /// Equivalent to side.ts updateRequestForPokemon()
    //
    // 	updateRequestForPokemon(pokemon: Pokemon, update: (req: PokemonMoveRequestData) => boolean | void) {
    // 		if (!(this.activeRequest as MoveRequest)?.active) {
    // 			throw new Error(`Can't update a request without active Pokemon`);
    // 		}
    // 		const req = (this.activeRequest as MoveRequest).active[pokemon.position];
    // 		if (!req) throw new Error(`Pokemon not found in request's active field`);
    // 		return update(req) ?? true;
    // 	}
    //
    pub fn update_request_for_pokemon(&mut self, pokemon_idx: usize) {
        // In the full implementation, this would update request data
        // For now, just update disabled moves
        self.update_disabled_request(pokemon_idx);
    }
}
