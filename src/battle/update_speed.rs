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

        // IMPORTANT: We must update each Pokemon's speed IMMEDIATELY after calculating it,
        // just like JavaScript does. This is because during speed calculation, events like
        // ModifyBoost are triggered, and their handlers read pokemon.speed from OTHER Pokemon.
        // If we batch all calculations first, handlers will see stale cached speeds.
        //
        // JavaScript behavior:
        //   for (const pokemon of this.getAllActive()) {
        //       pokemon.updateSpeed();  // Updates pokemon.speed IMMEDIATELY
        //   }
        for (side_idx, poke_idx) in &indices {
            if self.sides.get(*side_idx).and_then(|s| s.pokemon.get(*poke_idx)).is_some() {
                // Calculate speed (may trigger events that read other Pokemon's speeds)
                let new_speed = self.get_pokemon_action_speed(*side_idx, *poke_idx);
                // Update IMMEDIATELY before processing next Pokemon
                if let Some(pokemon) = self.sides.get_mut(*side_idx).and_then(|s| s.pokemon.get_mut(*poke_idx)) {
                    pokemon.speed = new_speed;
                }
            }
        }
    }
}
