use crate::*;

impl Pokemon {

    /// Get the weight in hectograms
    //
    // 	/* Commented out for now until a use for Combat Power is found in Let's Go
    // 	getCombatPower() {
    // 		let statSum = 0;
    // 		let awakeningSum = 0;
    // 		for (const stat in this.stats) {
    // 			statSum += this.calculateStat(stat, this.boosts[stat as BoostName]);
    // 			awakeningSum += this.calculateStat(
    // 				stat, this.boosts[stat as BoostName]) + this.set.evs[stat];
    // 		}
    // 		const combatPower = Math.floor(Math.floor(statSum * this.level * 6 / 100) +
    // 			(Math.floor(awakeningSum) * Math.floor((this.level * 4) / 100 + 2)));
    // 		return this.battle.clampIntRange(combatPower, 0, 10000);
    // 	}
    // 	*/
    //
    // 	getWeight() {
    // 		const weighthg = this.battle.runEvent('ModifyWeight', this, null, null, this.weighthg);
    // 		return Math.max(1, weighthg);
    // 	}
    //
    pub fn get_weight(&self) -> i32 {
        // TODO: implement the same logic as JavaScript
        
        // Base weight would come from species data
        // For now return stored weight
        self.weight_hg
    }
}
