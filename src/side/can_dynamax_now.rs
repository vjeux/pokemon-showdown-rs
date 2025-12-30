use crate::side::*;

impl Side {

    // ==========================================
    // Methods ported from side.ts
    // ==========================================

    /// String representation of Side
    //
    // 	toString() {
    // 		return `${this.id}: ${this.name}`;
    // 	}
    //
    /// Check if this side can dynamax now (Gen 8)
    //
    // 	canDynamaxNow(): boolean {
    // 		if (this.battle.gen !== 8) return false;
    // 		// In multi battles, players on a team are alternatingly given the option to dynamax each turn
    // 		// On turn 1, the players on their team's respective left have the first chance (p1 and p2)
    // 		if (this.battle.gameType === 'multi' && this.battle.turn % 2 !== [1, 1, 0, 0][this.n]) return false;
    // 		// if (this.battle.gameType === 'multitriples' && this.battle.turn % 3 !== [1, 1, 2, 2, 0, 0][this.side.n]) {
    // 		//		return false;
    // 		// }
    // 		return !this.dynamaxUsed;
    // 	}
    //
    pub fn can_dynamax_now(&self, gen: u8, game_type: &str, turn: i32) -> bool {
        if gen != 8 {
            return false;
        }
        // In multi battles, players alternate turns for dynamaxing
        if game_type == "multi" {
            let allowed_indices = match turn % 2 {
                1 => [0, 1],
                _ => [2, 3],
            };
            if !allowed_indices.contains(&self.n) {
                return false;
            }
        }
        !self.dynamax_used
    }
}
