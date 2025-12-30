use crate::*;

impl Battle {

    /// Get adjacent foes of a pokemon
    /// Equivalent to Pokemon.adjacentFoes() in pokemon.ts
    //
    // adjacentFoes(): Pokemon[] {
    //     if (this.battle.activePerHalf <= 2) return this.side.foes();
    //     return this.side.foes().filter(foe => this.isAdjacent(foe));
    // }
    //
    pub fn adjacent_foes(&self, side_idx: usize, poke_idx: usize) -> Vec<(usize, usize)> {
        let pokemon_pos = (side_idx, poke_idx);
        let mut result = Vec::new();

        // JS: if (this.battle.activePerHalf <= 2) return this.side.foes();
        // For singles and doubles, all active foes are adjacent
        if self.active_per_half <= 2 {
            // Get foe side(s)
            for (foe_side_idx, foe_side) in self.sides.iter().enumerate() {
                // Skip own side
                if foe_side_idx == side_idx {
                    continue;
                }
                // For non-multi battles, opposing sides are foes
                if self.game_type != GameType::Multi || (foe_side_idx % 2) != (side_idx % 2) {
                    for foe_idx in foe_side.active.iter().flatten() {
                        if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                            if !foe.is_fainted() {
                                result.push((foe_side_idx, *foe_idx));
                            }
                        }
                    }
                }
            }
        } else {
            // For triple battles and beyond, filter by adjacency
            // JS: return this.side.foes().filter(foe => this.isAdjacent(foe));
            for (foe_side_idx, foe_side) in self.sides.iter().enumerate() {
                // Skip own side
                if foe_side_idx == side_idx {
                    continue;
                }
                // For non-multi battles, opposing sides are foes
                if self.game_type != GameType::Multi || (foe_side_idx % 2) != (side_idx % 2) {
                    for foe_idx in foe_side.active.iter().flatten() {
                        let foe_pos = (foe_side_idx, *foe_idx);
                        if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                            if !foe.is_fainted() && self.is_adjacent(pokemon_pos, foe_pos) {
                                result.push(foe_pos);
                            }
                        }
                    }
                }
            }
        }

        result
    }
}
