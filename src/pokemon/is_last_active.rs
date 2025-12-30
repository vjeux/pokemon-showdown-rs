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
        // This would need access to the side to properly implement
        // For now, just return true if active
        self.is_active
    }
}
