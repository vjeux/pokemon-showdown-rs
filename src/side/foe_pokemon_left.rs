use crate::side::*;

impl Side {

    /// Get total Pokemon left on foe side
    // 	foePokemonLeft() {
    // 		if (this.battle.gameType === 'freeforall') {
    // 			return this.battle.sides.filter(side => side !== this).map(side => side.pokemonLeft).reduce((a, b) => a + b);
    // 		}
    //
    // 		if (this.foe.allySide) return this.foe.pokemonLeft + this.foe.allySide.pokemonLeft;
    //
    // 		return this.foe.pokemonLeft;
    // 	}
    //
    pub fn foe_pokemon_left(&self) -> usize {
        // This is a stub - needs battle context
        0
    }
}
