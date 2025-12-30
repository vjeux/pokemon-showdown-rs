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
        if !self.status.is_empty() {
            self.status = ID::empty();
            self.status_state = crate::event_system::EffectState::new(ID::empty());
            true
        } else {
            false
        }
    }
}
