use crate::*;

impl Pokemon {

    /// Set a specific boost value
    //
    // 	setBoost(boosts: SparseBoostsTable) {
    // 		let boostName: BoostID;
    // 		for (boostName in boosts) {
    // 			this.boosts[boostName] = boosts[boostName]!;
    // 		}
    // 	}
    //
    pub fn set_boost(&mut self, stat: crate::dex_data::BoostID, value: i8) {
        let clamped = value.clamp(-6, 6);
        self.boosts.set(stat, clamped);
    }
}
