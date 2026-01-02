use crate::*;

impl Pokemon {

    /// Check if this Pokemon is an ally of another
    //
    // 	isAlly(pokemon: Pokemon | null) {
    // 		return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);
    // 	}
    //
    pub fn is_ally(&self, other_side_index: usize) -> bool {
        // JS: return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);
        //
        // Note: Full implementation would require Battle reference to check allySide
        // Currently only checks if on same side, missing multi-battle ally check
        // Would need: battle.sides[self.side_index].ally_side == Some(other_side_index)
        self.side_index == other_side_index
    }
}
