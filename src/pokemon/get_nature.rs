// JS Source:
//
// 	getNature() {
// 		return this.battle.dex.natures.get(this.set.nature);
// 	}
//
// Note: In Rust, we store the nature string directly on Pokemon.
// The JavaScript version looks it up from dex, but we just return the stored value.

use crate::*;

impl Pokemon {
    /// Get nature
    /// Equivalent to getNature in pokemon.ts
    pub fn get_nature(&self) -> &str {
        // JS: return this.battle.dex.natures.get(this.set.nature);
        // In Rust: Pokemon.nature is already the nature string
        &self.nature
    }
}
