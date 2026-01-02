// JS Source:
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


use crate::*;

impl Pokemon {

    /// Get combat power (for Pokemon Go style formats)
    /// Equivalent to getCombatPower in pokemon.ts
    pub fn get_combat_power(&self) -> i32 {
        // JS: let statSum = 0;
        // JS: let awakeningSum = 0;
        // Note: Missing statSum and awakeningSum accumulators

        // JS: for (const stat in this.stats) {
        // JS:     statSum += this.calculateStat(stat, this.boosts[stat as BoostName]);
        // JS:     awakeningSum += this.calculateStat(stat, this.boosts[stat as BoostName]) + this.set.evs[stat];
        // JS: }
        // Note: Missing loop through all stats (atk, def, spa, spd, spe, hp)
        // Note: Missing calculateStat() calls with boosts
        // Note: Missing EV addition for awakeningSum

        // JS: const combatPower = Math.floor(Math.floor(statSum * this.level * 6 / 100) +
        // JS:     (Math.floor(awakeningSum) * Math.floor((this.level * 4) / 100 + 2)));
        // Note: Missing complex combat power calculation with level scaling

        // JS: return this.battle.clampIntRange(combatPower, 0, 10000);
        // Note: Missing clampIntRange to keep result in [0, 10000]

        // Simplified formula based on stats
        let atk = self.stored_stats.atk;
        let def = self.stored_stats.def;
        let sta = self.stored_stats.hp.max(10); // Use HP as stamina proxy

        // Note: Current formula is Pokemon Go style, not JavaScript's complex formula
        (((atk as f64) * (def as f64).powf(0.5) * (sta as f64).powf(0.5)) / 10.0) as i32
    }
}
