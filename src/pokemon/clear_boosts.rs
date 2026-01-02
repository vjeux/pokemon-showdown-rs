// JS Source:
//
// 	clearBoosts() {
// 		let boostName: BoostID;
// 		for (boostName in this.boosts) {
// 			this.boosts[boostName] = 0;
// 		}
// 	}
//
// Note: In Rust, BoostsTable is a struct with fields, not a HashMap.
// Using Default sets all i8 fields to 0.

use crate::*;
use crate::dex_data::BoostsTable;

impl Pokemon {
    /// Clear all boosts
    /// Equivalent to pokemon.ts clearBoosts()
    pub fn clear_boosts(&mut self) {
        // JS: for (boostName in this.boosts) { this.boosts[boostName] = 0; }
        self.boosts = BoostsTable::default();
    }
}
