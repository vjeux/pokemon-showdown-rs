use crate::*;

impl Pokemon {

    /// Clear all boosts
    //
    // 	clearBoosts() {
    // 		let boostName: BoostID;
    // 		for (boostName in this.boosts) {
    // 			this.boosts[boostName] = 0;
    // 		}
    // 	}
    //
    pub fn clear_boosts(&mut self) {
        self.boosts.clear();
    }
}
