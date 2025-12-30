use crate::*;
use std::collections::HashMap;

impl Pokemon {

    /// Boost stats by amounts, respecting caps
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
    pub fn boost_by(&mut self, boosts: HashMap<crate::dex_data::BoostID, i8>) -> i8 {
        // JS: boosts = this.getCappedBoost(boosts);
        let capped_boosts = self.get_capped_boost(boosts);
        let mut delta = 0;

        // JS: for (boostName in boosts) { delta = boosts[boostName]!; this.boosts[boostName] += delta; }
        for (boost_name, boost_delta) in capped_boosts {
            delta = boost_delta;
            self.boosts.boost(boost_name, boost_delta);

            // Track if stats were raised or lowered this turn
            if boost_delta > 0 {
                self.stats_raised_this_turn = true;
            } else if boost_delta < 0 {
                self.stats_lowered_this_turn = true;
            }
        }

        // JS: return delta;
        delta
    }
}
