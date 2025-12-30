use crate::*;

impl Battle {

    /// Get total Pokemon left for all foes of a side
    /// Rust helper - JavaScript calls side.foePokemonLeft() directly
    /// This helper calculates foe Pokemon count from the Battle level
    pub fn get_foe_pokemon_left(&self, side_idx: usize) -> usize {
        match self.game_type {
            GameType::FreeForAll => {
                // In FFA, all other sides are foes
                self.sides
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != side_idx)
                    .map(|(_, s)| s.pokemon_left)
                    .sum()
            }
            GameType::Multi => {
                // In multi battles, the opposing team is the foe
                // P1/P3 are a team, P2/P4 are a team
                if side_idx == 0 || side_idx == 2 {
                    // Foes are P2 and P4
                    self.sides.get(1).map(|s| s.pokemon_left).unwrap_or(0)
                        + self.sides.get(3).map(|s| s.pokemon_left).unwrap_or(0)
                } else {
                    // Foes are P1 and P3
                    self.sides.first().map(|s| s.pokemon_left).unwrap_or(0)
                        + self.sides.get(2).map(|s| s.pokemon_left).unwrap_or(0)
                }
            }
            _ => {
                // Singles or doubles - foe is the opposite side
                let foe_idx = if side_idx == 0 { 1 } else { 0 };
                self.sides.get(foe_idx).map(|s| s.pokemon_left).unwrap_or(0)
            }
        }
    }
}
