use crate::*;

impl Battle {

    /// Update speed for all active Pokemon
    /// Equivalent to battle.ts updateSpeed()
    ///
    //
    // 	updateSpeed() {
    // 		for (const pokemon of this.getAllActive()) {
    // 			pokemon.updateSpeed();
    // 		}
    // 	}
    //
    /// Update speed for all active Pokemon
    /// Equivalent to TypeScript updateSpeed() (battle.ts:387-391)
    pub fn update_speed(&mut self) {
        // Collect indices first to avoid borrow checker issues
        let indices: Vec<(usize, usize)> = self.get_all_active(false);

        // Update each active Pokemon's speed
        for (side_idx, poke_idx) in indices {
            if let Some(side) = self.sides.get_mut(side_idx) {
                if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                    pokemon.update_speed();
                }
            }
        }
    }
}
