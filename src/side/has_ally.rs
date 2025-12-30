use crate::side::*;

impl Side {

    /// Check if a Pokemon is an ally
    // 	hasAlly(pokemon: Pokemon) {
    // 		return pokemon.side === this || pokemon.side === this.allySide;
    // 	}
    //
    pub fn has_ally(&self, pokemon_side_index: usize) -> bool {
        pokemon_side_index == self.n || self.ally_index == Some(pokemon_side_index)
    }
}
