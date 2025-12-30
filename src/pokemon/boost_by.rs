use crate::*;

impl Pokemon {

    /// Boost stat by amount, respecting caps
    //
    // 	boostBy(boosts: SparseBoostsTable) {
    // 		boosts = this.getCappedBoost(boosts);
    // 		let delta = 0;
    // 		let boostName: BoostID;
    // 		for (boostName in boosts) {
    // 			delta = boosts[boostName]!;
    // 			this.boosts[boostName] += delta;
    // 		}
    // 		return delta;
    // 	}
    //
    pub fn boost_by(&mut self, stat: crate::dex_data::BoostID, amount: i8) -> i8 {
        let delta = self.get_capped_boost(stat, amount);
        self.boosts.boost(stat, delta);
        if delta > 0 {
            self.stats_raised_this_turn = true;
        } else if delta < 0 {
            self.stats_lowered_this_turn = true;
        }
        delta
    }
}
