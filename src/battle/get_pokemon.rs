use crate::*;

impl Battle {

    // =========================================================================
    // Pokemon Lookup Methods (ported from battle.ts)
    // =========================================================================

    /// Get a Pokemon by its full name (e.g., "p1: Pikachu")
    /// Equivalent to battle.ts getPokemon()
    //
    // 	getPokemon(fullname: string | Pokemon) {
    // 		if (typeof fullname !== 'string') fullname = fullname.fullname;
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.pokemon) {
    // 				if (pokemon.fullname === fullname) return pokemon;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn get_pokemon(&self, fullname: &str) -> Option<(usize, usize, &crate::pokemon::Pokemon)> {
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (poke_idx, pokemon) in side.pokemon.iter().enumerate() {
                let poke_fullname = format!("{}: {}", side.id_str(), pokemon.name);
                if poke_fullname == fullname {
                    return Some((side_idx, poke_idx, pokemon));
                }
            }
        }
        None
    }
}
