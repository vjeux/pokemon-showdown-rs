use crate::*;

impl Pokemon {

    /// Check if Pokemon is semi-invulnerable (Fly, Dig, Dive, etc.)
    //
    // 	isSemiInvulnerable() {
    // 		return (this.volatiles['fly'] || this.volatiles['bounce'] || this.volatiles['dive'] || this.volatiles['dig'] ||
    // 			this.volatiles['phantomforce'] || this.volatiles['shadowforce'] || this.isSkyDropped());
    // 	}
    //
    // ✅ NOW IMPLEMENTED: Full 1-to-1 with JavaScript
    // Refactored to associated function to properly call is_sky_dropped with Battle
    pub fn is_semi_invulnerable(battle: &Battle, pokemon_pos: (usize, usize)) -> bool {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: return (this.volatiles['fly'] || this.volatiles['bounce'] || this.volatiles['dive'] || this.volatiles['dig'] || this.volatiles['phantomforce'] || this.volatiles['shadowforce'] || this.isSkyDropped());
        pokemon.has_volatile(&ID::new("fly"))
            || pokemon.has_volatile(&ID::new("bounce"))
            || pokemon.has_volatile(&ID::new("dive"))
            || pokemon.has_volatile(&ID::new("dig"))
            || pokemon.has_volatile(&ID::new("phantomforce"))
            || pokemon.has_volatile(&ID::new("shadowforce"))
            || Pokemon::is_sky_dropped(battle, pokemon_pos)
    }
}
