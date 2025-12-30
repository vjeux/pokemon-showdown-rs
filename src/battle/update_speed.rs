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
        // Two-phase approach: calculate speeds first, then update
        let mut speeds: Vec<((usize, usize), i32)> = Vec::new();

        for (side_idx, poke_idx) in &indices {
            if let Some(pokemon) = self.sides.get(*side_idx).and_then(|s| s.pokemon.get(*poke_idx)) {
                let new_speed = pokemon.get_action_speed(self);
                speeds.push(((*side_idx, *poke_idx), new_speed));
            }
        }

        // Apply the calculated speeds
        for ((side_idx, poke_idx), new_speed) in speeds {
            if let Some(pokemon) = self.sides.get_mut(side_idx).and_then(|s| s.pokemon.get_mut(poke_idx)) {
                pokemon.speed = new_speed;
            }
        }
    }
}
