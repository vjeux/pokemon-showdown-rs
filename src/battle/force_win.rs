use crate::*;

impl Battle {

    /// Force a win for a specific side
    /// Equivalent to battle.ts forceWin()
    //
    // 	forceWin(side: SideID | null = null) {
    // 		if (this.ended) return false;
    // 		this.inputLog.push(side ? `>forcewin ${side}` : `>forcetie`);
    // 		return this.win(side);
    // 	}
    //
    pub fn force_win(&mut self, side: Option<SideID>) -> bool {
        if self.ended {
            return false;
        }
        // Log to inputLog (if we had inputLog field, would log here)
        // JavaScript: this.inputLog.push(side ? `>forcewin ${side}` : `>forcetie`);
        self.win(side)
    }
}
