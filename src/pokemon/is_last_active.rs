use crate::*;

impl Pokemon {

    /// Check if this is the last active Pokemon on the side
    //
    // 	isLastActive() {
    // 		if (!this.isActive) return false;
    // 		const allyActive = this.side.active;
    // 		for (let i = this.position + 1; i < allyActive.length; i++) {
    // 			if (allyActive[i] && !allyActive[i].fainted) return false;
    // 		}
    // 		return true;
    // 	}
    //
    // ✅ NOW IMPLEMENTED: Full 1-to-1 with JavaScript
    // Refactored to associated function to access Battle for side.active check
    pub fn is_last_active(battle: &Battle, pokemon_pos: (usize, usize)) -> bool {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: if (!this.isActive) return false;
        if !pokemon.is_active {
            return false;
        }

        let position = pokemon.position;

        // JS: const allyActive = this.side.active;
        let side = &battle.sides[pokemon_pos.0];

        // JS: for (let i = this.position + 1; i < allyActive.length; i++) {
        for i in (position + 1)..side.active.len() {
            // JS:     if (allyActive[i] && !allyActive[i].fainted) return false;
            if let Some(pokemon_idx) = side.active[i] {
                if let Some(ally_pokemon) = side.pokemon.get(pokemon_idx) {
                    if !ally_pokemon.fainted {
                        return false;
                    }
                }
            }
        }

        // JS: return true;
        true
    }
}
