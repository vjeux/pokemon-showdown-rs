use crate::*;
use crate::dex_data::StatID;

impl Pokemon {

    /// Calculate a stat with boost
    //
    // 	calculateStat(statName: StatIDExceptHP, boost: number, modifier?: number, statUser?: Pokemon) {
    // 		statName = toID(statName) as StatIDExceptHP;
    // 		// @ts-expect-error type checking prevents 'hp' from being passed, but we're paranoid
    // 		if (statName === 'hp') throw new Error("Please read `maxhp` directly");
    //
    // 		// base stat
    // 		let stat = this.storedStats[statName];
    //
    // 		// Wonder Room swaps defenses before calculating anything else
    // 		if ('wonderroom' in this.battle.field.pseudoWeather) {
    // 			if (statName === 'def') {
    // 				stat = this.storedStats['spd'];
    // 			} else if (statName === 'spd') {
    // 				stat = this.storedStats['def'];
    // 			}
    // 		}
    //
    // 		// stat boosts
    // 		let boosts: SparseBoostsTable = {};
    // 		const boostName = statName as BoostID;
    // 		boosts[boostName] = boost;
    // 		boosts = this.battle.runEvent('ModifyBoost', statUser || this, null, null, boosts);
    // 		boost = boosts[boostName]!;
    // 		const boostTable = [1, 1.5, 2, 2.5, 3, 3.5, 4];
    // 		if (boost > 6) boost = 6;
    // 		if (boost < -6) boost = -6;
    // 		if (boost >= 0) {
    // 			stat = Math.floor(stat * boostTable[boost]);
    // 		} else {
    // 			stat = Math.floor(stat / boostTable[-boost]);
    // 		}
    //
    // 		// stat modifier
    // 		return this.battle.modify(stat, (modifier || 1));
    // 	}
    //
    pub fn calculate_stat(&self, stat: StatID, boost: i8, modifier: f64) -> i32 {
        if stat == StatID::HP {
            return self.maxhp;
        }

        // Get base stat
        let base_stat = self.stored_stats.get(stat);

        // Apply boost
        let clamped_boost = boost.clamp(-6, 6);
        let boost_table: [f64; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

        let boosted_stat = if clamped_boost >= 0 {
            (base_stat as f64 * boost_table[clamped_boost as usize]) as i32
        } else {
            (base_stat as f64 / boost_table[(-clamped_boost) as usize]) as i32
        };

        // Apply modifier
        ((boosted_stat as f64) * modifier) as i32
    }
}
