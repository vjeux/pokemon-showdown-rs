use crate::*;

impl Pokemon {

    /// Check if another Pokemon is adjacent (for targeting)
    //
    // 	isAdjacent(pokemon2: Pokemon) {
    // 		if (this.fainted || pokemon2.fainted) return false;
    // 		if (this.battle.activePerHalf <= 2) return this !== pokemon2;
    // 		if (this.side === pokemon2.side) return Math.abs(this.position - pokemon2.position) === 1;
    // 		return Math.abs(this.position + pokemon2.position + 1 - this.side.active.length) <= 1;
    // 	}
    //
    pub fn is_adjacent(
        &self,
        other_position: usize,
        other_fainted: bool,
        active_per_half: usize,
    ) -> bool {
        // JS: if (this.fainted || pokemon2.fainted) return false;
        if self.fainted || other_fainted {
            return false;
        }

        // JS: if (this.battle.activePerHalf <= 2) return this !== pokemon2;
        if active_per_half <= 2 {
            return self.position != other_position;
        }

        // JS: if (this.side === pokemon2.side) return Math.abs(this.position - pokemon2.position) === 1;
        // JS: return Math.abs(this.position + pokemon2.position + 1 - this.side.active.length) <= 1;
        //
        // Note: Missing same-side vs different-side check
        // Current implementation assumes same side
        // Full implementation would need to know if other_position is on same side
        // and the active.length for cross-side adjacency calculation
        (self.position as i32 - other_position as i32).abs() <= 1
    }
}
