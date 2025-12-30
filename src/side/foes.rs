use crate::side::*;

impl Side {

    /// Get all foes
    /// Equivalent to side.ts foes()
    // 	foes(all?: boolean) {
    // 		if (this.battle.gameType === 'freeforall') {
    // 			return this.battle.sides.map(side => side.active[0])
    // 				.filter(pokemon => pokemon && pokemon.side !== this && (all || !!pokemon.hp));
    // 		}
    // 		return this.foe.allies(all);
    // 	}
    //
    pub fn foes<'a>(&self, battle_sides: &'a [Side]) -> Vec<&'a Pokemon> {
        // Return all active Pokemon from opponent sides
        let mut foes = Vec::new();
        for (idx, side) in battle_sides.iter().enumerate() {
            if idx != self.n {
                for &active_idx in &side.active {
                    if let Some(poke_idx) = active_idx {
                        if let Some(pokemon) = side.pokemon.get(poke_idx) {
                            if !pokemon.fainted {
                                foes.push(pokemon);
                            }
                        }
                    }
                }
            }
        }
        foes
    }
}
