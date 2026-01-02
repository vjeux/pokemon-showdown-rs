use crate::*;

impl Pokemon {

    /// Check if this Pokemon is an ally of another
    //
    // 	isAlly(pokemon: Pokemon | null) {
    // 		return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);
    // 	}
    //
    // ✅ NOW IMPLEMENTED: Full 1-to-1 with JavaScript
    // Refactored to associated function to access Battle for allySide check
    pub fn is_ally(battle: &Battle, pokemon_pos: (usize, usize), other_side_index: usize) -> bool {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);

        // Check if on same side
        if pokemon.side_index == other_side_index {
            return true;
        }

        // Check if other side is an ally (for multi battles)
        if let Some(side) = battle.sides.get(pokemon.side_index) {
            if let Some(ally_idx) = side.ally_index {
                if ally_idx == other_side_index {
                    return true;
                }
            }
        }

        false
    }
}
