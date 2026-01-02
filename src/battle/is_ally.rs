// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Check if two pokemon are allies
    pub fn is_ally(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        // JS: return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);
        if pos1.0 == pos2.0 {
            return true;
        }
        // Check if pos2's side is an ally of pos1's side (for multi battles)
        if let Some(side1) = self.sides.get(pos1.0) {
            if let Some(ally_idx) = side1.ally_index {
                if ally_idx == pos2.0 {
                    return true;
                }
            }
        }
        false
    }
}
