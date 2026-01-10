// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Get Pokemon at a location relative to a source Pokemon
    /// Helper for get_target
    /// Rust helper method - JavaScript uses pokemon.getAtLoc(targetLoc) directly on Pokemon object
    /// This method implements the logic as a Battle method since Pokemon doesn't have access to Battle state
    /// Returns (side_index, pokemon_index) tuple if Pokemon exists at location, None otherwise
    ///
    /// Convention (matches JavaScript getLocOf):
    /// - Positive target_loc = foe side (e.g., +1 is foe at slot 0)
    /// - Negative target_loc = own side (e.g., -1 is ally at slot 0)
    pub fn get_at_loc(&self, source: (usize, usize), target_loc: i8) -> Option<(usize, usize)> {
        let (source_side, _source_idx) = source;

        if target_loc == 0 {
            return None;
        }

        let (target_side, slot) = if target_loc > 0 {
            // Foe side (positive) - opponents
            let foe_side = if source_side == 0 { 1 } else { 0 };
            (foe_side, (target_loc - 1) as usize)
        } else {
            // Own side (negative) - allies
            (source_side, (-target_loc - 1) as usize)
        };

        if slot >= self.active_per_half {
            return None;
        }

        // Get active Pokemon at slot
        if let Some(side) = self.sides.get(target_side) {
            if let Some(Some(poke_idx)) = side.active.get(slot) {
                return Some((target_side, *poke_idx));
            }
        }

        None
    }
}
