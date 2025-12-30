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
        // JavaScript: this.inputLog.push(side ? `>forcewin ${side}` : `>forcetie`);
        let log_entry = if let Some(side_id) = side {
            format!(">forcewin {}", side_id.to_str())
        } else {
            ">forcetie".to_string()
        };
        self.input_log.push(log_entry);

        self.win(side)
    }
}
