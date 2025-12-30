use crate::*;

impl Pokemon {

    /// Get positive boost count (for Stored Power, etc.)
    //
    // 	positiveBoosts() {
    // 		let boosts = 0;
    // 		let boost: BoostID;
    // 		for (boost in this.boosts) {
    // 			if (this.boosts[boost] > 0) boosts += this.boosts[boost];
    // 		}
    // 		return boosts;
    // 	}
    //
    pub fn positive_boosts(&self) -> i32 {
        let mut count = 0;
        if self.boosts.atk > 0 {
            count += self.boosts.atk as i32;
        }
        if self.boosts.def > 0 {
            count += self.boosts.def as i32;
        }
        if self.boosts.spa > 0 {
            count += self.boosts.spa as i32;
        }
        if self.boosts.spd > 0 {
            count += self.boosts.spd as i32;
        }
        if self.boosts.spe > 0 {
            count += self.boosts.spe as i32;
        }
        if self.boosts.accuracy > 0 {
            count += self.boosts.accuracy as i32;
        }
        if self.boosts.evasion > 0 {
            count += self.boosts.evasion as i32;
        }
        count
    }
}
