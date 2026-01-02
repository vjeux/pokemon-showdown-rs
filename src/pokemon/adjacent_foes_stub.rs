// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get adjacent foes
    /// Equivalent to pokemon.ts adjacentFoes()
    ///
    // adjacentFoes(): Pokemon[] {
    //     if (this.battle.activePerHalf <= 2) return this.side.foes();
    //     return this.side.foes().filter(foe => this.isAdjacent(foe));
    // }
    //
    // Note: In Rust, Pokemon doesn't have a reference to Battle (borrow checker),
    // so we take Battle as a parameter instead of accessing this.battle
    pub fn adjacent_foes(&self, battle: &Battle) -> Vec<(usize, usize)> {
        let pokemon_pos = (self.side_index, self.position);
        let mut result = Vec::new();

        // JS: if (this.battle.activePerHalf <= 2) return this.side.foes();
        // For singles and doubles, all active foes are adjacent
        if battle.active_per_half <= 2 {
            // Get foe side(s)
            for (foe_side_idx, foe_side) in battle.sides.iter().enumerate() {
                // Skip own side
                if foe_side_idx == self.side_index {
                    continue;
                }
                // For non-multi battles, opposing sides are foes
                if battle.game_type != GameType::Multi || (foe_side_idx % 2) != (self.side_index % 2) {
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
            for (foe_side_idx, foe_side) in battle.sides.iter().enumerate() {
                // Skip own side
                if foe_side_idx == self.side_index {
                    continue;
                }
                // For non-multi battles, opposing sides are foes
                if battle.game_type != GameType::Multi || (foe_side_idx % 2) != (self.side_index % 2) {
                    for foe_idx in foe_side.active.iter().flatten() {
                        let foe_pos = (foe_side_idx, *foe_idx);
                        if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                            if !foe.is_fainted() && battle.is_adjacent(pokemon_pos, foe_pos) {
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
