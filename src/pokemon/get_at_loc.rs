use crate::*;

impl Pokemon {

    /// Get Pokemon at a location relative to this one
    /// Returns (side_index, position)
    /// Equivalent to getAtLoc in pokemon.ts
    //
    // 	getAtLoc(targetLoc: number) {
    // 		let side = this.battle.sides[targetLoc < 0 ? this.side.n % 2 : (this.side.n + 1) % 2];
    // 		targetLoc = Math.abs(targetLoc);
    // 		if (targetLoc > side.active.length) {
    // 			targetLoc -= side.active.length;
    // 			side = this.battle.sides[side.n + 2];
    // 		}
    // 		return side.active[targetLoc - 1];
    // 	}
    //
    /// Convention (matches JavaScript getLocOf):
    /// - Positive target_loc = foe side (opponents)
    /// - Negative target_loc = own side (allies)
    pub fn get_at_loc(&self, target_loc: i8, active_per_half: usize) -> Option<(usize, usize)> {
        if target_loc == 0 {
            return None;
        }

        if target_loc > 0 {
            // Foe side (positive) - opponents
            let foe_side = if self.side_index == 0 { 1 } else { 0 };
            let pos = (target_loc - 1) as usize;
            if pos < active_per_half {
                Some((foe_side, pos))
            } else {
                None
            }
        } else {
            // Own side (negative) - allies
            let pos = (-target_loc - 1) as usize;
            if pos < active_per_half {
                Some((self.side_index, pos))
            } else {
                None
            }
        }
    }
}
