use crate::*;

impl Pokemon {

    /// Check if Pokemon is in Sky Drop
    //
    // 	isSkyDropped() {
    // 		if (this.volatiles['skydrop']) return true;
    // 		for (const foeActive of this.side.foe.active) {
    // 			if (foeActive.volatiles['skydrop'] && foeActive.volatiles['skydrop'].source === this) {
    // 				return true;
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    // ✅ NOW IMPLEMENTED: Full 1-to-1 with JavaScript
    // Refactored to associated function to access Battle for foe.active check
    pub fn is_sky_dropped(battle: &Battle, pokemon_pos: (usize, usize)) -> bool {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: if (this.volatiles['skydrop']) return true;
        if pokemon.has_volatile(&ID::new("skydrop")) {
            return true;
        }

        // JS: for (const foeActive of this.side.foe.active) {
        let foe_side_indices: Vec<usize> = (0..battle.sides.len())
            .filter(|&idx| idx != pokemon.side_index && !Pokemon::is_ally(battle, pokemon_pos, idx))
            .collect();

        for foe_side_idx in foe_side_indices {
            if let Some(foe_side) = battle.sides.get(foe_side_idx) {
                for foe_pokemon_idx in foe_side.active.iter().flatten() {
                    if let Some(foe_pokemon) = foe_side.pokemon.get(*foe_pokemon_idx) {
                        // JS:     if (foeActive.volatiles['skydrop'] && foeActive.volatiles['skydrop'].source === this) {
                        if let Some(skydrop_volatile) = foe_pokemon.volatiles.get(&ID::new("skydrop")) {
                            if skydrop_volatile.borrow().source == Some(pokemon_pos) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        // JS: return false;
        false
    }
}
