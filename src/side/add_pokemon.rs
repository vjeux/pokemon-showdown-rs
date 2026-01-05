use crate::side::*;

impl Side {

    /// Add a Pokemon to the team
    //
    // 	addPokemon(set: PokemonSet) {
    // 		if (this.pokemon.length >= 24) return null;
    // 		const newPokemon = new Pokemon(set, this);
    // 		newPokemon.position = this.pokemon.length;
    // 		this.pokemon.push(newPokemon);
    // 		this.pokemonLeft++;
    // 		return newPokemon;
    // 	}
    //
    pub fn add_pokemon(&mut self, set: PokemonSet, dex: &crate::dex::Dex) -> Option<usize> {
        if self.pokemon.len() >= 24 {
            return None;
        }
        let pos = self.pokemon.len();
        let pokemon = Pokemon::new(&set, self.n, pos, dex);
        self.pokemon.push(pokemon);
        self.team.push(set);
        self.pokemon_left += 1;
        Some(pos)
    }
}
