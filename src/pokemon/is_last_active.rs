use crate::*;

impl Pokemon {

    /// Check if this is the last active Pokemon on the side
    //
    // 	isLastActive() {
    // 		if (!this.isActive) return false;
    // 		const allyActive = this.side.active;
    // 		for (let i = this.position + 1; i < allyActive.length; i++) {
    // 			if (allyActive[i] && !allyActive[i].fainted) return false;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn is_last_active(&self) -> bool {
        // JS: if (!this.isActive) return false;
        if !self.is_active {
            return false;
        }

        // JS: const allyActive = this.side.active;
        // JS: for (let i = this.position + 1; i < allyActive.length; i++) {
        // JS:     if (allyActive[i] && !allyActive[i].fainted) return false;
        // JS: }
        // JS: return true;
        //
        // Note: Missing check for other active Pokemon on same side
        // Would need Battle reference to access side.active array
        // Currently just returns is_active which is incorrect
        // Full implementation: loop through side.active from position+1 to end,
        // return false if any non-fainted Pokemon found
        self.is_active
    }
}
