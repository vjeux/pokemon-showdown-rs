// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Get list of Pokemon that can switch in
    //
    // 	private possibleSwitches(side: Side) {
    // 		if (!side.pokemonLeft) return [];
    //
    // 		const canSwitchIn = [];
    // 		for (let i = side.active.length; i < side.pokemon.length; i++) {
    // 			const pokemon = side.pokemon[i];
    // 			if (!pokemon.fainted) {
    // 				canSwitchIn.push(pokemon);
    // 			}
    // 		}
    // 		return canSwitchIn;
    // 	}
    //
    pub fn possible_switches(&self, side_idx: usize) -> Vec<usize> {
        if let Some(side) = self.sides.get(side_idx) {
            // JS: if (!side.pokemonLeft) return [];
            if side.pokemon_left == 0 {
                return Vec::new();
            }

            let mut can_switch_in = Vec::new();
            // JavaScript iterates from active.length to pokemon.length and swaps positions when switching.
            // Rust doesn't swap positions (see switch_in.rs line 275-277), so we iterate ALL Pokemon
            // and exclude active ones explicitly.
            for (idx, pokemon) in side.pokemon.iter().enumerate() {
                // Skip active Pokemon (this check is needed because Rust doesn't swap positions)
                if pokemon.is_active {
                    continue;
                }
                // JS: if (!pokemon.fainted) { canSwitchIn.push(pokemon); }
                if !pokemon.is_fainted() {
                    can_switch_in.push(idx);
                }
            }

            // IMPORTANT: JavaScript's list is ordered by the swapped positions (which are stored in pokemon.position).
            // Since Rust doesn't swap the Vec, we need to sort by position to match JavaScript's order.
            can_switch_in.sort_by_key(|&idx| side.pokemon[idx].position);

            return can_switch_in;
        }
        Vec::new()
    }
}
