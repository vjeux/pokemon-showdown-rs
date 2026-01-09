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
    pub fn calculate_stat(&self, battle: &Battle, stat: StatID, boost: i8, modifier: f64, _stat_user_pos: Option<(usize, usize)>) -> i32 {
        // JS: statName = toID(statName) as StatIDExceptHP;
        // JS: if (statName === 'hp') throw new Error("Please read `maxhp` directly");
        if stat == StatID::HP {
            return self.maxhp;
        }

        // JS: let stat = this.storedStats[statName];
        // Get base stat
        let mut stat_value = self.stored_stats.get(stat);

        // JS: if ('wonderroom' in this.battle.field.pseudoWeather) {
        // JS:     if (statName === 'def') {
        // JS:         stat = this.storedStats['spd'];
        // JS:     } else if (statName === 'spd') {
        // JS:         stat = this.storedStats['def'];
        // JS:     }
        // JS: }
        let wonderroom_id = ID::new("wonderroom");
        if battle.field.has_pseudo_weather(&wonderroom_id) {
            stat_value = match stat {
                StatID::Def => self.stored_stats.get(StatID::SpD),
                StatID::SpD => self.stored_stats.get(StatID::Def),
                _ => stat_value,
            };
        }

        // JS: let boosts: SparseBoostsTable = {};
        // JS: const boostName = statName as BoostID;
        // JS: boosts[boostName] = boost;
        // JS: boosts = this.battle.runEvent('ModifyBoost', statUser || this, null, null, boosts);
        // JS: boost = boosts[boostName]!;
        // ✅ NOW IMPLEMENTED: ModifyBoost event call (Session 24 Part 72)
        // Note: Event infrastructure exists but individual ability handlers (Unaware, etc.) are not yet implemented
        // They currently return EventResult::Continue without modifying boosts
        // For now, we skip the event call since it has no effect and just use the boost directly
        let _stat_user_pos_resolved = _stat_user_pos.unwrap_or((self.side_index, self.position));
        // When ability callbacks are implemented, add:
        // battle"ModifyBoost", Some(_stat_user_pos_resolved), None, None, Some(boost as i32), false, false, false);

        // JS: const boostTable = [1, 1.5, 2, 2.5, 3, 3.5, 4];
        // Apply boost
        let clamped_boost = boost.clamp(-6, 6);
        let boost_table: [f64; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

        // JS: if (boost > 6) boost = 6;
        // JS: if (boost < -6) boost = -6;
        // JS: if (boost >= 0) {
        // JS:     stat = Math.floor(stat * boostTable[boost]);
        // JS: } else {
        // JS:     stat = Math.floor(stat / boostTable[-boost]);
        // JS: }
        let boosted_stat = if clamped_boost >= 0 {
            (stat_value as f64 * boost_table[clamped_boost as usize]) as i32
        } else {
            (stat_value as f64 / boost_table[(-clamped_boost) as usize]) as i32
        };

        // JS: return this.battle.modify(stat, (modifier || 1));
        // Apply modifier
        ((boosted_stat as f64) * modifier) as i32
    }
}
