// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Check if two pokemon are adjacent
    /// Equivalent to Pokemon.isAdjacent() in pokemon.ts
    //
    // isAdjacent(pokemon2: Pokemon) {
    //     if (this.fainted || pokemon2.fainted) return false;
    //     if (this.battle.activePerHalf <= 2) return this !== pokemon2;
    //     if (this.side === pokemon2.side) return Math.abs(this.position - pokemon2.position) === 1;
    //     return Math.abs(this.position + pokemon2.position + 1 - this.side.active.length) <= 1;
    // }
    //
    pub fn is_adjacent(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        let (side1, idx1) = pos1;
        let (side2, idx2) = pos2;

        // Get both pokemon
        let poke1 = match self.sides.get(side1).and_then(|s| s.pokemon.get(idx1)) {
            Some(p) => p,
            None => return false,
        };
        let poke2 = match self.sides.get(side2).and_then(|s| s.pokemon.get(idx2)) {
            Some(p) => p,
            None => return false,
        };

        // JS: if (this.fainted || pokemon2.fainted) return false;
        if poke1.is_fainted() || poke2.is_fainted() {
            return false;
        }

        // JS: if (this.battle.activePerHalf <= 2) return this !== pokemon2;
        if self.active_per_half <= 2 {
            return pos1 != pos2;
        }

        // JS: if (this.side === pokemon2.side) return Math.abs(this.position - pokemon2.position) === 1;
        if side1 == side2 {
            let pos_diff = (poke1.position as i32 - poke2.position as i32).abs();
            return pos_diff == 1;
        }

        // JS: return Math.abs(this.position + pokemon2.position + 1 - this.side.active.length) <= 1;
        let active_length = self.sides.get(side1).map(|s| s.active.len()).unwrap_or(0);
        let sum = poke1.position as i32 + poke2.position as i32 + 1 - active_length as i32;
        sum.abs() <= 1
    }
}
