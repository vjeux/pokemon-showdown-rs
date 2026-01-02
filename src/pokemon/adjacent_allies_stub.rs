// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get adjacent allies
    /// Equivalent to pokemon.ts adjacentAllies()
    ///
    // adjacentAllies(): Pokemon[] {
    //     return this.side.allies().filter(ally => this.isAdjacent(ally));
    // }
    //
    // Note: In Rust, Pokemon doesn't have a reference to Battle (borrow checker),
    // so we take Battle as a parameter instead of accessing this.battle
    pub fn adjacent_allies(&self, battle: &Battle) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let pokemon_pos = (self.side_index, self.position);

        // JS: this.side.allies()
        // Get allies from the same side (active team)
        if let Some(side) = battle.sides.get(self.side_index) {
            for ally_idx in side.active.iter().flatten() {
                let ally_pos = (self.side_index, *ally_idx);
                // Skip if it's the same pokemon
                if ally_pos == pokemon_pos {
                    continue;
                }
                // JS: .filter(ally => this.isAdjacent(ally))
                if let Some(ally) = side.pokemon.get(*ally_idx) {
                    if !ally.is_fainted() && battle.is_adjacent(pokemon_pos, ally_pos) {
                        result.push(ally_pos);
                    }
                }
            }

            // For multi battles, also check ally side
            if battle.game_type == GameType::Multi {
                let ally_side_idx = if self.side_index < 2 {
                    self.side_index + 2
                } else {
                    self.side_index - 2
                };

                if let Some(ally_side) = battle.sides.get(ally_side_idx) {
                    for ally_idx in ally_side.active.iter().flatten() {
                        let ally_pos = (ally_side_idx, *ally_idx);
                        if let Some(ally) = ally_side.pokemon.get(*ally_idx) {
                            if !ally.is_fainted() && battle.is_adjacent(pokemon_pos, ally_pos) {
                                result.push(ally_pos);
                            }
                        }
                    }
                }
            }
        }

        result
    }
}
