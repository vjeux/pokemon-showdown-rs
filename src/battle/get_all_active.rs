use crate::*;

impl Battle {

    /// Get all active Pokemon
    /// Get all active Pokemon, optionally including fainted ones
    /// Equivalent to battle.ts getAllActive(includeFainted?)
    //
    // 	getAllActive(includeFainted?: boolean) {
    // 		const pokemonList: Pokemon[] = [];
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon && (includeFainted || !pokemon.fainted)) {
    // 					pokemonList.push(pokemon);
    // 				}
    // 			}
    // 		}
    // 		return pokemonList;
    // 	}
    //
    pub fn get_all_active(&self, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (side_idx, side) in self.sides.iter().enumerate() {
            for poke_idx in side.active.iter().flatten() {
                if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                    // JS: if (pokemon && (includeFainted || !pokemon.fainted))
                    if include_fainted || !pokemon.fainted {
                        result.push((side_idx, *poke_idx));
                    }
                }
            }
        }
        result
    }
}
