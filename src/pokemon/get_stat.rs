use crate::*;
use crate::event::EventResult;
use crate::dex_data::StatID;

impl Pokemon {

    /// Get a stat value with boosts applied
    //
    // 	getStat(statName: StatIDExceptHP, unboosted?: boolean, unmodified?: boolean) {
    // 		statName = toID(statName) as StatIDExceptHP;
    // 		// @ts-expect-error type checking prevents 'hp' from being passed, but we're paranoid
    // 		if (statName === 'hp') throw new Error("Please read `maxhp` directly");
    //
    // 		// base stat
    // 		let stat = this.storedStats[statName];
    //
    // 		// Download ignores Wonder Room's effect, but this results in
    // 		// stat stages being calculated on the opposite defensive stat
    // 		if (unmodified && 'wonderroom' in this.battle.field.pseudoWeather) {
    // 			if (statName === 'def') {
    // 				statName = 'spd';
    // 			} else if (statName === 'spd') {
    // 				statName = 'def';
    // 			}
    // 		}
    //
    // 		// stat boosts
    // 		if (!unboosted) {
    // 			let boosts = this.boosts;
    // 			if (!unmodified) {
    // 				boosts = this.battle.runEvent('ModifyBoost', this, null, null, { ...boosts });
    // 			}
    // 			let boost = boosts[statName];
    // 			const boostTable = [1, 1.5, 2, 2.5, 3, 3.5, 4];
    // 			if (boost > 6) boost = 6;
    // 			if (boost < -6) boost = -6;
    // 			if (boost >= 0) {
    // 				stat = Math.floor(stat * boostTable[boost]);
    // 			} else {
    // 				stat = Math.floor(stat / boostTable[-boost]);
    // 			}
    // 		}
    //
    // 		// stat modifier effects
    // 		if (!unmodified) {
    // 			const statTable: { [s in StatIDExceptHP]: string } = { atk: 'Atk', def: 'Def', spa: 'SpA', spd: 'SpD', spe: 'Spe' };
    // 			stat = this.battle.runEvent('Modify' + statTable[statName], this, null, null, stat);
    // 		}
    //
    // 		if (statName === 'spe' && stat > 10000 && !this.battle.format.battle?.trunc) stat = 10000;
    // 		return stat;
    // 	}
    //
    pub fn get_stat(&self, battle: &mut Battle, stat: StatID, unboosted: bool, unmodified: bool) -> i32 {
        // JS: let statName = toID(statName) as StatIDExceptHP;
        let mut stat_name = stat;

        // JS: Download ignores Wonder Room's effect, but this results in
        // JS: stat stages being calculated on the opposite defensive stat
        // ✅ NOW IMPLEMENTED: Wonder Room swap for unmodified (Download)
        if unmodified && battle.field.has_pseudo_weather(&ID::new("wonderroom")) {
            stat_name = match stat_name {
                StatID::Def => StatID::SpD,
                StatID::SpD => StatID::Def,
                _ => stat_name,
            };
        }

        // JS: let stat = this.storedStats[statName];
        let base_stat = self.stored_stats.get(stat_name);

        if unboosted {
            return base_stat;
        }

        // JS: let boost = boosts[statName];
        let boost = match stat_name {
            StatID::HP => return base_stat,
            StatID::Atk => self.boosts.atk,
            StatID::Def => self.boosts.def,
            StatID::SpA => self.boosts.spa,
            StatID::SpD => self.boosts.spd,
            StatID::Spe => self.boosts.spe,
        };

        // JS: const boostTable = [1, 1.5, 2, 2.5, 3, 3.5, 4];
        let boost_table: [f64; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

        // JS: if (boost > 6) boost = 6; if (boost < -6) boost = -6;
        let clamped_boost = boost.clamp(-6, 6);

        // JS: if (boost >= 0) { stat = Math.floor(stat * boostTable[boost]); } else { stat = Math.floor(stat / boostTable[-boost]); }
        let mut stat_value = if clamped_boost >= 0 {
            ((base_stat as f64) * boost_table[clamped_boost as usize]).floor() as i32
        } else {
            ((base_stat as f64) / boost_table[(-clamped_boost) as usize]).floor() as i32
        };

        // JS: if (!unmodified) {
        //         const statTable: { [s in StatIDExceptHP]: string } = { atk: 'Atk', def: 'Def', spa: 'SpA', spd: 'SpD', spe: 'Spe' };
        //         stat = this.battle.runEvent('Modify' + statTable[statName], this, null, null, stat);
        //     }
        if !unmodified {
            let event_name = match stat_name {
                StatID::Atk => "ModifyAtk",
                StatID::Def => "ModifyDef",
                StatID::SpA => "ModifySpA",
                StatID::SpD => "ModifySpD",
                StatID::Spe => "ModifySpe",
                StatID::HP => return stat_value, // HP never has Modify event
            };

            // Get pokemon position for event
            let pokemon_pos = (self.side_index, self.position);

            // Run the Modify* event (e.g., ModifySpe for Slow Start)
            if let EventResult::Number(modified_stat) = battle.run_event(event_name, Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Number(stat_value), false, false) {
                stat_value = modified_stat;
            }
        }

        // JS: if (statName === 'spe' && stat > 10000 && !this.battle.format.battle?.trunc) stat = 10000;
        if stat_name == StatID::Spe && stat_value > 10000 {
            stat_value = 10000;
        }

        stat_value
    }
}
