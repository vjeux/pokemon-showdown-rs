// JS Source:
// 	getOverflowedTurnCount(): number {
// 		return this.gen >= 8 ? (this.turn - 1) % 256 : this.turn - 1;
// 	}


use crate::*;

impl Battle {

    /// Get overflowed turn count (for endless battle detection and Gen 8+ move timing)
    /// Equivalent to battle.ts getOverflowedTurnCount() (battle.ts:3317-3319)
    /// Used by Wish, Future Sight, and other delayed moves
    // TypeScript source:
    // /**
    // 	 * Currently, we treat Team Preview as turn 0, but the games start counting their turns at turn 0
    // 	 * There is also overflow that occurs in Gen 8+ that affects moves like Wish / Future Sight
    // 	 * https://www.smogon.com/forums/threads/10352797
    // 	 */
    // 	getOverflowedTurnCount(): number {
    // 		return this.gen >= 8 ? (this.turn - 1) % 256 : this.turn - 1;
    // 	}
    //
    pub fn get_overflowed_turn_count(&self) -> i32 {
        // JavaScript: return this.gen >= 8 ? (this.turn - 1) % 256 : this.turn - 1;
        if self.gen >= 8 {
            (self.turn.saturating_sub(1)) % 256
        } else {
            self.turn.saturating_sub(1)
        }
    }
}
