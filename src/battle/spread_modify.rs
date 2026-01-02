// JS Source:
// 	spreadModify(baseStats: StatsTable, set: PokemonSet): StatsTable {
// 		const modStats: StatsTable = { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
// 		for (const statName in baseStats) {
// 			modStats[statName as StatID] = this.statModify(baseStats, set, statName as StatID);
// 		}
// 		return modStats;
// 	}


use crate::*;
use crate::dex_data::StatsTable;

impl Battle {

    /// Calculate modified stats from base stats
    /// Equivalent to battle.ts spreadModify(baseStats, set)
    ///
    // TypeScript source:
    // /** Given a table of base stats and a pokemon set, return the actual stats. */
    // 	spreadModify(baseStats: StatsTable, set: PokemonSet): StatsTable {
    // 		const modStats: StatsTable = { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
    // 		for (const statName in baseStats) {
    // 			modStats[statName as StatID] = this.statModify(baseStats, set, statName as StatID);
    // 		}
    // 		return modStats;
    // 	}
    //
    pub fn spread_modify(&self, base_stats: &StatsTable, set: &PokemonSet) -> StatsTable {
        StatsTable {
            hp: self.stat_modify(base_stats, set, "hp"),
            atk: self.stat_modify(base_stats, set, "atk"),
            def: self.stat_modify(base_stats, set, "def"),
            spa: self.stat_modify(base_stats, set, "spa"),
            spd: self.stat_modify(base_stats, set, "spd"),
            spe: self.stat_modify(base_stats, set, "spe"),
        }
    }
}
