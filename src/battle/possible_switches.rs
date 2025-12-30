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
            let mut can_switch_in = Vec::new();
            for (idx, pokemon) in side.pokemon.iter().enumerate() {
                // Skip active Pokemon
                if pokemon.is_active {
                    continue;
                }
                if !pokemon.is_fainted() {
                    can_switch_in.push(idx);
                }
            }
            return can_switch_in;
        }
        Vec::new()
    }
}
