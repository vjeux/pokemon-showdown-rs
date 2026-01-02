// JS Source:
// 	getBestStat(unboosted?: boolean, unmodified?: boolean): StatIDExceptHP {
// 		let statName: StatIDExceptHP = 'atk';
// 		let bestStat = 0;
// 		const stats: StatIDExceptHP[] = ['atk', 'def', 'spa', 'spd', 'spe'];
// 		for (const i of stats) {
// 			if (this.getStat(i, unboosted, unmodified) > bestStat) {
// 				statName = i;
// 				bestStat = this.getStat(i, unboosted, unmodified);
// 			}
// 		}
// 
// 		return statName;
// 	}


use crate::*;
use crate::dex_data::StatID;

impl Pokemon {

    /// Get best stat (for Beast Boost, Quark Drive, Protosynthesis)
    // TypeScript source:
    // /**
    // 	 * Gets the Pokemon's best stat.
    // 	 * Moved to its own method due to frequent use of the same code.
    // 	 * Used by Beast Boost, Quark Drive, and Protosynthesis.
    // 	 */
    // 	getBestStat(unboosted?: boolean, unmodified?: boolean): StatIDExceptHP {
    // 		let statName: StatIDExceptHP = 'atk';
    // 		let bestStat = 0;
    // 		const stats: StatIDExceptHP[] = ['atk', 'def', 'spa', 'spd', 'spe'];
    // 		for (const i of stats) {
    // 			if (this.getStat(i, unboosted, unmodified) > bestStat) {
    // 				statName = i;
    // 				bestStat = this.getStat(i, unboosted, unmodified);
    // 			}
    // 		}
    //
    // 		return statName;
    // 	}
    //
    pub fn get_best_stat(&self, battle: &mut Battle, unboosted: bool, unmodified: bool) -> StatID {
        let stats = [
            StatID::Atk,
            StatID::Def,
            StatID::SpA,
            StatID::SpD,
            StatID::Spe,
        ];
        let mut best_stat = StatID::Atk;
        let mut best_value = 0;

        let pokemon_pos = (self.side_index, self.position);

        for stat in stats {
            let value = battle.get_pokemon_stat(pokemon_pos, stat, unboosted, unmodified);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        best_stat
    }
}
