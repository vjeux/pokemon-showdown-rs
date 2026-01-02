use crate::*;
use std::collections::HashMap;

impl Pokemon {

    /// Get capped boost - returns the actual change that would be applied
    //
    // 	getCappedBoost(boosts: SparseBoostsTable) {
    // 		const cappedBoost: SparseBoostsTable = {};
    // 		let boostName: BoostID;
    // 		for (boostName in boosts) {
    // 			const boost = boosts[boostName];
    // 			if (!boost) continue;
    // 			cappedBoost[boostName] = this.battle.clampIntRange(this.boosts[boostName] + boost, -6, 6) - this.boosts[boostName];
    // 		}
    // 		return cappedBoost;
    // 	}
    //
    pub fn get_capped_boost(&self, boosts: HashMap<crate::dex_data::BoostID, i8>) -> HashMap<crate::dex_data::BoostID, i8> {
        let mut capped_boost = HashMap::new();
        for (boost_name, boost) in boosts {
            if boost == 0 {
                continue;
            }
            let current = self.boosts.get(boost_name);
            let new_value = (current + boost).clamp(-6, 6);
            capped_boost.insert(boost_name, new_value - current);
        }
        capped_boost
    }
}
