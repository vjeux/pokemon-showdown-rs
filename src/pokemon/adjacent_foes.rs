// JS Source:
//
// 	adjacentFoes(): Pokemon[] {
// 		if (this.battle.activePerHalf <= 2) return this.side.foes();
// 		return this.side.foes().filter(foe => this.isAdjacent(foe));
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
// Note: In Rust, Pokemon doesn't have a reference to Battle (borrow checker),
// so we take Battle as a parameter instead of accessing this.battle

use crate::*;

impl Pokemon {
    /// Get adjacent foes
    /// Equivalent to pokemon.ts adjacentFoes()
    pub fn adjacent_foes(&self, battle: &Battle) -> Vec<(usize, usize)> {
        // Get our party index from side.active using our active slot position
        // self.position is the active slot, side.active[slot] gives party index
        let our_party_idx = if let Some(side) = battle.sides.get(self.side_index) {
            if self.position < side.active.len() {
                side.active[self.position].unwrap_or(usize::MAX)
            } else {
                usize::MAX
            }
        } else {
            usize::MAX
        };
        let pokemon_pos = (self.side_index, our_party_idx);
        let mut result = Vec::new();

        // JS: if (this.battle.activePerHalf <= 2) return this.side.foes();
        // For singles and doubles, all active foes are adjacent
        if battle.active_per_half <= 2 {
            // Get all foes using the foes() method
            let source_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return result,
            };
            return source_pokemon.foes(battle, false);
        } else {
            // For triple battles and beyond, filter by adjacency
            // JS: return this.side.foes().filter(foe => this.isAdjacent(foe));
            let source_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return result,
            };
            let all_foes = source_pokemon.foes(battle, false);

            for foe_pos in all_foes {
                if battle.is_adjacent(pokemon_pos, foe_pos) {
                    result.push(foe_pos);
                }
            }
        }

        result
    }
}
