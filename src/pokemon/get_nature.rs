use crate::*;

impl Pokemon {

    /// Get nature
    /// Equivalent to getNature in pokemon.ts
    /// Note: Nature is applied at stat calculation time; we return default here
    //
    // 	getNature() {
    // 		return this.battle.dex.natures.get(this.set.nature);
    // 	}
    //
    pub fn get_nature(&self) -> &str {
        // TODO: implement the same logic as JavaScript
        
        // In battle, the nature is already applied to stored_stats
        // The actual nature value would need to be stored if needed
        "Hardy" // Default neutral nature
    }
}
