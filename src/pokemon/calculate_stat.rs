use crate::*;
use crate::dex_data::{StatID, BoostsTable};
use crate::event::EventResult;

impl Battle {

    /// Calculate a stat with boost
    /// Equivalent to pokemon.calculateStat in pokemon.ts
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
    pub fn calculate_stat(&mut self, pokemon_pos: (usize, usize), stat: StatID, boost: i8, modifier: f64, stat_user_pos: Option<(usize, usize)>) -> i32 {
        // JS: statName = toID(statName) as StatIDExceptHP;
        // JS: if (statName === 'hp') throw new Error("Please read `maxhp` directly");
        if stat == StatID::HP {
            return self.pokemon_at(pokemon_pos.0, pokemon_pos.1)
                .map(|p| p.maxhp)
                .unwrap_or(0);
        }

        // JS: let stat = this.storedStats[statName];
        // Get base stat and check Wonder Room
        let stat_value = {
            let pokemon = match self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return 0,
            };

            let base_stat = pokemon.stored_stats.get(stat);

            // JS: if ('wonderroom' in this.battle.field.pseudoWeather) {
            // JS:     if (statName === 'def') {
            // JS:         stat = this.storedStats['spd'];
            // JS:     } else if (statName === 'spd') {
            // JS:         stat = this.storedStats['def'];
            // JS:     }
            // JS: }
            let wonderroom_id = ID::new("wonderroom");
            if self.field.has_pseudo_weather(&wonderroom_id) {
                match stat {
                    StatID::Def => pokemon.stored_stats.get(StatID::SpD),
                    StatID::SpD => pokemon.stored_stats.get(StatID::Def),
                    _ => base_stat,
                }
            } else {
                base_stat
            }
        };

        // JS: let boosts: SparseBoostsTable = {};
        // JS: const boostName = statName as BoostID;
        // JS: boosts[boostName] = boost;
        // JS: boosts = this.battle.runEvent('ModifyBoost', statUser || this, null, null, boosts);
        // JS: boost = boosts[boostName]!;

        // Create boosts table with just this stat's boost set
        let boost_id = match stat.to_boost_id() {
            Some(b) => b,
            None => return stat_value, // HP case already handled above
        };

        let mut boosts_table = BoostsTable::default();
        boosts_table.set(boost_id, boost);

        // Run ModifyBoost event - allows abilities like Unaware to modify boosts
        let stat_user = stat_user_pos.unwrap_or(pokemon_pos);
        let modified_boosts = self.run_event(
            "ModifyBoost",
            Some(crate::event::EventTarget::Pokemon(stat_user)),
            None,
            None,
            EventResult::Boost(boosts_table),
            false,
            false,
        ).boost().unwrap_or(boosts_table);

        // Extract the (possibly modified) boost for this stat
        let final_boost = modified_boosts.get(boost_id);

        // JS: const boostTable = [1, 1.5, 2, 2.5, 3, 3.5, 4];
        // Apply boost
        let clamped_boost = final_boost.clamp(-6, 6);
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
