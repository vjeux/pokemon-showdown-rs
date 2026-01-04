// JS Source:
//
// 	getNature() {
// 		return this.battle.dex.natures.get(this.set.nature);
// 	}
//

use crate::*;

impl Pokemon {
    /// Get nature
    /// Equivalent to getNature in pokemon.ts
    pub fn get_nature(&self) -> &str {
        // JS: return this.battle.dex.natures.get(this.set.nature);
        // Returns the nature string from the PokemonSet
        &self.set.nature
    }
}
