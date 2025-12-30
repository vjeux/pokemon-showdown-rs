use crate::*;
use std::collections::HashMap;

impl Pokemon {

    /// Set boost values
    //
    // 	setBoost(boosts: SparseBoostsTable) {
    // 		let boostName: BoostID;
    // 		for (boostName in boosts) {
    // 			this.boosts[boostName] = boosts[boostName]!;
    // 		}
    // 	}
    //
    pub fn set_boost(&mut self, boosts: HashMap<crate::dex_data::BoostID, i8>) {
        // JS: for (boostName in boosts) { this.boosts[boostName] = boosts[boostName]!; }
        for (boost_name, value) in boosts {
            self.boosts.set(boost_name, value);
        }
    }
}
