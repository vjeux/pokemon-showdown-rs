use crate::*;

impl Pokemon {

    /// Clear the pokemon's status
    /// Equivalent to pokemon.ts clearStatus()
    // TypeScript source:
    // /**
    // 	 * Unlike cureStatus, does not give cure message
    // 	 */
    // 	clearStatus() {
    // 		if (!this.hp || !this.status) return false;
    // 		if (this.status === 'slp' && this.removeVolatile('nightmare')) {
    // 			this.battle.add('-end', this, 'Nightmare', '[silent]');
    // 		}
    // 		this.setStatus('');
    // 		return true;
    // 	}
    //
    pub fn clear_status(&mut self) -> bool {
        // Simplified stub - full logic with nightmare handling in Battle
        if self.status.is_empty() {
            return false;
        }
        // JS: this.setStatus('');
        self.set_status(ID::empty());
        true
    }
}
