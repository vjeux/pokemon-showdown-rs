// JS Source:
//
// 	allies(): Pokemon[] {
// 		return this.side.allies().filter(ally => ally !== this);
// 	}
//
// side.allies(all?: boolean) implementation:
// 	allies(all?: boolean) {
// 		// called during the first switch-in, so `active` can still contain nulls at this point
// 		let allies = this.activeTeam().filter(ally => ally);
// 		if (!all) allies = allies.filter(ally => !!ally.hp);
//
// 		return allies;
// 	}
//
// Note: In Rust, Pokemon doesn't have a reference to Battle (borrow checker),
// so we take Battle as a parameter instead of accessing this.battle

use crate::*;

impl Pokemon {
    /// Get allies (not including self)
    /// Equivalent to pokemon.ts allies()
    pub fn allies(&self, battle: &Battle, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let self_pos = (self.side_index, self.position);

        if let Some(side) = battle.sides.get(self.side_index) {
            // Iterate through active slots on this side
            for poke_idx in side.active.iter().flatten() {
                let ally_pos = (self.side_index, *poke_idx);

                // Skip self
                if ally_pos == self_pos {
                    continue;
                }

                if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                    // Include if: include_fainted=true OR pokemon has HP
                    if include_fainted || pokemon.hp > 0 {
                        result.push(ally_pos);
                    }
                }
            }
        }

        result
    }
}
