use crate::*;

impl Pokemon {

    /// Check if this Pokemon is an ally of another
    //
    // 	isAlly(pokemon: Pokemon | null) {
    // 		return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);
    // 	}
    //
    pub fn is_ally(&self, other_side_index: usize) -> bool {
        self.side_index == other_side_index
    }
}
