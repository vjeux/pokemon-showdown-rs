// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::event::EventResult;
use crate::dex_data::StatID;

impl Battle {

    /// Get a Pokemon's stat value with boosts applied
    /// Equivalent to pokemon.ts getStat() (pokemon.ts:20-63)
    ///
    /// JavaScript source:
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
    pub fn get_pokemon_stat(&mut self, pokemon_pos: (usize, usize), stat: StatID, unboosted: bool, unmodified: bool) -> i32 {
        // JS: let stat = this.storedStats[statName];
        // Get pokemon data we need
        let (base_stat, boost) = {
            let pokemon = match self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return 0,
            };

            let base_stat = pokemon.stored_stats.get(stat);
            if unboosted {
                return base_stat;
            }

            // JS: let boost = boosts[statName];
            let boost = match stat {
                StatID::HP => return base_stat,
                StatID::Atk => pokemon.boosts.atk,
                StatID::Def => pokemon.boosts.def,
                StatID::SpA => pokemon.boosts.spa,
                StatID::SpD => pokemon.boosts.spd,
                StatID::Spe => pokemon.boosts.spe,
            };

            (base_stat, boost)
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
            let event_name = match stat {
                StatID::Atk => "ModifyAtk",
                StatID::Def => "ModifyDef",
                StatID::SpA => "ModifySpA",
                StatID::SpD => "ModifySpD",
                StatID::Spe => "ModifySpe",
                StatID::HP => return stat_value, // HP never has Modify event
            };

            // Run the Modify* event (e.g., ModifySpe for Slow Start)
            if let EventResult::Number(modified_stat) = self.run_event(event_name, Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Number(stat_value), false, false) {
                stat_value = modified_stat;
            }
        }

        // JS: if (statName === 'spe' && stat > 10000 && !this.battle.format.battle?.trunc) stat = 10000;
        if stat == StatID::Spe && stat_value > 10000 {
            stat_value = 10000;
        }

        stat_value
    }
}
