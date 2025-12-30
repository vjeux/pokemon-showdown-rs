use crate::*;

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
    pub fn get_capped_boost(&self, stat: crate::dex_data::BoostID, amount: i8) -> i8 {
        let current = self.boosts.get(stat);
        let new_value = (current + amount).clamp(-6, 6);
        new_value - current
    }
}
