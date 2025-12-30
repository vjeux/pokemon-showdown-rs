use crate::*;
use crate::dex_data::StatsTable;

impl Battle {

    /// Calculate a single stat from base stat, IVs, EVs, level, and nature
    /// Equivalent to battle.ts statModify(baseStats, set, statName)
    ///
    //
    // 	statModify(baseStats: StatsTable, set: PokemonSet, statName: StatID): number {
    // 		const tr = this.trunc;
    // 		let stat = baseStats[statName];
    // 		if (statName === 'hp') {
    // 			return tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4) + 100) * set.level / 100 + 10);
    // 		}
    // 		stat = tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4)) * set.level / 100 + 5);
    // 		const nature = this.dex.natures.get(set.nature);
    // 		// Natures are calculated with 16-bit truncation.
    // 		// This only affects Eternatus-Eternamax in Pure Hackmons.
    // 		if (nature.plus === statName) {
    // 			stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 595) : stat;
    // 			stat = tr(tr(stat * 110, 16) / 100);
    // 		} else if (nature.minus === statName) {
    // 			stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 728) : stat;
    // 			stat = tr(tr(stat * 90, 16) / 100);
    // 		}
    // 		return stat;
    // 	}
    //
    pub fn stat_modify(&self, base_stats: &StatsTable, set: &PokemonSet, stat_name: &str) -> i32 {
        let base_stat = match stat_name {
            "hp" => base_stats.hp,
            "atk" => base_stats.atk,
            "def" => base_stats.def,
            "spa" => base_stats.spa,
            "spd" => base_stats.spd,
            "spe" => base_stats.spe,
            _ => return 0,
        };

        let iv = match stat_name {
            "hp" => set.ivs.hp,
            "atk" => set.ivs.atk,
            "def" => set.ivs.def,
            "spa" => set.ivs.spa,
            "spd" => set.ivs.spd,
            "spe" => set.ivs.spe,
            _ => 31,
        };

        let ev = match stat_name {
            "hp" => set.evs.hp,
            "atk" => set.evs.atk,
            "def" => set.evs.def,
            "spa" => set.evs.spa,
            "spd" => set.evs.spd,
            "spe" => set.evs.spe,
            _ => 0,
        };

        if stat_name == "hp" {
            // JS: return tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4) + 100) * set.level / 100 + 10);
            let ev_contrib = self.trunc(ev as f64 / 4.0, None) as i32;
            let inner = self.trunc((2 * base_stat + iv + ev_contrib + 100) as f64, None) as i32;
            return self.trunc(inner as f64 * set.level as f64 / 100.0 + 10.0, None) as i32;
        }

        // Non-HP stats
        // JS: stat = tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4)) * set.level / 100 + 5);
        let ev_contrib = self.trunc(ev as f64 / 4.0, None) as i32;
        let inner = self.trunc((2 * base_stat + iv + ev_contrib) as f64, None) as i32;
        let mut stat = self.trunc(inner as f64 * set.level as f64 / 100.0 + 5.0, None) as i32;

        // Apply nature
        // JS: const nature = this.dex.natures.get(set.nature);
        // JS: if (nature.plus === statName) {
        //       stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 595) : stat;
        //       stat = tr(tr(stat * 110, 16) / 100);
        //     }
        // JS: else if (nature.minus === statName) {
        //       stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 728) : stat;
        //       stat = tr(tr(stat * 90, 16) / 100);
        //     }
        if !set.nature.is_empty() {
            if let Some(nature_data) = self.dex.natures().get(&set.nature) {
                // Check if this stat is boosted by nature (+10%)
                if let Some(ref plus) = nature_data.plus {
                    if plus == stat_name {
                        // Apply overflow protection if rule exists
                        // This only affects Eternatus-Eternamax in Pure Hackmons
                        if let Some(ref rule_table) = self.rule_table {
                            if rule_table.has("overflowstatmod") {
                                stat = stat.min(595);
                            }
                        }
                        // Apply 1.1x multiplier with 16-bit truncation
                        // Natures are calculated with 16-bit truncation
                        stat = crate::dex::Dex::trunc(
                            crate::dex::Dex::trunc(stat as f64 * 110.0, 16) as f64 / 100.0,
                            0,
                        ) as i32;
                    }
                }

                // Check if this stat is reduced by nature (-10%)
                if let Some(ref minus) = nature_data.minus {
                    if minus == stat_name {
                        // Apply overflow protection if rule exists
                        if let Some(ref rule_table) = self.rule_table {
                            if rule_table.has("overflowstatmod") {
                                stat = stat.min(728);
                            }
                        }
                        // Apply 0.9x multiplier with 16-bit truncation
                        stat = crate::dex::Dex::trunc(
                            crate::dex::Dex::trunc(stat as f64 * 90.0, 16) as f64 / 100.0,
                            0,
                        ) as i32;
                    }
                }
            }
        }

        stat.max(0)
    }
}
