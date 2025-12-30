use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Cure status condition
    // TypeScript source:
    // /** Unlike clearStatus, gives cure message */
    // 	cureStatus(silent = false) {
    // 		if (!this.hp || !this.status) return false;
    // 		this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
    // 		if (this.status === 'slp' && this.removeVolatile('nightmare')) {
    // 			this.battle.add('-end', this, 'Nightmare', '[silent]');
    // 		}
    // 		this.setStatus('');
    // 		return true;
    // 	}
    //
    pub fn cure_status(&mut self) -> bool {
        if self.status.is_empty() {
            return false;
        }
        self.status = ID::empty();
        self.status_state = EffectState::new(ID::empty());
        true
    }
}
