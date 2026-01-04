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
        // JS: const weighthg = this.battle.runEvent('ModifyWeight', this, null, null, this.weighthg);
        // JS: return Math.max(1, weighthg);
        //
        // Note: In Rust, we cannot call runEvent without Battle reference.
        // For now, return the base weight. Full implementation would require:
        // - Refactoring to associated function: Pokemon::get_weight(battle, pokemon_pos)
        // - Calling battle.run_event_compat('ModifyWeight', pokemon_pos, self.weight_hg)
        // - Returning max(1, modified_weight)
        self.weight_hg.max(1)
    }
}
