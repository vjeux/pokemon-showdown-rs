use crate::*;

impl Pokemon {

    /// Get location of a target Pokemon relative to this one
    /// Equivalent to getLocOf in pokemon.ts
    // TypeScript source:
    // /**
    // 	 * Returns a relative location: 1-3, positive for foe, and negative for ally.
    // 	 * Use `getAtLoc` to reverse.
    // 	 */
    // 	getLocOf(target: Pokemon) {
    // 		const positionOffset = Math.floor(target.side.n / 2) * target.side.active.length;
    // 		const position = target.position + positionOffset + 1;
    // 		const sameHalf = (this.side.n % 2) === (target.side.n % 2);
    // 		return sameHalf ? -position : position;
    // 	}
    //
    pub fn get_loc_of(
        &self,
        target_side_index: usize,
        target_position: usize,
        active_per_half: usize,
    ) -> i8 {
        if self.side_index == target_side_index {
            // Same side - positive numbers
            (target_position as i8 - self.position as i8 + 1).max(-(active_per_half as i8))
        } else {
            // Opposing side - negative numbers
            -(target_position as i8 + 1)
        }
    }
}
