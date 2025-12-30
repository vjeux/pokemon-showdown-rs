use crate::*;

impl Battle {

    /// Check if teams have balanced EVs
    /// Equivalent to battle.ts checkEVBalance()
    ///
    //
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
    //
    pub fn check_ev_balance(&mut self) {
        let mut limited_evs: Option<bool> = None;
        let mut needs_warning = false;

        for side in &self.sides {
            // Check if any Pokemon on this side has >510 total EVs
            let side_limited_evs = !side.team.iter().any(|set| {
                let total_evs = set.evs.hp
                    + set.evs.atk
                    + set.evs.def
                    + set.evs.spa
                    + set.evs.spd
                    + set.evs.spe;
                total_evs > 510
            });

            if let Some(limited) = limited_evs {
                if limited != side_limited_evs {
                    needs_warning = true;
                }
            } else {
                limited_evs = Some(side_limited_evs);
            }
        }

        if needs_warning {
            self.add("bigerror", &[Arg::Str("Warning: One player isn't adhering to a 510 EV limit, and the other player is.")]);
        }
    }
}
