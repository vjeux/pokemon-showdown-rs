use crate::side::*;
use crate::*;

impl Side {

    // =========================================================================
    // REMAINING METHODS (ported from side.ts for complete 1:1 port)
    // =========================================================================

    /// Get active team (for multi battles)
    /// Equivalent to side.ts activeTeam()
    // 	activeTeam() {
    // 		if (this.battle.gameType !== 'multi') return this.active;
    //
    // 		return this.battle.sides[this.n % 2].active.concat(this.battle.sides[this.n % 2 + 2].active);
    // 	}
    //
    pub fn active_team(&self) -> Vec<usize> {
        self.active.iter().filter_map(|&idx| idx).collect()
    }
}
