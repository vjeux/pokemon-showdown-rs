use crate::*;

impl Battle {

    /// Get all foes
    /// Equivalent to pokemon.ts foes() which calls side.foes()
    ///
    // 	foes(all?: boolean): Pokemon[] {
    // 		return this.side.foes(all);
    // 	}
    //
    // side.foes(all?: boolean) implementation:
    // 	foes(all?: boolean) {
    // 		if (this.battle.gameType === 'freeforall') {
    // 			return this.battle.sides.map(side => side.active[0])
    // 				.filter(pokemon => pokemon && pokemon.side !== this && (all || !!pokemon.hp));
    // 		}
    // 		return this.foe.allies(all);
    // 	}
    //
    pub fn foes(&self, side_idx: usize, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        // JS: if (this.battle.gameType === 'freeforall') {
        if self.game_type == GameType::FreeForAll {
            // Return all active pokemon from other sides
            for (foe_side_idx, foe_side) in self.sides.iter().enumerate() {
                if foe_side_idx != side_idx {
                    if let Some(Some(foe_idx)) = foe_side.active.first() {
                        if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                            if include_fainted || foe.hp > 0 {
                                result.push((foe_side_idx, *foe_idx));
                            }
                        }
                    }
                }
            }
        } else {
            // JS: return this.foe.allies(all);
            // Get foe side index
            let foe_side_idx = if side_idx == 0 { 1 } else { 0 };
            // Return allies from foe side
            result = self.allies_and_self(foe_side_idx, include_fainted);
        }

        result
    }
}
