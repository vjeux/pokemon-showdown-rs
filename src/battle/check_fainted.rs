use crate::*;

impl Battle {

    /// Check all active Pokemon for fainting and update their status
    /// Equivalent to battle.ts checkFainted() (battle.ts:2487-2496)
    ///
    //
    // 	checkFainted() {
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon.fainted) {
    // 					pokemon.status = 'fnt' as ID;
    // 					pokemon.switchFlag = true;
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn check_fainted(&mut self) {
        eprintln!("[CHECK_FAINTED] Called, turn={}", self.turn);
        // JS: for (const side of this.sides) {
        for side_idx in 0..self.sides.len() {
            // JS: for (const pokemon of side.active) {
            // Collect active Pokemon indices first to avoid borrow issues
            let active_indices: Vec<usize> = self.sides[side_idx]
                .active
                .iter()
                .filter_map(|&opt_idx| opt_idx)
                .collect();

            for poke_idx in active_indices {
                if let Some(pokemon) = self.sides[side_idx].pokemon.get_mut(poke_idx) {
                    // JS: if (pokemon.fainted) {
                    if pokemon.fainted {
                        eprintln!("[CHECK_FAINTED] Found fainted Pokemon: side={}, poke={}, name={}",
                            side_idx, poke_idx, pokemon.name);
                        // JS: pokemon.status = 'fnt' as ID;
                        pokemon.status = ID::new("fnt");
                        // JS: pokemon.switchFlag = true;
                        pokemon.switch_flag = Some(String::new()); // Generic switch (fainted)
                    }
                }
            }
        }
    }
}
