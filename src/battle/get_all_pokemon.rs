use crate::*;

impl Battle {

    /// Get all Pokemon in the battle (not just active)
    /// Equivalent to battle.ts getAllPokemon()
    //
    // 	getAllPokemon() {
    // 		const pokemonList: Pokemon[] = [];
    // 		for (const side of this.sides) {
    // 			pokemonList.push(...side.pokemon);
    // 		}
    // 		return pokemonList;
    // 	}
    //
    pub fn get_all_pokemon(&self) -> Vec<&crate::pokemon::Pokemon> {
        let mut result = Vec::new();
        for side in &self.sides {
            for pokemon in &side.pokemon {
                result.push(pokemon);
            }
        }
        result
    }
}
