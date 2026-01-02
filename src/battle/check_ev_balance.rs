// 1:1 port of checkEVBalance from battle.ts

use crate::*;

impl Battle {
    /// Check if teams have balanced EVs
    /// Equivalent to battle.ts checkEVBalance()
    ///
    // JS Source:
    // 	checkEVBalance() {
    // 		let limitedEVs: boolean | null = null;
    // 		for (const side of this.sides) {
    // 			const sideLimitedEVs = !side.pokemon.some(
    // 				pokemon => Object.values(pokemon.set.evs).reduce((a, b) => a + b, 0) > 510
    // 			);
    // 			if (limitedEVs === null) {
    // 				limitedEVs = sideLimitedEVs;
    // 			} else if (limitedEVs !== sideLimitedEVs) {
    // 				this.add('bigerror', "Warning: One player isn't adhering to a 510 EV limit, and the other player is.");
    // 			}
    // 		}
    // 	}
    pub fn check_ev_balance(&mut self) {
        // JS: let limitedEVs: boolean | null = null;
        let mut limited_evs: Option<bool> = None;
        let mut needs_warning = false;

        // JS: for (const side of this.sides)
        for side in &self.sides {
            // JS: const sideLimitedEVs = !side.pokemon.some(
            //         pokemon => Object.values(pokemon.set.evs).reduce((a, b) => a + b, 0) > 510
            //     );
            // Check if ANY pokemon has total EVs > 510
            let side_limited_evs = !side.pokemon.iter().any(|pokemon| {
                let total_evs = pokemon.set.evs.hp
                    + pokemon.set.evs.atk
                    + pokemon.set.evs.def
                    + pokemon.set.evs.spa
                    + pokemon.set.evs.spd
                    + pokemon.set.evs.spe;
                total_evs > 510
            });

            // JS: if (limitedEVs === null)
            if limited_evs.is_none() {
                // JS: limitedEVs = sideLimitedEVs;
                limited_evs = Some(side_limited_evs);
            } else if limited_evs != Some(side_limited_evs) {
                // JS: this.add('bigerror', "Warning: One player isn't adhering to a 510 EV limit, and the other player is.");
                needs_warning = true;
            }
        }

        if needs_warning {
            self.add("bigerror", &[Arg::Str("Warning: One player isn't adhering to a 510 EV limit, and the other player is.")]);
        }
    }
}

