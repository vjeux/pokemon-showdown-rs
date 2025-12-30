use crate::*;

impl Battle {

    /// Check all active Pokemon for fainting and update their status
    /// Equivalent to battle.ts checkFainted() (battle.ts:2487-2496)
    ///
    //
    // 	checkFainted() {
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon.fainted) {
    // 					pokemon.status = 'fnt' as ID;
    // 					pokemon.switchFlag = true;
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn check_fainted(&mut self) {
        for side in &mut self.sides {
            for pokemon in &mut side.pokemon {
                if pokemon.is_active && pokemon.fainted {
                    pokemon.status = ID::new("fnt");
                    pokemon.switch_flag = true;
                }
            }
        }
    }
}
