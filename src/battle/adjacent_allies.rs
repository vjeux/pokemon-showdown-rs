use crate::*;

impl Battle {

    /// Get adjacent allies of a pokemon
    /// Equivalent to Pokemon.adjacentAllies() in pokemon.ts
    //
    // adjacentAllies(): Pokemon[] {
    //     return this.side.allies().filter(ally => this.isAdjacent(ally));
    // }
    //
    pub fn adjacent_allies(&self, side_idx: usize, poke_idx: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let pokemon_pos = (side_idx, poke_idx);

        // Get allies from the same side (active team)
        if let Some(side) = self.sides.get(side_idx) {
            for ally_idx in side.active.iter().flatten() {
                let ally_pos = (side_idx, *ally_idx);
                // Skip if it's the same pokemon
                if ally_pos == pokemon_pos {
                    continue;
                }
                // Check if alive
                if let Some(ally) = side.pokemon.get(*ally_idx) {
                    if !ally.is_fainted() && self.is_adjacent(pokemon_pos, ally_pos) {
                        result.push(ally_pos);
                    }
                }
            }

            // For multi battles, also check ally side
            // JS: activeTeam() - if (this.battle.gameType !== 'multi') return this.active;
            if self.game_type == GameType::Multi {
                // Get the ally side (n % 2 + 2 for sides beyond first two)
                let ally_side_idx = if side_idx < 2 {
                    side_idx + 2
                } else {
                    side_idx - 2
                };

                if let Some(ally_side) = self.sides.get(ally_side_idx) {
                    for ally_idx in ally_side.active.iter().flatten() {
                        let ally_pos = (ally_side_idx, *ally_idx);
                        if let Some(ally) = ally_side.pokemon.get(*ally_idx) {
                            if !ally.is_fainted() && self.is_adjacent(pokemon_pos, ally_pos) {
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
